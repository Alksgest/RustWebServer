// use crate::multithreading::thread_pool::ThreadPool;
use crate::server::api_settings::ApiSettings;
use crate::server::controller::ControllerBase;
use crate::server::response_wrapper::response_wrapper::bad_request;
use crate::server::response_wrapper::response_wrapper::method_not_allowed;
use crate::server::uri_parser::UriParser;
// use core::ops::Deref;
// use std::ops::DerefMut;
use std::io::prelude::*;
// use std::marker::PhantomData;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct ApiServer {
    listener: TcpListener,
    // thread_pool: ThreadPool,
    settings: ApiSettings,

    controllers: Vec<Box<dyn ControllerBase>>,
    // phantom: PhantomData<&'a u8>,
}

impl ApiServer {
    // -> ApiServer<'static>
    pub fn new(settings: ApiSettings) -> ApiServer {
        let url = settings.create_url();

        let listener = match TcpListener::bind(&url) {
            Ok(val) => {
                println!("Successfully binded to url {}", &url);
                val
            }
            Err(e) => {
                panic!("Cannot bind tcp listenter to url {} with error {}", &url, e)
            }
        };

        // let thread_pool = ThreadPool::new(4);
        ApiServer {
            listener,
            settings,
            // thread_pool,
            // phantom: PhantomData,
            controllers: vec![],
        }
    }

    // &'static self
    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(val) => val,
                _ => {
                    println!("Erorr while creating stream!");
                    continue;
                }
            };
            self.handle_connection(stream);
            // self.thread_pool.execute(move || {
            //     self.handle_connection(stream);
            // });
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = vec![0; self.settings.buffer_size];
        stream.read(&mut buffer).unwrap();
        if ApiServer::is_fave_icon(&buffer) {
            // or add some processing here
            return;
        }
        // self.process_middleware();
        self.process_contorollers(&buffer, &stream);
        // ApiServer::test_handler(&buffer, stream);
    }

    async fn process_contorollers(&self, buffer: &Vec<u8>, mut stream: &TcpStream) {
        let headers = buffer.lines();
        let header: Vec<_> = headers.take(1).collect();
        let header = match header.get(0) {
            Some(val) => val,
            None => {
                println!("There are no headers in request?");
                return;
            }
        };
        let header = header.as_ref().unwrap();

        let parsed_uri = &UriParser::parse_header(header.to_string());

        println!("{:?}", parsed_uri);

        let controller = self
            .controllers
            .iter()
            .find(|el| el.rout() == parsed_uri.route());
        let response = match controller {
            Some(val) => {
                let parsed_uri_option = Option::from(parsed_uri.clone());
                let response = match parsed_uri.rest_method().as_ref() {
                    "GET" => {
                        // println!("processing get request for url: {}", parsed_uri.route());
                        val.get(&parsed_uri_option)
                    }
                    "POST" => val.post(&parsed_uri_option),
                    "PUT" => val.put(&parsed_uri_option),
                    "UPDATE" => val.update(&parsed_uri_option),
                    "DELETE" => val.delete(&parsed_uri_option),
                    _ => method_not_allowed(None),
                };
                response
            }
            _ => bad_request(None),
        };
        println!("\n{}\n", response);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn register_middleware(&self) -> &Self {
        self
    }

    pub fn register_controller(&mut self, controller: Box<dyn ControllerBase>) -> &Self {
        self.controllers.push(controller);
        self
    }

    pub fn register_endpoint(&self) -> &Self {
        self
    }

    fn is_fave_icon(buffer: &Vec<u8>) -> bool {
        buffer.starts_with(b"GET /favicon.ico HTTP/1.1\r\n")
    }
}

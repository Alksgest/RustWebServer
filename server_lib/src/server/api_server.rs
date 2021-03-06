use crate::multithreading::thread_pool::ThreadPool;
use crate::server::api_settings::ApiSettings;
use crate::server::controller::ControllerBase;
use crate::server::response_wrapper::response_wrapper::bad_request;
use crate::server::response_wrapper::response_wrapper::method_not_allowed;
use crate::server::request_parser::RequestParser;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct ApiServer {
    listener: TcpListener,
    thread_pool: ThreadPool,
    settings: ApiSettings,

    controllers: Vec<Box<dyn ControllerBase>>,
}

unsafe impl Sync for ApiServer {}

impl ApiServer {
    pub fn new(settings: ApiSettings, controllers: Vec<Box<dyn ControllerBase>>) -> ApiServer {
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

        let thread_pool = ThreadPool::new(4);
        ApiServer {
            listener,
            settings,
            thread_pool,
            controllers,
        }
    }

    // &'static self
    pub fn start(&'static self) {
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(val) => val,
                _ => {
                    println!("Erorr while creating stream!");
                    continue;
                }
            };
            self.thread_pool.execute(move || {
                self.handle_connection(stream);
            });
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
    }

    fn process_contorollers(&self, buffer: &Vec<u8>, mut stream: &TcpStream) {
        let parsed_uri = &RequestParser::parse(buffer);

        let controller = self.controllers.iter().find(move |el| {
            let uri_route = &parsed_uri.route();
            let route = el.route();
            let slice: &str = route.as_ref();
            uri_route.starts_with(slice)
        });
        let response = match controller {
            Some(val) => {
                let parsed_uri_option = Option::from(parsed_uri.clone());
                let response = match parsed_uri.rest_method().as_ref() {
                    "GET" => val.get(&parsed_uri_option),
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

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn is_fave_icon(buffer: &Vec<u8>) -> bool {
        buffer.starts_with(b"GET /favicon.ico HTTP/1.1\r\n")
    }
}

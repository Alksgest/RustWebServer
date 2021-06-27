use crate::multithreading::thread_pool::ThreadPool;
use crate::server::api_settings::ApiSettings;

// use std::fs;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct ApiServer<'a> {
    listener: TcpListener,
    thread_pool: ThreadPool,
    settings: ApiSettings,

    phantom: PhantomData<&'a u8>,
}

impl ApiServer<'_> {
    pub fn new(settings: ApiSettings) -> ApiServer<'static> {
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
            phantom: PhantomData,
        }
    }

    pub fn start(&'static self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            self.thread_pool.execute(move || {
                self.handle_connection(stream);
            });
        }
    }

    pub fn register_middleware(&self) -> &Self {
        self
    }

    pub fn register_controller(&self) -> &Self {
        self
    }

    pub fn register_endpoint(&self) -> &Self {
        self
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = vec![0; self.settings.buffer_size];
        match stream.read(&mut buffer) {
            Ok(val) => val,
            Err(e) => {
                println!("Error while reading data from request: {}", e);
                0
            }
        };

        ApiServer::test_handler(&buffer, stream);
    }

    fn test_handler(buffer: &Vec<u8>, mut stream: TcpStream) {
        let test_get = b"GET /test HTTP/1.1\r\n";

        if buffer.starts_with(test_get) {
            let status_line = "HTTP/1.1 200 OK";
            let content = "Test string";
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

unsafe impl Sync for ApiServer<'_> {}

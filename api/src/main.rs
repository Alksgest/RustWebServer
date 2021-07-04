extern crate lazy_static;
extern crate server_lib;

use serde::{Deserialize, Serialize};
use server_lib::mongo::repository::MongoModel;
use server_lib::server::api_server::ApiServer;
use server_lib::server::api_settings::ApiSettings;
use server_lib::server::controller::ControllerBase;
use server_lib::server::response_wrapper::response_wrapper::success;
use server_lib::server::uri_parser::ParsedUri;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    age: u32,
}

impl MongoModel for User {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

struct TestController {}

impl ControllerBase for TestController {
    fn rout(&self) -> std::string::String {
        "/test".to_string()
    }
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        return success(Some(opt.as_ref().unwrap().route()));
    }
    fn post(&self, opt: &Option<ParsedUri> ) -> String {
        
    }
}

//#[macro_use]
// lazy_static! {
//     static ref SERVER: ApiServer<'static> =
//         ApiServer::new(ApiSettings::new("127.0.0.1", "4444", 2048));
// }
// SERVER
//     .register_controller(Box::new(TestController {}))
//     .start();

fn main() {
    let mut server = ApiServer::new(ApiSettings::new("127.0.0.1", "4444", 2048));
    server
        .register_controller(Box::new(TestController {}))
        .start();
}

extern crate lazy_static;
extern crate mut_static;
extern crate server_lib;

use server_lib::server::api_server::ApiServer;
use server_lib::server::api_settings::ApiSettings;
use server_lib::server::controller::ControllerBase;
use server_lib::server::response_wrapper::response_wrapper::success;
use server_lib::server::uri_parser::ParsedUri;

struct TestController {}

impl ControllerBase for TestController {
    fn rout(&self) -> std::string::String {
        "/test".to_string()
    }
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        // let ten_millis = time::Duration::from_secs(10);
        // thread::sleep(ten_millis);

        return success(Some(opt.as_ref().unwrap().route()));
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

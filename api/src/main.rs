extern crate lazy_static;
extern crate server_lib;

mod controllers;

use crate::controllers::user_controller::UserController;
use server_lib::server::api_server::ApiServer;
use server_lib::server::api_settings::ApiSettings;

fn main() {
    let mut server = ApiServer::new(ApiSettings::new("127.0.0.1", "4444", 2048));
    server
        .register_controller(Box::new(UserController::new(
            "mongodb://localhost:27017",
            "mydb",
            "users",
        )))
        .start();
}

//#[macro_use]
// lazy_static! {
//     static ref SERVER: ApiServer<'static> =
//         ApiServer::new(ApiSettings::new("127.0.0.1", "4444", 2048));
// }
// SERVER
//     .register_controller(Box::new(TestController {}))
//     .start();

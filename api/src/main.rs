#[macro_use]
extern crate lazy_static;
extern crate server_lib;

mod controllers;

use crate::controllers::user_controller::UserController;
use server_lib::server::api_server::ApiServer;
use server_lib::server::api_settings::ApiSettings;

lazy_static! {
    static ref SERVER: ApiServer = ApiServer::new(
        ApiSettings::new("127.0.0.1", "4444", 2048),
        vec![Box::new(UserController::new(
            "mongodb://localhost:27017",
            "mydb",
            "users",
        ))]
    );
}

fn main() {
    SERVER.start();
}

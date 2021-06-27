#[macro_use]
extern crate lazy_static;
extern crate server_lib;

use server_lib::server::api_server::ApiServer;
use server_lib::server::api_settings::ApiSettings;

fn main() {
    lazy_static! {
        static ref SERVER: ApiServer<'static> =
            ApiServer::new(ApiSettings::new("127.0.0.1", "4444", 2048));
    }

    SERVER.start();
}

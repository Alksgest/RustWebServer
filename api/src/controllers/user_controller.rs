use business::services::user_service::UserService;
use server_lib::server::controller::ControllerBase;
use server_lib::server::request_parser::ParsedRequest;

pub struct UserController {
    user_service: UserService,
}

impl<'a> UserController {
    const ROUTE: &'a str = "/user";

    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let user_service: UserService =
            UserService::new(mongo_host, collection_db, collection_name);
        UserController { user_service }
    }
}

impl<'a> ControllerBase for UserController {
    fn route(&self) -> std::string::String {
        UserController::ROUTE.to_string()
    }
    fn get(&self, opt: &Option<ParsedRequest>) -> String {
        let option: &ParsedRequest = opt.as_ref().unwrap();
        let route = option.route();
        if route == UserController::ROUTE.to_string() && option.query_params_len() == 0 {
            return self.user_service.get_list();
        } else {
            let route = option.route();
            let vec: Vec<&str> = route.split("/").collect();

            //TODO: improve this logic. Does not work correctly
            if vec.len() > 2 {
                return self.user_service.get_by_id(opt);
            } else {
                return self.user_service.get_by_params(opt);
            }
        }
    }

    fn post(&self, opt: &Option<ParsedRequest>) -> String {
        self.user_service.create(opt)
    }
    fn put(&self, _: &Option<ParsedRequest>) -> String {
        todo!()
    }
    fn update(&self, _: &Option<ParsedRequest>) -> String {
        todo!()
    }
    fn delete(&self, _: &Option<ParsedRequest>) -> String {
        todo!()
    }
}

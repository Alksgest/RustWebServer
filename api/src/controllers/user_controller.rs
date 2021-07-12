use domain::models::user::User;
use domain::repositories::user_repository::UserRepository;
use serde_json::json;
use server_lib::server::controller::ControllerBase;
use server_lib::server::request_parser::ParsedRequest;
use server_lib::server::response_wrapper::response_wrapper::method_not_allowed;
use server_lib::server::response_wrapper::response_wrapper::not_found;
use server_lib::server::response_wrapper::response_wrapper::success;

pub struct UserController {
    repo: UserRepository,
}

impl<'a> UserController {
    const ROUTE: &'a str = "/user";

    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let repo: UserRepository = UserRepository::new(mongo_host, collection_db, collection_name);
        UserController { repo }
    }

    fn handle_get_with_params(&self, opt: &Option<ParsedRequest>) -> String {
        let opt = opt.as_ref().unwrap();
        let params = opt.query_params();
        match params.get("id") {
            Some(val) => {
                let id = val.as_ref().unwrap().to_string();
                let response = match self.repo.get_by_id(id.clone()) {
                    Some(user) => {
                        let json = json!(user).to_string();
                        success(Option::from(json))
                    }
                    None => not_found(Option::from(format!("There are no user with id: {}", id))),
                };
                response
            }
            None => method_not_allowed(Some(opt.route())),
        }
    }

    fn handle_get_with_id_route(&self, opt: &Option<ParsedRequest>) -> String {
        let route = opt.as_ref().unwrap().route();
        let vec: Vec<&str> = route.split("/").collect();
        let id = vec[vec.len() - 1];

        let response = match self.repo.get_by_id(id.to_string()) {
            Some(user) => {
                let json = json!(user).to_string();
                success(Option::from(json))
            }
            None => not_found(Option::from(format!("There are no user with id: {}", id))),
        };
        response
    }

    fn handle_get_route(&self) -> String {
        match self.repo.get_list() {
            // Some(users) => success(Option::from(format!("{:?}", users))),
            Some(users) => {
                let json = json!(users).to_string();
                success(Option::from(String::from(json)))
            }
            None => not_found(Option::from(format!("There are no users"))),
        }
    }

    fn parse_content(&self, content: String) -> Option<User> {
        let content = content.trim_matches('\0');
        let json: Result<User, _> = serde_json::from_str(content);
        match json {
            Ok(val) => Some(val),
            Err(err) => {
                println!("Error while parsing content into User: {:?}", err);
                None
            }
        }
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
            return self.handle_get_route();
        } else {
            let route = option.route();
            let vec: Vec<&str> = route.split("/").collect();

            //TODO: improve this logic. Does not work correctly
            if vec.len() > 2 {
                return self.handle_get_with_id_route(opt);
            } else {
                return self.handle_get_with_params(opt);
            }
        }
    }

    fn post(&self, opt: &Option<ParsedRequest>) -> String {
        let opt = opt.as_ref().unwrap();
        // let params = opt.query_params();
        // let name = params.get("name").unwrap().clone();
        // let user: User = User::new(name.unwrap(), 15);

        let user = self.parse_content(opt.content());
        let user = user.unwrap();

        let id = self.repo.create(&user);

        format!("user: {:?}\nresult: {}", user, id)
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

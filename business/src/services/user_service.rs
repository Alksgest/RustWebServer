use domain::models::user::User;
use domain::repositories::user_repository::UserRepository;
use serde_json::json;
use server_lib::server::request_parser::ParsedRequest;
use server_lib::server::response_wrapper::response_wrapper::{
    method_not_allowed, not_found, success,
};
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let repo: UserRepository = UserRepository::new(mongo_host, collection_db, collection_name);
        UserService { repo }
    }
    pub fn create(&self, opt: &Option<ParsedRequest>) -> String {
        let opt = opt.as_ref().unwrap();
        let user = self.parse_content(opt.content());
        let user = user.unwrap();

        let id = self.repo.create(&user);

        format!("user: {:?}\nresult: {}", user, id)
    }
    pub fn get_by_params(&self, opt: &Option<ParsedRequest>) -> String {
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

    pub fn get_by_id(&self, opt: &Option<ParsedRequest>) -> String {
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

    pub fn get_list(&self) -> String {
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

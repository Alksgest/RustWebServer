use domain::models::user::User;
use domain::repositories::user_repository::UserRepository;
use server_lib::mongo::repository::GenericRepository;
use server_lib::server::controller::ControllerBase;
use server_lib::server::response_wrapper::response_wrapper::method_not_allowed;
use server_lib::server::response_wrapper::response_wrapper::not_found;
use server_lib::server::response_wrapper::response_wrapper::success;
use server_lib::server::uri_parser::ParsedUri;

pub struct UserController<'a> {
    repo: UserRepository<'a>,
}

impl<'a> UserController<'a> {
    const ROUTE: &'a str = "/test";

    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let repo: UserRepository<'a> =
            UserRepository::new(mongo_host, collection_db, collection_name);
        UserController { repo }
    }

    fn handle_get_with_params(&self, opt: &Option<ParsedUri>) -> String {
        let opt = opt.as_ref().unwrap();
        let params = opt.query_params();
        match params.get("id") {
            Some(val) => {
                let id = val.as_ref().unwrap().to_string();
                let response = match self.repo.get_by_id(id.clone()) {
                    Some(user) => success(Option::from(format!("got user: {:?}", user))),
                    None => not_found(Option::from(format!("There are no user with id: {}", id))),
                };
                response
            }
            None => method_not_allowed(Some(opt.route())),
        }
    }

    fn handle_get_with_id_route(&self, opt: &Option<ParsedUri>) -> String {
        let route = opt.as_ref().unwrap().route();
        let vec: Vec<&str> = route.split("/").collect();
        let id = vec[vec.len() - 1];

        let response = match self.repo.get_by_id(id.to_string()) {
            Some(user) => success(Option::from(format!("got user: {:?}", user))),
            None => not_found(Option::from(format!("There are no user with id: {}", id))),
        };
        response
    }

    fn handle_get_route(&self) -> String {
        match self.repo.get_list() {
            Some(users) => success(Option::from(format!("got users: {:?}", users))),
            None => not_found(Option::from(format!("There are no users"))),
        }
    }
}

impl<'a> ControllerBase for UserController<'_> {
    fn route(&self) -> std::string::String {
        UserController::ROUTE.to_string()
    }
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        let option: &ParsedUri = opt.as_ref().unwrap();
        let route = option.route();
        if route == UserController::ROUTE.to_string() && option.query_params().len() == 0 {
            return self.handle_get_route();
        } else {
            let route = option.route();
            let vec: Vec<&str> = route.split("/").collect();

            if vec.len() > 1 {
                return self.handle_get_with_id_route(opt);
            } else {
                return self.handle_get_with_params(opt);
            }
        }
    }

    fn post(&self, opt: &Option<ParsedUri>) -> String {
        let opt = opt.as_ref().unwrap();
        let params = opt.query_params();
        let name = params.get("name").unwrap().clone();
        let user: User = User::new(name.unwrap(), 15);

        let id = self.repo.create(&user);

        format!("user: {:?}\nresult: {}", user, id)
    }
}

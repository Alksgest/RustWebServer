use domain::models::User;
use crate::server_lib::mongo::repository::Repository;
use server_lib::mongo::repository::GenericRepository;
use server_lib::server::controller::ControllerBase;
use server_lib::server::response_wrapper::response_wrapper::method_not_allowed;
use server_lib::server::response_wrapper::response_wrapper::not_found;
use server_lib::server::response_wrapper::response_wrapper::success;
use server_lib::server::uri_parser::ParsedUri;

pub struct UserController<'a> {
    repo: GenericRepository<'a, User>,
}

impl<'a> UserController<'a> {
    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let repo: GenericRepository<'a, User> =
            GenericRepository::new(mongo_host, collection_db, collection_name);
            UserController { repo }
    }
}

impl<'a> ControllerBase for UserController<'_> {
    fn rout(&self) -> std::string::String {
        "/test".to_string()
    }
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        let opt = opt.as_ref().unwrap();
        let params = opt.query_params();
        match params.get("id") {
            Some(val) => {
                let id = val.as_ref().unwrap().to_string();
                let response = match self.repo.get(id.clone()) {
                    Some(user) => success(Option::from(format!("got user: {:?}", user))),
                    None => not_found(Option::from(format!("There are no user with id: {}", id))),
                };
                response
            }
            None => method_not_allowed(Some(opt.route())),
        }
    }
    fn post(&self, opt: &Option<ParsedUri>) -> String {
        let opt = opt.as_ref().unwrap();
        let params = opt.query_params();
        let name = params.get("name").unwrap().clone();
        let user: User = User::new(name.unwrap(), 15);

        let is_created = self.repo.create(&user);

        format!("user: {:?}\nresult: {}", user, is_created)
    }
}

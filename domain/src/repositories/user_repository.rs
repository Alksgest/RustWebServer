use crate::models::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use server_lib::mongo::repository::GenericRepository;
use server_lib::mongo::repository::Repository;

pub struct UserRepository {
    repo: GenericRepository<User>,
}

impl UserRepository {
    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let repo: GenericRepository<User> =
            GenericRepository::new(mongo_host, collection_db, collection_name);
        UserRepository { repo }
    }
    pub fn get_by_id(&self, id: String) -> Option<User> {
        let id = match ObjectId::parse_str(id) {
            Ok(val) => Some(val),
            Err(err) => {
                println!("Error while parsing id! {:?}", err);
                Some(ObjectId::new())
            }
        };
        match self.repo.get(doc! {"_id": id.unwrap()}, None) {
            Some(val) => Some(val),
            None => None,
        }
    }

    pub fn get_list(&self) -> Option<std::vec::Vec<User>> {
        self.repo.list(None, None)
    }

    pub fn create(&self, user: &User) -> String {
        self.repo.create(user)
    }
}

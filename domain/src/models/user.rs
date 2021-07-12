use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use server_lib::mongo::repository::MongoModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    name: String,
    age: i32,
}

impl User {
    pub fn new(name: String, age: i32) -> Self {
        User {
            id: None,
            name,
            age,
        }
    }
}

impl MongoModel for User {
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone().to_string()
    }
}

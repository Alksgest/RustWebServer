use mongodb::results::InsertManyResult;
use mongodb::results::InsertOneResult;
use mongodb::sync::Client;
use mongodb::sync::Collection;
use mongodb::sync::Database;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

use mongodb::bson::{doc, Document};

pub trait MongoModel: Serialize + Unpin + DeserializeOwned {
    fn get_id(&self) -> String;
}

pub trait Repository<'a, T>
where
    T: Serialize + Unpin + DeserializeOwned + MongoModel,
{
    fn get(&self, id: String) -> Option<T>;
    fn get_by_ids(&self, ids: &Vec<String>) -> Vec<T>;
    fn list(&self) -> Vec<T>;
    fn create(&self, value: &T) -> bool;
    fn create_many(&self, values: Vec<T>) -> bool;
    fn update(&self, value: &T) -> bool;
    fn delete(&self, id: String) -> bool;
}

pub struct GenericRepository<'a, T>
where
    T: MongoModel,
{
    // client: Client,
    // database: Database,
    collection: Collection<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> GenericRepository<'a, T>
where
    T: MongoModel,
{
    // "mongodb://localhost:27017", mydb, users
    pub fn new<S: AsRef<str>>(mongo_host: S, collection_db: S, collection_name: S) -> Self {
        let client: Client = Client::with_uri_str(mongo_host).unwrap();
        let database: Database = client.database(collection_db.as_ref());
        let collection = database.collection::<T>(collection_name.as_ref());

        GenericRepository {
            // client,
            // database,
            collection,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Repository<'a, T> for GenericRepository<'a, T>
where
    T: Serialize + Unpin + DeserializeOwned + MongoModel,
{
    fn get(&self, id: std::string::String) -> Option<T> {
        match self.collection.find_one(doc! {"id": id}, None) {
            Ok(val) => val,
            Err(_) => None,
        }
    }
    fn get_by_ids(&self, ids: &std::vec::Vec<std::string::String>) -> std::vec::Vec<T> {
        todo!()
    }
    fn list(&self) -> std::vec::Vec<T> {
        todo!()
    }
    fn create(&self, value: &T) -> bool {
        let res: InsertOneResult = self.collection.insert_one(value, None).unwrap();
        let id = res.inserted_id;
        println!("{:?}", id);

        true
    }
    fn create_many(&self, values: std::vec::Vec<T>) -> bool {
        let res: InsertManyResult = self.collection.insert_many(values, None).unwrap();
        let ids = res.inserted_ids;
        println!("{:?}", ids);
        // match self.collection.insert_many(values, None) {
        //     Ok(val) => println!("{:?}", val),
        //     Err(err) => println!("Error!")
        true
    }
    fn update(&self, value: &T) -> bool {
        // self.client.update_one()
        true
    }
    fn delete(&self, id: std::string::String) -> bool {
        match self.collection.delete_one(doc! {"id": id}, None) {
            Ok(val) => {
                println!("Deleting success: {:?}", val);
                true
            }
            Err(err) => {
                println!("Error while deleting: {:?}", err);
                false
            }
        }
    }
}

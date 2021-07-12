use mongodb::error::Error;
use mongodb::options::UpdateModifications;
use mongodb::options::UpdateOptions;
use mongodb::options::{FindOneOptions, FindOptions};
use mongodb::results::UpdateResult;
use mongodb::results::{InsertManyResult, InsertOneResult};
use mongodb::sync::{Client, Collection, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;

use mongodb::bson::{doc, document::Document};

pub trait MongoModel: Serialize + Unpin + DeserializeOwned {
    fn get_id(&self) -> String;
}

pub trait Repository<T>
where
    T: Serialize + Unpin + DeserializeOwned + MongoModel,
{
    fn get(
        &self,
        filter: impl Into<Option<Document>>,
        option: impl Into<Option<FindOneOptions>>,
    ) -> Option<T>;
    fn list(
        &self,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOptions>>,
    ) -> Option<std::vec::Vec<T>>;
    fn create(&self, value: &T) -> String;
    fn create_many(&self, values: Vec<T>) -> bool;
    fn update(
        &self,
        query: Document,
        update: impl Into<UpdateModifications>,
        options: impl Into<Option<UpdateOptions>>,
    ) -> Result<UpdateResult, Error>;
    fn delete(&self, id: String) -> bool;
}

pub struct GenericRepository<T>
where
    T: MongoModel,
{
    collection: Collection<T>,
    // phantom: PhantomData<&'a T>,
}

impl<T> GenericRepository<T>
where
    T: MongoModel,
{
    // "mongodb://localhost:27017", mydb, users
    pub fn new(mongo_host: &str, collection_db: &str, collection_name: &str) -> Self {
        let client: Client = Client::with_uri_str(mongo_host).unwrap();
        let database: Database = client.database(collection_db.as_ref());
        let collection = database.collection::<T>(collection_name.as_ref());

        GenericRepository {
            collection,
            // phantom: PhantomData,
        }
    }
}

impl<T> Repository<T> for GenericRepository<T>
where
    T: Serialize + Unpin + DeserializeOwned + MongoModel,
{
    fn get(
        &self,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOneOptions>>,
    ) -> Option<T> {
        match self.collection.find_one(filter, options) {
            Ok(val) => val,
            Err(err) => {
                println!("Error while getting element! {:?}", err);
                None
            }
        }
    }

    fn list(
        &self,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOptions>>,
    ) -> Option<std::vec::Vec<T>> {
        match self.collection.find(filter, options) {
            Ok(cursor) => {
                let mut vec = Vec::new();
                for doc in cursor {
                    match doc {
                        Ok(doc) => vec.push(doc),
                        Err(err) => println!("{:?}", err),
                    }
                }
                Some(vec)
            }
            Err(_) => None,
        }
    }
    fn create(&self, value: &T) -> String {
        let res: InsertOneResult = self.collection.insert_one(value, None).unwrap();
        let id = res.inserted_id.as_str();
        println!("{:?}", id);

        id.unwrap().to_string()
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
    fn update(
        &self,
        query: Document,
        update: impl Into<UpdateModifications>,
        options: impl Into<Option<UpdateOptions>>,
    ) -> Result<UpdateResult, Error> {
        self.collection.update_one(query, update, options)
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

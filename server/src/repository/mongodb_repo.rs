use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{InsertOneResult},
    Client, Collection
};
use crate::models::use_schema::TestData;

pub struct MongoRepo{
    col: Collection<TestData>
}

impl MongoRepo{
    pub async fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable")
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<TestData> = db.collection("User");
        MongoRepo{col}
    }

    pub async fn create_data(&self, new_user:TestData) -> Result<InsertOneResult, Error> {
        let new_doc = TestData {
            id : None,
            ramUsage: "Ram".to_string(),
            swapUsage: "Swap".to_string(),
            tempInfo: "temp".to_string()
        };
        let data = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .await
            .expect("Error creating user");
        Ok(data)
    }
}

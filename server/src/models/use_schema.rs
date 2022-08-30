use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestData{
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub ramUsage: String,
    pub swapUsage: String,
    pub tempInfo: String
}

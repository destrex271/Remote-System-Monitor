use crate::{models::use_schema::TestData, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse
};

#[post("/info")]
pub async fn create_data(db: Data<MongoRepo>, new_data: Json<TestData>) -> HttpResponse {
    let data = TestData{
        id: None,
        ramUsage: new_data.ramUsage.to_owned(),
        swapUsage: new_data.swapUsage.to_owned(),
        tempInfo: new_data.tempInfo.to_owned()
    };
    let details = db.create_data(data).await;
    match details{
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

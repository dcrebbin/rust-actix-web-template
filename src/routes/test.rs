use crate::constants::utility::{log_error, log_query};
use actix_web::{web, Error};
use models::test_models::TestRequest;

use crate::models;

pub async fn test_route(body: web::Json<TestRequest>) -> Result<String, Error> {
    log_query("Test route called");
    println!("{:?}", body);
    Ok(format!("Hello World"))
}

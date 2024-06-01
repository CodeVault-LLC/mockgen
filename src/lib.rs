mod mock_data;

use actix_web::{web, Responder, HttpResponse};
use lazy_static::lazy_static;
use mock_data::{GenerateDataRequest, MockDataGenerator}; // Ensure correct import

lazy_static! {
    static ref GENERATOR: MockDataGenerator = MockDataGenerator::new();
}

pub async fn generate_data(data: web::Json<GenerateDataRequest>) -> impl Responder {
    match GENERATOR.generate_data(&data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

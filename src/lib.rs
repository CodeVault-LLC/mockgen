mod mock_data;

use actix_web::{web, Responder, HttpResponse};
pub use mock_data::*;

pub async fn generate_data(data: web::Json<GenerateDataRequest>) -> impl Responder {
    let generator = MockDataGenerator::new();
    match generator.generate_data(&data) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

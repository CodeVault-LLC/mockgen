mod mock_data;

use actix_web::{web, App, HttpServer};
use ::mock_data::generate_data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 127.0.0.1:7070!");

    HttpServer::new(|| {
        App::new()
            .route("/generate", web::post().to(generate_data))
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await
}

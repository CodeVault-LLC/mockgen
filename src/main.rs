use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mock_data::{QueryRoot, MockDataSchema};

async fn graphql_handler(schema: web::Data<MockDataSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 127.0.0.1:7070!");

    let schema = Schema::build(QueryRoot, EmptyMutation, async_graphql::EmptySubscription)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await
}

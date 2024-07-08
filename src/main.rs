use actix_web::{http, web, App, HttpServer};
use actix_cors::Cors;
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mock_data::{MockDataSchema, QueryRoot, MutationRoot};

async fn graphql_handler(
    schema: web::Data<MockDataSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 127.0.0.1:7070!");
    let schema = Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription).finish();

    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("http://localhost:5173")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await
}

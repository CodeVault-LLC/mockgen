use actix_web::{http::header, test, App};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use mock_data::{MockDataSchema, QueryRoot};
use serde_json::json;

async fn graphql_handler(
    schema: actix_web::web::Data<MockDataSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::test]
async fn test_generate_data_api() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(schema))
            .route("/graphql", actix_web::web::post().to(graphql_handler)),
    )
    .await;

    let payload = json!({
        "query": "query {
            generateData(request: {
                rows: 5,
                fields: {
                    id: { type_: \"Integer\", nullable: false },
                    name: { type_: \"String\", length: 10, nullable: false },
                    email: { type_: \"String\", length: 15, nullable: false },
                    price: { type_: \"Float\", nullable: false },
                    is_active: { type_: \"Boolean\", nullable: false }
                }
            }) {
                result
            }
        }"
    });

    let req = test::TestRequest::post()
        .uri("/graphql")
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .set_payload(payload.to_string())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    let data = body
        .get("data")
        .unwrap()
        .get("generateData")
        .unwrap()
        .get("result")
        .unwrap();
    assert_eq!(data.as_array().unwrap().len(), 5);

    for record in data.as_array().unwrap() {
        let record = record.as_object().unwrap();
        assert!(record.contains_key("id"));
        assert!(record
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<i32>()
            .is_ok());
        assert!(record.contains_key("name"));
        assert_eq!(record.get("name").unwrap().as_str().unwrap().len(), 10);
        assert!(record.contains_key("email"));
        assert_eq!(record.get("email").unwrap().as_str().unwrap().len(), 15);
        assert!(record.contains_key("price"));
        assert!(record
            .get("price")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<f64>()
            .is_ok());
        assert!(record.contains_key("is_active"));
        assert!(record
            .get("is_active")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<bool>()
            .is_ok());
    }
}

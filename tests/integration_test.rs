use actix_web::{test, App};
use mock_data::generate_data;
use std::collections::HashMap;
use actix_web::http::header;

#[actix_web::test]
async fn test_generate_data_api() {
    let app = test::init_service(App::new().route("/generate", actix_web::web::post().to(generate_data))).await;

    let payload = r#"
        {
            "rows": 5,
            "fields": {
                "id": {
                    "type_": "Integer",
                    "nullable": false,
                    "default": null
                },
                "name": {
                    "type_": "String",
                    "length": 10,
                    "nullable": false,
                    "default": null
                },
                "email": {
                    "type_": "String",
                    "length": 15,
                    "nullable": false,
                    "default": null
                },
                "price": {
                    "type_": "Float",
                    "nullable": false,
                    "default": null
                },
                "is_active": {
                    "type_": "Boolean",
                    "nullable": false,
                    "default": null
                }
            }
        }
    "#;
    let req = test::TestRequest::post()
        .uri("/generate")
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<HashMap<String, String>> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 5);

    for record in body {
        assert!(record.contains_key("id"));
        assert!(record.get("id").unwrap().parse::<i32>().is_ok());
        assert!(record.contains_key("name"));
        assert_eq!(record.get("name").unwrap().len(), 10);
        assert!(record.contains_key("email"));
        assert_eq!(record.get("email").unwrap().len(), 15);
        assert!(record.contains_key("price"));
        assert!(record.get("price").unwrap().parse::<f64>().is_ok());
        assert!(record.contains_key("is_active"));
        assert!(record.get("is_active").unwrap().parse::<bool>().is_ok());
    }
}

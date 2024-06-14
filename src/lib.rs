mod engine;
mod mock_data;

use std::collections::HashMap;

use async_graphql::{Context, Object, Schema, SimpleObject};
use mock_data::{GenerateDataRequest, MockDataGenerator};
use once_cell::sync::Lazy;

pub type MockDataSchema =
    Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

#[derive(SimpleObject)]
struct GenerateDataResponse {
    result: Vec<HashMap<String, String>>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn generate_data(
        &self,
        _ctx: &Context<'_>,
        request: GenerateDataRequest,
    ) -> GenerateDataResponse {
        match GENERATOR.generate_data(&request) {
            Ok(result) => GenerateDataResponse { result },
            Err(err) => panic!("Error generating data: {}", err),
        }
    }
}

static GENERATOR: Lazy<MockDataGenerator> = Lazy::new(MockDataGenerator::new);

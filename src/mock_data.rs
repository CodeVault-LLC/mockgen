use async_graphql::{InputObject, SimpleObject};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engine::{generate_component_data, ComponentOptions, FieldType, ADDRESS, BALANCE};


#[derive(Deserialize, InputObject, Debug)]
pub struct GenerateDataRequest {
    pub rows: u32,
    pub fields: HashMap<String, FieldType>,
}

#[derive(SimpleObject)]
pub struct GenerateDataResponse {
    pub data: Vec<HashMap<String, String>>,
}

pub struct MockDataGenerator;

impl MockDataGenerator {
    pub fn new() -> Self {
        MockDataGenerator
    }

    pub fn generate_data(
        &self,
        request: &GenerateDataRequest,
    ) -> Result<Vec<HashMap<String, String>>, String> {
        let mut results = Vec::new();

        for _ in 0..request.rows {
            let mut record = HashMap::new();
            for (field_name, field_type) in &request.fields {
                let value = self.generate_field_value(field_type)?;
                record.insert(field_name.clone(), value);
            }
            results.push(record);
        }

        Ok(results)
    }

    fn generate_field_value(&self, field_type: &FieldType) -> Result<String, String> {
        if let Some(default) = &field_type.default {
            return Ok(default.clone());
        }

        let mut rng = rand::thread_rng();
        match field_type.type_.as_str() {
            "String" => {
                let length = field_type.length.unwrap_or(10) as usize;
                Ok((0..length)
                    .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                    .collect())
            }
            "Integer" => {
                let value: i32 = rng.gen();
                Ok(value.to_string())
            }
            "Float" => {
                let value: f64 = rng.gen();
                Ok(value.to_string())
            }
            "Boolean" => {
                let value: bool = rng.gen();
                Ok(value.to_string())
            }
            "Address" => {
                let mut options = HashMap::new();
                options.insert("field_type".to_string(), ComponentOptions::FieldType(FieldType {
                    type_: "String".to_string(),
                    length: Some(1),
                    nullable: false,
                    default: None,
                }));


                let value = generate_component_data(&*ADDRESS, options);

                let index = rng.gen_range(0..value.len());
                Ok(value[index].clone())
            }

            "Balance" => {
                let mut options = HashMap::new();
                options.insert("currency_code".to_string(), ComponentOptions::FieldType(FieldType {
                    type_: "String".to_string(),
                    length: Some(3),
                    nullable: false,
                    default: Some("USD".to_string()),
                }));

                let value = generate_component_data(&*BALANCE, options);
                Ok(value[0].clone())
            }

            _ => Err(format!("Unsupported field type: {}", field_type.type_)),
        }
    }
}

use rand::Rng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldType {
    type_: String,
    length: Option<u32>,
    nullable: bool,
    default: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateDataRequest {
    rows: u32,
    fields: HashMap<String, FieldType>,
}

pub struct MockDataGenerator;

impl MockDataGenerator {
    pub fn new() -> Self {
        MockDataGenerator
    }

    pub fn generate_data(&self, request: &GenerateDataRequest) -> Result<Vec<HashMap<String, String>>, String> {
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
                Ok((0..length).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect())
            },
            "Integer" => {
                let value: i32 = rng.gen();
                Ok(value.to_string())
            },
            "Float" => {
                let value: f64 = rng.gen();
                Ok(value.to_string())
            },
            "Boolean" => {
                let value: bool = rng.gen();
                Ok(value.to_string())
            },
            _ => Err(format!("Unsupported field type: {}", field_type.type_)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integer_field() {
        let generator = MockDataGenerator::new();
        let field_type = FieldType {
            type_: "Integer".to_string(),
            length: None,
            nullable: false,
            default: None,
        };

        let value = generator.generate_field_value(&field_type).unwrap();
        value.parse::<i32>().unwrap();
    }

    #[test]
    fn test_generate_string_field() {
        let generator = MockDataGenerator::new();
        let field_type = FieldType {
            type_: "String".to_string(),
            length: Some(10),
            nullable: false,
            default: None,
        };

        let value = generator.generate_field_value(&field_type).unwrap();
        assert_eq!(value.len(), 10);
    }

    #[test]
    fn test_generate_float_field() {
        let generator = MockDataGenerator::new();
        let field_type = FieldType {
            type_: "Float".to_string(),
            length: None,
            nullable: false,
            default: None,
        };

        let value = generator.generate_field_value(&field_type).unwrap();
        value.parse::<f64>().unwrap();
    }

    #[test]
    fn test_generate_boolean_field() {
        let generator = MockDataGenerator::new();
        let field_type = FieldType {
            type_: "Boolean".to_string(),
            length: None,
            nullable: false,
            default: None,
        };

        let value = generator.generate_field_value(&field_type).unwrap();
        value.parse::<bool>().unwrap();
    }

    #[test]
    fn test_generate_data() {
        let generator = MockDataGenerator::new();
        let request = GenerateDataRequest {
            rows: 5,
            fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "id".to_string(),
                    FieldType {
                        type_: "Integer".to_string(),
                        length: None,
                        nullable: false,
                        default: None,
                    },
                );
                fields.insert(
                    "name".to_string(),
                    FieldType {
                        type_: "String".to_string(),
                        length: Some(10),
                        nullable: false,
                        default: None,
                    },
                );
                fields
            },
        };

        let result = generator.generate_data(&request).unwrap();
        assert_eq!(result.len(), 5);
        for record in result {
            assert!(record.contains_key("id"));
            assert!(record.get("id").unwrap().parse::<i32>().is_ok());
            assert!(record.contains_key("name"));
            assert_eq!(record.get("name").unwrap().len(), 10);
        }
    }
}

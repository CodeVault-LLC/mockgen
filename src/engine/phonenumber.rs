use super::{Component, FieldType};
use std::collections::HashMap;

pub struct PhoneNumber {}

impl PhoneNumber {
    pub fn new() -> Self {
        PhoneNumber {}
    }
}

impl Component for PhoneNumber {
    fn get_data(&self, options: &FieldType) -> Vec<String> {
        let binding = <std::option::Option<HashMap<std::string::String, std::string::String>> as Clone>::clone(&options.properties).unwrap();

        let country_code = binding
            .get("country_code")
            .unwrap_or(&"1".to_string())
            .to_string();

        // Generate a random phone number, but make it look like a real phone number
        let phone_number = format!(
            "+{} {} {} {}",
            country_code,
            rand::random::<u32>() % 1000,
            rand::random::<u32>() % 100,
            rand::random::<u32>() % 1000
        );

        vec![phone_number]
    }
}

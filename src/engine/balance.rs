use super::{Component, FieldType};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
struct Record {
    symbol_native: String,
}

pub struct Balance {
    data: HashMap<String, Record>,
}

impl Balance {
    pub fn new() -> Self {
        let mut instance = Self {
            data: HashMap::new(),
        };
        instance.fetch_data();
        instance
    }

    fn fetch_data(&mut self) {
        // Read the JSON data from a file
        let mut file = File::open("src/data/currency.json").expect("Failed to open the file");

        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read the file");

        let records: HashMap<String, Record> = serde_json::from_str(&data)
            .expect("Failed to deserialize JSON data into a HashMap of records");

        self.data = records;

        if self.data.is_empty() {
            eprintln!("No data found");
            std::process::exit(1);
        }
    }
}

impl Component for Balance {
    fn get_data(&self, options: &FieldType) -> Vec<String> {
        let binding = <std::option::Option<HashMap<std::string::String, std::string::String>> as Clone>::clone(&options.properties).unwrap();

        let currency_code = binding
            .get("currency_code")
            .unwrap_or(&"USD".to_string())
            .to_string();

        if let Some(record) = self.data.get(&currency_code) {
            let balance = rand::random::<f64>() * 1000.0;

            vec![format!("{} {:.2}", record.symbol_native, balance)]
        } else {
            vec![format!("Currency code '{}' not found", currency_code)]
        }
    }
}

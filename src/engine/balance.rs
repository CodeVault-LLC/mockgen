use super::{Component, ComponentOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
struct Record {
    symbol: String,
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
        let mut file = File::open("src/data/currency.json")
            .expect("Failed to open the file");

        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read the file");
        // Example Data:
        /*
        {
            "USD": {
                "symbol": "$",
                "name": "US Dollar",
                "symbol_native": "$",
                "decimal_digits": 2,
                "rounding": 0,
                "code": "USD",
                "name_plural": "US dollars"
            },
            "CAD": {
                "symbol": "CA$",
                "name": "Canadian Dollar",
                "symbol_native": "$",
                "decimal_digits": 2,
                "rounding": 0,
                "code": "CAD",
                "name_plural": "Canadian dollars"
            },
        */

        // Deserialize the JSON data into a vector of records
        let records: HashMap<String, Record> = serde_json::from_str(&data)
            .expect("Failed to deserialize JSON data into a HashMap of records");

        // Populate the HashMap with currency code as key and Record as value
        self.data = records;

        if self.data.is_empty() {
            eprintln!("No data found");
            std::process::exit(1);
        }
    }
}

impl Component for Balance {
    fn get_data(&self, options: &HashMap<String, ComponentOptions>) -> Vec<String> {
        println!("Options: {:?}", options);
        // Get the currency code from the options
        let currency_code = match options.get("currency_code") {
            Some(ComponentOptions::FieldType(field_type)) => field_type.default.clone().unwrap(),
            _ => "USD".to_string(),
        };

        // Check if the currency code exists in the data
        if let Some(record) = self.data.get(&currency_code) {
            // It now exist, now generate a random balance for them.
            let balance = rand::random::<f64>() * 1000.0;

            // Return the balance in a way where it is formatted with the currency symbol
            vec![format!("{} {:.2}", record.symbol, balance)]
        } else {
            vec![format!("Currency code '{}' not found", currency_code)]
        }
    }
}

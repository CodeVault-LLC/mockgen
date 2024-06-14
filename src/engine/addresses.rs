use super::{Component, FieldType};
use std::process;

#[derive(Debug, serde::Deserialize)]
struct Record {
    address: String,
}

pub struct Addresses {
    data: Vec<String>,
}

impl Addresses {
    pub fn new() -> Self {
        let mut instance = Self { data: Vec::new() };
        instance.fetch_data();
        instance
    }

    fn fetch_data(&mut self) {
        let mut rdr = csv::Reader::from_path("src/data/addresses.csv").unwrap();
        for (index, result) in rdr.deserialize().enumerate() {
            if index >= 500 {
                break;
            }
            let record: Record = result.unwrap_or_else(|err| {
                eprintln!("error deserializing record: {}", err);
                process::exit(1);
            });
            self.data.push(record.address);
        }
    }
}

impl Component for Addresses {
    fn get_data(&self, _options: &FieldType) -> Vec<String> {
        self.data.clone()
    }
}

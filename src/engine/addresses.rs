use super::{Component, ComponentOptions};
use std::{collections::HashMap, process};

#[derive(Debug, serde::Deserialize)]
struct Record {
  address: String,
}

pub struct Addresses {
  data: Vec<String>,
}

impl Addresses {
    pub fn new() -> Self {
        let mut instance = Self {
            data: Vec::new(),
        };
        instance.fetch_data();
        instance
    }

    fn fetch_data(&mut self) {
        let current_dir = std::env::current_dir().unwrap();
        println!("The current directory is {}", current_dir.display());

        // Read a file from data/addresses.csv
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
    fn get_data(&self, _options: &HashMap<String, ComponentOptions>) -> Vec<String> {
        self.data.clone()
    }
}

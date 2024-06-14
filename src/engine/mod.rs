use std::collections::HashMap;

use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

pub mod addresses;
pub mod balance;
pub mod phonenumber;

#[derive(Serialize, Deserialize, Debug, InputObject)]
pub struct FieldType {
    pub type_: String,
    pub length: Option<u32>,
    pub nullable: bool,
    pub default: Option<String>,

    pub properties: Option<HashMap<String, String>>,
}
pub trait Component {
    fn get_data(&self, options: &FieldType) -> Vec<String>;
}

pub static ADDRESS: once_cell::sync::Lazy<addresses::Addresses> =
    once_cell::sync::Lazy::new(addresses::Addresses::new);
pub static BALANCE: once_cell::sync::Lazy<balance::Balance> =
    once_cell::sync::Lazy::new(balance::Balance::new);
pub static PHONENUMBER: once_cell::sync::Lazy<phonenumber::PhoneNumber> =
    once_cell::sync::Lazy::new(phonenumber::PhoneNumber::new);

pub fn generate_component_data(component: &dyn Component, options: &FieldType) -> Vec<String> {
    component.get_data(options)
}

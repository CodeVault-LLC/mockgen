pub mod addresses;

pub trait Component {
    fn get_data(&self) -> Vec<String>;
}

// Only runs once, and is thread-safe
pub static ADDRESS: once_cell::sync::Lazy<addresses::Addresses> = once_cell::sync::Lazy::new(addresses::Addresses::new);

pub fn generate_component_data(component: &dyn Component) -> Vec<String> {
    component.get_data()
}

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DriverState {
    Unloaded,
    Loading,
    Active,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Driver {
    pub name: String,
    pub version: String,
    pub device_class: String,
    pub state: DriverState,
}

impl Driver {
    pub fn new(name: &str, version: &str, device_class: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            device_class: device_class.to_string(),
            state: DriverState::Unloaded,
        }
    }
}

pub struct DriverManager {
    drivers: HashMap<String, Driver>,
}

impl DriverManager {
    pub fn new() -> Self { Self { drivers: HashMap::new() } }

    pub fn register(&mut self, driver: Driver) {
        self.drivers.insert(driver.name.clone(), driver);
    }

    pub fn load(&mut self, name: &str) -> Result<(), String> {
        match self.drivers.get_mut(name) {
            Some(d) => { d.state = DriverState::Active; Ok(()) }
            None => Err(format!("driver '{}' not found", name)),
        }
    }

    pub fn unload(&mut self, name: &str) -> Result<(), String> {
        match self.drivers.get_mut(name) {
            Some(d) => { d.state = DriverState::Unloaded; Ok(()) }
            None => Err(format!("driver '{}' not found", name)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Driver> {
        self.drivers.get(name)
    }

    pub fn count(&self) -> usize { self.drivers.len() }

    pub fn active_count(&self) -> usize {
        self.drivers.values().filter(|d| d.state == DriverState::Active).count()
    }
}

impl Default for DriverManager {
    fn default() -> Self { Self::new() }
}

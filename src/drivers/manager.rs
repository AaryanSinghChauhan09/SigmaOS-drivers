//! DriverManager: registers drivers, enumerates devices, probe/bind.

use crate::drivers::device::{Device, DeviceState};
use crate::drivers::driver::{Driver, DriverError};
use std::collections::BTreeMap;

/// A registered driver instance.
struct Registered {
    driver: Box<dyn Driver>,
    bound: Vec<String>, // device ids
}

/// The driver core.
#[derive(Default)]
pub struct DriverManager {
    drivers: Vec<Registered>,
    devices: BTreeMap<String, Device>,
}

impl DriverManager {
    pub fn new() -> Self { Self::default() }

    /// Register a driver.
    pub fn register_driver(&mut self, driver: Box<dyn Driver>) {
        self.drivers.push(Registered { driver, bound: Vec::new() });
    }

    /// Add a discovered device.
    pub fn add_device(&mut self, dev: Device) {
        self.devices.insert(dev.id.clone(), dev);
    }

    /// Number of registered drivers.
    pub fn driver_count(&self) -> usize { self.drivers.len() }
    /// Number of known devices.
    pub fn device_count(&self) -> usize { self.devices.len() }

    /// Find a driver index matching `dev` (first compatible match wins).
    fn find_driver(&self, dev: &Device) -> Option<usize> {
        self.drivers.iter().position(|r| r.driver.matches(dev))
    }

    /// Probe all unbound devices against registered drivers.
    /// Returns (bound_count, failed_count).
    pub fn probe_all(&mut self) -> (usize, usize) {
        let mut bound = 0;
        let mut failed = 0;
        let ids: Vec<String> = self.devices.keys().cloned().collect();
        for id in ids {
            let already = self.devices.get(&id)
                .map(|d| d.state == DeviceState::Active)
                .unwrap_or(true);
            if already { continue; }
            let dev = match self.devices.get(&id) { Some(d) => d.clone(), None => continue };
            if let Some(idx) = self.find_driver(&dev) {
                if let Some(d) = self.devices.get_mut(&id) { d.state = DeviceState::Probing; }
                let res = {
                    let drv = &mut self.drivers[idx].driver;
                    drv.probe(&dev)
                };
                match res {
                    Ok(()) => {
                        if let Some(d) = self.devices.get_mut(&id) {
                            d.state = DeviceState::Active;
                            d.driver = Some(self.drivers[idx].driver.name().to_string());
                        }
                        self.drivers[idx].bound.push(id.clone());
                        bound += 1;
                    }
                    Err(_) => {
                        if let Some(d) = self.devices.get_mut(&id) { d.state = DeviceState::Failed; }
                        failed += 1;
                    }
                }
            }
        }
        (bound, failed)
    }

    /// Unbind a device from its driver.
    pub fn unbind(&mut self, id: &str) -> Result<(), DriverError> {
        let dev = self.devices.get(id).cloned().ok_or(DriverError::Unsupported)?;
        let driver_name = dev.driver.clone().ok_or(DriverError::Unsupported)?;
        if let Some(idx) = self.drivers.iter().position(|r| r.driver.name() == driver_name) {
            self.drivers[idx].driver.remove(&dev)?;
            self.drivers[idx].bound.retain(|b| b != id);
        }
        if let Some(d) = self.devices.get_mut(id) {
            d.state = DeviceState::Inactive;
            d.driver = None;
        }
        Ok(())
    }

    /// List devices in a sysfs-like tree: (path, name, state, driver).
    pub fn device_tree(&self) -> Vec<(String, String, String, String)> {
        self.devices.values().map(|d| {
            (d.path.clone(), d.name.clone(), format!("{:?}", d.state),
             d.driver.clone().unwrap_or_else(|| "-".into()))
        }).collect()
    }

    /// Get a device.
    pub fn device(&self, id: &str) -> Option<&Device> { self.devices.get(id) }
    /// All devices.
    pub fn devices(&self) -> impl Iterator<Item = &Device> { self.devices.values() }
    /// Bound device ids for a driver name.
    pub fn bound_devices(&self, driver: &str) -> Vec<String> {
        self.drivers.iter()
            .find(|r| r.driver.name() == driver)
            .map(|r| r.bound.clone())
            .unwrap_or_default()
    }
}

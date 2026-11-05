#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverType { StorageBlock, VideoGop, SerialUart, NetworkInterface }

#[derive(Debug)]
pub struct DriverEntry {
    pub name: String,
    pub driver_type: DriverType,
    pub initialized: bool,
}

#[derive(Debug, Default)]
pub struct DriversCollection {
    drivers: Vec<DriverEntry>,
}

impl DriversCollection {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, name: &str, dt: DriverType) {
        self.drivers.push(DriverEntry {
            name: name.to_string(),
            driver_type: dt,
            initialized: true,
        });
    }
    pub fn count(&self) -> usize { self.drivers.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_driver_collection_registration() {
        let mut col = DriversCollection::new();
        col.register("nvme-driver", DriverType::StorageBlock);
        col.register("gop-driver", DriverType::VideoGop);
        assert_eq!(col.count(), 2);
    }
}

//! Driver core: device model, driver trait, and the probe/bind manager.

pub mod device;
pub mod driver;
pub mod manager;

pub use device::{Device, DeviceState};
pub use driver::{Driver, DriverError};
pub use manager::DriverManager;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// A test UART driver.
    struct Uart16550 {
        probed: RefCell<Vec<String>>,
    }
    impl Driver for Uart16550 {
        fn name(&self) -> &'static str { "uart16550" }
        fn compatible(&self) -> &'static [&'static str] {
            &["ns16550a", "snps,dw-apb-uart"]
        }
        fn probe(&mut self, dev: &Device) -> Result<(), DriverError> {
            if !dev.resources.contains_key("reg") {
                return Err(DriverError::MissingResource("reg".into()));
            }
            self.probed.borrow_mut().push(dev.id.clone());
            Ok(())
        }
        fn remove(&mut self, _dev: &Device) -> Result<(), DriverError> { Ok(()) }
    }

    /// A test NIC driver.
    struct VirtioNet;
    impl Driver for VirtioNet {
        fn name(&self) -> &'static str { "virtio-net" }
        fn compatible(&self) -> &'static [&'static str] { &["virtio,net-pci"] }
        fn probe(&mut self, _dev: &Device) -> Result<(), DriverError> { Ok(()) }
        fn remove(&mut self, _dev: &Device) -> Result<(), DriverError> { Ok(()) }
    }

    #[test]
    fn probe_binds_matching_devices() {
        let mut m = DriverManager::new();
        m.register_driver(Box::new(Uart16550 { probed: RefCell::new(Vec::new()) }));
        m.register_driver(Box::new(VirtioNet));

        m.add_device(
            Device::new("dev0", "UART0", "platform:soc:uart0")
                .with_compatible(&["ns16550a"])
                .with_resource("reg", 0x9000000),
        );
        m.add_device(
            Device::new("dev1", "eth0", "pci:0000:00:03.0")
                .with_compatible(&["virtio,net-pci"]),
        );
        m.add_device(
            Device::new("dev2", "unknown", "platform:foo:bar")
                .with_compatible(&["acme,unknown-1"]),
        );

        let (bound, failed) = m.probe_all();
        assert_eq!(bound, 2);
        assert_eq!(failed, 0);
        assert_eq!(m.device("dev0").unwrap().state, DeviceState::Active);
        assert_eq!(m.device("dev0").unwrap().driver.as_deref(), Some("uart16550"));
        assert_eq!(m.device("dev1").unwrap().driver.as_deref(), Some("virtio-net"));
        assert_eq!(m.device("dev2").unwrap().state, DeviceState::Discovered);
    }

    #[test]
    fn missing_resource_fails_probe() {
        struct Strict;
        impl Driver for Strict {
            fn name(&self) -> &'static str { "strict" }
            fn compatible(&self) -> &'static [&'static str] { &["need-reg"] }
            fn probe(&mut self, dev: &Device) -> Result<(), DriverError> {
                dev.resources.get("reg")
                    .map(|_| ())
                    .ok_or_else(|| DriverError::MissingResource("reg".into()))
            }
            fn remove(&mut self, _d: &Device) -> Result<(), DriverError> { Ok(()) }
        }
        let mut m = DriverManager::new();
        m.register_driver(Box::new(Strict));
        m.add_device(Device::new("d", "x", "p").with_compatible(&["need-reg"]));
        let (b, f) = m.probe_all();
        assert_eq!((b, f), (0, 1));
        assert_eq!(m.device("d").unwrap().state, DeviceState::Failed);
    }

    #[test]
    fn unbind_releases_device() {
        let mut m = DriverManager::new();
        m.register_driver(Box::new(VirtioNet));
        m.add_device(Device::new("d", "eth", "pci:x").with_compatible(&["virtio,net-pci"]));
        m.probe_all();
        assert!(m.device("d").unwrap().driver.is_some());
        m.unbind("d").unwrap();
        assert!(m.device("d").unwrap().driver.is_none());
        assert_eq!(m.device("d").unwrap().state, DeviceState::Inactive);
    }

    #[test]
    fn device_tree_lists() {
        let mut m = DriverManager::new();
        m.register_driver(Box::new(VirtioNet));
        m.add_device(Device::new("d", "eth", "pci:x").with_compatible(&["virtio,net-pci"]));
        m.probe_all();
        let tree = m.device_tree();
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].3, "virtio-net");
    }
}

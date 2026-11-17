//! Driver trait: compatible matching + probe/remove lifecycle.

use crate::drivers::device::Device;

/// Errors from driver operations.
#[derive(Debug)]
pub enum DriverError {
    /// Probe failed with a reason.
    ProbeFailed(String),
    /// Device missing a required resource.
    MissingResource(String),
    /// Driver does not support this device.
    Unsupported,
    /// Remove failed.
    RemoveFailed(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProbeFailed(m) => write!(f, "probe failed: {m}"),
            Self::MissingResource(r) => write!(f, "missing resource: {r}"),
            Self::Unsupported => write!(f, "unsupported device"),
            Self::RemoveFailed(m) => write!(f, "remove failed: {m}"),
        }
    }
}
impl std::error::Error for DriverError {}

/// The driver contract. Implementors are concrete device drivers.
pub trait Driver {
    /// Driver name (e.g. "e1000", "virtio-net", "uart16550").
    fn name(&self) -> &'static str;
    /// Compatible strings this driver binds to (most-specific first).
    fn compatible(&self) -> &'static [&'static str];
    /// Attempt to bind/probe the device. Called once per matching device.
    fn probe(&mut self, dev: &Device) -> Result<(), DriverError>;
    /// Detach from a device.
    fn remove(&mut self, dev: &Device) -> Result<(), DriverError>;
    /// Whether this driver claims `dev` (compatible-string match).
    fn matches(&self, dev: &Device) -> bool {
        let compat = self.compatible();
        dev.compatible.iter().any(|dc| compat.contains(&dc.as_str()))
    }
}

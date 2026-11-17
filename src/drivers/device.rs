//! Device model: identity, compatible strings, lifecycle state.

/// A kernel device discovered on a bus.
#[derive(Debug, Clone)]
pub struct Device {
    /// Unique device id (bus-assigned or synthetic).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Bus/path (e.g. "pci:0000:00:1f.2", "platform:soc:uart0").
    pub path: String,
    /// OpenFirmware-style compatible strings, most-specific first.
    pub compatible: Vec<String>,
    /// Bound driver name, if any.
    pub driver: Option<String>,
    /// Device state.
    pub state: DeviceState,
    /// Opaque device resources (MMIO base, IRQ, DMA masks) keyed by name.
    pub resources: std::collections::BTreeMap<String, u64>,
}

/// Device lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    /// Discovered, not yet probed.
    Discovered,
    /// A driver is being probed.
    Probing,
    /// Bound to a driver and active.
    Active,
    /// Probe failed.
    Failed,
    /// Unbound / suspended.
    Inactive,
}

impl Device {
    pub fn new(id: impl Into<String>, name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            id: id.into(), name: name.into(), path: path.into(),
            compatible: Vec::new(), driver: None, state: DeviceState::Discovered,
            resources: std::collections::BTreeMap::new(),
        }
    }
    /// Builder: set compatible strings.
    pub fn with_compatible(mut self, compat: &[&str]) -> Self {
        self.compatible = compat.iter().map(|s| s.to_string()).collect();
        self
    }
    /// Builder: add a resource.
    pub fn with_resource(mut self, name: impl Into<String>, value: u64) -> Self {
        self.resources.insert(name.into(), value);
        self
    }
    /// Whether any compatible string matches `needle`.
    pub fn matches_compatible(&self, needle: &str) -> bool {
        self.compatible.iter().any(|c| c == needle)
    }
}

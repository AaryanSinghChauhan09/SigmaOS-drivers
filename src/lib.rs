pub mod config;
pub mod driver_manager;
pub mod error;
pub mod service;

pub use config::Config;
pub use driver_manager::{Driver, DriverManager, DriverState};
pub use error::{ComponentError, Result};
pub use service::{Service, ServiceStatus};

pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}

pub struct DriversComponent {
    service: service::Service,
}

impl DriversComponent {
    pub fn new(config: Config) -> Self {
        Self { service: service::Service::new(config) }
    }
}

impl Component for DriversComponent {
    fn name(&self) -> &str { "sigmaos-drivers" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("drivers service not running".to_string())) }
    }
}

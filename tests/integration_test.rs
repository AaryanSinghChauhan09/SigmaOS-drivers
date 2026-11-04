use anyhow::Result;
use sigmaos_system_component::{Config, drivers};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_aaa() {
        // Arrange & Act
        let config = Config::default();
        // Assert
        assert!(config.enabled);
    }

    #[test]
    fn test_drivers_initialize_aaa() -> Result<()> {
        // Arrange
        let config = Config::default();
        let mut drv = drivers::new(config);
        // Act & Assert
        drv.initialize()?;
        Ok(())
    }
}

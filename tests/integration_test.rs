use sigmaos_drivers::{Config, Driver, DriverManager, DriverState, DriversComponent, Component};

#[test]
fn test_config_default_aaa() {
    let config = Config::default();
    assert!(config.enabled);
    assert!(config.auto_load);
}

#[test]
fn test_driver_manager_register_load_aaa() {
    // Arrange
    let mut mgr = DriverManager::new();
    mgr.register(Driver::new("nvidia", "545.0", "gpu"));
    // Act
    let result = mgr.load("nvidia");
    // Assert
    assert!(result.is_ok());
    assert_eq!(mgr.active_count(), 1);
}

#[test]
fn test_driver_manager_load_missing_aaa() {
    // Arrange
    let mut mgr = DriverManager::new();
    // Act
    let result = mgr.load("nonexistent");
    // Assert
    assert!(result.is_err());
}

#[test]
fn test_driver_manager_unload_aaa() {
    // Arrange
    let mut mgr = DriverManager::new();
    mgr.register(Driver::new("i915", "6.6", "gpu"));
    mgr.load("i915").unwrap();
    assert_eq!(mgr.active_count(), 1);
    // Act
    mgr.unload("i915").unwrap();
    // Assert
    assert_eq!(mgr.active_count(), 0);
}

#[test]
fn test_driver_state_initial_aaa() {
    // Arrange
    let mgr_driver = Driver::new("e1000e", "5.14", "network");
    // Assert
    assert_eq!(mgr_driver.state, DriverState::Unloaded);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut component = DriversComponent::new(Config::default());
    assert!(component.initialize().is_ok());
    assert!(component.health_check().is_ok());
    assert!(component.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let component = DriversComponent::new(Config::default());
    assert_eq!(component.name(), "sigmaos-drivers");
}

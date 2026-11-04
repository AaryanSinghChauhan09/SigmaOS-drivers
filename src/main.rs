use sigmaos_drivers::{Config, DriversComponent, Component};
fn main() {
    env_logger::init();
    let mut component = DriversComponent::new(Config::default());
    if let Err(e) = component.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS drivers running");
    let _ = component.shutdown();
}

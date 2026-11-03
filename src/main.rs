use clap::Parser;
use drivers::drivers;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = drivers::Config::default();
    let mut component = drivers::new(config);
    component.initialize()?;
    println!("SigmaOS-drivers ready");
    Ok(())
}

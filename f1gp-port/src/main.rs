use anyhow::Result;
use env_logger;
use log::info;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    info!("F1GP Modern Port - Starting...");

    println!("F1GP Modern Port v0.1.0");
    println!("Classic Formula 1 Grand Prix reimplementation");
    println!();
    println!("This is a work in progress!");

    Ok(())
}

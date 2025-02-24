use std::error::Error;
use tracing::info;
mod config;
mod core;
mod execution;
mod feed;
mod monitoring;
mod strategy;
mod system;
use system::System;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    info!("Starting HFT system...");
    let system = System::new();
    system.run_forever().await
}

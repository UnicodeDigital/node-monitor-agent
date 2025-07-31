use anyhow::Result;
use monitor_agent::config::{Args, Parser};
use monitor_agent::monitor::MonitorManager;
use tokio;
use env_logger;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    println!("Starting monitor agent with config: {:?}", args);
    
    println!("Starting {} monitors for multiple WS endpoints", args.ws.len());
    let manager = MonitorManager::new(args);
    manager.run().await?;

    Ok(())
}

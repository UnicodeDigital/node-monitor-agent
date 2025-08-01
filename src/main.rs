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
    
    let ws_count = args.ws.len();
    let http_count = args.http.len();
    
    if ws_count == 0 && http_count == 0 {
        println!("No endpoints configured! Please provide --ws or --http endpoints.");
        return Ok(());
    }
    
    println!("Starting {} WebSocket monitors and {} HTTP monitors", ws_count, http_count);
    let manager = MonitorManager::new(args);
    manager.run().await?;

    Ok(())
}

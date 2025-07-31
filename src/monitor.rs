use crate::{config::{Args, ClientType}, metrics::{self, traits::{Metrics, MetricsClient}}};
use anyhow::Result;
use log::{info, warn};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures_util::StreamExt;
use tokio::task::{self, JoinHandle};

pub struct MonitorManager {
    config: Args,
}

impl MonitorManager {
    pub fn new(config: Args) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<()> {
        let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();

        // Check if name and ws array lengths match
        if self.config.chain_name.len() != self.config.ws.len() {
            warn!("Name array length ({}) doesn't match WS array length ({}). Using default names for missing entries.", 
                  self.config.chain_name.len(), self.config.ws.len());
        }

        // Start a monitor for each WS URL, pairing with corresponding name
        for (index, ws_url) in self.config.ws.iter().enumerate() {
            // Get corresponding name, use default if not available
            let monitor_name = self.config.chain_name.get(index)
                .cloned()
                .unwrap_or_else(|| format!("monitor-{}", index));

            // Create a modified configuration with single name
            let mut monitor_config = self.config.clone();
            monitor_config.chain_name = vec![monitor_name.clone()];

            let metrics: Box<dyn MetricsClient> = match self.config.client {
                ClientType::Influx => Box::new(metrics::InfluxDBClient::new(monitor_config)),
                ClientType::Http => Box::new(metrics::HttpClient::new(monitor_config)),
            };

            let ws_url_clone = ws_url.clone();
            info!("Starting monitor '{}' for WS endpoint: {}", monitor_name, ws_url);
            
            let handle = task::spawn(async move {
                check_evm_node(ws_url_clone, metrics).await;
                Ok(())
            });
            handles.push(handle);
        }

        // Wait for all monitors to complete
        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}

pub fn now_ms() -> i64 {
    let now = chrono::Utc::now();
    now.timestamp_millis()
}

pub fn to_ms( block_timestamp: u64) -> i64 {
    (block_timestamp * 1000) as i64
}

async fn check_evm_node(ws_url: String, metrics: Box<dyn MetricsClient>) {
    loop {
        let ws = WsConnect::new(ws_url.clone());
        let provider_result: std::result::Result<alloy::providers::fillers::FillProvider<alloy::providers::fillers::JoinFill<alloy::providers::Identity, alloy::providers::fillers::JoinFill<alloy::providers::fillers::GasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::BlobGasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::NonceFiller, alloy::providers::fillers::ChainIdFiller>>>>, alloy::providers::RootProvider>, alloy::transports::RpcError<alloy::transports::TransportErrorKind>> = ProviderBuilder::new()
            .connect_ws(ws).await ;

        if let Ok(provider) = provider_result {
            info!("[{}]Node({}) connected successfully.", metrics.name(), ws_url);

            if let Ok(subscription) = provider.subscribe_blocks().await {
                let mut stream = subscription.into_stream();

                while let Some(header) = stream.next().await {
                    let now = now_ms();
                    info!("[{}] New block: {}, {}", ws_url, header.number, now );
                    // let block_timestamp = to_ms(header.timestamp);
                    
                    let _ = metrics.write(&Metrics {
                        name: metrics.name().to_string(),
                        block_height: header.number,
                        block_timestamp: header.timestamp,
                        os_timestamp: now,
                    });
                }
            } 
        }

        warn!("[{}]Retrying node({}) connection. ", metrics.name(), ws_url);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}


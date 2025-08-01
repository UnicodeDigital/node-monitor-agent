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

    // Unified function to create monitor tasks
    fn create_monitor_task<F, Fut>(
        &self,
        name: String,
        url: String,
        client_type: ClientType,
        monitor_fn: F,
    ) -> JoinHandle<Result<()>>
    where
        F: FnOnce(String, Box<dyn MetricsClient>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let monitor_config = self.config.for_monitor(name.clone(), url.clone(), client_type.clone());

        let client: Box<dyn MetricsClient> = match client_type {
            ClientType::Influx => Box::new(metrics::InfluxDBClient::new(monitor_config)),
            ClientType::Http => Box::new(metrics::HttpClient::new(monitor_config)),
        };
        
        info!("Starting {:?} monitor '{}' for endpoint: {}", client_type, name, url);
        
        task::spawn(async move {
            monitor_fn(url, client).await;
            Ok(())
        })
    }

    pub async fn run(self) -> Result<()> {
        let ws_configs = self.config.get_ws_configs();
        let http_configs = self.config.get_http_configs();

        info!("Starting {} WebSocket monitors and {} HTTP monitors", 
              ws_configs.len(), http_configs.len());

        // Create all monitor tasks using functional approach
        let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();

        // WebSocket monitors
        let ws_handles: Vec<JoinHandle<Result<()>>> = ws_configs.into_iter()
            .map(|config| self.create_monitor_task(
                config.tag, 
                config.url, 
                config.client, 
                check_evm_node_ws
            ))
            .collect();

        // HTTP monitors
        let http_handles: Vec<JoinHandle<Result<()>>> = http_configs.into_iter()
            .map(|config| self.create_monitor_task(
                config.name, 
                config.url, 
                config.client, 
                check_evm_node_http
            ))
            .collect();

        handles.extend(ws_handles);
        handles.extend(http_handles);

        if handles.is_empty() {
            warn!("No monitors configured! Please provide WebSocket or HTTP endpoints.");
            return Ok(());
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

async fn check_evm_node_ws(ws_url: String, metrics: Box<dyn MetricsClient>) {
    loop {
        let ws = WsConnect::new(ws_url.clone());
        let provider_result = ProviderBuilder::new()
            .connect_ws(ws).await;

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
                        diff: 0,
                    });
                }
            } 
        }

        warn!("[{}]Retrying node({}) connection. ", metrics.name(), ws_url);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn check_evm_node_http(http_url: String, metrics: Box<dyn MetricsClient>) {
    let url = http_url.parse().unwrap();
    let provider = ProviderBuilder::new().connect_http(url);
    info!("[{}] HTTP Node({}) connected successfully.", metrics.name(), http_url);

    loop {
        let before = now_ms();

        match provider.get_block_number().await {
            Ok(block_number) => {
                let after = now_ms();
                let used = after - before;
                info!("[{}] Current block number: {}, used time: {} ms", metrics.name(), block_number, used);

                // Get block details to get timestamp
                if let Ok(Some(block)) = provider.get_block(block_number.into()).await {
                    let _ = metrics.write(&Metrics {
                        name: metrics.name().to_string(),
                        block_height: block_number,
                        block_timestamp: block.header.timestamp,
                        os_timestamp: before,
                        diff: used as i64,
                    });
                }
            }
            Err(e) => {
                warn!("[{}] Failed to get block number: {}", metrics.name(), e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

}

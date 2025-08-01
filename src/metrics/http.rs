use serde::{Deserialize, Serialize};

use crate::{config::Args, metrics::traits::{Metrics, MetricsClient}};
use log::info;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub name: String,
    pub chain_name: String,
    pub config: Args,
    pub client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct DB {
    pub host: String,
    pub token: String,
    pub name: String,
    pub table: String
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageBody {
    pub name: String,
    pub chain_name: String,
    pub block_height: u64,
    pub block_timestamp: i64,
    pub os_timestamp: i64,
    pub diff: i64,
    pub db: DB
}

impl HttpClient {
    pub fn new(config: Args) -> Self {
        let chain_name = config.ws_names.first()
            .cloned()
            .unwrap_or_else(|| "http-client".to_string());
        let name = config.name.clone(); // Use the same name as chain_name for now
        
        let client = reqwest::Client::new();
        
        info!("HTTP Metrics client initialized with name: {}, chain_name: {}, server: {}, db_name: {}, table_name: {}", 
              name, chain_name, config.server, config.db_name, config.table_name);
        Self { name, chain_name, config, client }
    }
}

impl MetricsClient for HttpClient {

    fn name(&self) -> &str {
        &self.name
    }

    fn write(&self, metric: &Metrics) {

        let url = format!("{}/api/metrics/merge", self.config.server);
        let body = MessageBody {
            name: metric.name.clone(),
            chain_name: self.chain_name.clone(),
            block_height: metric.block_height,
            block_timestamp: metric.block_timestamp as i64,
            os_timestamp: metric.os_timestamp,
            diff: metric.diff,
            db: DB {
                host: self.config.db_host.clone(),
                token: self.config.token.clone(),
                name: self.config.db_name.clone(),
                table: self.config.table_name.clone()
            }
        };

        let client = self.client.clone();

        tokio::spawn( async move {
            if let Err(e) = client.post(&url)
                .json(&body)
                .send()
                .await {
                log::error!("Failed to write metric via HTTP: {}", e);
            }
        });

    }
}
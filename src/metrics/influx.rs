use influxdb::{ Client, InfluxDbWriteable };
use chrono::{DateTime, Utc};
use hostname;
use log::info;

use crate::{config::Args, metrics::traits::{Metrics, MetricsClient}};

#[derive(InfluxDbWriteable)]
struct DBMetrics<'a> {
    #[influxdb(tag)]
    name: &'a str,
    chain_name: &'a str,
    block_height: u64,
    block_timestamp: i64,
    os_timestamp: i64,
    diff: i64,
    time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct InfluxDBClient {
    pub name: String,
    pub chain_name: String,
    pub config: Args,
    pub influxdb: Client,
}

impl InfluxDBClient {
    pub fn new(config: Args) -> Self {
        let name = config.name.clone();
        let chain_name = config.http_names.first()
            .cloned()
            .unwrap_or_else(|| {
                hostname::get().unwrap_or_else(|_| "unknown_chain".into()).to_string_lossy().into_owned()
            });

        let influxdb = Client::new(config.db_host.clone(), config.db_name.clone())
            .with_token(config.token.clone());

        info!("InfluxDB Metrics client initialized with name: {}, chain_name: {}, db_host: {}, db_name: {}, table_name: {}", 
              name, chain_name, config.db_host, config.db_name, config.table_name);

        Self { name, chain_name, config, influxdb }
    }

}

impl MetricsClient for InfluxDBClient {

    fn write(&self, metric: &Metrics) {

        let table_name = format!("{}:{}", self.config.table_name, self.chain_name);

        let write_query = DBMetrics {
            name: &self.name,
            chain_name: &self.chain_name,
            block_height: metric.block_height,
            block_timestamp: metric.block_timestamp as i64,
            os_timestamp: metric.os_timestamp,
            diff: metric.diff,
            time: Utc::now(),
        }.into_query(table_name);

        let influxdb = self.influxdb.clone();

        tokio::spawn(async move {
            if let Err(e) = influxdb.query(write_query).await {
                log::error!("Failed to write metric: {}", e);
            }
        });
    }
    
    fn name(&self) -> &str {
        &self.chain_name
    }
}
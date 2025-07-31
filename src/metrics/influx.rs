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
        let chain_name = config.chain_name.first()
            .cloned()
            .unwrap_or_else(|| {
                hostname::get().unwrap_or_else(|_| "unknown".into()).to_string_lossy().into_owned()
            });

        let influxdb = Client::new(config.db_host.clone(), config.db_name.clone())
            .with_token(config.token.clone());

        info!("Metrics initialized with name: {}, db_host: {}, db_name: {}, table_name: {}", 
              name, config.db_host, config.db_name, config.table_name);

        Self { name, chain_name, config, influxdb }
    }

}

impl MetricsClient for InfluxDBClient {

    fn write(&self, metric: &Metrics) {
        let write_query = DBMetrics {
            name: &self.name,
            chain_name: &self.chain_name,
            block_height: metric.block_height,
            block_timestamp: metric.block_timestamp as i64,
            os_timestamp: metric.os_timestamp,
            diff: 0,
            time: Utc::now(),
        }.into_query(self.config.table_name.clone());

        let influxdb = self.influxdb.clone();

        tokio::spawn(async move {
            if let Err(e) = influxdb.query(write_query).await {
                log::error!("Failed to write WS metric: {}", e);
            }
        });
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
#[derive(Debug, Clone)]
pub struct Metrics {
    pub name: String,
    pub block_height: u64,
    pub block_timestamp: u64,
    pub os_timestamp: i64,
}

pub trait MetricsClient: Send + Sync {
    fn name(&self) -> &str;
    fn write(&self, metric: &Metrics);
}
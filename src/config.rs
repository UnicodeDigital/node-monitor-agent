pub use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum ClientType {
    #[value(name = "http")]
    Http,
    #[value(name = "influx")]
    Influx,
}

impl Default for ClientType {
    fn default() -> Self {
        ClientType::Http
    }
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(long, env="NAME")]
    pub name: String,

    #[arg(long,value_delimiter = ',', env="CHAIN_NAME")]
    pub chain_name: Vec<String>,

    #[arg(short,long, default_value = "http", env="CLIENT")]
    pub client: ClientType,

    #[arg(long, value_delimiter = ',', env="HTTP")]
    pub http: Vec<String>,

    #[arg(short,long, value_delimiter = ',', env="WS")]
    pub ws: Vec<String>,

    #[arg(short, long, default_value ="http://localhost:7777", env="SERVER")]
    pub server: String,

    #[arg(long, default_value = "http://localhost:8181", env="DB_HOST")]
    pub db_host: String,

    #[arg(short, long, default_value = "test", env="DB_NAME")]
    pub db_name: String,

    #[arg(long, default_value = "ws_metric", env="TABLE_NAME")]
    pub table_name: String,

    #[arg(short, long, env="TOKEN")]
    pub token: String,
}

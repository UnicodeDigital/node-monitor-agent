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

#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub name: String,
    pub tag: String,
    pub url: String,
    pub client: ClientType,
}

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub name: String,
    pub url: String,
    pub client: ClientType,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(short,long, env="NAME", help = "Name of the monitor agent")]
    pub name: String,

    // WebSocket configuration
    #[arg(long, value_delimiter = ',', env="WS_NAMES", help = "Names for WebSocket endpoints")]
    pub ws_names: Vec<String>,

    #[arg(short, long, value_delimiter = ',', env="WS", help = "WebSocket endpoints")]
    pub ws: Vec<String>,

    #[arg(long, default_value = "http", env="WS_CLIENT", help = "Client type for WebSocket monitors")]
    pub ws_client: ClientType,

    // HTTP configuration
    #[arg(long, value_delimiter = ',', env="HTTP_NAMES", help = "Names for HTTP endpoints")]
    pub http_names: Vec<String>,

    #[arg(long, value_delimiter = ',', env="HTTP", help = "HTTP endpoints")]
    pub http: Vec<String>,

    #[arg(long, default_value = "http", env="HTTP_CLIENT", help = "Client type for HTTP monitors")]
    pub http_client: ClientType,

    // Shared metrics configuration
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

impl Args {
    /// Get WebSocket configurations
    pub fn get_ws_configs(&self) -> Vec<WebSocketConfig> {
        self.ws.iter().enumerate().map(|(index, url)| {
            let tag = self.ws_names.get(index)
                .cloned()
                .unwrap_or_else(|| format!("websocket-{}", index));
            
            WebSocketConfig {
                name: self.name.clone(),
                tag,
                url: url.clone(),
                client: self.ws_client.clone(),
            }
        }).collect()
    }

    /// Get HTTP configurations
    pub fn get_http_configs(&self) -> Vec<HttpConfig> {
        self.http.iter().enumerate().map(|(index, url)| {
            let name = self.http_names.get(index)
                .cloned()
                .unwrap_or_else(|| format!("http-{}", index));
            
            HttpConfig {
                name,
                url: url.clone(),
                client: self.http_client.clone(),
            }
        }).collect()
    }

    /// Create a modified Args for a specific monitor
    pub fn for_monitor(&self, name: String, url: String,client: ClientType) -> Args {
        let mut config = self.clone();
        match client {
            ClientType::Http => {
                config.ws_names = vec![name];
                config.ws = vec![url];
            },
            ClientType::Influx => {
                config.http_names = vec![name];
                config.http = vec![url];
            },
        }
        config
    }
}

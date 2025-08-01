# Blockchain Node Monitor Agent

A high-performance Rust-based monitoring agent for blockchain nodes that supports real-time tracking of multiple WebSocket and HTTP endpoints with configurable metrics backends.

## Features

- 🚀 **Multi-Endpoint Monitoring**: Simultaneously monitor multiple blockchain node WebSocket and HTTP endpoints
- 📊 **Dual Protocol Support**: Monitor both WebSocket (real-time) and HTTP (polling) endpoints
- 🔧 **Flexible Client Configuration**: Independent metrics backend assignment per monitor type
- 📈 **Multiple Metrics Backends**: Support for InfluxDB and HTTP-based metrics collection
- ⛓️ **Chain-Specific Monitoring**: Configure custom names for different blockchain networks
- 🔄 **Real-Time Block Tracking**: Monitor new blocks in real-time with timestamp analysis
- ⏱️ **Response Time Metrics**: Track HTTP endpoint response times for performance monitoring
- 🏗️ **Concurrent Architecture**: Efficient concurrent monitoring using Tokio async runtime
- 🌍 **Environment Configuration**: Full support for environment variable configuration
- 📈 **Scalable Design**: Easily scale to monitor multiple chains and endpoints

## Quick Start

### Download Pre-built Binary (Recommended)

**One-liner download for Linux x86_64:**

```bash
curl -L https://raw.githubusercontent.com/UnicodeDigital/node-monitor-agent/main/shell/download.sh | bash
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/UnicodeDigital/node-monitor-agent.git
cd node-monitor-agent

# Build the project
cargo build --release

# Run with help to see all options
./target/release/monitor-agent -h
```

### Usage

#### Basic WebSocket Monitoring

```bash
# Monitor WebSocket endpoints
export NAME="my-monitor"
export WS_NAMES="ethereum-mainnet"
export WS="wss://mainnet.infura.io/ws/v3/YOUR_PROJECT_ID"
export WS_CLIENT="http"
export SERVER="http://localhost:7001"
export TOKEN="your-token"

cargo run
```

#### Basic HTTP Monitoring

```bash
# Monitor HTTP endpoints with response time tracking
export NAME="my-monitor"
export HTTP_NAMES="ethereum-rpc"
export HTTP="https://mainnet.infura.io/v3/YOUR_PROJECT_ID"
export HTTP_CLIENT="influx"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain_metrics"
export TOKEN="your-influx-token"

cargo run
```

#### Mixed Protocol Monitoring

```bash
# Monitor both WebSocket and HTTP endpoints simultaneously
export NAME="multi-protocol-monitor"
export WS_NAMES="eth-ws,polygon-ws"
export WS="wss://eth-mainnet.ws,wss://polygon-mainnet.ws"
export WS_CLIENT="http"
export HTTP_NAMES="base-rpc,arbitrum-rpc"
export HTTP="https://base-mainnet.rpc,https://arbitrum-mainnet.rpc"
export HTTP_CLIENT="influx"
export SERVER="http://localhost:7001"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain_metrics"
export TOKEN="your-token"

cargo run
```

## Configuration

### Environment Variables

| Variable | Description | Required | Default | Example |
|----------|-------------|----------|---------|---------|
| `NAME` | Monitor instance name | Yes | - | `"my-monitor"` |
| **WebSocket Configuration** |
| `WS_NAMES` | WebSocket monitor names (comma-separated) | No | auto-generated | `"eth-ws,polygon-ws"` |
| `WS` | WebSocket URLs (comma-separated) | No | - | `"wss://eth1.ws,wss://eth2.ws"` |
| `WS_CLIENT` | WebSocket metrics backend | No | `http` | `http` or `influx` |
| **HTTP Configuration** |
| `HTTP_NAMES` | HTTP monitor names (comma-separated) | No | auto-generated | `"eth-rpc,polygon-rpc"` |
| `HTTP` | HTTP URLs (comma-separated) | No | - | `"https://eth1.rpc,https://eth2.rpc"` |
| `HTTP_CLIENT` | HTTP metrics backend | No | `http` | `http` or `influx` |
| **Shared Configuration** |
| `SERVER` | HTTP metrics server URL | No | `http://localhost:7777` | `"http://localhost:7001"` |
| `DB_HOST` | InfluxDB host URL | No | `http://localhost:8181` | `"http://influxdb:8086"` |
| `DB_NAME` | Database name | No | `test` | `"blockchain_metrics"` |
| `TABLE_NAME` | Table/measurement name | No | `ws_metric` | `"block_metrics"` |
| `TOKEN` | Authentication token | Yes | - | `"your-secret-token"` |

### Command Line Arguments

```bash
# View all available options
./target/release/monitor-agent --help

# Example: WebSocket monitoring with HTTP client
./target/release/monitor-agent \
  --name "eth-monitor" \
  --ws-names "ethereum-mainnet" \
  --ws "wss://mainnet.infura.io/ws/v3/YOUR_ID" \
  --ws-client http \
  --server "http://localhost:7001" \
  --token "your-token"

# Example: HTTP monitoring with InfluxDB client
./target/release/monitor-agent \
  --name "rpc-monitor" \
  --http-names "ethereum-rpc" \
  --http "https://mainnet.infura.io/v3/YOUR_ID" \
  --http-client influx \
  --db-host "http://localhost:8086" \
  --db-name "blockchain" \
  --token "your-influx-token"

# Example: Mixed monitoring
./target/release/monitor-agent \
  --name "mixed-monitor" \
  --ws-names "eth-ws" \
  --ws "wss://eth.ws" \
  --ws-client http \
  --http-names "eth-rpc" \
  --http "https://eth.rpc" \
  --http-client influx \
  --token "your-token"
```

## Metrics Backends

### HTTP Backend

Sends metrics to an HTTP endpoint via POST requests.

**Configuration:**
```bash
export CLIENT="http"
export SERVER="http://your-metrics-server:7001"
```

**Payload Format:**
```json
{
  "name": "monitor-name",
  "chain_name": "ethereum-mainnet",
  "block_height": 18500000,
  "block_timestamp": 1698000000,
  "os_timestamp": 1698000001000,
  "diff": 1000,
  "db": {
    "host": "http://localhost:8181",
    "token": "your-token",
    "name": "test",
    "table": "ws_metric"
  }
}
```

### InfluxDB Backend

Writes metrics directly to InfluxDB using the native client. Includes response time tracking for HTTP monitors.

**Configuration:**
```bash
export HTTP_CLIENT="influx"  # or WS_CLIENT="influx"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain_metrics"
export TABLE_NAME="block_metrics"
export TOKEN="your-influxdb-token"
```

**Metrics Structure:**
- `name`: Monitor name
- `chain_name`: Chain/endpoint identifier
- `block_height`: Current block number
- `block_timestamp`: Block timestamp
- `os_timestamp`: System timestamp when received
- `diff`: Response time in milliseconds (for HTTP monitors)

## Examples

### Monitor Ethereum via WebSocket

```bash
#!/bin/bash
export NAME="ethereum-ws-monitor"
export WS_NAMES="ethereum-mainnet"
export WS="wss://mainnet.infura.io/ws/v3/YOUR_PROJECT_ID"
export WS_CLIENT="influx"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain"
export TABLE_NAME="ethereum_blocks"
export TOKEN="your-influxdb-token"

cargo run
```

### Monitor Multiple RPC Endpoints

```bash
#!/bin/bash
export NAME="multi-rpc-monitor"
export HTTP_NAMES="ethereum-rpc,polygon-rpc,base-rpc"
export HTTP="https://eth.rpc,https://polygon.rpc,https://base.rpc"
export HTTP_CLIENT="http"
export SERVER="http://localhost:7001"
export TOKEN="your-api-token"

cargo run
```

### Mixed Protocol Monitoring

```bash
#!/bin/bash
export NAME="comprehensive-monitor"
# WebSocket for real-time monitoring
export WS_NAMES="ethereum-ws,polygon-ws"
export WS="wss://eth.ws,wss://polygon.ws"
export WS_CLIENT="influx"
# HTTP for performance monitoring
export HTTP_NAMES="base-rpc,arbitrum-rpc"
export HTTP="https://base.rpc,https://arbitrum.rpc"
export HTTP_CLIENT="http"
# Shared configuration
export SERVER="http://localhost:7001"
export DB_HOST="http://localhost:8086"
export TOKEN="your-token"

cargo run
```

For more examples, check the `shell/` directory for various test scripts.

## Development

### Prerequisites

- Rust 1.70+ 
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running in Development

```bash
cargo run
```

## Architecture

The monitor agent uses a concurrent architecture where each WebSocket and HTTP endpoint runs in its own async task:

1. **MonitorManager**: Orchestrates multiple monitoring tasks for different protocols
2. **MetricsClient**: Abstraction for different metrics backends (HTTP/InfluxDB)
3. **WebSocket Connection**: Maintains persistent connections to blockchain nodes for real-time monitoring
4. **HTTP Polling**: Periodically polls HTTP endpoints and measures response times
5. **Block Processing**: Real-time processing of new block events with timestamp analysis
6. **Unified Task Creation**: Single `create_monitor_task` function handles both WebSocket and HTTP monitors

### Key Improvements in v1.0.10

- **Separate Configuration**: Independent configuration for WebSocket and HTTP monitors
- **Flexible Client Assignment**: Different monitor types can use different metrics backends
- **Response Time Tracking**: HTTP monitors now track and report response times
- **Functional Refactoring**: Improved code organization using functional programming patterns

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support and questions, please open an issue on GitHub or contact the maintainers.

---

**Version:** 1.0.10  
**Minimum Rust Version:** 1.70+


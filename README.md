# Blockchain Node Monitor Agent

A high-performance Rust-based monitoring agent for blockchain nodes that supports real-time tracking of multiple WebSocket endpoints with configurable metrics backends.

## Features

- 🚀 **Multi-Endpoint Monitoring**: Simultaneously monitor multiple blockchain node WebSocket endpoints
- 📊 **Multiple Metrics Backends**: Support for InfluxDB and HTTP-based metrics collection
- ⛓️ **Chain-Specific Monitoring**: Configure custom names for different blockchain networks
- 🔄 **Real-Time Block Tracking**: Monitor new blocks in real-time with timestamp analysis
- 🏗️ **Concurrent Architecture**: Efficient concurrent monitoring using Tokio async runtime
- 🌍 **Environment Configuration**: Full support for environment variable configuration
- 📈 **Scalable Design**: Easily scale to monitor multiple chains and endpoints

## Quick Start

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

#### Basic Usage

```bash
# Monitor a single WebSocket endpoint
export NAME="my-monitor"
export CHAIN_NAME="ethereum"
export WS="wss://mainnet.infura.io/ws/v3/YOUR_PROJECT_ID"
export CLIENT="http"
export SERVER="http://localhost:7001"
export TOKEN="your-token"

cargo run
```

#### Multi-Endpoint Monitoring

```bash
# Monitor multiple endpoints with different chain names
export NAME="multi-chain-monitor"
export CHAIN_NAME="ethereum,polygon,base"
export WS="wss://eth-mainnet.ws,wss://polygon-mainnet.ws,wss://base-mainnet.ws"
export CLIENT="influx"
export SERVER="http://localhost:8086"
export DB_NAME="blockchain_metrics"
export TOKEN="your-influx-token"

cargo run
```

## Configuration

### Environment Variables

| Variable | Description | Required | Default | Example |
|----------|-------------|----------|---------|---------|
| `NAME` | Monitor instance name | Yes | - | `"my-monitor"` |
| `CHAIN_NAME` | Chain names (comma-separated) | Yes | - | `"node1,node2"` |
| `CLIENT` | Metrics backend type | No | `http` | `http` or `influx` |
| `WS` | WebSocket URLs (comma-separated) | Yes | - | `"wss://eth1.ws,wss://eth2.ws"` |
| `HTTP` | HTTP URLs (comma-separated) | No | - | `"https://eth1.rpc,https://eth2.rpc"` |
| `SERVER` | Metrics server URL | No | `http://localhost:7777` | `"http://localhost:8086"` |
| `DB_HOST` | Database host | No | `http://localhost:8181` | `"http://influxdb:8086"` |
| `DB_NAME` | Database name | No | `test` | `"blockchain_metrics"` |
| `TABLE_NAME` | Table/measurement name | No | `ws_metric` | `"block_metrics"` |
| `TOKEN` | Authentication token | Yes | - | `"your-secret-token"` |

### Command Line Arguments

```bash
# View all available options
./target/release/monitor-agent --help

# Example with command line arguments
./target/release/monitor-agent \
  --name "my-monitor" \
  --chain-name "node1,node2" \
  --client influx \
  --ws "wss://eth1.ws,wss://eth2.ws" \
  --server "http://localhost:8086" \
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
  "chain_name": "ethereum",
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

Writes metrics directly to InfluxDB using the native client.

**Configuration:**
```bash
export CLIENT="influx"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain_metrics"
export TABLE_NAME="block_metrics"
export TOKEN="your-influxdb-token"
```

## Examples

### Monitor Ethereum Mainnet

```bash
#!/bin/bash
export NAME="ethereum-monitor"
export CHAIN_NAME="ethereum-mainnet"
export CLIENT="influx"
export WS="wss://mainnet.infura.io/ws/v3/YOUR_PROJECT_ID"
export DB_HOST="http://localhost:8086"
export DB_NAME="blockchain"
export TABLE_NAME="ethereum_blocks"
export TOKEN="your-influxdb-token"

cargo run
```

### Monitor Multiple Chains

```bash
#!/bin/bash
export NAME="multi-chain-monitor"
export CHAIN_NAME="ethereum,polygon,base"
export CLIENT="http"
export WS="wss://eth.ws,wss://polygon.ws,wss://base.ws"
export SERVER="http://localhost:7001"
export TOKEN="your-api-token"

cargo run
```

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

The monitor agent uses a concurrent architecture where each WebSocket endpoint runs in its own async task:

1. **MonitorManager**: Orchestrates multiple monitoring tasks
2. **MetricsClient**: Abstraction for different metrics backends (HTTP/InfluxDB)
3. **WebSocket Connection**: Maintains persistent connections to blockchain nodes
4. **Block Processing**: Real-time processing of new block events

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

**Version:** 1.0.9  
**Minimum Rust Version:** 1.70+


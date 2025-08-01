# New Configuration System Examples

## Separate WebSocket and HTTP Configuration

You can now configure different parameters for WebSocket and HTTP monitors separately:

### Basic Usage

```bash
# Configure WebSocket monitors
./monitor-agent \
  --ws-names "eth-mainnet,bsc-mainnet" \
  --ws "wss://eth.llamarpc.com,wss://bsc.llamarpc.com" \
  --ws-client http \
  --http-names "polygon-mainnet" \
  --http "https://polygon.llamarpc.com" \
  --http-client influx \
  --token "your-token"
```

### Environment Variable Configuration

```bash
export WS_NAMES="eth-mainnet,bsc-mainnet"
export WS="wss://eth.llamarpc.com,wss://bsc.llamarpc.com"
export WS_CLIENT="http"
export HTTP_NAMES="polygon-mainnet"
export HTTP="https://polygon.llamarpc.com"
export HTTP_CLIENT="influx"
export TOKEN="your-token"

./monitor-agent
```

### Mixed Configuration

```bash
# WebSocket uses HTTP client, HTTP uses InfluxDB client
./monitor-agent \
  --ws-names "ethereum" \
  --ws "wss://eth.llamarpc.com" \
  --ws-client http \
  --http-names "polygon" \
  --http "https://polygon.llamarpc.com" \
  --http-client influx \
  --token "your-token"
```

## Configuration Structure

- `--ws-names`: List of WebSocket monitor names
- `--ws`: List of WebSocket endpoints
- `--ws-client`: Client type for WebSocket monitors (http/influx)
- `--http-names`: List of HTTP monitor names
- `--http`: List of HTTP endpoints
- `--http-client`: Client type for HTTP monitors (http/influx)

## Advantages

1. **Flexibility**: Different client types can be configured for different monitor types
2. **Simplicity**: Uses unified `create_monitor_task` function to handle both types
3. **Extensibility**: Easy to add new monitor types
4. **Clear Configuration**: Each monitor type has independent configuration options

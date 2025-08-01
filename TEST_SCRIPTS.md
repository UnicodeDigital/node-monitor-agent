# Test Scripts Documentation

This project includes multiple test scripts to demonstrate different usage scenarios of the new configuration system:

## Test Script List

### 1. `test_multiple.sh` - Mixed Monitoring Test
- **Purpose**: Test simultaneous WebSocket and HTTP monitoring
- **Configuration**: 
  - WebSocket: Uses HTTP client
  - HTTP: Uses InfluxDB client
- **Usage**: `./test_multiple.sh`

### 2. `test_ws_only.sh` - WebSocket Only Monitoring Test
- **Purpose**: Test WebSocket monitoring only
- **Configuration**: 
  - Monitors 3 WebSocket endpoints (Ethereum, Polygon, BSC)
  - Uses HTTP client
- **Usage**: `./test_ws_only.sh`

### 3. `test_http_only.sh` - HTTP Only Monitoring Test
- **Purpose**: Test HTTP monitoring only
- **Configuration**: 
  - Monitors 2 HTTP endpoints (Ethereum, Polygon)
  - Uses InfluxDB client
- **Usage**: `./test_http_only.sh`

### 4. `test_mixed.sh` - Mixed Client Types Test
- **Purpose**: Test different client type combinations
- **Configuration**: 
  - WebSocket: Uses InfluxDB client
  - HTTP: Uses HTTP client
- **Usage**: `./test_mixed.sh`

### 5. `test_http.sh` - Legacy HTTP Test (Compatibility)
- **Purpose**: Original HTTP test script
- **Usage**: `./test_http.sh`

### 6. `test_influx.sh` - Legacy InfluxDB Test (Compatibility)
- **Purpose**: Original InfluxDB test script
- **Usage**: `./test_influx.sh`

## Usage Instructions

1. **Choose the appropriate test script**:
   ```bash
   # Full feature test
   ./test_multiple.sh
   
   # WebSocket only test
   ./test_ws_only.sh
   
   # HTTP only test
   ./test_http_only.sh
   
   # Mixed client types test
   ./test_mixed.sh
   ```

2. **View configuration**:
   Each script displays detailed configuration information when running

3. **Modify configuration**:
   Edit the respective test script to modify endpoints, client types, etc.

## Configuration Parameters

### WebSocket Configuration
- `WS_NAMES`: WebSocket monitor names list (comma-separated)
- `WS`: WebSocket endpoint URLs list (comma-separated)
- `WS_CLIENT`: WebSocket monitor client type (`http` or `influx`)

### HTTP Configuration
- `HTTP_NAMES`: HTTP monitor names list (comma-separated)
- `HTTP`: HTTP endpoint URLs list (comma-separated)
- `HTTP_CLIENT`: HTTP monitor client type (`http` or `influx`)

### Shared Configuration
- `SERVER`: HTTP server address
- `DB_HOST`: InfluxDB host address
- `DB_NAME`: Database name
- `TABLE_NAME`: Table name
- `TOKEN`: Authentication token

## Important Notes

1. **Token Security**: Test scripts contain example tokens, use real tokens in production
2. **Endpoint Availability**: Ensure test endpoint URLs are accessible
3. **Service Dependencies**: Some tests require InfluxDB or HTTP server to be running
4. **Permissions**: Ensure scripts have execute permissions (`chmod +x *.sh`)

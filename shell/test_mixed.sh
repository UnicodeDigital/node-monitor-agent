#!/bin/bash

# Test mixed client types - WebSocket with InfluxDB, HTTP with HTTP client
echo "Testing mixed client types monitoring..."

# Set environment variables for WebSocket monitoring (using InfluxDB client)
export WS_NAMES="ethereum-ws,polygon-ws"
export WS="wss://eth.llamarpc.com,wss://polygon.llamarpc.com"
export WS_CLIENT="influx"

# Set environment variables for HTTP monitoring (using HTTP client)
export HTTP_NAMES="bsc-http,avalanche-http"
export HTTP="https://bsc.llamarpc.com,https://avalanche.llamarpc.com"
export HTTP_CLIENT="http"

# Shared configuration
export SERVER="http://localhost:7001"
export DB_HOST="http://localhost:8181"
export DB_NAME="test"
export TABLE_NAME="mixed_test"
export TOKEN="apiv3_D7YA9JRwSRlnB1a4P1Qq3QFHDGIikm7fl7SV3DlgzLK-j6uilqL4AihrB8arsShJrjDm1YSADcerULPgmQQJ4g"

# Display configuration
echo "========================================="
echo "Mixed Client Types Configuration:"
echo "========================================="
echo "WebSocket Endpoints (InfluxDB client):"
echo "  Names: $WS_NAMES"
echo "  URLs:  $WS"
echo "  Client: $WS_CLIENT"
echo ""
echo "HTTP Endpoints (HTTP client):"
echo "  Names: $HTTP_NAMES"
echo "  URLs:  $HTTP"
echo "  Client: $HTTP_CLIENT"
echo ""
echo "Shared Settings:"
echo "  Server: $SERVER"
echo "  DB Host: $DB_HOST"
echo "  DB Name: $DB_NAME"
echo "  Table: $TABLE_NAME"
echo "========================================="

# Run monitoring program
echo "Starting mixed client type monitors..."
cargo run

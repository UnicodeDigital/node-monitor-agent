#!/bin/bash

# Test multiple WS and HTTP endpoint monitoring with new configuration system
echo "Testing multiple WebSocket and HTTP endpoints monitoring..."

# Set environment variables for WebSocket monitoring
export WS_NAMES="base-ws-zan,base-ws-local"
export WS="ws://base.zan.top:8546,ws://65.108.5.46:8546"
export WS_CLIENT="http"

# Set environment variables for HTTP monitoring
export HTTP_NAMES="base-http-zan,bsc-http-local"
export HTTP="http://base.zan.top:8546,http://65.108.5.46:8546"
export HTTP_CLIENT="influx"

# Shared configuration
export SERVER="http://localhost:7001"
export DB_NAME="test"
export TABLE_NAME="multi_test"
export TOKEN="apiv3_D7YA9JRwSRlnB1a4P1Qq3QFHDGIikm7fl7SV3DlgzLK-j6uilqL4AihrB8arsShJrjDm1YSADcerULPgmQQJ4g"

# Display configuration
echo "========================================="
echo "Configuration Summary:"
echo "========================================="
echo "WebSocket Endpoints:"
echo "  Names: $WS_NAMES"
echo "  URLs:  $WS"
echo "  Client: $WS_CLIENT"
echo ""
echo "HTTP Endpoints:"
echo "  Names: $HTTP_NAMES"
echo "  URLs:  $HTTP"
echo "  Client: $HTTP_CLIENT"
echo ""
echo "Shared Settings:"
echo "  Server: $SERVER"
echo "  DB Name: $DB_NAME"
echo "  Table: $TABLE_NAME"
echo "========================================="

# Run monitoring program
echo "Starting monitor with multiple WebSocket and HTTP endpoints..."
cargo run

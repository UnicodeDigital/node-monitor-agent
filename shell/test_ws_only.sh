#!/bin/bash

# Test WebSocket only monitoring
echo "Testing WebSocket only monitoring..."

# Set environment variables for WebSocket monitoring
export NAME="ws(zan/local)"
export WS_NAMES="base(zan),base(local)"
export WS="ws://base.zan.top:8546,ws://65.108.5.46:8546"
export WS_CLIENT="http"

# Shared configuration
export SERVER="http://localhost:7001"
export DB_NAME="test"
export TABLE_NAME="ws_only_test"
export TOKEN="apiv3_D7YA9JRwSRlnB1a4P1Qq3QFHDGIikm7fl7SV3DlgzLK-j6uilqL4AihrB8arsShJrjDm1YSADcerULPgmQQJ4g"

# Display configuration
echo "========================================="
echo "WebSocket Only Configuration:"
echo "========================================="
echo "WebSocket Endpoints:"
echo "  Names: $WS_NAMES"
echo "  URLs:  $WS"
echo "  Client: $WS_CLIENT"
echo ""
echo "Shared Settings:"
echo "  Server: $SERVER"
echo "  DB Name: $DB_NAME"
echo "  Table: $TABLE_NAME"
echo "========================================="

# Run monitoring program
echo "Starting WebSocket only monitors..."
cargo run

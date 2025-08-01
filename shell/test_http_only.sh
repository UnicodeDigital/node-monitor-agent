#!/bin/bash

# Test HTTP only monitoring
echo "Testing HTTP only monitoring..."

# Set environment variables for HTTP monitoring
export NAME="http(zan/local)"
export HTTP_NAMES="zan-base-http,local-base-http"
export HTTP="http://base.zan.top:8545,http://65.108.5.46:8545"
export HTTP_CLIENT="influx"

# Shared configuration
export SERVER="http://localhost:7001"
export DB_HOST="http://localhost:8181"
export DB_NAME="test"
export TABLE_NAME="http_only_test"
export TOKEN="apiv3_D7YA9JRwSRlnB1a4P1Qq3QFHDGIikm7fl7SV3DlgzLK-j6uilqL4AihrB8arsShJrjDm1YSADcerULPgmQQJ4g"

# Display configuration
echo "========================================="
echo "HTTP Only Configuration:"
echo "========================================="
echo "HTTP Endpoints:"
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
echo "Starting HTTP only monitors..."
cargo run

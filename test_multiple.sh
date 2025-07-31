#!/bin/bash

# Test multiple WS endpoint monitoring
echo "Testing multiple WS endpoints monitoring..."

# Set environment variables
export NAME="base(v0)"
export CHAIN_NAME="base(zan),base(local)"
export CLIENT="http"
export WS="ws://base.zan.top:8546,ws://65.108.5.46:8546"
export SERVER="http://localhost:7001"
export DB_NAME="test"
export TABLE_NAME="base_test_2"
export TOKEN="apiv3_D7YA9JRwSRlnB1a4P1Qq3QFHDGIikm7fl7SV3DlgzLK-j6uilqL4AihrB8arsShJrjDm1YSADcerULPgmQQJ4g"

# Run monitoring program
echo "Starting monitor with multiple WS endpoints:"
echo "WS endpoints: $WS"

cargo run

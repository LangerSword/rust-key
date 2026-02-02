#!/bin/bash

# Webhook Test CLI Tool
# Simple script to test webhook connectivity and POST requests

echo "=========================================="
echo "  Webhook Test Tool"
echo "=========================================="
echo ""

# Check if URL is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <WEBHOOK_URL>"
    echo ""
    echo "Example:"
    echo "  $0 https://webhook.site/your-unique-id"
    echo ""
    exit 1
fi

WEBHOOK_URL="$1"

# Validate URL format
if [[ ! "$WEBHOOK_URL" =~ ^https?:// ]]; then
    echo "❌ Error: URL must start with http:// or https://"
    exit 1
fi

echo "Testing webhook: $WEBHOOK_URL"
echo ""

# Test 1: Simple connectivity test
echo "Test 1: Checking connectivity..."
if command -v curl &> /dev/null; then
    if curl -s -o /dev/null -w "%{http_code}" --connect-timeout 5 "$WEBHOOK_URL" | grep -qE "^[2-5][0-9]{2}$"; then
        echo "✅ Webhook is reachable"
    else
        echo "❌ Webhook is not reachable or returned an error"
        echo "   This might be normal if the webhook requires specific data"
    fi
else
    echo "⚠️  curl not found, skipping connectivity test"
fi

echo ""

# Test 2: Send test POST request with JSON payload
echo "Test 2: Sending test POST request..."
TEST_PAYLOAD='{
  "keystrokes": [
    {
      "timestamp": "2024-01-01 12:00:00.000",
      "device": "Test Device",
      "key": "t"
    },
    {
      "timestamp": "2024-01-01 12:00:00.100",
      "device": "Test Device",
      "key": "e"
    },
    {
      "timestamp": "2024-01-01 12:00:00.200",
      "device": "Test Device",
      "key": "s"
    },
    {
      "timestamp": "2024-01-01 12:00:00.300",
      "device": "Test Device",
      "key": "t"
    }
  ]
}'

if command -v curl &> /dev/null; then
    RESPONSE=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "$TEST_PAYLOAD" \
        -w "\nHTTP_CODE:%{http_code}" \
        --connect-timeout 10 \
        "$WEBHOOK_URL" 2>&1)
    
    HTTP_CODE=$(echo "$RESPONSE" | grep "HTTP_CODE:" | cut -d':' -f2)
    BODY=$(echo "$RESPONSE" | sed '/HTTP_CODE:/d')
    
    if [ -n "$HTTP_CODE" ] && [ "$HTTP_CODE" -ge 200 ] && [ "$HTTP_CODE" -lt 300 ]; then
        echo "✅ POST request successful (HTTP $HTTP_CODE)"
        if [ -n "$BODY" ]; then
            echo "   Response: $BODY"
        fi
    elif [ -n "$HTTP_CODE" ]; then
        echo "⚠️  POST request returned HTTP $HTTP_CODE"
        if [ -n "$BODY" ]; then
            echo "   Response: $BODY"
        fi
    else
        echo "❌ POST request failed"
        echo "   Error: $RESPONSE"
    fi
else
    echo "❌ curl not found. Please install curl to test webhooks."
    exit 1
fi

echo ""
echo "=========================================="
echo "  Test Complete"
echo "=========================================="
echo ""
echo "If the tests passed, your webhook is ready to use with:"
echo "  sudo ./target/release/rust-key $WEBHOOK_URL"
echo ""

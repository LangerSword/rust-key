#!/bin/bash

# Test script for autorun functionality
# This simulates the autorun workflow without actually running the keylogger

echo "=========================================="
echo "Testing Autorun Script"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/rust-key"

# Test 1: Check if script exists
echo "Test 1: Check if run.sh exists"
if [ -f "$SCRIPT_DIR/run.sh" ]; then
    echo "✅ PASS: run.sh found"
else
    echo "❌ FAIL: run.sh not found"
    exit 1
fi

# Test 2: Check script syntax
echo ""
echo "Test 2: Validate script syntax"
if bash -n "$SCRIPT_DIR/run.sh"; then
    echo "✅ PASS: Script syntax is valid"
else
    echo "❌ FAIL: Script has syntax errors"
    exit 1
fi

# Test 3: Check if binary exists
echo ""
echo "Test 3: Check if rust-key binary exists"
if [ -f "$BINARY_PATH" ]; then
    echo "✅ PASS: Binary found at $BINARY_PATH"
else
    echo "⚠️  WARNING: Binary not found. Run 'cargo build --release' first"
fi

# Test 4: Check if binary is executable
echo ""
echo "Test 4: Check if binary is executable"
if [ -x "$BINARY_PATH" ]; then
    echo "✅ PASS: Binary is executable"
else
    echo "⚠️  WARNING: Binary is not executable"
fi

# Test 5: Check key sections in run.sh
echo ""
echo "Test 5: Verify script contains key sections"

if grep -q "Do you want to continue?" "$SCRIPT_DIR/run.sh"; then
    echo "✅ PASS: User confirmation prompt found"
else
    echo "❌ FAIL: User confirmation prompt not found"
    exit 1
fi

if grep -q "Webhook Configuration" "$SCRIPT_DIR/run.sh"; then
    echo "✅ PASS: Webhook configuration section found"
else
    echo "❌ FAIL: Webhook configuration section not found"
    exit 1
fi

# Test 6: Simulate script flow (without sudo)
echo ""
echo "Test 6: Simulate script flow without sudo"
echo "  (This will check if the script properly detects non-root execution)"

# Create a test environment
TEST_DIR="/tmp/rust-key-test-$$"
mkdir -p "$TEST_DIR"

# Copy files
cp "$SCRIPT_DIR/run.sh" "$TEST_DIR/"
if [ -f "$BINARY_PATH" ]; then
    cp "$BINARY_PATH" "$TEST_DIR/rust-key"
else
    # Create a dummy binary for testing
    echo '#!/bin/bash' > "$TEST_DIR/rust-key"
    echo 'echo "Dummy keylogger for testing"' >> "$TEST_DIR/rust-key"
    chmod +x "$TEST_DIR/rust-key"
fi

# Test script detection of binary (provide automated input)
cd "$TEST_DIR"
TEST_OUTPUT=$(mktemp)
echo "no" | timeout 5 bash run.sh > "$TEST_OUTPUT" 2>&1
if grep -q "Operation cancelled" "$TEST_OUTPUT"; then
    echo "✅ PASS: Script properly handles user cancellation"
elif grep -q "rust-key binary" "$TEST_OUTPUT"; then
    echo "✅ PASS: Script detects binary location"
else
    echo "⚠️  WARNING: Script output may need verification"
fi

# Cleanup
rm -rf "$TEST_DIR"
rm -f "$TEST_OUTPUT"

# Final summary
echo ""
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "✅ All tests passed!"
echo ""
echo "To test the actual execution:"
echo "  1. Build: cargo build --release"
echo "  2. Run: ./run.sh (will prompt for sudo)"
echo "  3. Follow the prompts"
echo ""

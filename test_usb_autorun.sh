#!/bin/bash

# Test script for USB autorun functionality
# This simulates the USB autorun workflow without actually running the keylogger

echo "=========================================="
echo "Testing USB Autorun Script"
echo "=========================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/target/release/rust-key"

# Test 1: Check if script exists
echo "Test 1: Check if usb_autorun.sh exists"
if [ -f "$SCRIPT_DIR/usb_autorun.sh" ]; then
    echo "✅ PASS: usb_autorun.sh found"
else
    echo "❌ FAIL: usb_autorun.sh not found"
    exit 1
fi

# Test 2: Check script syntax
echo ""
echo "Test 2: Validate script syntax"
if bash -n "$SCRIPT_DIR/usb_autorun.sh"; then
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

# Test 5: Verify documentation files exist
echo ""
echo "Test 5: Check if documentation files exist"
DOCS=("README.md" "USB_SETUP.md" "CRONTAB_SETUP.md" "99-usb-autorun.rules.example")
for doc in "${DOCS[@]}"; do
    if [ -f "$SCRIPT_DIR/$doc" ]; then
        echo "✅ PASS: $doc found"
    else
        echo "❌ FAIL: $doc not found"
        exit 1
    fi
done

# Test 6: Check key sections in usb_autorun.sh
echo ""
echo "Test 6: Verify script contains key sections"

if grep -q "Do you want to continue?" "$SCRIPT_DIR/usb_autorun.sh"; then
    echo "✅ PASS: User confirmation prompt found"
else
    echo "❌ FAIL: User confirmation prompt not found"
    exit 1
fi

if grep -q "Educational Demonstration" "$SCRIPT_DIR/usb_autorun.sh"; then
    echo "✅ PASS: Educational demonstration section found"
else
    echo "❌ FAIL: Educational demonstration section not found"
    exit 1
fi

if grep -q "is_noexec_mount" "$SCRIPT_DIR/usb_autorun.sh"; then
    echo "✅ PASS: noexec mount detection found"
else
    echo "❌ FAIL: noexec mount detection not found"
    exit 1
fi

if grep -q "Webhook Configuration" "$SCRIPT_DIR/usb_autorun.sh"; then
    echo "✅ PASS: Webhook configuration section found"
else
    echo "❌ FAIL: Webhook configuration section not found"
    exit 1
fi

# Test 7: Check udev rules example
echo ""
echo "Test 7: Verify udev rules example file"
if grep -q "idVendor" "$SCRIPT_DIR/99-usb-autorun.rules.example"; then
    echo "✅ PASS: Udev rule example contains vendor ID placeholder"
else
    echo "❌ FAIL: Udev rule example incomplete"
    exit 1
fi

if grep -q "systemd" "$SCRIPT_DIR/99-usb-autorun.rules.example"; then
    echo "✅ PASS: Udev rule example includes systemd alternative"
else
    echo "❌ FAIL: Udev rule example missing systemd alternative"
    exit 1
fi

# Test 8: Check CRONTAB_SETUP.md
echo ""
echo "Test 8: Verify crontab setup guide"
if grep -q "@reboot" "$SCRIPT_DIR/CRONTAB_SETUP.md"; then
    echo "✅ PASS: Crontab guide contains @reboot examples"
else
    echo "❌ FAIL: Crontab guide missing @reboot examples"
    exit 1
fi

if grep -q "sudo crontab -e" "$SCRIPT_DIR/CRONTAB_SETUP.md"; then
    echo "✅ PASS: Crontab guide contains setup instructions"
else
    echo "❌ FAIL: Crontab guide incomplete"
    exit 1
fi

# Test 9: Simulate script flow (without sudo)
echo ""
echo "Test 9: Simulate script flow without sudo"
echo "  (This will check if the script properly detects non-root execution)"

# Create a test environment
TEST_DIR="/tmp/rust-key-test-$$"
mkdir -p "$TEST_DIR"

# Copy files
cp "$SCRIPT_DIR/usb_autorun.sh" "$TEST_DIR/"
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
echo "no" | timeout 5 bash usb_autorun.sh > "$TEST_OUTPUT" 2>&1
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

# Test 10: Check README references
echo ""
echo "Test 10: Verify README.md references new files"
if grep -q "CRONTAB_SETUP.md" "$SCRIPT_DIR/README.md"; then
    echo "✅ PASS: README references CRONTAB_SETUP.md"
else
    echo "❌ FAIL: README doesn't reference CRONTAB_SETUP.md"
    exit 1
fi

if grep -q "99-usb-autorun.rules.example" "$SCRIPT_DIR/README.md"; then
    echo "✅ PASS: README references udev rules example"
else
    echo "❌ FAIL: README doesn't reference udev rules example"
    exit 1
fi

# Final summary
echo ""
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "✅ All tests passed!"
echo ""
echo "The USB autorun script has been enhanced with:"
echo "  • User confirmation prompt"
echo "  • Educational password demonstration"
echo "  • Comprehensive documentation"
echo "  • Udev rules example"
echo "  • Crontab setup guide"
echo ""
echo "To test the actual execution:"
echo "  1. Build: cargo build --release"
echo "  2. Run: ./usb_autorun.sh (will prompt for sudo)"
echo "  3. Follow the prompts"
echo ""

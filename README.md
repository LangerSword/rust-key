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
# With sudo (recommended)
sudo ./target/release/rust-key

# Or add your user to the input group first
sudo usermod -a -G input $USER
# Log out and log back in
./target/release/rust-key
```

The program will:
1. Set the default locale to en_US.UTF-8
2. Scan for all input devices (including USB keyboards)
3. Display detected keyboard devices with their type (USB/Internal)
4. Start monitoring all keyboards
5. Log keystrokes to `keylog.txt` with timestamps

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
Monitoring device: AT Translated Set 2 keyboard (/dev/input/event3)
Monitoring device: USB Keyboard (/dev/input/event4)
[2024-01-01 12:00:05.123] [USB Keyboard] Key: h
[2024-01-01 12:00:05.234] [USB Keyboard] Key: e
[2024-01-01 12:00:05.345] [USB Keyboard] Key: l
[2024-01-01 12:00:05.456] [USB Keyboard] Key: l
[2024-01-01 12:00:05.567] [USB Keyboard] Key: o
```

## Technical Details

### Linux Implementation
- Uses `evdev` crate to access `/dev/input/event*` devices
- Automatically detects USB keyboards by checking physical path
- Non-blocking event loop for efficient processing
- Supports multiple simultaneous keyboards
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

## Security Considerations

- This tool captures all keyboard input, including passwords
- The log file contains sensitive information
- Always secure the log file with appropriate permissions
- Delete or encrypt logs when no longer needed
- Only use on systems you own or have explicit permission to monitor
- Be aware of your local laws regarding keylogging

## Development Status

- ✅ Linux/evdev implementation complete
- ✅ USB keyboard detection complete
- ✅ en_US.UTF-8 locale support complete
- ✅ Cross-platform dependency structure complete
- 🚧 Windows implementation (stub ready)
- 🚧 macOS implementation (stub ready)

## Contributing

Contributions are welcome! Areas for contribution:
- Complete Windows implementation using winapi
- Complete macOS implementation using Core Graphics
- Add encryption for log files
- Add network transmission capabilities
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

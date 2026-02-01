# Usage Examples

This document provides practical examples for using rust-key.

## Basic Usage

### Start Monitoring (Linux)
```bash
# Run with sudo (simplest method)
sudo ./target/release/rust-key
```

### Output Example
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

hello world
```

## Common Scenarios

### Testing USB Keyboard Detection
1. Start the program
2. Plug in a USB keyboard
3. The program should automatically detect and start monitoring it
4. Note: You may need to restart the program to detect newly connected devices

### Monitoring Multiple Keyboards
The program automatically monitors all detected keyboard devices simultaneously:
```bash
sudo ./target/release/rust-key
# All keyboards (USB, built-in, wireless) will be monitored
```

### Viewing the Log File
```bash
# Real-time monitoring
tail -f keylog.txt

# View entire log
cat keylog.txt

# View with timestamps
less keylog.txt
```

### Stopping the Keylogger
```bash
# Press Ctrl+C in the terminal where it's running
# Or send SIGTERM
kill <pid>
```

## Advanced Usage

### Running as a Background Service

#### Using systemd (recommended for production)
Create a service file:
```bash
sudo nano /etc/systemd/system/rust-key.service
```

Add this content:
```ini
[Unit]
Description=Rust Keylogger Service
After=network.target

[Service]
Type=simple
User=root
ExecStart=/path/to/rust-key/target/release/rust-key
WorkingDirectory=/path/to/rust-key
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-key.service
sudo systemctl start rust-key.service
sudo systemctl status rust-key.service
```

View logs:
```bash
sudo journalctl -u rust-key.service -f
```

#### Using nohup (simple background process)
```bash
sudo nohup ./target/release/rust-key > /dev/null 2>&1 &
echo $! > rust-key.pid  # Save PID for later
```

Stop with:
```bash
sudo kill $(cat rust-key.pid)
```

### Custom Log Location
Currently, the log is saved to `keylog.txt` in the current directory. To use a different location, modify `src/main.rs`:

```rust
let log_path = "/var/log/rust-key.log";  // Change this line
```

Then rebuild:
```bash
cargo build --release
```

### Filtering Specific Keyboards
If you want to monitor only USB keyboards, modify `src/linux.rs` to skip non-USB devices:

```rust
// In the device scanning loop, after checking phys.contains("usb"):
if !phys.contains("usb") {
    continue;  // Skip non-USB devices
}
```

## Security Best Practices

### Secure Log Files
```bash
# Set appropriate permissions on the log file
chmod 600 keylog.txt
chown root:root keylog.txt

# Or encrypt the log file
gpg --encrypt --recipient your@email.com keylog.txt
```

### Rotate Logs
```bash
# Create a log rotation script
cat > rotate-keylog.sh << 'EOF'
#!/bin/bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mv keylog.txt "keylog_$TIMESTAMP.txt"
gzip "keylog_$TIMESTAMP.txt"
# Optionally remove old logs
find . -name "keylog_*.txt.gz" -mtime +7 -delete
EOF

chmod +x rotate-keylog.sh
```

Add to crontab:
```bash
crontab -e
# Add: 0 0 * * * /path/to/rotate-keylog.sh
```

### Network Transmission (Future Feature)
While not currently implemented, you could extend the code to send logs over the network:
1. Add a networking crate (e.g., reqwest, tokio)
2. Modify the logging function to batch and send logs
3. Implement encryption for transmission

## Troubleshooting Usage Issues

### Permission Denied
```bash
# Check device permissions
ls -l /dev/input/event*

# Verify group membership
groups

# Test with sudo
sudo ./target/release/rust-key
```

### No Keys Captured
1. Verify devices are detected:
   ```bash
   sudo evtest
   # Select your keyboard and test keys
   ```

2. Check if the program is actually running:
   ```bash
   ps aux | grep rust-key
   ```

3. Verify log file is being written:
   ```bash
   ls -lh keylog.txt
   tail -f keylog.txt
   ```

### High CPU Usage
This shouldn't normally happen, but if it does:
1. Check the number of devices being monitored
2. Verify you're running the release build (not debug)
3. Consider adjusting the sleep interval in `src/linux.rs`

## Integration Examples

### Parse Log File with Python
```python
import re
from datetime import datetime

def parse_keylog(filename):
    with open(filename, 'r') as f:
        for line in f:
            # Match log entries
            match = re.match(r'\[([^\]]+)\] \[([^\]]+)\] Key: (.+)', line)
            if match:
                timestamp, device, key = match.groups()
                print(f"{timestamp}: {device} pressed '{key}'")

parse_keylog('keylog.txt')
```

### Real-time Alert Script
```bash
#!/bin/bash
# Alert when sensitive keywords are typed

tail -f keylog.txt | while read line; do
    if echo "$line" | grep -iq "password\|secret\|confidential"; then
        echo "ALERT: Sensitive keyword detected at $(date)"
        # Could send email, notification, etc.
    fi
done
```

## Performance Considerations

### Resource Usage
- Memory: ~1-2 MB
- CPU: <1% on idle, <5% during heavy typing
- Disk I/O: Minimal, logs are flushed periodically

### Optimization Tips
1. Use release builds for production
2. Consider log rotation for long-running deployments
3. If monitoring many devices, test performance impact

## Legal and Ethical Usage

### Authorized Testing Scenarios
✅ Security testing on your own systems
✅ Research in controlled environments
✅ Educational demonstrations with consent
✅ Monitoring systems you own or manage

### Unauthorized Usage Scenarios
❌ Monitoring others without consent
❌ Installing on shared/public computers
❌ Capturing credentials for unauthorized access
❌ Any use that violates local laws or regulations

### Documentation
Always document:
- Authorization obtained
- Purpose of monitoring
- Duration of monitoring
- Data handling procedures
- Disposal of captured data

## Support

For questions or issues:
- GitHub Issues: https://github.com/LangerSword/rust-key/issues
- Check INSTALL.md for installation help
- Check README.md for general information

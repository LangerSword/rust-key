# Installation Guide

This guide provides detailed installation instructions for rust-key on various operating systems.

## Table of Contents
- [Linux Installation](#linux-installation)
  - [Arch Linux](#arch-linux)
  - [Ubuntu/Debian](#ubuntudebian)
  - [Fedora/RHEL](#fedorarhel)
  - [Other Distributions](#other-distributions)
- [Windows Installation](#windows-installation)
- [macOS Installation](#macos-installation)
- [Troubleshooting](#troubleshooting)

## Linux Installation

### Arch Linux

Arch Linux is the primary target platform for this project.

#### Prerequisites
```bash
# Update system
sudo pacman -Syu

# Install Rust and Cargo
sudo pacman -S rust cargo

# Install base development tools (if not already installed)
sudo pacman -S base-devel
```

#### Build and Install
```bash
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release version
cargo build --release

# The binary will be at: ./target/release/rust-key
```

#### Set Up Permissions
```bash
# Option 1: Add your user to the input group (recommended)
sudo usermod -a -G input $USER
# Log out and log back in for changes to take effect

# Option 2: Run with sudo each time
sudo ./target/release/rust-key
```

### Ubuntu/Debian

#### Prerequisites
```bash
# Update package list
sudo apt-get update

# Install Rust (using rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build dependencies
sudo apt-get install build-essential pkg-config
```

#### Build and Install
```bash
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release version
cargo build --release
```

#### Set Up Permissions
```bash
# Add your user to the input group
sudo usermod -a -G input $USER
# Log out and log back in
```

### Fedora/RHEL

#### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install development tools
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config
```

#### Build and Install
```bash
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release version
cargo build --release
```

#### Set Up Permissions
```bash
# Add your user to the input group
sudo usermod -a -G input $USER
# Log out and log back in
```

### Other Distributions

For other Linux distributions:
1. Install Rust using [rustup](https://rustup.rs/)
2. Install your distribution's equivalent of build-essential
3. Follow the build instructions above
4. Ensure your user has access to `/dev/input/event*` devices

## Windows Installation

**Note:** Windows implementation is currently in development. The dependencies are included but the functionality is not yet complete.

### Prerequisites
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Install Visual Studio Build Tools or Visual Studio with C++ development tools

### Build (When Available)
```powershell
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release version
cargo build --release

# The executable will be at: .\target\release\rust-key.exe
```

### Run (When Available)
```powershell
# Run as Administrator
# Right-click on Command Prompt or PowerShell and select "Run as Administrator"
.\target\release\rust-key.exe
```

## macOS Installation

**Note:** macOS implementation is currently in development. The dependencies are included but the functionality is not yet complete.

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Xcode Command Line Tools
xcode-select --install
```

### Build (When Available)
```bash
# Clone the repository
git clone https://github.com/LangerSword/rust-key.git
cd rust-key

# Build the release version
cargo build --release
```

### Grant Permissions (When Available)
1. Go to System Preferences > Security & Privacy > Privacy
2. Select "Accessibility" from the left panel
3. Click the lock to make changes
4. Add the rust-key binary to the list of allowed applications

### Run (When Available)
```bash
sudo ./target/release/rust-key
```

## Troubleshooting

### Linux: "No input devices found!"
**Cause:** Your user doesn't have permission to access `/dev/input/event*` devices.

**Solutions:**
1. Run with sudo: `sudo ./target/release/rust-key`
2. Add your user to the input group:
   ```bash
   sudo usermod -a -G input $USER
   ```
   Then log out and log back in.

### Linux: "Permission denied" when accessing devices
**Cause:** Even with input group membership, some systems require additional configuration.

**Solution:**
Create a udev rule:
```bash
# Create a udev rule file
sudo nano /etc/udev/rules.d/99-input.rules

# Add this line:
KERNEL=="event*", SUBSYSTEM=="input", MODE="0660", GROUP="input"

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Build Errors: "failed to compile evdev"
**Cause:** Missing build dependencies.

**Solution:**
- Arch: `sudo pacman -S base-devel`
- Ubuntu/Debian: `sudo apt-get install build-essential pkg-config`
- Fedora: `sudo dnf groupinstall "Development Tools"`

### USB Keyboard Not Detected
**Cause:** The USB keyboard might not be creating an evdev device.

**Solution:**
1. Check available devices:
   ```bash
   ls -l /dev/input/event*
   ```
2. Check device information:
   ```bash
   sudo evtest
   ```
3. Verify USB connection:
   ```bash
   lsusb
   dmesg | tail -50
   ```

### Locale Issues
The program sets `LANG=en_US.UTF-8` by default. If you need a different locale:
1. Edit `src/main.rs`
2. Change the `DEFAULT_LOCALE` constant
3. Rebuild: `cargo build --release`

## Verification

To verify your installation:

1. Check Rust version:
   ```bash
   rustc --version
   cargo --version
   ```

2. Build the project:
   ```bash
   cd rust-key
   cargo build --release
   ```

3. Check for errors in the build output

4. Try running (with sudo if needed):
   ```bash
   sudo ./target/release/rust-key
   ```

5. Press Ctrl+C to stop after verifying it starts

## Support

For issues or questions:
- GitHub Issues: https://github.com/LangerSword/rust-key/issues
- Check the main README.md for additional information

## Contributing

If you've successfully installed on a system not listed here, please contribute installation instructions via a pull request!

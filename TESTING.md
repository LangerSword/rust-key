# Testing Guide

This document describes how to test the rust-key keylogger.

## Prerequisites

- Linux system (Arch Linux, Ubuntu, Debian, Fedora, etc.)
- Rust toolchain installed
- sudo/root access for testing input device access

## Building for Testing

```bash
# Debug build (faster compilation, includes debug symbols)
cargo build

# Release build (optimized)
cargo build --release
```

## Unit Testing

Currently, this project focuses on integration testing due to the nature of hardware interaction. Unit tests can be added for:
- Key formatting functions
- Configuration parsing
- Log file operations

To add and run tests:
```bash
# Add tests to modules with #[cfg(test)] mod tests { ... }
# Run tests
cargo test
```

## Integration Testing

### Test 1: Basic Compilation
```bash
cd /home/runner/work/rust-key/rust-key
cargo build --release
echo "Test 1: $([ $? -eq 0 ] && echo 'PASSED' || echo 'FAILED')"
```

Expected: Build succeeds without errors

### Test 2: Binary Execution
```bash
timeout 2 ./target/release/rust-key 2>&1 || true
```

Expected output should include:
- "Rust Keylogger starting..."
- "Default locale: en_US.UTF-8"
- "Running on Linux with evdev support"

### Test 3: Locale Setting
```bash
./target/release/rust-key 2>&1 | grep "en_US.UTF-8" || echo "FAILED"
```

Expected: Should display "Default locale: en_US.UTF-8"

### Test 4: Device Detection (requires sudo)
```bash
sudo timeout 5 ./target/release/rust-key 2>&1
```

Expected output variations:
- If devices found: Lists keyboard devices with names and paths
- If no devices: "No input devices found!" with permission instructions

### Test 5: USB Keyboard Detection (requires USB keyboard)
1. Plug in a USB keyboard
2. Run: `sudo ./target/release/rust-key`
3. Look for device with "Type: USB Keyboard" in output

Expected: USB keyboards should be identified separately from internal keyboards

### Test 6: Key Logging (requires sudo and keyboard)
1. Start the logger: `sudo ./target/release/rust-key`
2. Type some test keys (e.g., "test123")
3. Press Ctrl+C to stop
4. Check the log file: `cat keylog.txt`

Expected: Log file should contain the typed characters with timestamps

### Test 7: Multiple Keyboard Support
1. Connect multiple USB keyboards
2. Run: `sudo ./target/release/rust-key`
3. Type on different keyboards

Expected: All keyboards should be detected and logged

### Test 8: Log File Format
1. Run the keylogger: `sudo ./target/release/rust-key`
2. Type some keys
3. Stop and check: `cat keylog.txt`

Expected format:
```
=== Keylogger Started at YYYY-MM-DD HH:MM:SS.mmm ===
Locale: en_US.UTF-8
Monitoring device: Device Name (/dev/input/eventX)
[YYYY-MM-DD HH:MM:SS.mmm] [Device Name] Key: X
```

## Manual Testing Scenarios

### Scenario 1: Fresh Installation
1. Clone repository on a fresh system
2. Follow INSTALL.md instructions
3. Verify successful build and execution

### Scenario 2: Permission Testing
1. Run without sudo as regular user
2. Verify appropriate error message
3. Add user to input group
4. Verify works without sudo (after re-login)

### Scenario 3: Cross-Platform Compatibility
Test on different Linux distributions:
- [ ] Arch Linux
- [ ] Ubuntu 20.04/22.04
- [ ] Debian 11/12
- [ ] Fedora 38+
- [ ] Other

### Scenario 4: Special Key Testing
Type the following and verify they're logged correctly:
- Letters: a-z, A-Z
- Numbers: 0-9
- Special characters: !@#$%^&*()
- Function keys: F1-F12
- Arrow keys: Up, Down, Left, Right
- Modifiers: Shift, Ctrl, Alt, Meta
- Other: Enter, Tab, Backspace, Space, Escape

### Scenario 5: Performance Testing
Monitor CPU and memory usage:
```bash
sudo ./target/release/rust-key &
top -p $! -n 1
```

Expected:
- Memory: < 5 MB
- CPU: < 5% during normal typing

### Scenario 6: Long-Running Stability
Run for extended period and verify:
- Memory remains stable
- Log file grows appropriately
- No crashes or errors

## Automated Testing Script

Create a file `automated_test.sh`:
```bash
#!/bin/bash
echo "Starting rust-key automated tests..."

# Test 1: Build
echo -n "Test 1 (Build): "
cargo build --release 2>&1 > /dev/null
[ $? -eq 0 ] && echo "✓ PASSED" || echo "✗ FAILED"

# Test 2: Binary exists
echo -n "Test 2 (Binary): "
[ -f target/release/rust-key ] && echo "✓ PASSED" || echo "✗ FAILED"

# Test 3: Locale setting
echo -n "Test 3 (Locale): "
timeout 2 ./target/release/rust-key 2>&1 | grep -q "en_US.UTF-8"
[ $? -eq 0 ] && echo "✓ PASSED" || echo "✗ FAILED"

# Test 4: Platform detection
echo -n "Test 4 (Platform): "
timeout 2 ./target/release/rust-key 2>&1 | grep -q "Running on Linux"
[ $? -eq 0 ] && echo "✓ PASSED" || echo "✗ FAILED"

# Test 5: Startup message
echo -n "Test 5 (Startup): "
timeout 2 ./target/release/rust-key 2>&1 | grep -q "Rust Keylogger starting"
[ $? -eq 0 ] && echo "✓ PASSED" || echo "✗ FAILED"

echo "Automated tests complete!"
```

Run with: `bash automated_test.sh`

## Testing Checklist

Before releasing:
- [x] Compiles without errors on Linux
- [x] Sets en_US.UTF-8 locale correctly
- [x] Platform detection works
- [x] Startup messages are correct
- [ ] Detects keyboard devices correctly (requires keyboard)
- [ ] Identifies USB keyboards specifically (requires USB keyboard)
- [ ] Logs keystrokes with correct timestamps (requires keyboard)
- [ ] Handles multiple keyboards simultaneously (requires multiple keyboards)
- [ ] Produces properly formatted log files (requires keyboard)
- [ ] Handles permission errors gracefully
- [ ] Memory usage remains stable over time
- [ ] CPU usage is minimal
- [ ] Can be interrupted cleanly with Ctrl+C
- [x] Documentation is accurate and complete

## Reporting Issues

When reporting issues, include:
1. Operating system and version
2. Rust version (`rustc --version`)
3. Full error messages
4. Steps to reproduce
5. Expected vs actual behavior
6. Output of `ls -l /dev/input/event*`
7. Output of `groups` command

## Contributing Tests

To contribute test cases:
1. Add test functions to relevant modules
2. Use `#[cfg(test)]` and `#[test]` attributes
3. Include both positive and negative test cases
4. Document what each test validates
5. Ensure tests can run without sudo when possible

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_key_letter() {
        assert_eq!(format_key(Key::KEY_A), "a");
    }

    #[test]
    fn test_format_key_special() {
        assert_eq!(format_key(Key::KEY_ENTER), "\n[ENTER]\n");
    }
}
```

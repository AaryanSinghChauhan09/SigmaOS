// Sigma GPU Driver CLI
// Command-line interface for GPU driver management

use sigma_gpu::{GPUArchitecture, GPUDriver, GPUDevice, GPUVendor, PixelFormat, Resolution};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut driver = GPUDriver::new();
    
    match args[1].as_str() {
        "detect" => handle_detect(&mut driver),
        "list" => handle_list(&driver),
        "info" => handle_info(&driver, &args),
        "init" => handle_init(&mut driver, &args),
        "mode" => handle_mode(&mut driver, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Sigma GPU Driver CLI");
    println!();
    println!("Usage:");
    println!("  gpu_driver detect");
    println!("  gpu_driver list");
    println!("  gpu_driver info <device_id>");
    println!("  gpu_driver init <device_id> <width> <height> <format>");
    println!("  gpu_driver mode <device_id> <width> <height> <format>");
    println!();
    println!("Example:");
    println!("  gpu_driver detect");
    println!("  gpu_driver init device_id 1920 1080 rgba32");
    println!("  gpu_driver mode device_id 2560 1440 bgra32");
}

fn handle_detect(driver: &mut GPUDriver) {
    driver.detect_devices();
    
    println!("GPU detection complete");
    println!("Found {} GPU device(s)", driver.device_count());
    println!();
    
    for device in driver.list_devices() {
        println!("Device ID: {}", device.get_device_id());
        println!("Vendor: {}", device.vendor.as_str());
        println!("Architecture: {}", device.architecture.as_str());
        println!("VRAM: {} MB", device.vram_size / (1024 * 1024));
        println!();
    }
}

fn handle_list(driver: &GPUDriver) {
    let devices = driver.list_devices();
    
    if devices.is_empty() {
        println!("No GPU devices found. Run 'gpu_driver detect' first.");
        return;
    }
    
    println!("GPU Devices ({}):", devices.len());
    println!();
    
    for device in devices {
        println!("Device ID: {}", device.get_device_id());
        println!("Vendor: {}", device.vendor.as_str());
        println!("Architecture: {}", device.architecture.as_str());
        println!("VRAM: {} MB", device.vram_size / (1024 * 1024));
        println!("Initialized: {}", device.initialized);
        println!();
    }
}

fn handle_info(driver: &GPUDriver, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Device ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    
    match driver.get_device(device_id) {
        Some(device) => {
            let info = device.get_info();
            println!("{}", info);
        }
        None => {
            eprintln!("Device not found: {}", device_id);
            std::process::exit(1);
        }
    }
}

fn handle_init(driver: &mut GPUDriver, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for init command");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let width: u32 = args[3].parse().expect("Invalid width");
    let height: u32 = args[4].parse().expect("Invalid height");
    let format_str = &args[5];
    
    let format = match format_str.to_lowercase().as_str() {
        "rgb24" => PixelFormat::RGB24,
        "rgba32" => PixelFormat::RGBA32,
        "bgr24" => PixelFormat::BGR24,
        "bgra32" => PixelFormat::BGRA32,
        _ => {
            eprintln!("Error: Invalid format. Use: rgb24, rgba32, bgr24, bgra32");
            std::process::exit(1);
        }
    };
    
    let resolution = Resolution::new(width, height);
    
    match driver.initialize_device(device_id, resolution, format) {
        Ok(_) => {
            println!("GPU initialized successfully");
            if let Some(device) = driver.get_device(device_id) {
                let info = device.get_info();
                println!("Resolution: {}", info.resolution.map(|r| r.to_string()).unwrap_or_else(|| "N/A".to_string()));
                println!("Format: {}", info.format.map(|fmt| fmt.as_str().to_string()).unwrap_or_else(|| "N/A".to_string()));
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize GPU: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_mode(driver: &mut GPUDriver, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for mode command");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let width: u32 = args[3].parse().expect("Invalid width");
    let height: u32 = args[4].parse().expect("Invalid height");
    let format_str = &args[5];
    
    let format = match format_str.to_lowercase().as_str() {
        "rgb24" => PixelFormat::RGB24,
        "rgba32" => PixelFormat::RGBA32,
        "bgr24" => PixelFormat::BGR24,
        "bgra32" => PixelFormat::BGRA32,
        _ => {
            eprintln!("Error: Invalid format. Use: rgb24, rgba32, bgr24, bgra32");
            std::process::exit(1);
        }
    };
    
    let resolution = Resolution::new(width, height);
    
    match driver.set_mode(device_id, resolution, format) {
        Ok(_) => {
            println!("Display mode set successfully");
            if let Some(device) = driver.get_device(device_id) {
                let info = device.get_info();
                println!("Resolution: {}", info.resolution.map(|r| r.to_string()).unwrap_or_else(|| "N/A".to_string()));
                println!("Format: {}", info.format.map(|fmt| fmt.as_str().to_string()).unwrap_or_else(|| "N/A".to_string()));
            }
        }
        Err(e) => {
            eprintln!("Failed to set display mode: {}", e);
            std::process::exit(1);
        }
    }
}

// Sigma Storage Driver CLI
// Command-line interface for storage driver management

use sigma_storage::{BlockDeviceType, BlockSize, IORequest, StorageController, StorageDriver};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut driver = StorageDriver::new();
    
    match args[1].as_str() {
        "detect" => handle_detect(&mut driver),
        "list" => handle_list(&driver),
        "info" => handle_info(&driver, &args),
        "init" => handle_init(&mut driver, &args),
        "read" => handle_read(&mut driver, &args),
        "write" => handle_write(&mut driver, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Sigma Storage Driver CLI");
    println!();
    println!("Usage:");
    println!("  storage_driver detect");
    println!("  storage_driver list");
    println!("  storage_driver info <device_id>");
    println!("  storage_driver init <device_id>");
    println!("  storage_driver read <device_id> <block_number> <block_count>");
    println!("  storage_driver write <device_id> <block_number> <data>");
    println!();
    println!("Example:");
    println!("  storage_driver detect");
    println!("  storage_driver init device_id");
    println!("  storage_driver read device_id 0 1");
}

fn handle_detect(driver: &mut StorageDriver) {
    driver.detect_devices();
    
    println!("Storage device detection complete");
    println!("Found {} storage device(s)", driver.device_count());
    println!();
    
    for device in driver.list_devices() {
        println!("Device ID: {}", device.get_device_id());
        println!("Name: {}", device.name);
        println!("Type: {}", device.device_type.as_str());
        println!("Controller: {}", device.controller.as_str());
        println!("Size: {} GB", device.size / (1024 * 1024 * 1024));
        println!();
    }
}

fn handle_list(driver: &StorageDriver) {
    let devices = driver.list_devices();
    
    if devices.is_empty() {
        println!("No storage devices found. Run 'storage_driver detect' first.");
        return;
    }
    
    println!("Storage Devices ({}):", devices.len());
    println!();
    
    for device in devices {
        println!("Device ID: {}", device.get_device_id());
        println!("Name: {}", device.name);
        println!("Type: {}", device.device_type.as_str());
        println!("Controller: {}", device.controller.as_str());
        println!("Size: {} GB", device.size / (1024 * 1024 * 1024));
        println!("Model: {}", device.model);
        println!("Serial: {}", device.serial);
        println!("Initialized: {}", device.initialized);
        println!();
    }
}

fn handle_info(driver: &StorageDriver, args: &[String]) {
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

fn handle_init(driver: &mut StorageDriver, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Device ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    
    match driver.initialize_device(device_id) {
        Ok(_) => {
            println!("Device initialized successfully");
            if let Some(device) = driver.get_device(device_id) {
                println!("Name: {}", device.name);
                println!("Size: {} GB", device.size / (1024 * 1024 * 1024));
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize device: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_read(driver: &mut StorageDriver, args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: Insufficient arguments for read command");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let block_number: u64 = args[3].parse().expect("Invalid block number");
    let block_count: u64 = args[4].parse().expect("Invalid block count");
    
    match driver.read_blocks(device_id, block_number, block_count) {
        Ok(blocks) => {
            println!("Read {} block(s) successfully", blocks.len());
            println!("Total bytes: {}", blocks.iter().map(|b| b.len()).sum::<usize>());
        }
        Err(e) => {
            eprintln!("Failed to read blocks: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_write(driver: &mut StorageDriver, args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: Insufficient arguments for write command");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let block_number: u64 = args[3].parse().expect("Invalid block number");
    let data_str = &args[4];
    
    let data: Vec<u8> = data_str.as_bytes().to_vec();
    
    match driver.write_blocks(device_id, block_number, &data) {
        Ok(_) => {
            println!("Write completed successfully");
            println!("Bytes written: {}", data.len());
        }
        Err(e) => {
            eprintln!("Failed to write blocks: {}", e);
            std::process::exit(1);
        }
    }
}

// Sigma Audio Driver CLI
// Command-line interface for audio driver management

use sigma_audio::{AudioConfig, AudioDeviceType, AudioDriver, ChannelCount, SampleFormat, SampleRate};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut driver = AudioDriver::new();
    
    match args[1].as_str() {
        "detect" => handle_detect(&mut driver),
        "list" => handle_list(&driver),
        "info" => handle_info(&driver, &args),
        "init" => handle_init(&mut driver, &args),
        "volume" => handle_volume(&mut driver, &args),
        "play" => handle_play(&driver, &args),
        "capture" => handle_capture(&driver, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Sigma Audio Driver CLI");
    println!();
    println!("Usage:");
    println!("  audio_driver detect");
    println!("  audio_driver list");
    println!("  audio_driver info <device_id>");
    println!("  audio_driver init <device_id> <format> <rate> <channels>");
    println!("  audio_driver volume <device_id> <volume>");
    println!("  audio_driver play <device_id>");
    println!("  audio_driver capture <device_id>");
    println!();
    println!("Example:");
    println!("  audio_driver detect");
    println!("  audio_driver init device_id s16le 44100 stereo");
    println!("  audio_driver volume device_id 75");
}

fn handle_detect(driver: &mut AudioDriver) {
    driver.detect_devices();
    
    println!("Audio device detection complete");
    println!("Found {} audio device(s)", driver.device_count());
    println!();
    
    for device in driver.list_devices() {
        println!("Device ID: {}", device.get_device_id());
        println!("Name: {}", device.name);
        println!("Type: {}", device.device_type.as_str());
        println!();
    }
}

fn handle_list(driver: &AudioDriver) {
    let devices = driver.list_devices();
    
    if devices.is_empty() {
        println!("No audio devices found. Run 'audio_driver detect' first.");
        return;
    }
    
    println!("Audio Devices ({}):", devices.len());
    println!();
    
    for device in devices {
        println!("Device ID: {}", device.get_device_id());
        println!("Name: {}", device.name);
        println!("Type: {}", device.device_type.as_str());
        println!("Initialized: {}", device.initialized);
        println!("Volume: {}%", device.volume);
        println!();
    }
}

fn handle_info(driver: &AudioDriver, args: &[String]) {
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

fn handle_init(driver: &mut AudioDriver, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for init command");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let format_str = &args[3];
    let rate_str = &args[4];
    let channels_str = &args[5];
    
    let format = match format_str.to_lowercase().as_str() {
        "u8" => SampleFormat::U8,
        "s16le" => SampleFormat::S16LE,
        "s16be" => SampleFormat::S16BE,
        "s24le" => SampleFormat::S24LE,
        "s24be" => SampleFormat::S24BE,
        "s32le" => SampleFormat::S32LE,
        "s32be" => SampleFormat::S32BE,
        "floatle" => SampleFormat::Float32LE,
        "floatbe" => SampleFormat::Float32BE,
        _ => {
            eprintln!("Error: Invalid format. Use: u8, s16le, s16be, s24le, s24be, s32le, s32be, floatle, floatbe");
            std::process::exit(1);
        }
    };
    
    let rate = match rate_str.to_lowercase().as_str() {
        "8000" => SampleRate::Hz8000,
        "11025" => SampleRate::Hz11025,
        "16000" => SampleRate::Hz16000,
        "22050" => SampleRate::Hz22050,
        "44100" => SampleRate::Hz44100,
        "48000" => SampleRate::Hz48000,
        "96000" => SampleRate::Hz96000,
        "192000" => SampleRate::Hz192000,
        _ => {
            eprintln!("Error: Invalid rate. Use: 8000, 11025, 16000, 22050, 44100, 48000, 96000, 192000");
            std::process::exit(1);
        }
    };
    
    let channels = match channels_str.to_lowercase().as_str() {
        "mono" => ChannelCount::Mono,
        "stereo" => ChannelCount::Stereo,
        "5.1" => ChannelCount::Surround51,
        "7.1" => ChannelCount::Surround71,
        _ => {
            eprintln!("Error: Invalid channels. Use: mono, stereo, 5.1, 7.1");
            std::process::exit(1);
        }
    };
    
    let config = AudioConfig::new(format, rate, channels);
    
    match driver.initialize_device(device_id, config) {
        Ok(_) => {
            println!("Device initialized successfully");
            if let Some(device) = driver.get_device(device_id) {
                if let Some(ref config) = device.config {
                    println!("Format: {}", config.sample_format.as_str());
                    println!("Rate: {}", config.sample_rate.as_str());
                    println!("Channels: {}", config.channels.as_str());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize device: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_volume(driver: &mut AudioDriver, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Device ID and volume required");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    let volume: u8 = args[3].parse().expect("Invalid volume");
    
    match driver.set_volume(device_id, volume) {
        Ok(_) => {
            println!("Volume set successfully");
            if let Some(device) = driver.get_device(device_id) {
                println!("New volume: {}%", device.get_volume());
            }
        }
        Err(e) => {
            eprintln!("Failed to set volume: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_play(driver: &AudioDriver, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Device ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    
    // Simulate PCM data
    let data = vec![0u8; 4096];
    
    match driver.play_pcm(device_id, &data) {
        Ok(_) => {
            println!("PCM playback started");
            println!("Bytes: {}", data.len());
        }
        Err(e) => {
            eprintln!("Failed to play PCM: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_capture(driver: &AudioDriver, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Device ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let device_id = &args[2];
    
    // Simulate capture buffer
    let mut buffer = vec![0u8; 4096];
    
    match driver.capture_pcm(device_id, &mut buffer) {
        Ok(_) => {
            println!("PCM capture started");
            println!("Buffer size: {}", buffer.len());
        }
        Err(e) => {
            eprintln!("Failed to capture PCM: {}", e);
            std::process::exit(1);
        }
    }
}

# Desktop Sound Manager

## Overview

The Desktop Sound Manager provides comprehensive audio management inspired by Linux Mint's Sound settings and Omarchy's audio utilities. It supports device management, volume control, output profiles, and per-application audio control.

## Features

- **Audio Device Types**: Speaker, Microphone, Headphones, Headset, Line Out, Line In, SPDIF, HDMI, Bluetooth
- **Device Status**: Active, Inactive, Unplugged, Error
- **Device Management**: Add, remove, enable, disable audio devices
- **Volume Control**: Per-device volume (0-100) and master volume
- **Mute Control**: Per-device mute and master mute
- **Default Devices**: Set default output and input devices
- **Sound Applications**: Per-application volume and mute control
- **Output Profiles**: Analog Stereo, Analog Surround 5.1/7.1, Digital Stereo, Digital Surround 5.1/7.1
- **Device Filtering**: List devices by type
- **Default Configuration**: Built-in speakers, microphone, and headphones
- **Statistics**: Track device and application counts

## Components

### DesktopAudioDeviceType

```rust
pub enum DesktopAudioDeviceType {
    Speaker,      // Built-in or external speakers
    Microphone,   // Input microphone
    Headphones,   // Headphones
    Headset,      // Headset with mic
    LineOut,      // Line output
    LineIn,       // Line input
    SPDIF,        // S/PDIF digital output
    HDMI,         // HDMI audio
    Bluetooth,    // Bluetooth audio
}
```

### DesktopAudioDeviceStatus

```rust
pub enum DesktopAudioDeviceStatus {
    Active,      // Device is active
    Inactive,    // Device is inactive
    Unplugged,   // Device is unplugged
    Error,       // Device error
}
```

### DesktopSoundProfile

```rust
pub enum DesktopSoundProfile {
    AnalogStereo,        // Analog stereo output
    AnalogSurround51,    // Analog 5.1 surround
    AnalogSurround71,    // Analog 7.1 surround
    DigitalStereo,       // Digital stereo output
    DigitalSurround51,   // Digital 5.1 surround
    DigitalSurround71,   // Digital 7.1 surround
}
```

### DesktopSoundManager

Main management interface with:
- Device registration and management
- Default device configuration
- Volume and mute control
- Application audio control
- Output profile selection
- Master volume control
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopSoundManager;

let mut manager = DesktopSoundManager::new();

// Get all devices
let devices = manager.get_devices();
println!("Total devices: {}", devices.len());

// Get master volume
let volume = manager.get_master_volume();
println!("Master volume: {}", volume);
```

### Device Management

```rust
// Add a new device
let id = manager.add_device(
    "USB Speaker".to_string(),
    DesktopAudioDeviceType::Speaker,
    DesktopAudioDeviceStatus::Active,
);

// Get device by ID
if let Some(device) = manager.get_device(&id) {
    println!("Device: {}", device.name);
}

// Remove device
manager.remove_device(&id);
```

### Default Devices

```rust
// Get default output device
if let Some(device) = manager.get_default_output_device() {
    println!("Default output: {}", device.name);
}

// Get default input device
if let Some(device) = manager.get_default_input_device() {
    println!("Default input: {}", device.name);
}

// Set default device
manager.set_default_device(&id);
```

### Volume Control

```rust
// Set device volume
manager.set_device_volume(&id, 75);

// Set device mute
manager.set_device_muted(&id, true);

// Set master volume
manager.set_master_volume(80);

// Set master mute
manager.set_master_muted(true);
```

### Application Audio

```rust
// Add application
let app_id = manager.add_application("Firefox".to_string());

// Set application volume
manager.set_application_volume(&app_id, 50);

// Set application mute
manager.set_application_muted(&app_id, true);

// Get all applications
let apps = manager.get_applications();
```

### Output Profile

```rust
// Get current output profile
let profile = manager.get_output_profile();
println!("Profile: {}", profile.as_str());

// Set output profile
manager.set_output_profile(DesktopSoundProfile::DigitalSurround51);
```

### Device Filtering

```rust
// Get devices by type
let speakers = manager.get_devices_by_type(DesktopAudioDeviceType::Speaker);
println!("Speakers: {}", speakers.len());

let microphones = manager.get_devices_by_type(DesktopAudioDeviceType::Microphone);
println!("Microphones: {}", microphones.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Devices: {}", stats.device_count);
println!("Applications: {}", stats.application_count);
println!("Master volume: {}", stats.master_volume);
println!("Master muted: {}", stats.master_muted);
println!("Profile: {}", stats.output_profile.as_str());
```

## Default Configuration

The Desktop Sound Manager includes default devices:

- **Built-in Speakers**: Default output device (Speaker type, Active)
- **Built-in Microphone**: Default input device (Microphone type, Active)
- **Headphones**: Available device (Headphones type, Inactive)

Default settings:
- **Master Volume**: 50%
- **Master Muted**: False
- **Output Profile**: Analog Stereo

## AI Agent Maintenance Instructions

When maintaining the Desktop Sound Manager:

1. **Device Validation**: Ensure device types and statuses are valid before adding
2. **Volume Validation**: Ensure volumes are within 0-100 range
3. **Default Protection**: Prevent removal of default devices without replacement
4. **Profile Validation**: Ensure output profiles are valid for hardware
5. **Application Cleanup**: Remove applications when they exit
6. **Hotplug Detection**: Detect and handle device hotplug events
7. **Audio Backend**: Integrate with actual audio backend (PipeWire, PulseAudio, ALSA)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::sound_manager
```

## Future Enhancements

- Integration with actual audio backend (PipeWire, PulseAudio, ALSA)
- Per-user audio device profiles
- Audio routing matrix
- Audio effects (equalizer, compression)
- Bluetooth audio pairing and management
- JACK audio server integration
- Audio device latency configuration
- Per-device balance control
- Audio recording and playback
- Audio visualizer integration

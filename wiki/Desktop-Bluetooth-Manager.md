# Desktop Bluetooth Manager

## Overview

The Desktop Bluetooth Manager provides comprehensive Bluetooth management inspired by Linux Mint's Bluetooth settings and Omarchy's Bluetooth utilities. It supports device pairing, connection management, adapter configuration, and device discovery.

## Features

- **Device Types**: Audio, Input, Network, Printer, Other
- **Device Status**: Disconnected, Connected, Paired, Discovering
- **Device Management**: Add, remove, and manage Bluetooth devices
- **Pairing**: Pair and unpair Bluetooth devices
- **Connection**: Connect and disconnect paired devices
- **Trusted Devices**: Mark devices as trusted
- **Adapter Configuration**: Enable/disable adapter, set adapter name
- **Discoverable Mode**: Set adapter discoverable mode
- **Scanning**: Start/stop device scanning
- **Device Filtering**: Filter devices by type or status
- **Statistics**: Track device counts, paired devices, connected devices

## Components

### DesktopBluetoothDeviceType

```rust
pub enum DesktopBluetoothDeviceType {
    Audio,    // Audio devices (headphones, speakers)
    Input,    // Input devices (keyboard, mouse)
    Network,  // Network devices
    Printer,  // Printer devices
    Other,    // Other device types
}
```

### DesktopBluetoothDeviceStatus

```rust
pub enum DesktopBluetoothDeviceStatus {
    Disconnected, // Device is disconnected
    Connected,     // Device is connected
    Paired,        // Device is paired
    Discovering,   // Device is being discovered
}
```

### DesktopBluetoothDevice

Bluetooth device with:
- Device ID and name
- MAC address
- Device type
- Status
- Paired flag
- Trusted flag
- Connected flag

### DesktopBluetoothManager

Main management interface with:
- Device management (add, remove, retrieve)
- Pairing and unpairing
- Connection and disconnection
- Adapter configuration
- Discoverable mode
- Scanning control
- Device filtering by type or status
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopBluetoothManager;

let mut manager = DesktopBluetoothManager::new();

// Get configuration
println!("Total devices: {}", manager.get_devices().len());
println!("Adapter enabled: {}", manager.is_adapter_enabled());
println!("Scanning: {}", manager.is_scanning());
```

### Device Management

```rust
// Add device
let device = DesktopBluetoothDevice::new(
    "custom".to_string(),
    "Custom Device".to_string(),
    "00:11:22:33:44:57".to_string(),
    DesktopBluetoothDeviceType::Audio,
);

let id = manager.add_device(device);

// Remove device
manager.remove_device(&id);
```

### Pairing

```rust
// Pair device
manager.pair_device(&device_id);

// Unpair device
manager.unpair_device(&device_id);
```

### Connection

```rust
// Connect device (must be paired first)
manager.pair_device(&device_id);
manager.connect_device(&device_id);

// Disconnect device
manager.disconnect_device(&device_id);
```

### Adapter Configuration

```rust
// Enable/disable adapter
manager.set_adapter_enabled(false);
manager.set_adapter_enabled(true);

// Set adapter name
manager.set_adapter_name("My Bluetooth".to_string());

// Set discoverable mode
manager.set_discoverable(true);

// Start/stop scanning
manager.set_scanning(true);
manager.set_scanning(false);
```

### Device Filtering

```rust
// Get devices by type
let audio = manager.get_devices_by_type(DesktopBluetoothDeviceType::Audio);
let input = manager.get_devices_by_type(DesktopBluetoothDeviceType::Input);

// Get devices by status
let paired = manager.get_devices_by_status(DesktopBluetoothDeviceStatus::Paired);
let connected = manager.get_devices_by_status(DesktopBluetoothDeviceStatus::Connected);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total devices: {}", stats.total_devices);
println!("Paired devices: {}", stats.paired_devices);
println!("Connected devices: {}", stats.connected_devices);
println!("Audio devices: {}", stats.audio_devices);
println!("Input devices: {}", stats.input_devices);
println!("Adapter enabled: {}", stats.adapter_enabled);
println!("Scanning: {}", stats.scanning);
```

## Default Devices

The manager includes default devices:

- **Wireless Headphones**: Audio device
- **Bluetooth Keyboard**: Input device

## Default Configuration

The Bluetooth Manager includes default configuration:

- **Adapter Enabled**: true
- **Adapter Name**: Default Adapter
- **Discoverable**: false
- **Scanning**: false

## AI Agent Maintenance Instructions

When maintaining the Bluetooth Manager:

1. **BlueZ Integration**: Integrate with BlueZ Bluetooth stack
2. **Device Discovery**: Implement actual device discovery
3. **PIN Pairing**: Add PIN/passkey pairing support
4. **Audio Profiles**: Add audio profile support (A2DP, HFP)
5. **Input Profiles**: Add input profile support (HID)
6. **Battery Level**: Add battery level reporting
7. **Device Info**: Add device information retrieval
8. **LE Support**: Add Bluetooth Low Energy support
9. **File Transfer**: Add OBEX file transfer
10. **Network Access**: Add PAN network access

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::desktop_bluetooth_manager
```

## Future Enhancements

- BlueZ Bluetooth stack integration
- Actual device discovery
- PIN/passkey pairing support
- Audio profile support (A2DP, HFP)
- Input profile support (HID)
- Battery level reporting
- Device information retrieval
- Bluetooth Low Energy support
- OBEX file transfer
- PAN network access

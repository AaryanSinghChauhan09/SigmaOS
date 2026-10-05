# Bluetooth Device Manager

## Overview

The Bluetooth Device Manager provides comprehensive Bluetooth management inspired by Linux Mint's Bluetooth settings and Omarchy's Bluetooth utilities. It supports device discovery, pairing, connection management, and device filtering.

## Features

- **Device Types**: Phone, Computer, Headset, Headphones, Audio/Video, Peripheral, Keyboard, Mouse, Joystick, Tablet
- **Device Status**: Disconnected, Connecting, Connected, Paired
- **Adapter Management**: Power, discoverability, and pairability control
- **Device Discovery**: Scan for nearby Bluetooth devices
- **Connection Management**: Connect, disconnect, pair, unpair
- **Trust Management**: Trust or block devices
- **Device Filtering**: By type, by connection status
- **RSSI Tracking**: Signal strength monitoring
- **Statistics**: Track device counts and status

## Components

### BluetoothDeviceType

```rust
pub enum BluetoothDeviceType {
    Unknown,      // Unknown device type
    Phone,        // Mobile phone
    Computer,     // Computer/laptop
    Headset,      // Audio headset
    Headphones,   // Headphones
    AudioVideo,   // Audio/video devices
    Peripheral,   // Peripheral devices
    Keyboard,     // Keyboard
    Mouse,        // Mouse
    Joystick,     // Gaming controller
    Tablet,       // Tablet device
}
```

### BluetoothDeviceStatus

```rust
pub enum BluetoothDeviceStatus {
    Disconnected, // Device is disconnected
    Connecting,   // Device is connecting
    Connected,    // Device is connected
    Paired,       // Device is paired
}
```

### BLEDevice

Represents a Bluetooth device with:
- MAC address
- Device name
- Device type
- Connection status
- RSSI (signal strength)
- Trust status
- Block status

### BluetoothAdapter

Represents the Bluetooth adapter with:
- Adapter name
- MAC address
- Power status
- Discoverability
- Pairability

### BluetoothDeviceManager

Main management interface with:
- Device scanning
- Connection/disconnection
- Pairing/unpairing
- Trust/block management
- Device listing and filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::bluetooth::BluetoothDeviceManager;

let mut manager = BluetoothDeviceManager::new();

// Scan for devices
let devices = manager.scan();
for device in devices {
    println!("Found: {} ({})", device.name, device.address);
}
```

### Adapter Management

```rust
// Get adapter
if let Some(adapter) = manager.get_adapter() {
    println!("Adapter: {}", adapter.name);
}

// Get adapter mutably to change settings
if let Some(adapter) = manager.get_adapter_mut() {
    adapter.set_powered(true);
    adapter.set_discoverable(true);
    adapter.set_pairable(true);
}
```

### Device Connection

```rust
// Connect to a device
manager.connect("00:1A:2B:3C:4D:5E")?;

// Disconnect from a device
manager.disconnect("00:1A:2B:3C:4D:5E")?;
```

### Pairing and Trust

```rust
// Pair a device
manager.pair("00:1A:2B:3C:4D:5E")?;

// Trust a device
manager.trust("00:1A:2B:3C:4D:5E")?;

// Unpair a device
manager.unpair("00:1A:2B:3C:4D:5E")?;
```

### Blocking Devices

```rust
// Block a device (prevents connection)
manager.block("00:1A:2B:3C:4D:5E")?;

// Cannot connect to blocked devices
assert!(manager.connect("00:1A:2B:3C:4D:5E").is_err());
```

### Device Filtering

```rust
// List all devices
let all = manager.list_devices();

// List by type
let headsets = manager.list_by_type(BluetoothDeviceType::Headset);

// List connected devices
let connected = manager.list_connected();
```

### Device Information

```rust
// Get a specific device
if let Some(device) = manager.get_device("00:1A:2B:3C:4D:5E") {
    println!("Name: {}", device.name);
    println!("Type: {:?}", device.device_type);
    println!("Status: {:?}", device.status);
    println!("RSSI: {}", device.rssi);
    println!("Trusted: {}", device.is_trusted);
    println!("Blocked: {}", device.is_blocked);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total devices: {}", stats.total_devices);
println!("Connected: {}", stats.connected_count);
println!("Paired: {}", stats.paired_count);
println!("Trusted: {}", stats.trusted_count);
println!("Blocked: {}", stats.blocked_count);
println!("Adapter powered: {}", stats.adapter_powered);
```

## Device Discovery

The `scan()` method simulates device discovery by populating the device list with common Bluetooth device types:

- Headset (00:1A:2B:3C:4D:5E)
- Wireless Mouse (00:1A:2B:3C:4D:5F)
- Bluetooth Keyboard (00:1A:2B:3C:4D:60)
- Phone (00:1A:2B:3C:4D:61)

## AI Agent Maintenance Instructions

When maintaining the Bluetooth Device Manager:

1. **Device Addressing**: Ensure MAC addresses are properly validated
2. **Blocking Logic**: Ensure blocked devices cannot connect
3. **Trust Management**: Maintain trust status separately from connection status
4. **RSSI Updates**: Update RSSI values during scanning
5. **Adapter State**: Track adapter power, discoverability, and pairability state
6. **Type Detection**: Properly detect and classify device types

## Testing

Run the unit tests with:

```bash
cargo test --lib bluetooth::bluetooth_manager
```

## Future Enhancements

- Integration with actual Bluetooth daemon (BlueZ)
- Real device discovery via hcitool/ble scan
- GATT client for BLE device interaction
- Audio profile support (A2DP, HFP)
- File transfer (OBEX)
- Device name resolution
- Battery level monitoring
- Device persistence across reboots

# SigmaOS Bluetooth Manager Components

## Overview
SigmaOS bluetooth manager providing bluetooth device pairing, connection, and management, competitive with Omarchy bluetooth controls (omancy-bluetooth-*, bluez).

## Features
- **Device management** - Add, list, remove bluetooth devices
- **Power control** - Power on/off bluetooth adapter
- **Pairing** - Pair with bluetooth devices
- **Connection** - Connect/disconnect from paired devices
- **Trust** - Trust paired devices
- **Discoverable/Pairable** - Configure discoverability and pairability
- **Connected/paired device filtering** - Filter by connection/pairing status

## Structures

| Structure | Description |
|-----------|-------------|
| **BluetoothManager** | Bluetooth manager with device registry |
| **BluetoothDevice** | Individual bluetooth device |

## Usage Examples

```rust
// Create bluetooth manager
let mut mgr = BluetoothManager::new();

// Power on
mgr.power_on();

// Add a device
let device = BluetoothDevice {
    id: "dev0".to_string(),
    name: "Headphones".to_string(),
    address: "AA:BB:CC:DD:EE:FF".to_string(),
    is_paired: false,
    is_connected: false,
    is_trusted: false,
    device_class: "headphones".to_string(),
    rssi: None,
};
mgr.add_device(device);

// Pair with device
mgr.pair("dev0").unwrap();

// Connect to device
mgr.connect("dev0").unwrap();

// Trust device
mgr.trust("dev0").unwrap();

// Get connected devices
let connected = mgr.get_connected_devices();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Hardware (P1)

## Related Files
- `src/bluetooth/manager.rs` - Bluetooth manager implementation
- `wiki/Bluetooth.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL
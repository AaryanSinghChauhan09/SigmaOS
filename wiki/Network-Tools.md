# SigmaOS Network Tools Components

## Overview
SigmaOS network tools manager providing network status, passwords, QR generation, and speedtest, competitive with Omarchy network (omancy-network-band, omancy-network-password, omancy-network-qr, omancy-network-speedtest, omancy-network-status).

## Features
- **Network status** - Interface, IP, MAC, SSID, signal, band, speeds
- **Password management** - Save and list network passwords
- **Network QR** - Generate WiFi QR codes
- **Speedtest** - Run download/upload/ping tests

## Structures

| Structure | Description |
|-----------|-------------|
| **NetworkToolsManager** | Network tools manager |
| **NetworkStatus** | Network interface status |
| **NetworkPassword** | Saved network password |
| **NetworkQrData** | QR code data |
| **SpeedtestResult** | Speedtest results |
| **NetworkBand** | TwoGhz, FiveGhz, SixGhz, Unknown |

## Usage Examples

```rust
// Create network tools manager
let mut mgr = NetworkToolsManager::new();

// Set network status
let status = NetworkStatus { ... };
mgr.set_status(status);

// Save password
mgr.save_password(NetworkPassword { ... });

// Generate network QR
let qr = mgr.generate_network_qr("MyWiFi", "password123", "WPA2");

// Run speedtest
let result = mgr.run_speedtest();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Networking (P1)

## Related Files
- `src/network/tools.rs` - Network tools implementation
- `wiki/Network-Tools.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
NETWORK_ISOLATION, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL
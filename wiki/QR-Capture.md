# SigmaOS QR Capture Components

## Overview
SigmaOS QR capture system providing QR code reading and generation, competitive with Omarchy QR capture (omancy-capture-qr, omancy-network-qr).

## Features
- **Camera capture** - Read QR from camera
- **File capture** - Read QR from image file
- **Screen capture** - Read QR from screen
- **Clipboard capture** - Read QR from clipboard
- **WiFi QR generation** - Generate WiFi QR codes
- **Network QR generation** - Generate network sharing QR codes
- **QR code types**: Text, URL, WiFi, Network, Contact, Custom

## Structures

| Structure | Description |
|-----------|-------------|
| **QrCaptureManager** | QR capture manager |
| **QrCaptureResult** | Individual capture result |
| **QrCodeType** | Text, Url, Wifi, Network, Contact, Custom |
| **WifiEncryption** | Wpa, Wpa2, Wpa3, Wep, None |
| **QrSource** | Camera, File, Screen, Clipboard |

## Usage Examples

```rust
// Create QR capture manager
let mut mgr = QrCaptureManager::new();

// Capture from camera
let id = mgr.capture_from_camera().unwrap();

// Generate WiFi QR
mgr.generate_wifi_qr(
    "MyWiFi".to_string(),
    "password123".to_string(),
    WifiEncryption::Wpa2,
).unwrap();

// Generate network QR
mgr.generate_network_qr("HomeNet".to_string(), "secret".to_string()).unwrap();

// List results
let results = mgr.list();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/qr/capture.rs` - QR capture implementation
- `wiki/QR-Capture.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
SECURE_DEFAULTS, INPUT_VALIDATION, MEMORY_SAFETY, DIRECT_DEVICE_CONTROL,
LOW_LEVEL_CONTROL
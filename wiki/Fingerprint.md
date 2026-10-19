# SigmaOS Fingerprint Components

## Overview
SigmaOS fingerprint manager providing fingerprint authentication, competitive with Omarchy fingerprint (omancy-hw-fingerprint, omancy-setup-security-fingerprint, omancy-remove-security-fingerprint).

## Features
- **Device management** - Add, list, set active fingerprint devices
- **Enrollment** - Enroll fingers with names
- **Verification** - Verify fingerprints
- **Enable/disable** - Enable/disable fingerprint auth
- **Template management** - Template count tracking

## Structures

| Structure | Description |
|-----------|-------------|
| **FingerprintManager** | Fingerprint manager |
| **FingerprintDevice** | Individual fingerprint device |
| **Enrollment** | Individual enrollment |

## Usage Examples

```rust
// Create fingerprint manager
let mut mgr = FingerprintManager::new();

// Add device
let device = FingerprintDevice { ... };
mgr.add_device(device);

// Set active and enable
mgr.set_active_device("fpc0").unwrap();
mgr.enable().unwrap();

// Enroll finger
mgr.enroll("fpc0", "thumb".to_string(), "Right Thumb".to_string()).unwrap();

// Verify fingerprint
let verified = mgr.verify("fpc0", "thumb").unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Security (P1)

## Related Files
- `src/fingerprint/manager.rs` - Fingerprint manager implementation
- `wiki/Fingerprint.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
LEAST_PRIVILEGE, MEMORY_SAFETY, LOW_LEVEL_CONTROL, USER_CONTROL
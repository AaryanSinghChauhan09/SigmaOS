# SigmaOS Driver Auto-Updater

> Signed driver packages with Ed25519 verification and atomic rollback.

## Overview

The SigmaOS Driver Auto-Updater ensures all hardware drivers are cryptographically signed, compatibility-tested against the Hardware Compatibility List (HCL), and installed atomically with rollback support.

## Driver Package Format

Each driver package contains:
- **Module binary** (`.ko` kernel module)
- **Ed25519 signature** (64 bytes)
- **SHA-256 checksum** (32 bytes)
- **Signer public key** (32 bytes)
- **Compatibility metadata** (vendor/device IDs, min kernel version)

## Update Flow

```
1. Check Registry  ──→  2. Download Package  ──→  3. Verify Signature
                                                        │
6. Rollback (if fail) ←── 5. Test Hardware ←── 4. Install Atomically
```

## Driver Classes

| Class      | Examples                          |
|------------|-----------------------------------|
| GPU        | NVIDIA, AMD, Intel integrated     |
| Network    | Intel e1000e, Realtek, Broadcom   |
| Storage    | NVMe, SATA AHCI, USB Mass Storage|
| Audio      | ALSA/PipeWire drivers             |
| Input      | HID, touchscreen, stylus          |
| USB        | xHCI, EHCI controllers            |
| Bluetooth  | Intel AX, Broadcom BCM            |
| Sensor     | IMU, ambient light, proximity     |
| Camera     | V4L2-compatible webcams           |

## Implementation

- **Source**: `drivers/sigma_driver_updater.rs`
- **Language**: Rust (`no_std`)
- **Key APIs**:
  - `verify_signature(pkg)` — Ed25519 verification
  - `check_compatibility(pkg, kernel)` — HCL + kernel version
  - `install_driver(pkg, kernel)` — atomic install with rollback
  - `rollback()` — revert to previous driver

## Security

1. Only drivers signed by **trusted keys** can be installed
2. Signature is verified against the **checksum**, not the raw binary
3. Failed installations trigger **automatic rollback**
4. All operations are **audit-logged**

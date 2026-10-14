# SigmaOS Hardware Support Evidence Matrix

## Overview

A driver existing in the SigmaOS source tree does not imply fully verified hardware support. Driver support claims are classified based on observable hardware or emulator test evidence.

## Driver Evidence Categories

- **Hardware Verified**: Tested on physical hardware; diagnostic logs collected and archived in repo.
- **Emulator Verified**: Tested in QEMU/Bochs/KVM virtualized environments.
- **Model / Unit Tested**: Unit tests compile and pass against synthetic register abstractions.
- **Untested**: Code exists in driver module, but runtime execution has not been verified.

## Hardware Support Ledger

| Driver Class | Driver / Chipset | Status | Test Evidence / Script |
|---|---|---|---|
| **Serial / Console** | 16550 UART | Emulator Verified | `scripts/ci/qemu_boot_smoke.sh` |
| **RTC** | CMOS / MC146818 | Emulator Verified | `tests/hardware/driver_evidence_test.rs` |
| **Display** | VBE / VirtIO-GPU / DRM KMS | Emulator Verified | `tests/hardware/driver_evidence_test.rs` |
| **Input** | PS/2 Keyboard & Mouse, USB HID | Emulator Verified | `src/drivers/usb_subsystem.rs` |
| **Storage** | VirtIO-Block, IDE/AHCI, NVMe | Emulator Verified | `tests/storage/block.rs` |
| **Network** | Realtek RTL8139, Intel e1000, virtio-net | Emulator Verified | `tests/hardware/driver_evidence_test.rs` |
| **Audio** | Intel HD Audio / AC97 | Model / Unit Tested | `src/drivers/distro_device_expansion.rs` |
| **ISA / Expansion** | S3 Trio64, NE2000 ISA | Model / Unit Tested | `src/drivers/phase7_hardware_expansion.rs` |

## Diagnostic Collection

Hardware diagnostics can be collected on running systems using:
```bash
./scripts/hardware/collect_diagnostics.sh
```
Output report is saved to `reports/hardware_diagnostics.json` for validation and triage.

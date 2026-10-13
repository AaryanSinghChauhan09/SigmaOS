# SigmaOS Hardware Support Evidence Ledger

## Overview

Driver availability in source code does not equate to tested hardware support. SigmaOS distinguishes **Code Exists**, **Tests Pass**, and **System Behavior Verified**.

## Driver Status Classification

- **Verified:** Tested on physical hardware or QEMU emulator with pass logs attached.
- **Model / Emulated:** Compiles and passes unit test models, verified in QEMU.
- **Untested:** Driver implementation exists in `src/drivers/`, but no hardware/emulator evidence has been submitted.

## Hardware Support Matrix

| Driver Category | Device Class / Driver Name | Implementation File | Status | Test Evidence | Known Limitations |
|---|---|---|---|---|---|
| **Serial / Console** | 16550 UART Serial | `src/drivers/uart.rs` | Verified | `tests/hardware/driver_evidence_test.rs` | Clamped to 115200 baud |
| **Storage** | VirtIO Block Device | `src/storage/virtio.rs` | Verified | `tests/storage/block.rs` | QEMU legacy & modern virtio |
| **Storage** | NVMe 2.0 Controller | `src/drivers/phase7_hardware_expansion.rs` | Model | `build/phase7_tests` | ZNS support mocked |
| **Input** | PS/2 Keyboard & Mouse | `src/drivers/ps2.rs` | Verified | `tests/hardware/driver_evidence_test.rs` | Standard US layout |
| **USB Subsystem** | xHCI Host Controller & HID | `src/drivers/usb_subsystem.rs` | Model | `build/test_usb_subsystem` | USB 2.0 / 3.0 transfers |
| **Network** | Realtek R8169 NIC | `src/drivers/distro_device_expansion.rs` | Model | `tests/hardware/driver_evidence_test.rs` | Emulated PCI descriptor |
| **Network** | Intel IGB Gigabit NIC | `src/drivers/distro_device_expansion.rs` | Model | `tests/hardware/driver_evidence_test.rs` | Basic packet send/receive |
| **Display** | Linear Framebuffer / DRM KMS | `src/drivers/drm_kms.rs` | Verified | `build/zenith_bridge_tests` | Software double-buffering |

## Hardware Diagnostics Collection

Platform diagnostic dumps are generated via `./scripts/hardware/collect_diagnostics.sh` and verified through `tests/hardware/driver_evidence_test.rs`.

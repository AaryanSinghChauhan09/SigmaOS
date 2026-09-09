---
name: Driver Submission
about: Submit a new WDM/NT style driver for SigmaOS
title: '[DRIVER] '
labels: 'driver, hardware'
assignees: ''
---

## Driver Overview
- **Driver Name**:
- **Target Hardware**: (PCI / USB / VirtIO / ACPI)
- **Vendor ID / Device ID**:

## WDM Driver Lifecycle Implementation
- [ ] Implements `DriverObject` entrypoint initialization (`DriverEntry`).
- [ ] Implements `DeviceObject` allocation and attachment to stack.
- [ ] Allocates driver context state inside `DeviceExtension` (NonPaged Pool).
- [ ] Handles cleanup and unload routines safely.

## Testing & Verification
- [ ] Includes standalone unit test verifying `DriverObject` and `DeviceObject` lifecycle.
- [ ] Verified on QEMU / Bare-metal hardware.
- [ ] Includes Dilithium-5 signature or recipe manifest.

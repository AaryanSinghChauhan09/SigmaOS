# SigmaOS Driver API Specification
**Version:** 0.1 (Draft) | **Status:** Active Development

This document outlines the architecture, lifecycle, and API contracts for writing drivers in SigmaOS.

## 1. Architecture Overview

SigmaOS uses a **Microkernel-inspired Monolithic** driver model. Drivers run in kernel mode (for now) but are strictly sandboxed using the `sigma_driver_framework`.
- **Bus Adapters:** PCI, USB, and VirtIO buses enumerate devices and instantiate driver instances.
- **Device Lifecycle:** Probe → Initialize → Start → Suspend/Resume → Stop → Remove.
- **Safe by Default:** All drivers must be written in Safe Rust. `unsafe` is only permitted for direct MMIO or DMA operations and must be heavily commented.

## 2. The `Driver` Trait

All SigmaOS drivers must implement the core `SovereignDriver` trait.

```rust
pub trait SovereignDriver {
    /// Returns the unique name of the driver (e.g., "nvme_core")
    fn name(&self) -> &'static str;

    /// Called when the bus enumerates a compatible device.
    /// Returns a DriverInstance on success.
    fn probe(&self, device: &DeviceNode) -> Result<Box<dyn DriverInstance>, DriverError>;
}

pub trait DriverInstance {
    /// Initialize the device (allocate queues, map MMIO)
    fn init(&mut self) -> Result<(), DriverError>;

    /// Start processing I/O requests
    fn start(&mut self) -> Result<(), DriverError>;

    /// Handle power state changes (ACPI)
    fn suspend(&mut self) -> Result<(), DriverError>;
    fn resume(&mut self) -> Result<(), DriverError>;

    /// Stop the device and release resources
    fn stop(&mut self) -> Result<(), DriverError>;
}
```

## 3. Subsystem APIs

### 3.1 Block Storage (NVMe, AHCI)
Block drivers implement the `BlockDevice` trait, providing asynchronous read/write operations to the VFS.

```rust
pub trait BlockDevice {
    fn read_blocks(&self, start_lba: u64, count: u32, buffer: &mut [u8]) -> Result<(), DriverError>;
    fn write_blocks(&self, start_lba: u64, count: u32, buffer: &[u8]) -> Result<(), DriverError>;
    fn capacity(&self) -> u64; // In sectors
}
```

### 3.2 USB (Host Controllers & HID)
USB drivers interact with the `UsbCore` subsystem via Request Blocks (URBs).
- **Host Controllers (xHCI):** Manage the root hub and port routing.
- **Class Drivers (HID, Mass Storage):** Bind to specific interfaces and communicate via `submit_urb()`.

### 3.3 GPU & Display
Currently based on a simple linear Framebuffer.
- Future: Port to `sigma_drm` (Direct Rendering Manager) supporting GEM (Graphics Execution Manager) and KMS (Kernel Mode Setting).

### 3.4 Audio
Audio drivers implement the `AudioStream` trait for PCM playback and capture.

## 4. Writing a New Driver

1. **Create the Skeleton:** Place your driver in `kernel/drivers/<subsystem>/`.
2. **Implement `SovereignDriver`:** Define your `probe` logic based on PCI Vendor/Device IDs or USB Class Codes.
3. **Register:** Call `sigma_driver_registry::register_driver()` in your subsystem initialization.
4. **Test:** Add a QEMU test case in `tools/qemu_driver_test.sh`.

## 5. Subsystem Maintainers
- **USB & NVMe:** @DriversLead
- **GPU & Display:** @DriversLead
- **Audio:** @DriversLead
- **Network (NIC/Wi-Fi):** @NetworkLead

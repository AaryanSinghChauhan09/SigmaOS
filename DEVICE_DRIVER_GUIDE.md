# SigmaOS Device Driver Development Guide

## Overview

This guide provides comprehensive instructions for developing device drivers for SigmaOS, leveraging the OOP-based driver framework and capability-based security model.

## Table of Contents

1.  [Driver Architecture](#driver-architecture)
2.  [Development Environment Setup](#development-environment-setup)
3.  [Basic Driver Structure](#basic-driver-structure)
4.  [Driver Types](#driver-types)
5.  [Security and Capabilities](#security-and-capabilities)
6.  [Testing and Debugging](#testing-and-debugging)
7.  [Driver Integration](#driver-integration)
8.  [Best Practices](#best-practices)

## Driver Architecture

### OOP-Based Driver Framework

SigmaOS uses an object-oriented programming approach with Rust traits to define driver interfaces:

```rust
pub trait Device {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn device_class(&self) -> DeviceClass;
    fn initialize(&mut self) -> Result<(), DeviceError>;
    fn shutdown(&mut self) -> Result<(), DeviceError>;
}

pub trait DeviceDriver {
    fn device_id(&self) -> DeviceID;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError>;
    fn ioctl(&mut self, request: u32, arg: usize) -> Result<(), DeviceError>;
}
```

### Device Manager Integration

All drivers are registered with the central device manager:

```rust
pub trait DeviceManager {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, DeviceError>;
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), DeviceError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn list_devices(&self, device_class: DeviceClass) -> Vec<DeviceID>;
}
```

## Development Environment Setup

### Prerequisites

```bash
# Install required toolchain
sudo apt install -y build-essential nasm cmake qemu-system-x86

# Install Rust with no_std support
rustup target add x86_64-unknown-none
rustup component add rust-src

# Clone SigmaOS repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

### Driver Development Directory Structure

    src/drivers/
    ├── mod.rs
    ├── network/
    │   ├── mod.rs
    │   ├── ethernet.rs
    │   └── wifi.rs
    ├── storage/
    │   ├── mod.rs
    │   ├── nvme.rs
    │   └── ahci.rs
    ├── input/
    │   ├── mod.rs
    │   ├── keyboard.rs
    │   └── mouse.rs
    └── graphics/
        ├── mod.rs
        └── vesa.rs

## Basic Driver Structure

### Simple Device Driver Template

```rust
#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use sigmaos::device::{Device, DeviceClass, DeviceError, DeviceID};
use sigmaos::device::manager::DeviceManager;

pub struct MyCustomDevice {
    id: DeviceID,
    name: [u8; 64],
    device_class: DeviceClass,
    // Device-specific fields
    base_address: usize,
    interrupt_line: u8,
}

impl MyCustomDevice {
    pub fn new(id: DeviceID, name: &str, base_address: usize, interrupt_line: u8) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(
                name.as_ptr(),
                name_array.as_mut_ptr(),
                name_len
            );
        }
        
        Self {
            id,
            name: name_array,
            device_class: DeviceClass::Character,
            base_address,
            interrupt_line,
        }
    }
}

impl Device for MyCustomDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    
    fn device_class(&self) -> DeviceClass {
        self.device_class
    }
    
    fn initialize(&mut self) -> Result<(), DeviceError> {
        // Hardware initialization sequence
        self.reset_hardware();
        self.configure_interrupts();
        self.enable_device();
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), DeviceError> {
        // Cleanup and shutdown sequence
        self.disable_interrupts();
        self.reset_hardware();
        Ok(())
    }
}

impl MyCustomDevice {
    fn reset_hardware(&self) {
        // Implement hardware reset
        unsafe {
            let reset_reg = (self.base_address + 0x00) as *mut u32;
            core::ptr::write_volatile(reset_reg, 0x01);
        }
    }
    
    fn configure_interrupts(&self) {
        // Configure interrupt handling
        // This would integrate with SigmaOS interrupt manager
    }
    
    fn enable_device(&self) {
        unsafe {
            let enable_reg = (self.base_address + 0x04) as *mut u32;
            core::ptr::write_volatile(enable_reg, 0x01);
        }
    }
    
    fn disable_interrupts(&self) {
        // Disable interrupt handling
    }
    
    fn disable_device(&self) {
        unsafe {
            let enable_reg = (self.base_address + 0x04) as *mut u32;
            core::ptr::write_volatile(enable_reg, 0x00);
        }
    }
}
```

## Driver Types

### Network Drivers

Network drivers implement the `NetworkDriver` trait:

```rust
pub trait NetworkDriver {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), NetworkError>;
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
    fn get_mac_address(&self) -> [u8; 6];
    fn set_promiscuous_mode(&mut self, enabled: bool);
}

pub struct EthernetDriver {
    device_id: DeviceID,
    mac_address: [u8; 6],
    base_address: usize,
    promiscuous: bool,
}

impl NetworkDriver for EthernetDriver {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), NetworkError> {
        // Transmit packet via hardware
        Ok(())
    }
    
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        // Receive packet from hardware
        Ok(0)
    }
    
    fn get_mac_address(&self) -> [u8; 6] {
        self.mac_address
    }
    
    fn set_promiscuous_mode(&mut self, enabled: bool) {
        self.promiscuous = enabled;
    }
}
```

### Storage Drivers

Storage drivers implement the `StorageDriver` trait:

```rust
pub trait StorageDriver {
    fn read_block(&mut self, lba: u64, buffer: &mut [u8]) -> Result<(), StorageError>;
    fn write_block(&mut self, lba: u64, data: &[u8]) -> Result<(), StorageError>;
    fn flush_cache(&mut self) -> Result<(), StorageError>;
    fn get_capacity(&self) -> u64;
}

pub struct NvmeDriver {
    device_id: DeviceID,
    namespace_id: u32,
    queue_pairs: Vec<QueuePair>,
}

impl StorageDriver for NvmeDriver {
    fn read_block(&mut self, lba: u64, buffer: &mut [u8]) -> Result<(), StorageError> {
        // NVMe read command implementation
        Ok(())
    }
    
    fn write_block(&mut self, lba: u64, data: &[u8]) -> Result<(), StorageError> {
        // NVMe write command implementation
        Ok(())
    }
    
    fn flush_cache(&mut self) -> Result<(), StorageError> {
        // NVMe flush implementation
        Ok(())
    }
    
    fn get_capacity(&self) -> u64 {
        // Return namespace capacity
        0
    }
}
```

### Input Drivers

Input drivers handle user input devices:

```rust
pub trait InputDriver {
    fn read_event(&mut self) -> Option<InputEvent>;
    fn set_led_state(&mut self, led: LedType, state: bool);
}

pub enum InputEvent {
    KeyEvent { key_code: u8, pressed: bool },
    MouseEvent { x: u16, y: u16, buttons: u8 },
}

pub enum LedType {
    NumLock,
    CapsLock,
    ScrollLock,
}

pub struct Ps2KeyboardDriver {
    device_id: DeviceID,
    data_port: u16,
    command_port: u16,
}
```

### Graphics Drivers

Graphics drivers handle display output:

```rust
pub trait GraphicsDriver {
    fn set_mode(&mut self, mode: DisplayMode) -> Result<(), GraphicsError>;
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32);
    fn blit_buffer(&mut self, x: u32, y: u32, buffer: &[u8], width: u32, height: u32);
    fn get_framebuffer(&mut self) -> Option<&mut [u8]>;
}

pub struct VesaDriver {
    device_id: DeviceID,
    framebuffer: Option<*mut u8>,
    current_mode: DisplayMode,
}
```

## Security and Capabilities

### Capability-Based Access Control

SigmaOS uses capability-based security for device access:

```rust
use sigmaos::security::{CapabilityToken, SecurityEnforcer};

pub struct DriverCapabilities {
    pub can_read: bool,
    pub can_write: bool,
    pub can_ioctl: bool,
    pub resource_limits: ResourceLimits,
}

impl DriverCapabilities {
    pub fn new_minimal() -> Self {
        Self {
            can_read: true,
            can_write: false,
            can_ioctl: false,
            resource_limits: ResourceLimits::minimal(),
        }
    }
    
    pub fn new_full() -> Self {
        Self {
            can_read: true,
            can_write: true,
            can_ioctl: true,
            resource_limits: ResourceLimits::maximum(),
        }
    }
}

// Usage in driver
impl MyCustomDevice {
    pub fn read_with_capability(&mut self, buffer: &mut [u8], capabilities: &DriverCapabilities) -> Result<usize, DeviceError> {
        if !capabilities.can_read {
            return Err(DeviceError::PermissionDenied);
        }
        // Perform read operation
        Ok(0)
    }
}
```

### Windows NT Pool Tags

For memory allocation tracking, use Windows NT-style pool tags:

```rust
use sigmaos::kernel::memory::{KernelPoolManager, PoolType};

impl MyCustomDevice {
    pub fn allocate_pool(&self, size: usize, pool_manager: &mut KernelPoolManager) -> Result<(), &'static str> {
        let tag = b"MyD "; // 4-character driver tag
        pool_manager.allocate_pool(PoolType::NonPaged, size, tag)?;
        Ok(())
    }
}
```

## Testing and Debugging

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_initialization() {
        let device = MyCustomDevice::new(1, "test_device", 0x1000, 10);
        assert_eq!(device.id(), 1);
        assert_eq!(device.device_class(), DeviceClass::Character);
    }
    
    #[test]
    fn test_memory_pool_allocation() {
        let mut pool_manager = KernelPoolManager::new();
        let device = MyCustomDevice::new(1, "test_device", 0x1000, 10);
        assert!(device.allocate_pool(1024, &mut pool_manager).is_ok());
    }
}
```

### Standalone Compilation

Test driver compilation independently:

```bash
# Test driver compilation
rustc --test --edition=2021 src/drivers/network/ethernet.rs -o build/ethernet_tests
./build/ethernet_tests
rm build/ethernet_tests
```

### Debugging Techniques

1.  **Serial Output Debugging**:

```rust
use sigmaos::serial::SerialWriter;

let mut serial = SerialWriter::new();
writeln!(serial, "Driver initialization: device_id={}", device.id());
```

2.  **Memory Debugging**:

```rust
// Enable memory debugging in configuration
let debug_config = MemoryDebugConfig {
    track_allocations: true,
    detect_leaks: true,
    verbose_logging: true,
};
```

3.  **Interrupt Debugging**:

```rust
// Log interrupt handling
pub fn handle_interrupt(&mut self) {
    log_interrupt(self.interrupt_line, "MyCustomDevice");
    // Handle interrupt
}
```

## Driver Integration

### Registration with Device Manager

```rust
use sigmaos::device::manager::SimpleDeviceManager;

fn register_custom_driver(manager: &mut SimpleDeviceManager) -> Result<(), DeviceError> {
    let device = Box::new(MyCustomDevice::new(
        1,
        "custom_device",
        0x1000,
        10
    ));
    manager.register_device(device)?;
    Ok(())
}
```

### Module Integration

Add driver to `src/drivers/mod.rs`:

```rust
pub mod network;
pub mod storage;
pub mod input;
pub mod graphics;
pub mod custom; // Your custom driver

pub use custom::MyCustomDevice;
```

### Build System Integration

Update `Cargo.toml` to include driver module:

```toml
[lib]
name = "sigmaos_drivers"
path = "src/drivers/mod.rs"

[dependencies]
sigmaos_core = { path = "../sigmaos_core" }
```

## Best Practices

### Memory Safety

1.  **Use Safe Abstractions**: Prefer safe Rust abstractions over unsafe code
2.  **Bounds Checking**: Always validate array indices and buffer sizes
3.  **Memory Pools**: Use appropriate memory pools (Paged vs NonPaged)
4.  **Zero-Copy**: Use zero-copy techniques where possible for performance

### Error Handling

```rust
pub enum DeviceError {
    Success = 0,
    NotFound = 1,
    AlreadyRegistered = 2,
    InitFailed = 3,
    PermissionDenied = 4,
    InvalidParameter = 5,
    HardwareError = 6,
    Timeout = 7,
}

impl Device for MyCustomDevice {
    fn initialize(&mut self) -> Result<(), DeviceError> {
        // Validate parameters
        if self.base_address == 0 {
            return Err(DeviceError::InvalidParameter);
        }
        
        // Try hardware initialization
        match self.try_hardware_init() {
            Ok(_) => Ok(()),
            Err(_) => Err(DeviceError::InitFailed),
        }
    }
}
```

### Resource Management

```rust
impl Drop for MyCustomDevice {
    fn drop(&mut self) {
        // Ensure proper cleanup
        let _ = self.shutdown();
    }
}
```

### Documentation

````rust
/// Custom device driver for XYZ hardware
/// 
/// This driver provides support for the XYZ device family with the following features:
/// - DMA transfers for high-performance I/O
/// - Interrupt-driven operation
/// - Power management support
/// 
/// # Hardware Specifications
/// - Base Address: Configurable via device tree
/// - Interrupt Line: IRQ 10-15
/// - DMA Channels: 0-3
/// 
/// # Example
/// ```rust
/// let device = MyCustomDevice::new(1, "xyz_device", 0x1000, 10);
/// device.initialize()?;
/// ```
pub struct MyCustomDevice {
    // Implementation
}
````

## Advanced Topics

### DMA Operations

```rust
pub struct DmaTransfer {
    physical_address: u64,
    size: usize,
    direction: DmaDirection,
}

pub enum DmaDirection {
    ToDevice,
    FromDevice,
    Bidirectional,
}

impl MyCustomDevice {
    pub fn setup_dma_transfer(&mut self, buffer: &[u8]) -> Result<DmaTransfer, DeviceError> {
        // Setup DMA transfer
        Ok(DmaTransfer {
            physical_address: 0,
            size: buffer.len(),
            direction: DmaDirection::ToDevice,
        })
    }
}
```

### Power Management

```rust
pub enum PowerState {
    D0,  // Fully on
    D1,  // Partial power savings
    D2,  // Greater power savings
    D3,  // Sleep
}

impl MyCustomDevice {
    pub fn set_power_state(&mut self, state: PowerState) -> Result<(), DeviceError> {
        match state {
            PowerState::D0 => self.restore_full_power(),
            PowerState::D3 => self.enter_low_power_mode(),
            _ => Ok(()),
        }
    }
}
```

### Hot-Plug Support

```rust
pub trait DeviceHotplug {
    fn on_device_added(&mut self, device_id: DeviceID);
    fn on_device_removed(&mut self, device_id: DeviceID);
    fn enable_hotplug(&mut self, enabled: bool);
}

impl DeviceHotplug for MyCustomDevice {
    fn on_device_added(&mut self, device_id: DeviceID) {
        log_info("Device added: {}", device_id);
    }
    
    fn on_device_removed(&mut self, device_id: DeviceID) {
        log_info("Device removed: {}", device_id);
    }
    
    fn enable_hotplug(&mut self, enabled: bool) {
        // Enable/disable hot-plug detection
    }
}
```

## Resources

*   [SigmaOS API Reference](API_REFERENCE)
*   [Kernel Customization Guide](KERNEL_CUSTOMIZATION_GUIDE)
*   [Security Hardening Guide](SECURITY_HARDENING_GUIDE)
*   [Hardware Compatibility List](HARDWARE_COMPATIBILITY)

## Contributing

When contributing drivers:

1.  Follow the driver development guidelines
2.  Include comprehensive tests
3.  Document hardware specifications
4.  Provide usage examples
5.  Ensure security best practices

## License

Copyright © 2026 SigmaOS Project. Licensed under MIT License.

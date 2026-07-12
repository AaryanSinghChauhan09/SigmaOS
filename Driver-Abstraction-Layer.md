# Driver Abstraction Layer

## Overview

The SigmaOS Driver Abstraction Layer provides a universal framework for device drivers using OOP principles with Rust traits. This ensures that old drivers remain functional even as the kernel evolves, achieving the goal of eternal compatibility.

## Architecture

### DeviceDriver Trait

All SigmaOS drivers must implement the `DeviceDriver` trait for a consistent interface:

```rust
pub trait DeviceDriver {
    fn init(&mut self) -> SigmaI32;
    fn shutdown(&mut self) -> SigmaI32;
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32;
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32;
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32;
    fn get_info(&self) -> DriverInfo;
    fn get_stats(&self) -> DriverStats;
    fn reset(&mut self) -> SigmaI32;
    fn suspend(&mut self) -> SigmaI32;
    fn resume(&mut self) -> SigmaI32;
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool;
    fn get_status(&self) -> DriverStatus;
}
```

### Driver Capabilities

Drivers declare their capabilities using the `DriverCapability` enum:

- `Read` - Read operations supported
- `Write` - Write operations supported
- `Ioctl` - IOCTL operations supported
- `Mmap` - Memory mapping supported
- `Dma` - DMA operations supported
- `Interrupt` - Interrupt handling supported
- `Hotplug` - Hot-plug support
- `PowerManagement` - Power management features

### Driver Lifecycle

Drivers follow a well-defined lifecycle:

1. **Uninitialized** - Driver not yet loaded
2. **Initializing** - Driver is initializing
3. **Active** - Driver is operational
4. **Suspended** - Driver is suspended (power saving)
5. **Failed** - Driver failed to initialize
6. **Deprecated** - Driver marked as deprecated

## BaseDriver Implementation

The `BaseDriver` struct provides common functionality for all drivers:

```rust
pub struct BaseDriver {
    pub info: DriverInfo,
    pub stats: DriverStats,
    pub status: DriverStatus,
}
```

### Creating a Custom Driver

To create a custom driver:

1. Inherit from `BaseDriver`
2. Implement the `DeviceDriver` trait
3. Add driver-specific functionality
4. Register with the driver registry

```rust
pub struct MyCustomDriver {
    base: BaseDriver,
    // Custom fields
}

impl DeviceDriver for MyCustomDriver {
    fn init(&mut self) -> SigmaI32 {
        self.base.init();
        self.base.add_capability(DriverCapability::Read);
        self.base.add_capability(DriverCapability::Write);
        // Custom initialization
        0
    }
    
    // Implement other trait methods...
}
```

## Driver Registry

The `DriverRegistry` manages all loaded drivers:

```rust
pub struct DriverRegistry {
    pub drivers: [*mut dyn DeviceDriver; 256],
    pub driver_count: SigmaU32,
}
```

### Registry Operations

- `register()` - Register a new driver
- `unregister()` - Unregister a driver
- `get_driver()` - Get a driver by index
- `initialize_all()` - Initialize all registered drivers
- `shutdown_all()` - Shutdown all registered drivers

## Driver Categories

### GPU Drivers
- `sigma_gpu.rs` - Native GPU driver for NVIDIA, AMD, Intel
- Supports OpenGL, Vulkan, DirectX
- Power management and overclocking

### Network Drivers
- `sigma_wifi.rs` - Native Wi-Fi driver
- `sigma_ethernet.rs` - Ethernet driver
- Support for various chipsets

### Storage Drivers
- `sigma_nvme.rs` - NVMe SSD driver
- `sigma_ahci.rs` - SATA/AHCI driver
- Support for modern storage devices

### Input Drivers
- PS/2 keyboard and mouse
- USB HID devices
- Touchpad and touchscreen support

### Audio Drivers
- HDA (High Definition Audio)
- ALSA compatibility layer
- Support for various audio codecs

## Integration with Legacy Drivers

The abstraction layer provides compatibility shims for legacy drivers:

1. **C API wrappers** - Wrap C-based driver APIs
2. **Legacy emulation** - Emulate old hardware interfaces
3. **Capability mapping** - Map legacy features to modern capabilities

## Performance Considerations

- Zero-copy operations where possible
- Efficient interrupt handling
- DMA support for high-throughput devices
- Lock-free data structures for driver statistics

## Security

- All drivers run in kernel space with privilege checks
- Capability-based access control
- Secure DMA with IOMMU
- Signed driver modules

## Testing

Driver testing framework provides:

- Unit tests for driver operations
- Integration tests with hardware simulation
- Performance benchmarks
- Fuzzing for robustness

## Future Enhancements

- Hot-plug support for all device types
- Power management integration
- Driver sandboxing for user-space drivers
- Live driver updates without reboot

## References

- [Device Driver Implementation Guide](Driver-Implementation-Guide.md)
- [Legacy Driver Archive](Legacy-Driver-Archive.md)
- [Driver Development Tutorial](Driver-Development-Tutorial.md)

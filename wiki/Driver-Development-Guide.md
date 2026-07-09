# SigmaOS Driver Development Guide

## Overview

This guide provides comprehensive information for developing drivers for SigmaOS. SigmaOS follows Linux's proven model where most drivers are open source and kernel-integrated, providing hardware support tested against every kernel change.

## Driver Architecture

SigmaOS uses an Object-Oriented Programming (OOP) approach with Rust traits to define common interfaces for different device types. This ensures:

- **Type Safety**: Rust's type system ensures memory safety and prevents common driver bugs
- **Polymorphism**: Traits allow different devices to share common interfaces
- **Encapsulation**: Device-specific implementation details are hidden behind trait abstractions
- **Extensibility**: New drivers can be added by implementing standard traits

## Driver Categories

### 1. Network Drivers

Located in `drivers/net/`

#### Supported Drivers
- **e1000e**: Intel e1000e Ethernet Controller (I219-V, I219-LM, etc.)
- **r8169**: Realtek r8169/r8168 Ethernet Controller (8169, 8168, 8411, etc.)
- **sigma_virtio_net**: VirtIO network driver for virtualized environments

#### Base Trait: `EthernetDevice`

```rust
pub trait EthernetDevice {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    fn is_initialized(&self) -> bool;
    fn get_device_name(&self) -> &'static str;
    fn get_mac_address(&self) -> EthernetAddress;
    fn set_mac_address(&mut self, mac: EthernetAddress) -> I32;
    fn get_mtu(&self) -> U32;
    fn set_mtu(&mut self, mtu: U32) -> I32;
    fn get_link_status(&self) -> EthernetLinkStatus;
    fn set_link_config(&mut self, speed: EthernetSpeed, duplex: EthernetDuplex) -> I32;
    fn set_autoneg(&mut self, enable: bool) -> I32;
    fn set_promiscuous(&mut self, enable: bool) -> I32;
    fn set_multicast(&mut self, enable: bool) -> I32;
    fn add_multicast_address(&mut self, mac: EthernetAddress) -> I32;
    fn remove_multicast_address(&mut self, mac: EthernetAddress) -> I32;
    fn set_all_multicast(&mut self, enable: bool) -> I32;
    fn enable(&mut self) -> I32;
    fn disable(&mut self) -> I32;
    fn transmit(&mut self, buffer: *const U8, length: U32) -> I32;
    fn receive(&mut self, buffer: *mut U8, max_length: U32) -> I32;
    fn get_stats(&self) -> EthernetStats;
    fn reset_stats(&mut self);
    fn reset(&mut self) -> I32;
    fn shutdown(&mut self) -> I32;
}
```

#### Base Trait: `EthernetPhy`

```rust
pub trait EthernetPhy {
    fn read_phy(&self, phy_addr: U8, reg: U8) -> U16;
    fn write_phy(&mut self, phy_addr: U8, reg: U8, value: U16) -> I32;
    fn get_phy_id(&self, phy_addr: U8) -> U32;
    fn reset_phy(&mut self, phy_addr: U8) -> I32;
    fn get_phy_link_status(&self, phy_addr: U8) -> bool;
    fn get_phy_speed_duplex(&self, phy_addr: U8) -> (EthernetSpeed, EthernetDuplex);
}
```

#### Example: Creating a Network Driver

```rust
use super::ethernet_device_base::{EthernetDevice, EthernetPhy, ...};

pub struct MyNetworkController {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub mac_address: EthernetAddress,
    // ... device-specific fields
}

impl MyNetworkController {
    pub const fn new() -> Self {
        // Initialize device-specific fields
    }

    fn init_my_controller(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        // Device-specific initialization
    }
}

impl EthernetDevice for MyNetworkController {
    // Implement all required trait methods
}

impl EthernetPhy for MyNetworkController {
    // Implement PHY operations
}
```

### 2. Storage Drivers

Located in `drivers/storage/`

#### Supported Drivers
- **ahci**: AHCI SATA Controller (Intel, AMD, VIA, NVIDIA, Marvell)
- **nvme**: NVMe SSD Controller
- **sigma_virtio_blk**: VirtIO block driver for virtualized environments

#### Base Trait: `StorageDevice`

```rust
pub trait StorageDevice {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    fn is_initialized(&self) -> bool;
    fn get_device_name(&self) -> &'static str;
    fn get_storage_type(&self) -> StorageType;
    fn get_storage_protocol(&self) -> StorageProtocol;
    fn get_geometry(&self) -> StorageGeometry;
    fn get_identify(&self, identify: *mut StorageIdentify) -> I32;
    fn read(&mut self, lba: U64, buffer: *mut U8, sector_count: U32) -> I32;
    fn write(&mut self, lba: U64, buffer: *const U8, sector_count: U32) -> I32;
    fn submit_request(&mut self, request: *mut StorageRequest) -> I32;
    fn cancel_request(&mut self, request: *mut StorageRequest) -> I32;
    fn flush(&mut self) -> I32;
    fn get_capacity(&self) -> U64;
    fn get_sector_size(&self) -> U32;
    fn reset(&mut self) -> I32;
    fn shutdown(&mut self) -> I32;
}
```

### 3. GPU Drivers

Located in `drivers/gpu/`

#### Supported Drivers
- **sigma_amdgpu**: AMD Radeon GPU (Vega, Navi, RDNA2)
- **sigma_i915**: Intel GPU (Gen 6-12, Arc)
- **sigma_nvidia**: NVIDIA GPU (Kepler, Maxwell, Pascal)
- **sigma_virtio_gpu**: VirtIO GPU driver for virtualized environments

#### Base Trait: `GpuDevice`

```rust
pub trait GpuDevice: Device {
    fn set_mode(&mut self, width: U32, height: U32, refresh: U32) -> I32;
    fn enable_display(&mut self) -> I32;
    fn disable_display(&mut self) -> I32;
    fn get_framebuffer_info(&self) -> Option<FramebufferInfo>;
    fn submit_command(&mut self, cmd: U32, data: U64) -> I32;
    fn map_page(&mut self, physical: U64, virtual_addr: U64) -> I32;
    fn unmap_page(&mut self, virtual_addr: U64) -> I32;
    fn get_memory_info(&self) -> GpuMemoryInfo;
}
```

### 4. USB Drivers

Located in `drivers/usb/`

#### Supported Drivers
- **xhci**: USB 3.0/3.1 xHCI Controller
- **ehci**: USB 2.0 EHCI Controller
- **uhci**: USB 1.1 UHCI Controller
- **ohci**: USB 1.1 OHCI Controller

#### Base Trait: `UsbController`

```rust
pub trait UsbController {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    fn is_initialized(&self) -> bool;
    fn get_controller_name(&self) -> &'static str;
    fn get_speed(&self) -> UsbSpeed;
    fn reset(&mut self) -> I32;
    fn shutdown(&mut self) -> I32;
    fn get_device_descriptor(&self, descriptor: *mut UsbDeviceDescriptor) -> I32;
    fn get_configuration_descriptor(&self, config_index: U8, buffer: *mut U8, length: *mut U16) -> I32;
    fn control_transfer(&mut self, request_type: U8, request: U8, value: U16, index: U16, buffer: *mut U8, length: U16) -> I32;
    fn bulk_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    fn interrupt_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    fn isochronous_transfer(&mut self, endpoint: U8, direction: UsbDirection, buffer: *mut U8, length: U32) -> I32;
    // ... more methods
}
```

### 5. Input Drivers

Located in `drivers/input/`

#### Supported Drivers
- **hid**: USB HID (Human Interface Device) driver
- **ps2_keyboard**: PS/2 Keyboard driver
- **ps2_mouse**: PS/2 Mouse driver
- **synaptics**: Synaptics Touchpad driver
- **elan**: ELAN Touchpad driver

#### Base Trait: `HidDriver`

```rust
pub trait HidDriver {
    fn init(&mut self, device_id: U16, vendor_id: U16) -> I32;
    fn is_initialized(&self) -> bool;
    fn get_device_name(&self) -> &'static str;
    fn get_usage_page(&self) -> HidUsagePage;
    fn get_usage(&self) -> U16;
    fn get_input_report(&mut self, report_id: U8) -> I32;
    fn set_output_report(&mut self, report_id: U8, data: &[U8]) -> I32;
    fn get_feature_report(&mut self, report_id: U8, buffer: &mut [U8]) -> I32;
    fn set_feature_report(&mut self, report_id: U8, data: &[U8]) -> I32;
    fn enable(&mut self) -> I32;
    fn disable(&mut self) -> I32;
    fn reset(&mut self) -> I32;
    fn shutdown(&mut self) -> I32;
}
```

## PCI Configuration Access

All drivers need to access PCI configuration space to probe for devices and read device information. SigmaOS provides helper functions for this:

```rust
/// Read 8-bit value from PCI configuration space
unsafe fn read_pci_config_u8(bus: U8, device: U8, function: U8, offset: U8) -> U8 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 3) as u32) * 8;
    ((value >> shift) & 0xFF) as U8
}

/// Read 16-bit value from PCI configuration space
unsafe fn read_pci_config_u16(bus: U8, device: U8, function: U8, offset: U8) -> U16 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 2) as u32) * 8;
    ((value >> shift) & 0xFFFF) as U16
}

/// Read 32-bit value from PCI configuration space
unsafe fn read_pci_config_u32(bus: U8, device: U8, function: U8, offset: U8) -> U32 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    inl(0xCFC)
}
```

**Note**: The `outl` and `inl` functions are placeholders that should be implemented with inline assembly for the target architecture.

## Device Probing

Drivers should implement a probe function to scan the PCI bus for supported devices:

```rust
#[no_mangle]
pub unsafe extern "C" fn my_driver_probe() -> I32 {
    let mut found_devices = 0;
    
    // Scan PCI buses 0-255
    for bus in 0..256u8 {
        // Scan devices 0-31
        for device in 0..32u8 {
            // Scan functions 0-7
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                
                // Check if this is a supported device
                if vendor_id == MY_VENDOR_ID && is_my_device(device_id) {
                    let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                    let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                    
                    let result = G_MY_DRIVER.init(mmio_base, device_id);
                    
                    if result == DRIVER_OK {
                        found_devices += 1;
                        return DRIVER_OK;
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        DRIVER_OK
    } else {
        DRIVER_ERR_NO_DEVICE
    }
}
```

## MMIO Access

Drivers access device registers through Memory-Mapped I/O (MMIO):

```rust
/// Read MMIO register
unsafe fn read_mmio(&self, offset: U32) -> U32 {
    let ptr = (self.mmio_base + offset as U64) as *const U32;
    *ptr
}

/// Write MMIO register
unsafe fn write_mmio(&self, offset: U32, value: U32) {
    let ptr = (self.mmio_base + offset as U64) as *mut U32;
    *ptr = value
}
```

## Error Handling

SigmaOS uses standard error codes across all drivers:

```rust
pub const DRIVER_OK: I32 = 0;
pub const DRIVER_ERR_NO_DEVICE: I32 = -1;
pub const DRIVER_ERR_INIT_FAILED: I32 = -2;
pub const DRIVER_ERR_OUT_OF_MEM: I32 = -3;
pub const DRIVER_ERR_NOT_SUPPORTED: I32 = -4;
pub const DRIVER_ERR_INVALID_PARAM: I32 = -5;
pub const DRIVER_ERR_TIMEOUT: I32 = -6;
```

## C-ABI Exports

Drivers should export C-compatible functions for integration with the kernel:

```rust
#[no_mangle]
pub unsafe extern "C" fn my_driver_init(pci_bar: U64, device_id: U16) -> I32 {
    G_MY_DRIVER.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn my_driver_is_initialized() -> I32 {
    if G_MY_DRIVER.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn my_driver_shutdown() -> I32 {
    G_MY_DRIVER.shutdown()
}
```

## Best Practices

1. **Use `#![no_std]`**: All kernel drivers must use `#![no_std]` as they run in a freestanding environment
2. **Type Aliases**: Use type aliases (e.g., `type U8 = u8`) for consistency with Linux kernel conventions
3. **Unsafe Code**: Mark all hardware access as `unsafe` and document safety invariants
4. **Error Handling**: Always check return values and propagate errors appropriately
5. **Resource Cleanup**: Ensure proper cleanup in `shutdown()` methods
6. **Documentation**: Document register offsets, bit fields, and device-specific quirks
7. **Testing**: Implement unit tests for driver logic where possible

## Linux Kernel Driver References

When developing SigmaOS drivers, reference the following Linux kernel drivers:

- **Network**: `drivers/net/ethernet/intel/e1000e/`, `drivers/net/ethernet/realtek/r8169/`
- **Storage**: `drivers/ata/ahci.c`, `drivers/nvme/host/`
- **GPU**: `drivers/gpu/drm/amd/amdgpu/`, `drivers/gpu/drm/i915/`
- **USB**: `drivers/usb/host/`
- **Input**: `drivers/hid/`, `drivers/input/keyboard/`, `drivers/input/mouse/`

## Contributing

When contributing a new driver:

1. Follow the SigmaOS coding standards
2. Implement the appropriate base trait
3. Add comprehensive documentation
4. Include probe and initialization functions
5. Add error handling for all failure cases
6. Test on real hardware when possible
7. Update this guide with device-specific information

## License

All SigmaOS drivers are licensed under GPL-2.0-or-later, consistent with the Linux kernel.

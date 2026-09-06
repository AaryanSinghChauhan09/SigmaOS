# SigmaOS Driver Development Guide

## Overview

SigmaOS drivers live in `src/driver/`, `src/drivers/`, and the C++ layer in `drivers/`. All Rust drivers implement the `SigmaDriver` trait.

## Driver Architecture

```
Application / Kernel Service
         ↓
   Driver Manager (src/driver/mod.rs)
         ↓
   Device Driver (implements SigmaDriver)
         ↓
   Hardware Abstraction Layer
         ↓
   Physical Hardware
```

## Driver Trait

```rust
pub trait SigmaDriver: Send + Sync {
    fn name(&self) -> &str;
    fn probe(&mut self, device: &DeviceInfo) -> Result<(), DriverError>;
    fn remove(&mut self) -> Result<(), DriverError>;
    fn read(&self, offset: usize, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write(&mut self, offset: usize, buf: &[u8]) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DriverError>;
    fn interrupt_handler(&mut self) -> bool { false }
}
```

## Available Drivers

### Storage
| Driver | File | Status |
|--------|------|--------|
| NVMe | `src/driver/nvme.rs` | ✅ |
| AHCI SATA | `src/driver/ahci.rs` | ✅ |
| USB Mass Storage | `src/driver/usb_mass_storage.rs` | ✅ |
| VirtIO Block | `src/driver/virtio_blk.rs` | ✅ |

### Network
| Driver | File | Status |
|--------|------|--------|
| Intel e1000e | `src/driver/e1000e.rs` | ✅ |
| Realtek RTL8139 | `src/driver/rtl8139.rs` | ✅ |
| VirtIO Net | `src/driver/virtio_net.rs` | ✅ |
| WiFi (iwlwifi) | `src/driver/iwlwifi.rs` | ⬜ Beta |

### Graphics
| Driver | File | Status |
|--------|------|--------|
| VESA Framebuffer | `src/driver/vesa_fb.rs` | ✅ |
| VirtIO GPU | `src/driver/virtio_gpu.rs` | ✅ |
| Intel i915 | `src/driver/intel_i915.rs` | ⬜ Beta |

### Input
| Driver | File | Status |
|--------|------|--------|
| PS/2 Keyboard | `src/driver/ps2_keyboard.rs` | ✅ |
| HID USB Input | `src/driver/hid_input_device.rs` | ✅ |
| xHCI USB Host | `src/driver/xhci_usb.rs` | ✅ |

## Writing a New Driver

1. Create `src/driver/my_device.rs`
2. Implement `SigmaDriver` trait
3. Register with driver manager in `src/driver/mod.rs`
4. Add PCI device ID if applicable
5. Write tests

```rust
pub struct MyDriver {
    mmio_base: *mut u8,
    irq: u8,
}

unsafe impl Send for MyDriver {}
unsafe impl Sync for MyDriver {}

impl SigmaDriver for MyDriver {
    fn name(&self) -> &str { "my-device" }
    
    fn probe(&mut self, device: &DeviceInfo) -> Result<(), DriverError> {
        // Map MMIO, configure hardware
        Ok(())
    }
    
    fn read(&self, _offset: usize, buf: &mut [u8]) -> Result<usize, DriverError> {
        // Read from hardware
        Ok(buf.len())
    }
    
    fn write(&mut self, _offset: usize, buf: &[u8]) -> Result<usize, DriverError> {
        // Write to hardware
        Ok(buf.len())
    }
    
    fn remove(&mut self) -> Result<(), DriverError> {
        // Clean up
        Ok(())
    }
}
```

## x86 Port I/O

For legacy x86 port I/O (replacing unsafe `_inl`/`_outl`):

```rust
// Safe wrappers using inline assembly
pub fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe { core::arch::asm!("inl %dx, %eax", in("dx") port, out("eax") value, options(nostack, preserves_flags, att_syntax)); }
    value
}

pub fn outl(port: u16, value: u32) {
    unsafe { core::arch::asm!("outl %eax, %dx", in("eax") value, in("dx") port, options(nostack, preserves_flags, att_syntax)); }
}
```

## PCI Enumeration

```rust
// Scan PCI bus for devices
let devices = pci::enumerate_bus()?;
for dev in &devices {
    println!("{:04x}:{:04x} - {}", dev.vendor_id, dev.device_id, dev.description());
    if let Some(driver) = DRIVER_REGISTRY.find(dev.vendor_id, dev.device_id) {
        driver.probe(dev)?;
    }
}
```

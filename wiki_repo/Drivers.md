# SigmaOS Drivers

## Overview

SigmaOS includes drivers for various hardware components. This document describes the driver architecture and available drivers.

## Driver Architecture

### Driver Model

SigmaOS uses a modular driver model:

```
┌─────────────────────────────────────┐
│         Userland Applications       │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         Device Abstraction Layer    │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         Hardware Drivers            │
│  ┌──────┐ ┌──────┐ ┌──────┐       │
│  │Network│ │ GPU  │ │Storage│       │
│  └──────┘ └──────┘ └──────┘       │
└─────────────────────────────────────┘
              │
┌─────────────────────────────────────┐
│         Hardware Abstraction Layer   │
└─────────────────────────────────────┘
```

### Driver Interface

```rust
pub trait Driver {
    fn name(&self) -> &str;
    fn init(&mut self) -> Result<(), DriverError>;
    fn probe(&self) -> bool;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u64, arg: u64) -> Result<u64, DriverError>;
}
```

## Network Drivers

### Ethernet Drivers

#### Intel e1000

**Location**: `kernel/drivers/net/e1000.rs`

**Features**:
- Gigabit Ethernet support
- Interrupt-driven I/O
- DMA transfers
- Jumbo frames

**Initialization**:
```rust
pub unsafe fn e1000_init(mmio_base: u64) -> Result<(), DriverError> {
    // Reset device
    // Configure DMA
    // Setup RX/TX rings
    // Enable interrupts
}
```

#### Realtek r8169

**Location**: `kernel/drivers/net/r8169.rs`

**Features**:
- Fast Ethernet support
- PCI interface
- Hardware checksumming

#### Virtio-net

**Location**: `kernel/drivers/net/virtio_net.rs`

**Features**:
- Paravirtualized network
- High performance in VMs
- Multi-queue support

### Wireless Drivers

#### Intel iwlwifi

**Location**: `kernel/drivers/net/iwlwifi.rs`

**Features**:
- 802.11a/b/g/n/ac support
- MIMO
- WPA2/WPA3 encryption

#### MediaTek mt7921

**Location**: `kernel/drivers/net/mt7921.rs`

**Features**:
- 802.11ax (Wi-Fi 6)
- Bluetooth coexistence
- Low power consumption

## Storage Drivers

### AHCI/SATA

**Location**: `kernel/drivers/storage/ahci.rs`

**Features**:
- SATA 3.0 support
- NCQ (Native Command Queuing)
- Hot-plug support

**Initialization**:
```rust
pub unsafe fn ahci_init(abar: u64) -> Result<(), DriverError> {
    // Enable AHCI
    // Scan for devices
    // Initialize ports
    // Setup command lists
}
```

### NVMe

**Location**: `kernel/drivers/storage/nvme.rs`

**Features**:
- PCIe SSD support
- High performance
- Multiple namespaces

### Virtio-blk

**Location**: `kernel/drivers/storage/virtio_blk.rs`

**Features**:
- Paravirtualized block device
- High performance in VMs
- Support for multiple queues

## Graphics Drivers

### Intel i915

**Location**: `kernel/drivers/gpu/i915.rs`

**Features**:
- Intel integrated graphics
- DRM/KMS support
- Hardware acceleration

### AMD amdgpu

**Location**: `kernel/drivers/gpu/amdgpu.rs`

**Features**:
- AMD Radeon GPUs
- Vulkan support
- Hardware video decoding

### Virtio-gpu

**Location**: `kernel/drivers/gpu/virtio_gpu.rs`

**Features**:
- Paravirtualized GPU
- 2D acceleration
- 3D support (virgl)

## Input Drivers

### Keyboard

**Location**: `kernel/drivers/input/keyboard.rs`

**Features**:
- PS/2 keyboard support
- USB keyboard support
- Layout configuration

### Mouse

**Location**: `kernel/drivers/input/mouse.rs`

**Features**:
- PS/2 mouse support
- USB mouse support
- Scroll wheel support

## Audio Drivers

### Intel HDA

**Location**: `kernel/drivers/audio/hda.rs`

**Features**:
- High Definition Audio
- Multi-channel support
- Hardware mixing

### USB Audio

**Location**: `kernel/drivers/audio/usb_audio.rs`

**Features**:
- USB audio class
- Plug-and-play
- Low latency

## Interrupt Controller Drivers

### APIC

**Location**: `kernel/core/hal/apic.rs`

**Features**:
- Local APIC
- I/O APIC
- Interrupt routing
- MSI support

**Initialization**:
```rust
pub unsafe fn apic_init() -> Result<(), DriverError> {
    // Enable APIC
    // Configure interrupt vectors
    // Setup I/O APIC
    // Enable MSI
}
```

### PIC (8259)

**Location**: `kernel/core/hal/pic.rs`

**Features**:
- Legacy 8259 PIC
- Cascade mode
- IRQ masking

## Timer Drivers

### HPET

**Location**: `kernel/core/hal/hpet.rs`

**Features**:
- High Precision Event Timer
- Nanosecond precision
- Multiple timers

### APIC Timer

**Location**: `kernel/core/hal/apic_timer.rs`

**Features**:
- Per-CPU timers
- One-shot mode
- Periodic mode

## UART/Serial Driver

**Location**: `kernel/drivers/uart.rs`

**Features**:
- 16550 UART compatibility
- Configurable baud rate
- Interrupt-driven I/O

**Initialization**:
```rust
pub unsafe fn uart_init(base: u64, baud: u32) -> Result<(), DriverError> {
    // Configure baud rate
    // Set data format
    // Enable interrupts
}
```

## USB Drivers

### USB Core

**Location**: `kernel/drivers/usb/core.rs`

**Features**:
- USB 2.0/3.0 support
- Hub support
- Device enumeration

### USB Host Controllers

#### EHCI (USB 2.0)

**Location**: `kernel/drivers/usb/ehci.rs`

**Features**:
- Enhanced Host Controller Interface
- High-speed USB
- Isochronous transfers

#### XHCI (USB 3.0)

**Location**: `kernel/drivers/usb/xhci.rs`

**Features**:
- Extensible Host Controller Interface
- SuperSpeed USB
- USB 3.1 support

## Driver Development

### Writing a New Driver

1. **Create driver file**:
   ```rust
   // kernel/drivers/my_driver.rs
   use kernel::drivers::Driver;

   pub struct MyDriver {
       // Driver state
   }

   impl Driver for MyDriver {
       fn name(&self) -> &str {
           "my_driver"
       }

       fn init(&mut self) -> Result<(), DriverError> {
           // Initialize hardware
           Ok(())
       }

       // Implement other methods
   }
   ```

2. **Register driver**:
   ```rust
   // kernel/drivers/mod.rs
   pub mod my_driver;

   pub fn init_drivers() {
       let mut driver = my_driver::MyDriver::new();
       driver.init().expect("Failed to init driver");
   }
   ```

3. **Add to build system**:
   ```toml
   # kernel/Cargo.toml
   [dependencies]
   # ...
   ```

### Driver Best Practices

1. **Error handling**: Always check return values
2. **Resource cleanup**: Implement cleanup on failure
3. **Interrupt safety**: Use proper synchronization
4. **DMA**: Use proper DMA mappings
5. **Power management**: Implement suspend/resume

## Driver Debugging

### Debug Output

Add debug prints to driver:
```rust
#[cfg(debug_assertions)]
println!("Driver: {}", message);
```

### Hardware Inspection

Use tools to inspect hardware:
```bash
# List PCI devices
lspci

# List USB devices
lsusb

# Inspect memory
cat /proc/iomem
```

### Tracing

Enable driver tracing:
```rust
pub fn trace_read(&self, offset: usize, value: u32) {
    println!("Read offset={:x} value={:x}", offset, value);
}
```

## Future Drivers

### Planned Drivers

1. **Bluetooth**: Bluetooth controller support
2. **Camera**: USB camera support
3. **Touchscreen**: Touchscreen input
4. **Fingerprint**: Biometric authentication
5. **TPM**: Trusted Platform Module

### Research Areas

1. **GPU compute**: OpenCL/CUDA support
2. **AI accelerators**: NPU support
3. **FPGA**: Programmable hardware
4. **Quantum**: Quantum computing interfaces

## Troubleshooting

### Driver Not Loading

**Symptoms**: Driver fails to initialize

**Solutions**:
1. Check hardware is present
2. Verify driver is registered
3. Check for resource conflicts
4. Review debug output

### Device Not Working

**Symptoms**: Device initialized but not functional

**Solutions**:
1. Check configuration
2. Verify firmware is loaded
3. Test with different hardware
4. Review driver logs

### Performance Issues

**Symptoms**: Poor device performance

**Solutions**:
1. Enable DMA
2. Use interrupts instead of polling
3. Optimize buffer sizes
4. Profile driver code

## References

- [Linux Device Drivers](https://lwn.net/Kernel/LDD3/)
- [OSDev Drivers](https://wiki.osdev.org/Category:Device_Drivers)
- [PCI Specification](https://pcisig.com/specifications)

# SigmaOS Driver Development Guide

## Overview

SigmaOS implements drivers using a **clean-room, no-std Rust** approach with zero dependency on Linux kernel headers or Windows WDK. All drivers follow the SigmaOS Driver Model (SDM).

---

## Driver Architecture

```
drivers/
├── mod.rs           # Driver registry and bus manager
├── pci/             # PCI/PCIe bus enumeration
├── usb/             # USB host controller drivers
├── storage/         # AHCI, NVMe, virtio-blk
├── net/             # Network device drivers
├── gpu/             # GPU/display drivers
└── input/           # HID: keyboard, mouse, touchpad
```

---

## Writing a Driver

### Step 1: Implement the Driver Trait

```rust
use crate::drivers::{Driver, DeviceId, DriverError};

pub struct MyDriver {
    base_addr: usize,
    irq: u8,
}

impl Driver for MyDriver {
    fn name(&self) -> &str { "my-device" }

    fn probe(&mut self, device: &DeviceId) -> Result<(), DriverError> {
        // Check if this device is ours
        if device.vendor_id != 0x1234 || device.device_id != 0x5678 {
            return Err(DriverError::NotSupported);
        }
        self.init_hardware()
    }

    fn remove(&mut self) {
        self.shutdown_hardware();
    }

    fn interrupt_handler(&mut self) {
        // Handle IRQ
        self.process_pending_events();
    }
}
```

### Step 2: Register the Driver

```rust
// In drivers/mod.rs or your module's init
use crate::drivers::DRIVER_REGISTRY;

pub fn register() {
    DRIVER_REGISTRY.lock().register(Box::new(MyDriver::new()));
}
```

### Step 3: Device Tree / ACPI Binding

For hardware discovery, drivers bind via:
- **PCI subsystem**: Vendor ID + Device ID matching
- **ACPI**: HID/CID string matching (e.g., "PNP0303" for PS/2 keyboard)
- **Device Tree** (ARM): Compatible string matching

---

## DRM/KMS Display Driver

**Module:** `src/distro/linux_bsd_inspirations.rs` — `DrmModeInfo`

For display drivers, implement the KMS interface:

```rust
let mode = DrmModeInfo::new(1920, 1080, 60);

// Validate timing before applying
if mode.verify_timing_boundaries() {
    display.apply_mode(&mode);
}
```

### Timing Specification
- `hdisplay` / `vdisplay`: Active pixel area
- `hsync_start`, `hsync_end`, `htotal`: Horizontal timing
- `vsync_start`, `vsync_end`, `vtotal`: Vertical timing
- `clock`: Pixel clock in kHz
- `vrefresh`: Refresh rate in Hz

---

## Memory-Mapped I/O (MMIO)

For MMIO register access, use the safe wrappers:

```rust
use crate::klib::mmio::{read32, write32};

// Safe MMIO read/write with memory barriers
let value = unsafe { read32(base_addr + REG_STATUS) };
unsafe { write32(base_addr + REG_CTRL, CTRL_ENABLE) };
```

---

## Interrupt Handling

Register interrupt handlers through the IRQ manager:

```rust
use crate::kernel::irq::IrqManager;

IrqManager::register(irq_num, |irq| {
    // Handle interrupt
    // Return true if interrupt was ours
    true
});
```

---

## DMA Support

For DMA operations, use the DMA allocator:

```rust
use crate::drivers::dma::{DmaBuffer, DmaDirection};

// Allocate physically contiguous DMA buffer
let buf = DmaBuffer::new(4096, DmaDirection::ToDevice)?;
let phys_addr = buf.phys_addr();

// Program DMA controller with physical address
device.set_dma_addr(phys_addr);
```

---

## PCI Driver Example: NVMe

```rust
pub struct SigmaNvme {
    bar0: usize,       // BAR0: NVMe register base
    admin_sq: Queue,   // Admin Submission Queue
    admin_cq: Queue,   // Admin Completion Queue
    io_queues: Vec<NvmeQueue>,
}

impl SigmaNvme {
    pub fn identify_controller(&mut self) -> NvmeIdentify {
        let cmd = NvmeCommand::identify(NSID_CONTROLLER);
        self.submit_admin(cmd);
        self.wait_completion()
    }

    pub fn read_sectors(&mut self, lba: u64, count: u16, buf: &mut [u8]) {
        let cmd = NvmeCommand::read(lba, count, buf.as_mut_ptr() as u64);
        self.submit_io(cmd);
    }
}
```

---

## USB Stack

**Module:** `src/drivers/usb/`

SigmaOS implements a clean-room USB stack:
- **XHCI** (USB 3.x) host controller
- **EHCI** (USB 2.0) host controller
- **USB HID**: Keyboard, mouse, gamepad
- **USB Mass Storage**: Flash drives, external HDDs
- **USB Serial**: CDC-ACM devices

```rust
use crate::drivers::usb::{UsbDevice, UsbClass};

// Enumerate connected devices
for device in usb_host.enumerate() {
    match device.class() {
        UsbClass::HID => hid_driver.attach(device),
        UsbClass::MassStorage => msd_driver.attach(device),
        _ => {}
    }
}
```

---

## Driver Safety Guidelines

1. **No panics in interrupt context** — use `Option`/`Result` instead
2. **No blocking in IRQ handlers** — defer work to kernel threads
3. **Cache-aligned DMA buffers** — use `DmaBuffer::new_aligned()`
4. **Memory barriers** — use `fence(Ordering::SeqCst)` around MMIO
5. **Document all `unsafe` blocks** — explain invariants maintained

---

## Testing Drivers

```bash
# Run driver unit tests
cargo test --features=driver-tests -p sigmaos -- drivers::

# QEMU virtual hardware testing
make qemu-driver-test DRIVER=nvme

# USB testing with virtual USB device
make usb-test
```

---

*See also:*
- [DRIVER_DEVELOPMENT_PLAN_2026.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DRIVER_DEVELOPMENT_PLAN_2026.md)
- [UNIVERSAL_DRIVER_SUPPORT_PLAN.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/UNIVERSAL_DRIVER_SUPPORT_PLAN.md)
- [DRIVER_MANAGEMENT_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DRIVER_MANAGEMENT_ROADMAP.md)

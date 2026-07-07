# Driver Development Guide

> SigmaOS v15.0 "Zenith" — Writing Kernel Drivers

## Overview

SigmaOS drivers are Rust modules compiled into the kernel image. All driver code must be `#![no_std]` compatible. Drivers interact with hardware via MMIO (Memory-Mapped I/O) and PCI configuration space.

---

## Quick Start: Adding a New Driver

### 1. Create the driver file

```
kernel/drivers/<category>/sigma_<device>.rs
```

Categories: `gpu/`, `net/`, `audio/`, `bt/`, `storage/`, `input/`, `usb/`

### 2. PCI Device Detection

Use the `pci_find()` helper pattern to locate your device:

```rust
fn pci_find_device(vendor_id: u16, device_id: u16) -> Option<(usize, usize)> {
    for bus in 0u8..=255 {
        for slot in 0u8..32 {
            let addr = 0x8000_0000u32 | ((bus as u32) << 16) | ((slot as u32) << 11);
            let id = pci_read32(addr);
            if id == 0xFFFF_FFFF { continue; }
            if (id & 0xFFFF) as u16 == vendor_id && (id >> 16) as u16 == device_id {
                let bar0 = (pci_read32(addr | 0x10) & !0xF) as usize; // MMIO base
                let bar1 = (pci_read32(addr | 0x14) & !0xF) as usize;
                return Some((bar0, bar1));
            }
        }
    }
    None
}
```

### 3. MMIO Access Pattern

Always use `read_volatile` / `write_volatile` for MMIO registers:

```rust
fn read32(base: usize, offset: usize) -> u32 {
    unsafe { core::ptr::read_volatile((base + offset) as *const u32) }
}

fn write32(base: usize, offset: usize, val: u32) {
    unsafe { core::ptr::write_volatile((base + offset) as *mut u32, val) }
}
```

### 4. Register the Driver

Add to `kernel/drivers/mod.rs`:

```rust
pub mod sigma_mydevice;
```

Add init call in `kernel_main()`:
```rust
if sigma_mydevice::init() {
    log!("MyDevice driver loaded");
}
```

---

## Interrupt Handling

Register your IRQ handler:

```rust
// In your driver init:
sigma_irq::register_handler(irq_number, my_irq_handler);

// Your handler:
fn my_irq_handler() {
    // Read interrupt status register
    let status = read32(BASE, STATUS_REG);
    // Handle the interrupt
    // ...
    // Acknowledge
    sigma_irq::eoi(irq_number);
}
```

---

## DMA

For DMA operations, allocate physically contiguous pages:

```rust
let phys_addr = buddy_allocator::alloc_pages(order)?;
let virt_addr = sigma_vmm::phys_to_virt(phys_addr);
```

Always flush CPU cache before device DMA reads, and invalidate after device DMA writes.

---

## ARM64 Drivers

For ARM64 targets, use the GIC instead of APIC:

```rust
sigma_gic::enable_irq(irq_number);
// ... your handler ...
sigma_gic::eoi(irq_number);
```

The BCM2711 (Raspberry Pi 4) HAL is in `arch/arm64/sigma_bcm2711.rs`.

---

## Existing Drivers Reference

| Driver | File | Hardware |
|---|---|---|
| VirtIO GPU | `sigma_virtio_gpu.rs` | QEMU/KVM virtual GPU |
| Intel i915 | `sigma_i915.rs` | Intel Gen9+ integrated graphics |
| AMD GPU | `sigma_amdgpu.rs` | AMD discrete/integrated GPU |
| Intel Wi-Fi 6 | `sigma_iwlwifi.rs` | AX200/AX210 802.11ax |
| Realtek USB Wi-Fi | `sigma_rtl8xxxu.rs` | RTL8XXXU USB devices |
| Intel HDA | `sigma_hda.rs` | Intel HD Audio (CORB/RIRB) |
| USB Bluetooth | `sigma_hci_usb.rs` | Generic USB HCI |
| ARM GIC | `sigma_gic.rs` | ARM Generic Interrupt Controller |
| BCM2711 | `sigma_bcm2711.rs` | Raspberry Pi 4 HAL |

---

## Coding Standards

- Use `#![no_std]` and `#![allow(dead_code)]` at the top of every driver file
- Static driver state must use `static mut` guarded by an `AtomicBool` ready flag
- Never `panic!` in interrupt context
- All public functions must have doc comments
- Run `cargo clippy` before submitting a PR

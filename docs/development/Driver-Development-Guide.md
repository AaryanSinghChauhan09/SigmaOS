# SigmaOS Driver Development Guide

> Build stable, sovereign, AI-portable drivers for SigmaOS.
> One framework. Stable ABI. Works on every SigmaOS version forever.

---

## Why SigmaOS Drivers Are Different

| | Windows | Linux | SigmaOS |
|---|---|---|---|
| ABI stability | Stable (decades) | Breaks on kernel update | **Stable across all versions** |
| Source model | Mostly closed | Mostly open | **Open + closed coexist** |
| Isolation | Ring-0 (kernel crash) | Ring-0 | **Ring-3 option (crash-safe)** |
| Security model | Vendor trust | None | **sigma_pledge per driver** |
| AI porting | None | None | **sigma-driver-porter** |
| Vendor friction | High (WHQL cert) | Medium | **Low (open DDK + AI help)** |

---

## Quick Start

```bash
# Install DDK tools
sigma-pkg install sigma-ddk

# Scaffold a new driver
sigma-shard-new my-nic-driver --template networking

# Or port a Linux driver
sigma-driver-porter port linux_driver.c
sigma-driver-porter port linux_driver.c --ai  # AI-assisted translation

# Build
cd sigma-my-nic-driver && cargo build --release

# Validate
sigma-ddk validate target/release/libsigma_my_nic_driver.so

# List registered drivers
sigma-ddk list
```

---

## The SDF Lifecycle

Every SigmaOS driver follows the Sovereign Driver Framework (SDF):

```
sigma_ddk loads driver
       │
       ▼
  probe(pci_bar, irq)   → return 0 if hardware found, -1 if not
       │
       ▼
  init()                → map MMIO, alloc DMA, request IRQ, call sigma_pledge()
       │
       ▼
  run() / IRQ loop      → handle events, communicate via sigma-bus
       │
       ▼
  shutdown()            → release all resources
```

---

## Minimal Driver Example

```rust
// SPDX-License-Identifier: MIT
// my-nic/src/lib.rs

#![no_std]
use sigma_ddk::*;

const REG_CTRL:   u32 = 0x00;
const REG_STATUS: u32 = 0x04;

#[no_mangle]
pub extern "C" fn my_nic_probe(pci_bar: u64, irq: u8) -> i32 {
    // Check PCI vendor/device ID at config space offset 0
    let id = pci_config_read32(0, 0, 0, 0);
    if id & 0xFFFF != 0x8086 { return -1; }  // not our device
    let _ = (pci_bar, irq);
    0
}

#[no_mangle]
pub extern "C" fn my_nic_init() -> i32 {
    // 1. Restrict capabilities (sigma_pledge)
    unsafe {
        extern "C" { fn sigma_pledge(p: *const u8, l: usize) -> i32; }
        let pledge = b"stdio rpath inet\0";
        sigma_pledge(pledge.as_ptr(), pledge.len());
    }
    // 2. Map MMIO (production: use pci_bar from probe)
    let _bar = iomap(0xFEB00000, 0x10000);
    // 3. Configure hardware registers
    // mmio_write32(bar as *mut u32, REG_CTRL, 0x01);  // enable
    0
}

#[no_mangle]
pub extern "C" fn my_nic_shutdown() {
    // Disable hardware, free DMA, release IRQ
}

#[no_mangle]
pub extern "C" fn my_nic_irq() -> bool {
    // Read interrupt status, handle RX/TX
    // sigma_bus_send(BUS_NETWORK, &event, size_of::<NetEvent>());
    true  // interrupt was ours
}

// Register with SDF
sigma_register_driver!(SigmaDriverDescriptor {
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,
    vendor_id:   0x8086,
    device_id:   0x100E,   // Intel e1000
    class:       DriverClass::Network as u16,
    flags:       SIGMA_DRV_FLAG_OPEN_SOURCE,
    ring:        3,        // ring-3 isolated (recommended)
    fn_probe:    Some(my_nic_probe),
    fn_init:     Some(my_nic_init),
    fn_shutdown: Some(my_nic_shutdown),
    fn_irq:      Some(my_nic_irq),
    ..Default::default()
});
```

---

## sigma_pledge for Drivers

Every driver must call `sigma_pledge()` at the start of `init()` to declare its capabilities. This limits the damage if the driver is exploited.

| Driver type | Recommended pledge |
|---|---|
| Network NIC | `"stdio rpath inet"` |
| Storage block | `"stdio rpath wpath"` |
| GPU display | `"stdio video"` |
| Audio | `"stdio audio"` |
| USB HID | `"stdio device"` |
| Serial/I2C | `"stdio device"` |
| Crypto accelerator | `"stdio"` |

---

## sigma-bus Communication

Drivers communicate with userspace via sigma-bus typed channels:

```rust
// Notify userspace of received network packet
let event = NetRxEvent { len: pkt.len, flags: 0 };
sigma_bus_send(BUS_NETWORK, &event as *const _ as *const u8,
               core::mem::size_of::<NetRxEvent>());

// Channels
const BUS_NETWORK:  u32 = 0x0100;
const BUS_STORAGE:  u32 = 0x0200;
const BUS_DISPLAY:  u32 = 0x0300;
const BUS_AUDIO:    u32 = 0x0400;
const BUS_INPUT:    u32 = 0x0500;
```

---

## Ring-3 Driver Isolation

Setting `ring: 3` in the descriptor runs the driver in an isolated ring-3 process. If it crashes, the kernel keeps running.

```rust
// Ring-3 isolated driver — crashes don't take down kernel
sigma_register_driver!(SigmaDriverDescriptor {
    ring:  3,    // 0 = ring-0 (kernel), 3 = userspace process
    flags: SIGMA_DRV_FLAG_RING3 | SIGMA_DRV_FLAG_OPEN_SOURCE,
    // ... rest of descriptor
});
```

Recommended: use ring-3 for all third-party and community drivers. Ring-0 only for performance-critical core drivers (storage, NIC) after security review.

---

## AI-Assisted Porting from Linux

If you have a Linux driver to port (cleanroom — don't copy GPL code, study patterns):

```bash
# Analyse the Linux driver structure
sigma-driver-porter analyse linux_rtl8169.c

# Generate SigmaOS skeleton from patterns
sigma-driver-porter port linux_rtl8169.c

# Full AI translation (needs sigma-agent daemon)
sigma-driver-porter port linux_rtl8169.c --ai

# The tool maps Linux APIs → SigmaOS equivalents:
# ioremap          → ddk::iomap
# readl/writel     → ddk::mmio_read32/write32
# request_irq      → ddk::request_irq
# kmalloc          → kfree/kmalloc
# netdev_alloc     → sigma_bus_send
# pci_register_driver → sigma_register_driver
```

---

## Stable ABI Guarantee

The `SigmaDriverDescriptor` struct layout is **frozen at DDK v1.0**. Drivers compiled today will work on SigmaOS v20.0 without recompilation.

Rules:
- New fields only added at the end of the struct
- ABI version bumped only for breaking changes (never planned)
- Old drivers gracefully ignored if ABI version < required

```rust
// Check ABI version at driver load time
if desc.abi_version != DDK_ABI_VERSION {
    // Kernel handles version mismatch gracefully
}
```

---

## Dual Mode: Open + Closed Drivers

SigmaOS supports both:

```toml
# Open source driver (preferred)
flags = SIGMA_DRV_FLAG_OPEN_SOURCE

# Vendor-supplied closed blob (e.g., NVIDIA proprietary)
flags = SIGMA_DRV_FLAG_CERTIFIED   # vendor-signed binary
```

Closed drivers are accepted with:
1. Dilithium-5 vendor signature
2. Published security contact
3. Ring-3 isolation enforced (no ring-0 for closed drivers)

---

## Submitting to sigma_pkg_registry

```bash
# 1. Write sigma-shard.toml with driver metadata
# 2. Build and validate
cargo build --release
sigma-ddk validate target/release/libmy_driver.so

# 3. Create package recipe
sigma-pkg recipe create my-nic-driver

# 4. Submit PR to sigma_pkg_registry/recipes/
# File: sigma_pkg_registry/recipes/sigma-driver-my-nic.toml
```

---

## Getting Vendor Certification

Want the **SIGMA_DRV_FLAG_CERTIFIED** badge?

1. Submit driver to https://github.com/AaryanSinghChauhan09/SigmaOS/issues (Driver Certification)
2. Provide: source code (or binary + security contact), test results, hardware to lend for CI
3. SigmaOS team reviews, signs with project Dilithium-5 key
4. Listed in sigma-ddk certified registry

Benefits: driver shows "✓ Certified" in sigma-capstore, higher trust score, auto-included in SigmaOS ISO for supported hardware.

---

*See also: [sigma-ddk CLI](Driver-Development-Guide) · [Shard Development Guide](Shard-Development-Guide) · [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model)*

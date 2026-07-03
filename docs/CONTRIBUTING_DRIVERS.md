# Contributing a Driver

Step-by-step guide to contributing a hardware driver to SigmaOS.

---

## Before You Start

1. Check if the driver already exists: `sigma-ddk-vendors list`
2. Check hardware status: [Linux-Parity-Roadmap](../wiki_repo/Linux-Parity-Roadmap.md)
3. Find the Linux upstream driver for reference (GPL source, cleanroom study only — don't copy code)
4. Get the vendor datasheet if available (preferred for register definitions)

---

## Method 1: Use sigma-driver-porter (Recommended)

```bash
# 1. Analyse the Linux driver structure
sigma-driver-porter analyse linux_rtl8169.c

# 2. Generate SigmaOS skeleton (cleanroom — no GPL code copied)
sigma-driver-porter port linux_rtl8169.c -o my_drivers/

# 3. Fill in register definitions from datasheet
cd my_drivers/rtl8169
# Edit src/lib.rs — fill in REG_CTRL, REG_STATUS, etc.

# 4. Build
cargo build --release --target x86_64-sigmaos.json

# 5. Validate
sigma-ddk validate target/x86_64-sigmaos/release/librtl8169.a
```

---

## Method 2: Use the Driver SDK

```bash
# Create a new driver from scratch
cd sdk/driver
cp -r examples/virtio_blk ../my_device_driver
cd my_device_driver
```

Edit `src/main.rs`:

```rust
use sigma_driver_sdk::{Driver, DeviceInfo, DriverContext, DriverResult};

struct MyDeviceDriver { /* hardware state */ }

impl Driver for MyDeviceDriver {
    fn name(&self) -> &str { "my-device" }

    fn probe(&self, device: &DeviceInfo) -> bool {
        device.vendor_id == 0xABCD && device.device_id == 0x1234
    }

    fn init(&mut self, ctx: &mut DriverContext) -> DriverResult<()> {
        ctx.map_bar0(4096)?;   // map MMIO
        ctx.bind_irq()?;       // bind interrupt
        // sigma_pledge("stdio video") — declare capabilities
        Ok(())
    }

    fn handle_irq(&mut self) -> bool {
        // handle interrupt, return true if IRQ was ours
        true
    }

    fn shutdown(&mut self) { /* quiesce hardware */ }
}
```

---

## Method 3: Kernel-space SDF Driver (no_std)

For drivers that need kernel-space access:

```rust
// kernel/drivers/my_device.rs
#![no_std]

use crate::ddk::*;

pub extern "C" fn my_device_probe(bar: u64, irq: u8) -> i32 {
    // Verify PCI vendor/device ID
    let ids = unsafe { pci_config_read32(0, 0, 0, 0) };
    if ids & 0xFFFF != 0xABCD { return -1; }
    0
}

pub extern "C" fn my_device_init() -> i32 {
    // Map MMIO, setup DMA rings, request IRQ
    0
}

pub extern "C" fn my_device_irq() -> bool { true }
pub extern "C" fn my_device_shutdown() {}

sigma_register_driver!(SigmaDriverDescriptor {
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,
    vendor_id:   0xABCD,
    device_id:   0x1234,
    ring:        3,   // ring-3 isolated
    flags:       SIGMA_DRV_FLAG_OPEN_SOURCE,
    fn_probe:    Some(my_device_probe),
    fn_init:     Some(my_device_init),
    fn_shutdown: Some(my_device_shutdown),
    fn_irq:      Some(my_device_irq),
    ..Default::default()
});
```

---

## Testing Your Driver

```bash
# QEMU test (inject PCI device)
qemu-system-x86_64 \
  -kernel sigma-kernel.bin \
  -device pci-stub,subsystem-vendor-id=0xABCD,subsystem-id=0x1234 \
  -serial stdio

# Run unit tests
cargo test

# Check for memory safety issues
cargo miri test    # (when possible)
```

---

## Driver Checklist

Before submitting a PR:

- [ ] sigma_pledge() called at start of init()
- [ ] IRQ handler returns correct bool (true = handled)
- [ ] shutdown() quiesces hardware (no DMA after shutdown)
- [ ] No unsafe code without `// SAFETY:` comment
- [ ] No predefined stdlib functions (kernel drivers: `#![no_std]`)
- [ ] `sigma-ddk validate` passes
- [ ] QEMU smoke test passes
- [ ] Hardware register definitions from datasheet (not copied from Linux)

---

## Submitting

```bash
git checkout -b driver/my-device
git add drivers/my_device/
git commit -m "driver: add MyDevice (0xABCD:0x1234)"
git push origin driver/my-device
# Open PR — title: "Driver: <Vendor> <Device>"
```

PR description must include:
- Hardware name and PCI IDs
- Test results (QEMU output)
- `sigma-ddk validate` output
- Link to datasheet or reference

---

*See also: [Driver Framework](../wiki_repo/Driver-Framework.md) · [Vendor Playbook](VENDOR_PLAYBOOK.md) · [Kernel ABI Stability](../wiki_repo/Kernel-ABI-Stability.md)*

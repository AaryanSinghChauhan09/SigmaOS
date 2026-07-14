# SigmaOS Vendor Playbook

A step-by-step guide for hardware vendors to write, certify, and distribute
SigmaOS drivers for their devices.

---

## Why Write a SigmaOS Driver?

- **Stable ABI**: a driver compiled today works on all future SigmaOS versions (DDK v1.0 frozen)

- **Ring-3 isolation**: driver crashes don't take down the kernel — better UX for your customers

- **sigma_pledge**: fine-grained capability declaration shows customers exactly what your driver does

- **AI porting**: `sigma-driver-porter` generates a skeleton from your Linux driver in minutes

- **Certification**: official "Sigma Certified" badge in the app store increases customer trust

- **Open source incentive**: open drivers get higher transparency scores + community maintenance

---

## Step 1: Understand the Framework

Read:

- [Driver Framework](../wiki_repo/Driver-Framework.md) — SDF architecture

- [Kernel ABI Stability](../wiki_repo/Kernel-ABI-Stability.md) — DDK v1.0 guarantee

- `drivers/ddk/sigma_ddk.rs` — descriptor format

---

## Step 2: Generate a Driver Skeleton

```bash

# If you have a Linux driver source (cleanroom study — don't copy GPL code)

sigma-driver-porter analyse /path/to/linux_mydevice.c
sigma-driver-porter port /path/to/linux_mydevice.c -o my_drivers/

# Or start from scratch

sigma-shard-new my-device-driver --template network

# Generated:

#   my-device-driver/Cargo.toml

#   my-device-driver/src/lib.rs

#   my-device-driver/sigma-shard.toml

```

---

## Step 3: Implement the Driver

Fill in the four lifecycle functions:

```rust
// my-device-driver/src/lib.rs

use sigma_driver_sdk::{Driver, DeviceInfo, DriverContext, DriverResult};

pub struct MyDeviceDriver { /* hardware state */ }

impl Driver for MyDeviceDriver {
    fn name(&self) -> &str { "my-device" }

    fn probe(&self, device: &DeviceInfo) -> bool {
        // Check vendor_id and device_id
        device.vendor_id == 0xABCD && device.device_id == 0x1234
    }

    fn init(&mut self, ctx: &mut DriverContext) -> DriverResult<()> {
        // sigma_pledge declares what this driver is allowed to do
        // sigma_pledge("stdio video");  // GPU driver
        // sigma_pledge("stdio inet");   // NIC driver

        // Map MMIO, set up DMA, bind IRQ
        ctx.map_bar0(4096)?;
        ctx.bind_irq()?;
        Ok(())
    }

    fn handle_irq(&mut self) -> bool {
        // Handle interrupt, return true if IRQ was ours
        true
    }

    fn shutdown(&mut self) {
        // Quiesce hardware, release DMA, unmap MMIO
    }
}
```

---

## Step 4: Register with SDF

```rust
// At the bottom of src/lib.rs
sigma_register_driver!(SigmaDriverDescriptor {
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,        // 1 — stable forever
    vendor_id:   0xABCD,
    device_id:   0x1234,
    name:        *b"My Device Driver\0\0\0\0...",  // 64 bytes
    version:     *b"1.0.0\0\0...",
    author:      *b"MyCompany\0...",
    license:     *b"MIT\0...",
    flags:       SIGMA_DRV_FLAG_OPEN_SOURCE | SIGMA_DRV_FLAG_CERTIFIED,
    pledge_caps: 0x0001,               // stdio only
    ring:        3,                    // ring-3 isolated (required for Certified)
    fn_probe:    Some(mydevice_probe),
    fn_init:     Some(mydevice_init),
    fn_shutdown: Some(mydevice_shutdown),
    fn_irq:      Some(mydevice_irq),
    ..Default::default()
});
```

---

## Step 5: Build + Test

```bash

# Build for SigmaOS target

cargo build --release --target x86_64-sigmaos.json

# Validate DDK magic + ABI version

sigma-ddk validate target/x86_64-sigmaos/release/libmy_device.a

# Test in QEMU (inject fake PCI device)

qemu-system-x86_64 \
  -kernel sigma-kernel.bin \
  -device pci-stub,subsystem-vendor-id=0xABCD,subsystem-id=0x1234

# Run unit tests

cargo test --release
```

---

## Step 6: Apply for Certification

1. Open a GitHub Issue titled: `"Driver Certification: MyCompany MyDevice"`

2. Include:
   - Source code link (open source) or signed binary (closed source)
   - Hardware datasheet or register spec
   - Test results (QEMU + real hardware if possible)
   - `sigma-ddk validate` output

3. SigmaOS team reviews and signs with Dilithium-5

4. Driver appears as `SIGMA_DRV_FLAG_CERTIFIED` in the vendor registry

---

## Step 7: Distribute

```bash

# Package as sigpkg

sigma-pkg build sigma.toml

# Output: mydevice-driver-1.0.0-x86_64.sigpkg

# Publish

sigma-pkg publish mydevice-driver-1.0.0-x86_64.sigpkg \
  --registry https://pkg.sigmaos.io

# Users install with:

sigma-pkg install mydevice-driver
```

---

## Transparency Score

Vendors who open-source their drivers get:

```bash
sigma-ddk-vendors score

# MyCompany    ████████████████░░░░ 82/100

```

Scoring:

- Open-source drivers: +50 points

- Working drivers: +30 points

- Certified: +20 points

- Closed-source: -30 points penalty

---

## Contact

- Open a GitHub Discussion: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

- Driver certification issues: tag with `driver-certification`

- DDK questions: tag with `ddk`

---

*See also: [Driver Framework](../wiki_repo/Driver-Framework.md) · [Driver Development Guide](../wiki_repo/Driver-Development-Guide.md) · [SDK Guide](../wiki_repo/SDK-Guide.md)*

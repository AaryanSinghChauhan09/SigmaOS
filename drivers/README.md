# SigmaOS Drivers Subsystem

> Sovereign Driver Framework (SDF) — PQC-attested, lifecycle-managed, Ring-3-capable.
>
> Full reference: [docs/Open_Source_Drivers.md](../docs/Open_Source_Drivers.md)

---

## Philosophy

SigmaOS follows Linux's proven model: **most drivers are open source and kernel-integrated**.
This gives us:

- Hardware support tested against every kernel change.

- No unsigned code executing in Ring 0.

- Community contributors can improve drivers without vendor permission.

Proprietary blobs (NVIDIA closed, Broadcom Wi-Fi firmware) are supported as an **opt-in**
via `sigma-pkg install sigma-nonfree/<driver>` — isolated, integrity-checked, and audited.

---

## Directory Layout

```
drivers/
├── audio/          # HDA, PipeWire backend (planned v16.0)

├── block/          # VirtIO-blk, loop device

├── bsd/            # BSD driver compatibility shims

├── core/           # SDF bus infrastructure, probe/init/shutdown core

├── ddk/            # Driver Development Kit headers

├── display/        # KMS framebuffer, VESA

├── gpu/            # i915, amdgpu, Nouveau, Xe (open GPU drivers)

├── graphics/       # sigma_kms.rs — kernel mode-setting

├── hal/            # Architecture-specific HAL glue

├── input/          # HID, keyboard, mouse, touchscreen

├── linux/          # Linux driver compatibility layer

├── linux_distros/  # Distro-specific driver quirks

├── multimedia/     # V4L2, libcamera (planned)

├── net/            # e1000, r8169, VirtIO-net

├── network/        # Wi-Fi (iwlwifi, ath9k, ath11k, mt76)

├── printing/       # CUPS / HPLIP port (planned)

├── sigma/          # SigmaOS-native drivers (UVC, sigma-specific HW)

├── storage/        # AHCI, NVMe, SD/eMMC

├── unified/        # Cross-arch unified driver interfaces

└── usb/            # xHCI, HID, UVC, printer class

```

---

## Open-Source Driver Categories

### ✅ Stable Now (v15.0)

| Driver | Hardware | File |
|--------|----------|------|
| sigma-e1000 | Intel Gigabit | `net/sigma_e1000.rs` (via `sovereignnic.rs`) |
| sigma-nvme | NVMe SSDs | `sovereignnvme.rs` |
| sigma-xhci | USB 3.x | `sovereignusb.rs` |
| sigma-virtio-net | VirtIO NIC | `net/sigma_virtio_net.rs` |
| sigma-virtio-blk | VirtIO block | `block/sigma_virtio_blk.rs` |

### 🔄 In Progress (v15.1)

| Driver | Hardware |
|--------|----------|
| sigma-r8169 | Realtek Gigabit |
| sigma-ahci | SATA AHCI |
| sigma-iwlwifi | Intel Wi-Fi 6/6E |
| sigma-hid | USB keyboard/mouse |
| sigma-i915 | Intel GPU (Gen 6–12) |

### ⬜ Planned (v16.0 Apex)

- `sigma-amdgpu` — AMD Radeon RX 400+ open driver

- `sigma-nouveau` — Community NVIDIA open driver

- `sigma-xe` — Intel Arc open driver

- `sigma-mesa` — Mesa OpenGL 4.6 / Vulkan 1.3

- `sigma-bluez` — Bluetooth stack

- `sigma-ath9k` / `sigma-ath11k` — Qualcomm Wi-Fi

---

## SDF Driver Skeleton

```rust
use sigma_sdf::{SdfDriver, SdfResult, DeviceId};

pub struct MyDriver { base: usize }

impl SdfDriver for MyDriver {
    fn probe(dev: &DeviceId) -> bool {
        dev.vendor == 0xVVVV && dev.device == 0xDDDD
    }
    fn init(&mut self) -> SdfResult<()> {
        self.base = sigma_hal::pci_map_bar(0)?;
        Ok(())
    }
    fn shutdown(&mut self) {
        sigma_hal::pci_unmap_bar(self.base);
    }
}

sigma_sdf::register_driver!(MyDriver, "sigma-mydriver");
```

See [docs/Open_Source_Drivers.md](../docs/Open_Source_Drivers.md) for the complete guide,
proprietary blob policy, and contribution instructions.

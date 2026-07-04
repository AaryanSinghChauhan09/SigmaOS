# SigmaOS Open-Source Driver Strategy

> This document defines the driver philosophy for SigmaOS — what is open, what is
> proprietary, and how the Sovereign Driver Framework (SDF) integrates both.

---

## Why Open-Source Drivers Matter

Linux's greatest hardware-support strength comes directly from its open-source driver
ecosystem. Because the kernel is open source, hardware vendors and the community can
ship drivers that live **inside** the kernel tree — no binary blobs, no ABI promises
to maintain, no external module compilation at install time.

SigmaOS adopts this philosophy wholesale:

- **Stability first**: kernel-integrated drivers are tested with every kernel change.
- **Security by default**: no unsigned code executes in Ring 0 without PQC attestation.
- **Community velocity**: open drivers attract contributors; proprietary blobs stagnate.

---

## 1. Kernel-Integrated Drivers (SDF Tier 1)

These drivers live in `drivers/` and are compiled directly into the SigmaOS kernel or
loaded as PQC-attested kernel modules. They follow the
`probe() → init() → shutdown()` lifecycle enforced by the Sovereign Driver Framework.

### Network Cards

| Driver | Hardware | Source File | Status |
|--------|----------|-------------|--------|
| `sigma-e1000` | Intel Gigabit (82540–82579) | `drivers/net/sigma_e1000.rs` | ✅ Stable |
| `sigma-r8169` | Realtek Gigabit RTL8111/8168 | `drivers/net/sigma_r8169.rs` | 🔄 In Progress |
| `sigma-virtio-net` | VirtIO network (QEMU/KVM) | `drivers/net/sigma_virtio_net.rs` | ✅ Stable |
| `sigma-e1000e` | Intel PCIe Gigabit | `drivers/net/sigma_e1000e.rs` | ⬜ Planned |

### Storage Controllers

| Driver | Hardware | Source File | Status |
|--------|----------|-------------|--------|
| `sigma-nvme` | NVMe SSDs (PCIe) | `drivers/sovereignnvme.rs` | ✅ Stable |
| `sigma-ahci` | SATA AHCI controllers | `drivers/storage/sigma_ahci.rs` | 🔄 In Progress |
| `sigma-virtio-blk` | VirtIO block device | `drivers/block/sigma_virtio_blk.rs` | ✅ Stable |
| `sigma-sd` | SD/eMMC (ARM mobile) | `drivers/storage/sigma_sd.rs` | ⬜ Planned |

### USB

| Driver | Hardware | Source File | Status |
|--------|----------|-------------|--------|
| `sigma-xhci` | USB 3.x xHCI host controller | `drivers/sovereignusb.rs` | ✅ Stable |
| `sigma-hid` | USB HID (keyboard, mouse) | `drivers/input/sigma_hid.rs` | 🔄 In Progress |
| `sigma-uvc` | USB webcams (UVC class) | `drivers/sigma/sigma_uvc.rs` | ⬜ Planned |
| `sigma-usbprint` | USB printers | `drivers/printing/sigma_usbprint.rs` | ⬜ Planned |

---

## 2. Open GPU Drivers (SDF Tier 2)

### Intel

Intel graphics are **fully open source** under the MIT/GPL dual-license.
SigmaOS targets the modern `Xe` architecture and legacy `i915`.

| Driver | Chips | Source File | Status |
|--------|-------|-------------|--------|
| `sigma-i915` | Intel Gen 6–12 (HD/Iris/UHD) | `drivers/gpu/sigma_i915.rs` | 🔄 In Progress |
| `sigma-xe` | Intel Arc (Alchemist/Battlemage) | `drivers/gpu/sigma_xe.rs` | ⬜ Planned |

**Mesa integration**: `sigma-mesa` provides the OpenGL/Vulkan userspace stack
(`drivers/graphics/sigma_kms.rs` + Mesa 24.x via `sigma-pkg`).

### AMD

AMD's open-source driver stack (`amdgpu` / `radeon`) is maintained upstream in
the Linux kernel and Mesa.

| Driver | Chips | Source File | Status |
|--------|-------|-------------|--------|
| `sigma-amdgpu` | AMD Radeon RX 400+ (GCN 4+) | `drivers/gpu/sigma_amdgpu.rs` | ⬜ Planned |
| `sigma-radeon` | AMD Radeon HD 5000–7000 | `drivers/gpu/sigma_radeon.rs` | ⬜ Planned |

### NVIDIA (Open / Nouveau)

| Driver | Notes | Status |
|--------|-------|--------|
| `sigma-nouveau` | Community reverse-engineered open driver. No reclocking on Ampere+. | ⬜ Planned |
| `sigma-nvidia-open` | NVIDIA's own open-source kernel modules (Turing+, R560+). Requires firmware blobs. | ⬜ Planned v16.0 |

> **Note**: The official closed-source NVIDIA driver is supported via the
> `sigma-proprietary` compatibility layer (see §4 below).

---

## 3. Wireless & Bluetooth Drivers (SDF Tier 1/2)

### Wi-Fi

| Driver | Hardware | Source File | Status |
|--------|----------|-------------|--------|
| `sigma-iwlwifi` | Intel Wi-Fi 5/6/6E/7 (iwlwifi) | `drivers/sovereignwifi.rs` | 🔄 In Progress |
| `sigma-ath9k` | Qualcomm Atheros 802.11n | `drivers/network/sigma_ath9k.rs` | ⬜ Planned |
| `sigma-ath11k` | Qualcomm Wi-Fi 6 (QCA6390+) | `drivers/network/sigma_ath11k.rs` | ⬜ Planned |
| `sigma-mt76` | MediaTek Wi-Fi 5/6 | `drivers/network/sigma_mt76.rs` | ⬜ Planned |
| `sigma-rtw89` | Realtek 802.11ax | `drivers/network/sigma_rtw89.rs` | ⬜ Planned |

**Broadcom note**: Most Broadcom Wi-Fi chips require proprietary firmware blobs
(`brcmfmac` in Linux). SigmaOS supports loading these blobs via the
`sigma-firmware-loader` shim — see §4.

### Bluetooth

| Component | Description | Status |
|-----------|-------------|--------|
| `sigma-bluez` | BlueZ Bluetooth stack port | ⬜ Planned |
| `sigma-hci-usb` | HCI over USB transport | ⬜ Planned |
| `sigma-hci-uart` | HCI over UART (embedded) | ⬜ Planned |

---

## 4. Community & Mesa Projects

| Project | SigmaOS Integration | Status |
|---------|-------------------|--------|
| **Mesa** | OpenGL 4.6, Vulkan 1.3 via `sigma-mesa` shard | ⬜ Planned v16.0 |
| **Nouveau** | `sigma-nouveau` kernel module | ⬜ Planned |
| **HPLIP** | HP printer driver port via `sigma-cups` | ⬜ Planned |
| **libcamera** | Unified camera framework | ⬜ Planned |
| **PipeWire** | Audio/video routing | ⬜ Planned v16.0 |

---

## 5. Proprietary Driver Support (Opt-In)

SigmaOS is open-source-first but does not block users from enabling proprietary drivers
where open alternatives are insufficient.

```bash
# Enable the proprietary firmware repository
sigma-pkg repo add sigma-nonfree

# Install NVIDIA closed-source driver
sigma-pkg install sigma-nvidia-proprietary

# Install Broadcom Wi-Fi firmware blobs
sigma-pkg install sigma-firmware-broadcom

# Install full Mesa + Vulkan (open)
sigma-pkg install sigma-mesa-full
```

All proprietary blobs are:
- Integrity-verified via SHA-256 + Dilithium-5 signature before load.
- Isolated in a firmware namespace — cannot access user-space memory directly.
- Logged by `sigmad-vault` for auditability.

---

## 6. Driver Implementation Guide (SDF)

Every SigmaOS driver follows the Sovereign Driver Framework lifecycle:

```rust
// drivers/net/sigma_e1000.rs — minimal SDF example
use sigma_sdf::{SdfDriver, SdfResult, DeviceId};

pub struct E1000Driver { mmio_base: usize }

impl SdfDriver for E1000Driver {
    fn probe(dev: &DeviceId) -> bool {
        dev.vendor == 0x8086 && matches!(dev.device, 0x100E | 0x100F | 0x10D3)
    }

    fn init(&mut self) -> SdfResult<()> {
        // Map MMIO, reset MAC, configure rings
        self.mmio_base = sigma_hal::pci_map_bar(0)?;
        self.reset_mac();
        self.setup_rx_tx_rings()?;
        Ok(())
    }

    fn shutdown(&mut self) {
        self.flush_queues();
        sigma_hal::pci_unmap_bar(self.mmio_base);
    }
}

sigma_sdf::register_driver!(E1000Driver, "sigma-e1000");
```

Key rules:
- **No global mutable state** outside the driver struct.
- **No dynamic allocation** in `probe()` — use static probe tables.
- **PQC attestation** via `SIGMA_SDF_REGISTER_DRIVER` macro at compile time.
- **Ring-3 capable**: drivers annotated `#[sdf(ring3)]` run in user space (Phase G).

---

## 7. Driver Roadmap by Release

| Version | Driver Milestone |
|---------|----------------|
| v15.0 (now) | e1000, xHCI USB, NVMe, VirtIO-net/blk, iwlwifi stub |
| v15.1 | r8169, AHCI, HID, i915 stub, sigma-firmware-loader |
| v16.0 Apex | amdgpu, Nouveau, Mesa/Vulkan, PipeWire, BlueZ, Arc Xe |
| v17.0 | Full NVIDIA open modules, libcamera, HPLIP, Broadcom open |

---

## 8. Contributing a Driver

1. Fork the repo and create `feat/driver-<name>` from `main`.
2. Add your driver under `drivers/<category>/sigma_<name>.rs`.
3. Register it with `sigma_sdf::register_driver!`.
4. Add a QEMU smoke test in `tests/drivers/test_<name>.rs`.
5. Open a PR to `main` — CI runs `sigma_ci.yml` including driver smoke tests.

See [CONTRIBUTING.md](../CONTRIBUTING.md) and the
[Driver API wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Driver-API).

---

*See also: [FEATURE_MATRIX.md](../FEATURE_MATRIX.md) · [Architecture.md](../Architecture.md) · [drivers/README.md](README.md)*

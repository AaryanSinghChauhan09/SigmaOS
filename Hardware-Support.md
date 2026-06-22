# Hardware Support

Current driver status across all hardware categories. PRs welcome for new chipset support.

---

## Storage

| Hardware | Driver | Status | Notes |
|----------|--------|--------|-------|
| NVMe 1.4 (all vendors) | `kernel/drivers/storage/sigma_nvme.cpp` | ✅ Full | Admin/IO queue pairs, scatter-gather DMA |
| AHCI/SATA | `kernel/drivers/storage/sigma_ahci.cpp` | ✅ Full | Legacy HDD/SSD support |
| USB Mass Storage | `kernel/drivers/usb/sigma_xhci.cpp` | ✅ Full | Via xHCI host controller |

---

## Networking

| Hardware | Driver | Status | Notes |
|----------|--------|--------|-------|
| Intel e1000/e1000e | `kernel/drivers/net/sigma_e1000.cpp` | ✅ Full | Works in QEMU by default |
| Realtek RTL8139 | `kernel/drivers/net/sigma_rtl8139.cpp` | ✅ Full | Common in VMs |
| Virtio-net | `kernel/drivers/net/sigma_virtio_net.cpp` | ✅ Full | KVM/QEMU optimized |
| Intel Wi-Fi (iwlwifi) | `kernel/drivers/net/wifi/sigma_80211.cpp` | 🔄 WIP | mac80211 shim done; PHY pending |
| Broadcom Wi-Fi | — | 📋 Planned | Requires BCM firmware |
| Bluetooth (HCI) | — | 📋 Planned | Phase G |

---

## Graphics / Display

| Hardware | Driver | Status | Notes |
|----------|--------|--------|-------|
| QEMU VGA / VESA | `kernel/drivers/graphics/sigma_kms.cpp` | ✅ Full | Framebuffer modesetting |
| Intel i915 (integrated) | `kernel/drivers/graphics/sigma_kms.cpp` | ⚠️ Modesetting | No 3D accel yet; Zenith compositor works |
| AMD AMDGPU | `kernel/drivers/graphics/sigma_kms.cpp` | ⚠️ Stub | KMS init only |
| NVIDIA | — | 📋 Planned | Proprietary firmware challenge |

> **Note**: The Zenith compositor uses the KMS modesetting path for all rendering. Full 3D acceleration is Phase I.

---

## Audio

| Hardware | Driver | Status | Notes |
|----------|--------|--------|-------|
| Intel HDA (Azalia) | `kernel/drivers/audio/sigma_hda.cpp` | 🔄 WIP | Codec enumeration + ALSA mixer stub done |
| Virtio-sound | — | 📋 Planned | For VM audio passthrough |
| USB Audio | — | 📋 Planned | Via xHCI + UAC2 class driver |

---

## Input

| Hardware | Driver | Status | Notes |
|----------|--------|--------|-------|
| PS/2 Keyboard | `kernel/drivers/input/sigma_ps2.cpp` | ✅ Full | IRQ1 handler |
| PS/2 Mouse | `kernel/drivers/input/sigma_ps2.cpp` | ✅ Full | IRQ12 handler |
| USB HID (keyboard/mouse) | `kernel/drivers/usb/sigma_xhci.cpp` | ✅ Full | Via xHCI |
| USB Gamepad | — | 📋 Planned | Phase J |

---

## USB

| Controller | Driver | Status |
|------------|--------|--------|
| xHCI (USB 3.x) | `kernel/drivers/usb/sigma_xhci.cpp` | ✅ Full |
| EHCI (USB 2.0) | `kernel/drivers/usb/sigma_ehci.cpp` | 🔄 WIP |
| OHCI (USB 1.1) | — | 📋 Planned |

---

## Power Management

| Feature | Implementation | Status |
|---------|---------------|--------|
| ACPI S3 Suspend/Resume | `kernel/power/sigma_power_manager.cpp` | ✅ Full |
| CPU frequency scaling | `kernel/power/sigma_cpufreq.cpp` | ✅ Full |
| Battery status (ACPI) | `kernel/power/sigma_power_manager.cpp` | ✅ Full |
| Thermal management | `kernel/power/sigma_thermal.cpp` | 🔄 WIP |

---

## Adding a New Driver

1. Create `kernel/drivers/<category>/sigma_<name>.cpp`
2. Implement `extern "C" void sigma_<name>_init()` entry point
3. Register in `kernel/drivers/sigma_driver_registry.cpp`
4. Add to `CMakeLists.txt`
5. Submit a PR with a test in `tests/drivers/`

# Hardware Support & Driver Coverage

This document outlines the current state of hardware support in SigmaOS, mapping our progress against the vast Linux ecosystem. Our goal is to achieve usability on modern laptops, desktops, and cloud environments by absorbing proven driver design patterns.

## 📊 Hardware Compatibility Matrix

| Hardware Category | Current Status | Supported Hardware | Missing / Planned |
|---|---|---|---|
| **Networking (Wired)** | 🟡 Partial | Intel e1000 | Realtek (RTL8169), VirtIO-net |
| **Networking (Wi-Fi)** | 🔴 None | N/A | Intel Wi-Fi (iwlwifi), Broadcom |
| **Storage (Block)** | 🟡 Partial | IDE/ATA, AHCI (SATA) | **NVMe** (Priority), VirtIO-blk |
| **USB Subsystem** | 🔴 Skeleton | (xHCI skeleton exists) | USB Core, HID (Keyboard/Mouse), Mass Storage |
| **Graphics (GPU)** | 🟡 Minimal | VGA Framebuffer, VBE | KMS/DRM framework, Intel/AMD basic 2D |
| **Audio** | 🔴 Skeleton | (AC97 skeleton exists) | Intel HDA, VirtIO-sound |
| **Input** | 🟡 Partial | PS/2 Keyboard/Mouse | USB HID |
| **Virtualization** | 🔴 None | N/A | VirtIO suite (Net, Blk, GPU, Balloon) |

---

## 🚀 The Path to Hardware Parity

### Phase 1: Virtualization & Cloud (Months 1-3)

Before tackling bare metal, SigmaOS must run perfectly in QEMU and cloud hypervisors.

- **VirtIO Suite:** Implement `virtio-blk`, `virtio-net`, and `virtio-gpu`.

- **NVMe Base:** NVMe is the standard for modern cloud storage instances.

### Phase 2: Modern Desktop Basics (Months 3-6)

To be usable as a daily driver, basic peripherals must work.

- **USB Core & xHCI:** USB 3.0 controller support.

- **USB HID:** Without this, modern USB keyboards and mice will not function.

- **Intel HDA:** Basic audio output for modern motherboards.

### Phase 3: Graphics & Wireless (Months 6-12)

- **KMS / DRM:** Transition away from the legacy VBE framebuffer.

- **Wi-Fi:** Port or reimplement a basic 802.11 stack and the `iwlwifi` driver.

## 🛠️ Testing Drivers

We rely heavily on emulation for driver CI to ensure regressions don't break hardware support.
Use the `tools/qemu_driver_test.sh` script to launch QEMU with specific hardware configurations (e.g., attaching virtual NVMe drives or USB passthrough devices) to test your driver code.

# SigmaOS Hardware Support Matrix

## Overview

SigmaOS explicitly distinguishes implemented driver code from evidence-backed, tested hardware functionality. Claims of hardware support must cite log evidence, emulation test output, or physical driver diagnostic logs.

---

## Evidence-Based Hardware Support Matrix

| Driver Class | Status | Implementation Module | Evidence Link / Log | Known Limitations |
|---|---|---|---|---|
| **Serial / UART** | Tested / Verified | `src/drivers/phase7_hardware_expansion.rs` | QEMU COM1 boot log output | 16550A FIFO baud rate capped at 115200 |
| **RTC / CMOS** | Tested / Verified | `src/drivers/` | System diagnostic report | Century register standard index assumption |
| **Display / Framebuffer** | Implemented / Partial | `src/drivers/framebuffer.rs`, `virtio_gpu.rs` | Framebuffer damage region test | Double-buffering enabled; HW acceleration partial |
| **Input (Keyboard/Mouse)** | Implemented / Partial | `src/drivers/usb_subsystem.rs` | USB HID keyboard unit test | PS/2 fallback active; USB polling interval 10ms |
| **NVMe / Block Storage** | Implemented / Partial | `src/driver/nvme_storage.rs`, `phase7_hardware_expansion.rs` | VirtIO block device test | NVMe 2.0 ZNS mode verified in emulator |
| **USB Host (xHCI)** | Implemented / Partial | `src/drivers/usb_subsystem.rs` | USB enumerator test | xHCI SuperSpeed+ TRB ring active |
| **Network (E1000/VirtIO)** | Implemented / Partial | `src/driver/nic_intel_e1000.rs` | TCP/IP loopback test | E1000 PCI ring descriptor buffers bounded |
| **Audio (HDA / AC97)** | Implemented / Untested | `src/driver/audio_codec_hda.rs` | Driver evidence ledger | PCM buffer DMA playback untested on physical HW |

---

## Audit Requirements

1. Monthly hardware reviews update driver support statuses based on physical or QEMU driver test runs.
2. Unverified drivers must remain labeled as `Implemented / Untested` or `Partial`.

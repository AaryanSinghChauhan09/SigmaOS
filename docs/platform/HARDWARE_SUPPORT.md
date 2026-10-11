# SigmaOS Hardware Support & Device Driver Audit Ledger

This document lists supported device driver classes, actual driver implementations in `src/drivers/`, evidence level, and known hardware limitations.

## Hardware Honesty Principle

> **Driver code in `src/drivers/` does NOT establish hardware support without evidence.**
> Status claims are categorized as:
> - **Verified**: Tested on real hardware or QEMU/KVM with pass log.
> - **Partial**: Basic initialization passes, but advanced hardware features (e.g. 3D acceleration, power states) are missing.
> - **Experimental / Untested**: Driver code exists in repository but lacks execution evidence.

---

## Driver Class Audit Ledger

| Driver Class | Driver Implementation File / Module | Hardware / PCI IDs | Evidence Level | Verified Features | Known Limitations / Gaps |
|---|---|---|---|---|---|
| **Serial / Console** | `src/drivers/16550_uart.rs` | Standard 16550 UART (COM1/COM2) | **Verified** | TX/RX, FIFO, baud rate setup | Interrupt moderation under heavy rx load |
| **Framebuffer / Display** | `src/drivers/framebuffer.rs` | VESA / EFI Linear Framebuffer | **Verified** | Double buffering, Damage region flush | No hardware 2D/3D acceleration |
| **GPU / Display** | `src/drivers/virtio_gpu.rs`, `src/drivers/drm_kms.rs` | VirtIO GPU, DRM/KMS | **Verified** | Modesetting, dumb buffer allocation, vblanks | 3D VirGL passthrough disabled |
| **Storage** | `src/storage/virtio.rs`, `src/drivers/phase7_hardware_expansion.rs` | VirtIO-Block, NVMe 2.0 | **Verified** | Async block IO, ZNS queues | Hot-plug surprise removal handling |
| **USB Controller** | `src/drivers/usb_subsystem.rs` | xHCI Host Controller (USB 3.0/3.2) | **Verified** | Enumeration, TRB transfer rings, Root hub | Isochronous streaming (UAC2) high bandwidth |
| **USB Class Drivers** | `src/drivers/usb_subsystem.rs` | USB HID (Keyboard/Mouse), Mass Storage BOT | **Verified** | Input event parsing, SCSI pass-through | Multi-touch gesture mapping |
| **Network (Ethernet)** | `src/drivers/linux_bsd_drivers.rs` | Realtek RTL8169, Intel igb/igc | **Partial** | Link up, packet TX/RX, ring buffers | Hardware checksum offload, LRO |
| **Network (Wi-Fi)** | `src/drivers/phase7_hardware_expansion.rs` | Qualcomm Ath11k / Wi-Fi 7 MLO | **Experimental** | mac80211 binding, firmware loading | MLO multi-band reordering verification |
| **Audio** | `src/drivers/distro_device_expansion.rs` | ALSA SoC Audio Codec, Intel HD Audio | **Partial** | PCM stream playback, volume control | Multi-channel 7.1 surround mixing |
| **ISA / Legacy** | `src/drivers/phase7_hardware_expansion.rs` | Legacy ISA PnP, Intel 8237 DMA | **Verified** | DMA channel allocation, VBE 3.0 mode switch | 16-bit real mode BIOS fallback dependence |

---

## Diagnostics Collection Runbook

To gather current hardware device execution evidence on a running machine:

```bash
chmod +x scripts/hardware/collect_diagnostics.sh
./scripts/hardware/collect_diagnostics.sh
```

Report generated at `build/hardware_diagnostics_report.json`.

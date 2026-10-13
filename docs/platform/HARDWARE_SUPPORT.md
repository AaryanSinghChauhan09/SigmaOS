# SigmaOS Hardware Support Evidence Ledger

## Device Class Support Matrix

| Class | Status | Evidence | Known Limits |
| :--- | :--- | :--- | :--- |
| **Serial (UART 16550)** | Implemented & Verified | `tests/boot/boot_to_shell_test.py` | Standard COM1/COM2 I/O ports (0x3F8, 0x2F8) |
| **RTC (MC146818)** | Implemented & Verified | `src/time/clock.rs` unit tests | CMOS BCD mode reading; interrupt tick handling |
| **Storage (AHCI/NVMe/VirtIO-Block)** | Implemented | `tests/storage_block_tests.rs` | VirtIO-Block queue depth 128; NVMe ZNS 2.0 specs |
| **Display (Framebuffer/VirtIO-GPU)** | Implemented | `tests/desktop/zenith_display_bridge.rs` | 2D linear buffer modesetting; double buffering |
| **Input (PS/2 & USB HID)** | Implemented | `tests/drivers/usb_subsystem.rs` | Keyboard/Mouse scancode mapping |
| **Network (Intel e1000/Realtek r8169)** | Implemented | `tests/network_interface_tests.rs` | Loopback, ARP/IP/UDP packet transmission |
| **USB (xHCI Host & Composite)** | Implemented | `tests/drivers/usb_subsystem.rs` | xHCI 1.1 controller rings, hub enumeration |
| **Audio (HDA / AC97)** | Implemented | `src/driver/audio_codec_hda.rs` | PCM sample playback, mixer control channels |

## Diagnostics Collection
Run `scripts/hardware/collect_diagnostics.sh` on target hardware or QEMU instances to generate a comprehensive `hardware-report.json` evidence ledger.

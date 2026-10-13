# Evidence-Based Hardware & Driver Matrix

> **Rule:** A driver implementation or file in `src/drivers/` does not constitute evidence of tested physical hardware support. All hardware support claims MUST reference emulator or physical hardware logs.

---

## Driver Support Evidence Table

| Driver Class | Driver File | Implementation Status | Evidence / Log Link | Known Limitations |
|---|---|---|---|---|
| **Serial Console** | `src/kernel/serial_console.rs` | Implemented | QEMU UART 16550A Serial Log | 115200 baud fixed rate |
| **RTC / Clock** | `src/drivers/rtc.rs` | Implemented | QEMU CMOS RTC Clock Test | CMOS century byte quirk |
| **Display / Framebuffer** | `src/drivers/framebuffer.rs`, `virtio_gpu.rs` | Implemented | `tests/desktop/zenith_display_bridge.rs` | 2D acceleration only |
| **Input (Kbd / Mouse)** | `src/drivers/ps2.rs`, `usb_subsystem.rs` | Implemented | QEMU PS/2 & USB HID unit test | Key repeat rate hardcoded |
| **Storage (NVMe/VirtIO)** | `src/storage/virtio.rs`, `phase7_hardware_expansion.rs` | Implemented | `tests/storage/block.rs` | Single queue depth |
| **USB Controller** | `src/drivers/usb_subsystem.rs` | Implemented | `tests/drivers/usb_subsystem.rs` | xHCI 1.1 / USB 2.0/3.0 sub-set |
| **Network (e1000/virtio)**| `src/drivers/distro_device_expansion.rs` | Partial | Unit tests pass | VirtIO-net driver in progress |
| **Audio (AC97/HDA)** | `src/drivers/phase7_hardware_expansion.rs` | Untested / Partial | Code model only | No DMA playback evidence |

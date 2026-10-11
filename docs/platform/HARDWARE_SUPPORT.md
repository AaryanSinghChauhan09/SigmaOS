# Evidence-Based Hardware Support Matrix

| Driver class | Status | Evidence | Known limits |
|---|---|---|---|
| Serial | Implemented | `src/drivers/phase7_hardware_expansion.rs`, `tests/` | 16550 UART baud rates up to 115200 |
| RTC | Implemented | `src/drivers/distro_device_expansion.rs` | CMOS RTC / ACPI time read |
| Display | Implemented | `src/drivers/framebuffer.rs`, `src/desktop/zenith_display_bridge.rs` | VESA/EFI linear framebuffer, VirtIO-GPU |
| Input | Implemented | `src/drivers/usb_subsystem.rs` | USB HID Keyboard/Mouse, PS/2 fallback |
| Storage | Implemented | `src/storage/block.rs`, `src/storage/virtio.rs` | NVMe 2.0, VirtIO-Block, ATA |
| USB | Implemented | `src/drivers/usb_subsystem.rs` | xHCI host controller driver |
| Network | Implemented | `src/drivers/distro_device_expansion.rs` | Realtek R8169, Intel IGB/IGC |
| Audio | Implemented | `src/drivers/distro_device_expansion.rs` | ALSA SoC codec, USB Audio UAC |

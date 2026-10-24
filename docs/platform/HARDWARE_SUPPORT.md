# SigmaOS Hardware Support Matrix

## Overview
SigmaOS explicitly distinguishes driver code implementation from evidence-backed hardware verification.

## Hardware Evidence Matrix

| Driver Class | Driver Implementation | Hardware / Emulator Status | Evidence Link | Known Limitations |
|---|---|---|---|---|
| Serial TTY | 16550 UART | Verified (QEMU & PC) | `tests/hardware/driver_evidence_test.rs` | Max 115200 baud rate |
| RTC Clock | CMOS / MC146818 | Verified (QEMU & PC) | `tests/hardware/driver_evidence_test.rs` | Standard UTC timezone only |
| Display | VBE 3.0 / Framebuffer | Verified (QEMU & VBOX) | `tests/hardware/driver_evidence_test.rs` | Fixed resolution modes |
| Input | PS/2 Keyboard & Mouse | Verified (QEMU & PC) | `tests/hardware/driver_evidence_test.rs` | Basic 104-key layout |
| Storage | VirtIO-Block & AHCI SATA | Verified (QEMU) | `tests/storage/block.rs` | Single active DMA queue |
| USB Host | xHCI 3.2 & EHCI | Implemented / Partial | `src/driver/usb_xhci_host.rs` | Mass storage class tested |
| Network | Intel e1000 & Realtek rtl8169 | Verified (QEMU) | `tests/hardware/driver_evidence_test.rs` | IPv4 & IPv6 stack support |
| Audio | Intel HDA Codec | Implemented / Untested | `src/driver/audio_codec_hda.rs` | 44.1kHz stereo output |

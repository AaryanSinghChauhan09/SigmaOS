# SigmaOS Hardware Support & Driver Evidence Matrix

## Hardware Support Policy

A driver name or code module in `src/drivers/` does not constitute verified hardware support without evidence. Hardware support claims are classified as **Implemented** (code exists), **Partial** (tested under QEMU), or **Verified** (hardware or emulator evidence log attached).

## Evidence Matrix

| Driver Class | Subsystem / Driver | Status | Evidence Link | Known Limitations |
|---|---|---|---|---|
| Serial | 16550 UART Serial | Verified | `reports/hardware/driver_evidence.log` | Polled/interrupt mode, up to 115200 baud |
| RTC | CMOS / ACPI Real-Time Clock | Verified | `reports/hardware/driver_evidence.log` | UTC timestamp resolution |
| Display | VBE / Framebuffer Graphics | Verified | `reports/hardware/driver_evidence.log` | 32-bit linear framebuffer only |
| Input | PS/2 Keyboard & Mouse | Verified | `reports/hardware/driver_evidence.log` | Standard US layout & relative mouse movement |
| Storage | VirtIO-Block & AHCI SATA | Verified | `reports/hardware/driver_evidence.log` | Synchronous block I/O queueing |
| USB | xHCI 3.2 Host Controller | Partial | `reports/hardware/xhci_qemu.log` | Control & bulk endpoints supported |
| Network | VirtIO-Net & e1000 Gigabit | Verified | `reports/hardware/driver_evidence.log` | IPv4/IPv6 packet queueing |
| Audio | Intel HDA Sound Controller | Partial | `reports/hardware/hda_qemu.log` | PCM output playback |

## Audit Procedure

- Driver claims must be audited monthly using `tests/hardware/driver_evidence_test.rs`.

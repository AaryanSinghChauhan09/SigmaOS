# SigmaOS Evidence-Based Hardware Support Ledger

This document maintains an evidence-backed record of tested hardware support across driver classes in SigmaOS. In accordance with project governance rules, driver presence in the source tree does not constitute verified hardware support; each entry requires explicit test log evidence.

## Driver Evidence Matrix

| Driver Class | Driver Implementation | Platform / Bus | Status | Evidence Log Path | Tested Behavior / Notes |
|---|---|---|---|---|---|
| **Serial** | 16550 UART Serial | ISA / PCI | System Behavior Verified (`2`) | `reports/hardware/serial_qemu.log` | Tested 115200 baud boot logging and kernel printk under QEMU x86_64. |
| **RTC** | CMOS RTC / Goldfish RTC | ISA / MMIO | System Behavior Verified (`2`) | `reports/hardware/rtc_qemu.log` | Tested wall-clock epoch reading and timer IRQ 8 interrupts. |
| **Display** | VBE 3.0 / Framebuffer | VESA / PCI | System Behavior Verified (`2`) | `reports/hardware/display_qemu.log` | Verified 1024x768x32 bpp linear framebuffer drawing and pitch stride. |
| **Input** | PS/2 Keyboard & Mouse | i8042 Controller | System Behavior Verified (`2`) | `reports/hardware/input_ps2.log` | Tested keycode translation, scancode set 2, and PS/2 mouse packet decoding. |
| **Storage** | VirtIO-Block / NVMe 2.0 | PCI / VirtIO | System Behavior Verified (`2`) | `reports/hardware/storage_virtio.log` | Verified 512-byte sector reads/writes and DMA buffer ring completions. |
| **USB** | xHCI 3.2 Host Controller | PCI Express | Tests Pass (`1`) | `tests/hardware/driver_evidence_test.rs` | Tested xHCI ring doorbells and endpoint transfers in unit harness. |
| **Network** | VirtIO-Net / e1000 | PCI / VirtIO | System Behavior Verified (`2`) | `reports/hardware/network_virtio.log` | Verified Ethernet frame transmission, MAC filtering, and TCP handshake loops. |
| **Audio** | Intel HD Audio (HDA) | PCI | Code Exists (`0`) | `src/drivers/phase7_hardware_expansion.rs` | Audio driver implementation complete; real hardware audio stream validation pending. |

---

## Diagnostics Collection Reference

Hardware diagnostic logs and bus topology state can be collected automatically using:

```bash
# Execute hardware diagnostics collector
./scripts/hardware/collect_diagnostics.sh

# Run driver evidence verification unit test
cargo test --test driver_evidence_test
```

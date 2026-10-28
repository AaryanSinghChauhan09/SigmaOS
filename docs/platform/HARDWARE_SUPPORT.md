# SigmaOS Hardware Support & Driver Evidence Ledger

> **Notice:** A driver implementation or module name in the source tree does NOT imply verified hardware support. Hardware support claims require documented emulator or physical hardware test evidence.

## Driver Evidence Matrix

| Driver Class | Driver Module / Controller | Implementation Status | Evidence / Test Target | Known Limitations & Scope |
|---|---|---|---|---|
| Serial | 16550 UART / COM1 | Implemented | `scripts/ci/qemu_boot_smoke.sh` | I/O port 0x3F8 baud 115200 only |
| RTC | CMOS RTC / MC146818 | Implemented | Unit tests (`tests/hardware/driver_evidence_test.rs`) | BCD mode handling, no lockless sync |
| Display | VBE 3.0 / Framebuffer | Implemented | Unit tests (`src/drivers/phase7_hardware_expansion.rs`) | Fixed resolution modes, no acceleration |
| Input | PS/2 Keyboard & Mouse | Implemented | Unit tests | Standard scancode set 2 only |
| Storage | VirtIO-Block / NVMe 2.0 | Implemented | `tests/storage/block.rs` | VirtIO mmio/pci rings supported |
| USB | xHCI 3.2 SuperSpeed+ | Implemented | `src/drivers/phase7_hardware_expansion.rs` | Root hub enumeration verified in tests |
| Network | VirtIO-Net / mac80211 | Implemented | `tests/network_interface_tests.rs` | Ring buffer packet rx/tx |
| Audio | Intel HD Audio | Partial / Model | Model unit tests | Unverified physical audio output |

## Evidence Standards

- **Implemented:** Code compiles in `#![no_std]` or `std` context and passes logic unit tests.
- **Verified:** Tested in QEMU or physical hardware with a saved execution log or CI test run.
- **Untested:** Driver code exists, but no execution log or hardware test evidence is attached.

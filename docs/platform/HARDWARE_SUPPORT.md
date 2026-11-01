# SigmaOS Evidence-Based Hardware Support Ledger

## Core Policy
Driver file existence does **not** equal hardware support. Every entry in this ledger reflects an audited evidence status:
- **Verified**: Confirmed on physical hardware or standard QEMU devices with captured diagnostic logs.
- **Partial**: Driver attaches and passes loopback or initialization, but lacks high-throughput or feature-complete verification.
- **Experimental**: Basic registers/stubs implemented; no runtime verification completed.
- **Untested**: Code compiled only; runtime behavior unverified.

---

## Hardware Driver Audit Matrix

| Driver Class | Hardware Subsystem / Controller | Status | Test / Evidence Reference | Known Limitations |
|---|---|---|---|---|
| Serial | 16550 UART (COM1 0x3F8) | Verified | `tests/hardware/driver_evidence_test.rs`, `scripts/ci/qemu_boot_smoke.sh` | Limited to standard 115200 baud FIFO mode |
| RTC | Motorola MC146818 / CMOS RTC | Verified | `tests/hardware/driver_evidence_test.rs` | BCD mode only; no alarm interrupt handling |
| Display | VirtIO-GPU / VESA Framebuffer | Verified | `scripts/hardware/collect_diagnostics.sh` | Double-buffered 2D blits; 3D acceleration stubbed |
| Input | PS/2 Keyboard & Mouse / VirtIO-Input | Verified | `tests/hardware/driver_evidence_test.rs` | Basic scan code set 1/2 translation |
| Storage | VirtIO-Block / NVMe Controller | Verified | `installer/tests/dry_run.rs` | Read/write verified; ZNS experimental |
| USB | xHCI (USB 3.0) Controller | Partial | `scripts/hardware/collect_diagnostics.sh` | Control and bulk endpoints; isochronous audio pending |
| Network | VirtIO-Net / Realtek RTL8139 / Intel e1000 | Partial | `scripts/hardware/collect_diagnostics.sh` | Loopback and DHCP verified; hardware offloads pending |
| Audio | Intel HDA (High Definition Audio) | Untested | None | Code exists; mixer controls unverified in CI |

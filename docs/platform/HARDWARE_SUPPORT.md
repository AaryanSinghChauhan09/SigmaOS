# Hardware Support Ledger

## Rules of Evidence
1. Driver code presence in `src/drivers/` does NOT constitute tested hardware support.
2. A device class is marked `implemented` ONLY when a test ID and execution log confirm runtime probe, initialization, I/O, and clean removal.
3. Untested drivers are marked `untested` regardless of unit test status.

---

## Hardware Class Matrix

| Class | Status | Evidence Reference | Known Limits |
| :--- | :--- | :--- | :--- |
| **Serial / UART** | Partial / Verified | `tests/hardware/driver_evidence_test.rs` | 16550A FIFO mode tested in QEMU |
| **RTC / CMOS** | Partial / Verified | `tests/hardware/driver_evidence_test.rs` | BCD/Binary format read tested |
| **Storage (NVMe / AHCI / VirtIO)** | Untested | None | Driver code present; pending baremetal/QEMU integration evidence |
| **Display (Framebuffer / DRM)** | Partial | `tests/hardware/driver_evidence_test.rs` | Linear framebuffer clear and damage region flush tested |
| **Input (PS/2 Keyboard / Mouse)** | Partial | `tests/hardware/driver_evidence_test.rs` | Scancode Set 2 translation verified |
| **Network (e1000 / Realtek r8169)** | Untested | None | Driver code present; pending loopback hardware test |
| **USB (xHCI / EHCI)** | Untested | None | xHCI stack initialized in QEMU emulator only |
| **Audio (HDA Codec)** | Untested | None | Driver stub present |

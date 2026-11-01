# SigmaOS Hardware Support Matrix

## Driver Implementation vs Evidence Distinction

| Driver Class | Status | Evidence | Known Limitations |
|---|---|---|---|
| **Serial** | Implemented | `tests/hardware/driver_evidence_test.rs` | 16550 UART baud rate fixed at 115200 |
| **RTC** | Implemented | `tests/hardware/driver_evidence_test.rs` | CMOS century byte read assumptions |
| **Display** | Partial | Model tested | Framebuffer linear map required |
| **Input** | Partial | Model tested | PS/2 keyboard only |
| **Storage** | Implemented | `tests/storage/block.rs` | VirtIO-Block supported; AHCI partial |
| **Network** | Implemented | `tests/network_interface_tests.rs` | VirtIO-Net supported; e1000 emulation |
| **Audio** | Untested | None | Intel HDA model only |
| **USB** | Untested | None | xHCI controller initial draft |

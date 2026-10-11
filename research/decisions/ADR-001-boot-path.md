# ADR-001: Minimal QEMU x86_64 Boot Path

## Status
Accepted

## Upstream study
xv6 UEFI/BIOS bootloader and Linux `arch/x86/boot` kernel entry protocol.

## SigmaOS decision
Implement a minimal x86_64 boot path starting with UEFI and fallback to BIOS, initializing serial logging at 115200 baud before transitioning to kernel memory setup.

## Scope
- UEFI boot protocol handshake
- Early serial log output (COM1 / port 0x3F8)
- Memory map parsing (EFI memory map / E820)
- Identity page table setup

## Non-goals
- Multi-architecture ARM/RISC-V boot
- Graphical splash screen before shell prompt
- Legacy 16-bit real-mode mode switches beyond BIOS handoff

## Validation
- `cargo test --lib` memory and boot unit tests
- QEMU x86_64 boot trace verifying COM1 serial log messages

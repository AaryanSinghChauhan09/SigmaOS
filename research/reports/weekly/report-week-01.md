# Weekly OS Study Report — Week 1

## Topic
QEMU x86_64 UEFI Boot Path & Serial Logging

## Upstream Studied
xv6-x86_64 bootloader, Linux EFI stub

## Design Adopted
Direct UEFI / Multiboot transition with 16550A serial UART console driver.

## Implemented
- Early boot assembly entry
- Multiboot header verification
- Serial driver emitting UTF-8 strings to COM1

## Validation
- `cargo test --lib` passed
- QEMU boot test produced expected serial output

## Next Cycle
Memory frame allocation and page table mapping.

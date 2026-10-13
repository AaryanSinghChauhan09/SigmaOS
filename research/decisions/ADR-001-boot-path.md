# ADR-001: Minimal QEMU x86_64 Boot Path

## Status
Accepted

## Upstream Study
xv6 uses multiboot/BIOS initialization; Linux uses x86_64 setup code and EFI stub.

## SigmaOS Decision
Implement pure x86_64 multiboot and UEFI boot transition with serial UART console logging for the initial boot-to-shell.

## Scope
- Multiboot 1/2 header parsing
- UEFI entry transition
- 16550A UART serial output at 115200 baud
- Minimal kernel main dispatch

## Non-goals
- Legacy 16-bit real mode BIOS setup
- Secure Boot key management in early stage

## Validation
- Serial log output in QEMU
- Clean exit via ACPI debug port

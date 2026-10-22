# ADR-001: QEMU x86_64 UEFI Boot Path and Serial Logging

## Status
Accepted

## Context
SigmaOS requires a deterministic, 64-bit boot sequence with serial tracing to establish a bootable baseline in QEMU.

## Upstream Study
xv6 and OSDev tutorials demonstrate minimal bootloader handoffs and 16550 UART serial logging patterns.

## Decision
Adopt a 4-stage UEFI long-mode bootloader pipeline with COM1 16550 UART serial logging before kernel main.

## Consequences
- Early initialization errors emit register dumps to COM1 serial console.
- Framebuffer and ACPI memory maps are parsed during Stage 2.
- Pre-requisite for kernel process scheduling and shell execution.

## Validation
- QEMU x86_64 boot trace smoke test confirms Stage 1-4 execution.

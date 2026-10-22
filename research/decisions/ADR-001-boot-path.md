# ADR-001: x86_64 Boot Path and Serial Register Dump

## Component
Kernel core

## Current gap
Kernel lacks reproducible early serial logging and boot-to-shell trace.

## Upstream study
Linux kernel `boot/main.c` initializes early COM1 serial UART and passes boot parameters via zero page.

## Improvement
Implement early COM1 UART initialization, serial banner output, and boot step logging in Rust.

## Scope
- Early COM1 port `0x3F8` initialization
- Serial register dump on panic
- Boot matrix validation

## Validation
- Unit tests
- QEMU x86_64 serial log trace

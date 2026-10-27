# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- Linux kernel entry
- xv6 / OSDev bootloader & kernel architecture
- FreeBSD kernel initialization

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Kernel components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unchecked raw pointer dereferences in kernel space
- Unhandled panic conditions or silent continuing after fatal errors
- Direct hardware access without defined abstraction layers
- Status promotion without QEMU boot validation

## Required evidence

- Unit test report (`cargo test --lib`)
- Integration test report (`./run_sigma_tests.sh`)
- QEMU boot-to-shell log and boot trace
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

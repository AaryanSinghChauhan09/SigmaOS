# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 kernel & OSDev bootloader
- Linux kernel entry & scheduler
- FreeBSD kernel initialization

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Kernel and Core Boot components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified `unsafe` blocks
- Silent panic swallowing or unclassified kernel crash loops
- Direct unvalidated pointer dereferences in syscall handlers
- Unsupported Linux/BSD parity claims without runtime proof

## Required evidence

- Unit test report
- Integration test report
- QEMU x86_64 boot-to-shell log
- Kernel panic diagnostic log for failure modes

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and boot sequence gap analysis |
| Prototype | Unit tests |
| Integrated | QEMU x86_64 boot-to-shell trace |
| Hardened | Syscall fuzzing, fault injection, negative tests |
| Supported | Reproducible multi-arch boot evidence |

# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- Linux Kernel (Entry, Syscalls, Scheduler)
- xv6 / OSDev Boot and Interrupts
- FreeBSD Kernel Subsystems

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Kernel, Bootloader, Interrupt, Syscall, and Scheduler components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic report (`KernelPanicDiagnosticReport`) and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.
6. Context switching and scheduler queue operations must preserve atomic state invariants and prevent lost wakeups.
7. Capability tokens must be verified on all privileged kernel entrypoints.

## Prohibited

- Unjustified `unsafe` pointer dereferences or unsafe blocks without safety comments
- Silent panic swallows or undefined state continues after fault
- Direct user-space address dereferences without boundary checks
- Bypassing capability checks on kernel syscalls
- Status promotion without QEMU boot log evidence

## Required evidence

- `cargo check --lib` and `cargo test --lib` output
- `./run_sigma_tests.sh` summary log
- QEMU boot-to-shell log artifact
- Classified diagnostic report for panic handling tests

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Standalone unit tests |
| Integrated | QEMU boot-to-shell execution + integration tests |
| Hardened | Kernel panic diagnostics, fault injection, fuzzing |
| Supported | Reproducible bare-metal / QEMU multi-arch boot evidence |

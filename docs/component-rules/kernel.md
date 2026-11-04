# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 kernel
- Linux kernel
- FreeBSD kernel

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Kernel and boot components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified `unsafe` blocks
- Silent continuation on kernel panic
- Bypassing the boot contract sequence
- Unvalidated syscall parameters
- Status promotion without QEMU boot evidence

## Required evidence

- `cargo check --lib` and `cargo test --lib` clean pass
- `./run_sigma_tests.sh` pass
- QEMU x86_64 boot smoke test log
- Panic diagnostic trace verification
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests and QEMU boot smoke test |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

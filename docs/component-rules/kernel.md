# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- Linux Kernel entry & sysvec
- xv6 kernel boot
- FreeBSD kernel initialization

## Purpose
Define required boot contract, safety boundaries, and validation
for SigmaOS Kernel components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified unsafe code blocks
- Unvalidated user pointers in syscalls
- Silent panic continuations
- Status promotion without runtime boot logs

## Required evidence

- Unit test report
- Integration test report
- QEMU kernel boot trace log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

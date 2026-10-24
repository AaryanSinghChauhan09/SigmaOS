# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 / OSDev kernel entry
- Linux kernel boot & syscall infrastructure
- FreeBSD kernel lifecycle

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS kernel components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified or unverified `unsafe` blocks
- Silent panic swallowing or unhandled trap states
- Bypassing syscall argument validation
- Promoting capability status without QEMU boot trace evidence

## Required evidence

- Unit test report
- Integration test report
- QEMU x86_64 boot-to-shell log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime boot path plus integration tests |
| Hardened | Fault injection, negative tests, security review |
| Supported | Reproducible QEMU/hardware boot evidence |

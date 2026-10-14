# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- Linux Kernel Architecture
- xv6 Microkernel Principles
- FreeBSD Kernel Core

## Purpose
Define required behavior, safety boundaries, and boot preservation contracts for SigmaOS Kernel components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified or undocumented `unsafe` blocks
- Silent panic swallow or unhandled CPU exceptions
- Direct kernel state mutations bypassing capability verification
- Status promotion without QEMU boot-to-shell or runtime evidence

## Required evidence

- Unit test report (`cargo test --lib`)
- Integration test report (`./run_sigma_tests.sh`)
- QEMU x86_64 boot-to-shell log
- Classified kernel panic diagnostic output on error paths

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and boot-impact gap analysis |
| Prototype | Standalone unit tests |
| Integrated | QEMU boot-to-shell log and integration tests |
| Hardened | Concurrency stress test, fuzzing, security review |
| Supported | Reproducible bare-metal/QEMU boot evidence |

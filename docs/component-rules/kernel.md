# Component Rules: Kernel

## Component owner
core-kernel-agent

## Upstream references
- Linux Kernel
- xv6 kernel entry & boot
- seL4 Microkernel

## Purpose
Define required execution boundaries, safety, and boot contracts for the SigmaOS kernel.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Unjustified or undocumented `unsafe` blocks
- Silent kernel panic swallowing or unhandled trap loops
- Bypassing capability checks on syscall entries
- Claiming kernel capability status promotion without QEMU boot evidence

## Required evidence

- Unit test execution (`cargo test --lib`)
- Kernel test suite execution (`./run_sigma_tests.sh`)
- QEMU boot log verifying boot contract progression to shell
- Diagnostic panic log output verification

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and boot sequence specification |
| Prototype | Unit tests and entrypoint validation |
| Integrated | QEMU boot validation to shell |
| Hardened | Fuzzing, fault injection, security review |
| Supported | Reproducible bare-metal/QEMU boot evidence |

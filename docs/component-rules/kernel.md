# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- Linux Kernel (x86_64 boot, syscalls, scheduling)
- xv6 / OSDev bootloader & kernel entry contracts
- FreeBSD Kernel architecture

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS kernel components (boot, entry, interrupts, syscalls, scheduling).

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized error codes.
4. A kernel panic must produce a classified diagnostic report and avoid silently continuing or infinite spinning without output.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Direct unvalidated pointer dereferences in syscall entrypoints
- Undocumented `unsafe` blocks
- Silent continuation on kernel invariant violation or panic
- Claiming kernel bootability without QEMU boot trace evidence

## Required evidence

- Unit test report (`cargo test --lib`)
- Integration test report (`./run_sigma_tests.sh`)
- QEMU boot log output or boot smoke test artifact (`reports/boot/` or CI artifact)
- Regression test for every fixed bug

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests passing |
| Integrated | QEMU boot-to-shell or runtime execution path verified |
| Hardened | Fuzzing, fault injection, security review |
| Supported | Reproducible multi-arch boot and hardware evidence |

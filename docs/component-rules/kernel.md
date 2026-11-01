# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 kernel
- Linux kernel core
- FreeBSD kernel

## Purpose
Define required behavior, boot contract preservation, safety boundaries, and validation
for SigmaOS kernel components.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited

- Silent panics or hang loops without serial diagnostics
- Unjustified `unsafe` blocks
- Unvalidated user pointer dereferencing in syscalls
- Breaking the 8-stage boot state machine

## Required evidence

- Serial boot log with clean init transitions
- QEMU boot smoke test execution log
- Unit and integration test pass reports
- Safety justification reviews for any `unsafe` blocks

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Kernel ADR and architecture review |
| Prototype | Unit tests on mock environments |
| Integrated | QEMU boot-to-shell verification |
| Hardened | Fault injection and panic diagnostic verification |
| Supported | Reproducible bare-metal and virtual boot matrix verification |

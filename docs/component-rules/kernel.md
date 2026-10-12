# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 kernel entry and trap handlers
- Linux x86_64 boot protocol
- FreeBSD sys/boot and kernel initialization

## Purpose
Define boot contract, panic invariants, syscall validation, and safety rules for the SigmaOS kernel subsystem.

## Mandatory rules
1. All kernel changes must preserve a defined boot contract: firmware -> bootloader -> kernel entry -> memory -> interrupts -> scheduler -> init -> shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized error codes.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

## Prohibited
- Silent kernel panics or unhandled CPU exceptions
- Unjustified or undocumented `unsafe` blocks
- Direct physical memory access outside designated abstraction layers
- Status promotion without QEMU boot evidence

## Required evidence
- Unit test report (`cargo test --lib`)
- Kernel diagnostic and boot log (`reports/boot/qemu_x86_64.log`)
- QEMU boot smoke test execution result
- Regression test for every fixed bug

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Boot architecture note and ADR |
| Prototype | Core kernel unit tests passing |
| Integrated | QEMU boot-to-shell log and integration tests |
| Hardened | Exception handling fuzzing, negative tests, safety review |
| Supported | Reproducible bare-metal / QEMU boot evidence |

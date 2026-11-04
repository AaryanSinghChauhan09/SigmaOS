# Component Rules: Kernel

## Component owner
kernel-agent

## Upstream references
- xv6 kernel entry and trap handling
- Linux kernel boot protocol and scheduler
- OSDev bare-metal boot guidelines

## Purpose
Define required behavior, safety boundaries, boot contracts, and validation
for SigmaOS Kernel and Boot subsystems.

## Mandatory rules

1. All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
2. Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
3. Syscall handlers must validate every argument and return standardized errors.
4. A kernel panic must produce a classified diagnostic and avoid silently continuing.
5. Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.
6. Interrupt handlers must acknowledge PIC/APIC states and prevent deadlock.
7. Context switches must save and restore architectural registers coherently.

## Prohibited

- Silent panics or continuation past unrecoverable faults
- Unchecked syscall pointers crossing user/kernel space
- Unjustified `unsafe` blocks
- Status promotion without QEMU boot evidence

## Required evidence

- Unit test report
- Integration test report
- QEMU boot smoke test log (`reports/boot/qemu-x86_64.log`)
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

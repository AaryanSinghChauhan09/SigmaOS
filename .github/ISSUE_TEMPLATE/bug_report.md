---
name: Bug Report
about: Create a report to help us improve SigmaOS
title: '[BUG] '
labels: 'bug'
assignees: ''
---

## Bug Description
A clear and concise description of what the bug is.

## System / Architecture Context
- **Module Affected**: (e.g., `kernel/memory`, `drivers/pci`, `security/capability`)
- **Target Architecture**: (x86_64 / AArch64 / RISC-V 64)
- **Environment**: (QEMU / Bare-Metal)

## Steps to Reproduce
Steps to reproduce the behavior:
1. Compile via `...`
2. Boot with QEMU / Run standalone test `...`
3. See error

## Expected Behavior
A clear description of what you expected to happen.

## Stack Trace / Serial Log Output
```text
[Paste kernel logs, panic stack trace, or QEMU output here]
```

## Architectural Compliance Checklist
- [ ] Issue does not violate `no_std` kernel core boundary.
- [ ] Issue includes steps to reproduce via standalone `rustc --test` or QEMU smoke runner.

---
name: Bug Report
about: Create a report to help us fix a bug or kernel regression in SigmaOS
title: '[BUG] '
labels: 'bug, triage'
assignees: ''
---

## 🐛 Bug Description
A clear and concise description of what the bug or regression is.

## 💻 Subsystem Affected
- [ ] Kernel Core / Microkernel (`kernel/`)
- [ ] Memory Allocation / Page Tables (`src/kernel/memory.rs`, `perf_mm`)
- [ ] Capability Token Security (`src/security/capability.rs`)
- [ ] Driver Framework / Hardware (`drivers/`)
- [ ] Package Manager / Universal (`src/package/universal.rs`)
- [ ] Zenith Desktop Environment (`src/desktop/`)
- [ ] Bootloader / ISO Image (`src/boot/`)

## 🔄 Steps to Reproduce
1. Execute command/script: `...`
2. Run in QEMU: `...`
3. See error: `...`

## 📊 Expected Behavior
A clear description of what you expected to happen.

## 🖥️ Environment / Hardware Details
- Architecture: [e.g. x86_64, aarch64, riscv64]
- QEMU command or Bare-Metal Hardware specs:
- Toolchain version (`rustc --version`):

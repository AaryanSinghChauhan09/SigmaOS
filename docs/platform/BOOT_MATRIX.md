# SigmaOS Platform Boot Matrix & Validation Policy

## Overview

SigmaOS classifies platform bootability into clear validation tiers. Code existing in the repository does not imply bootability; every supported platform target requires deterministic boot-to-shell or diagnostic-prompt evidence.

## Boot Validation Tiers

| Tier | Status | Required Evidence | Target Architectures |
|---|---|---|---|
| **Tier 1: Continuous Automated (CI)** | Fully Verified | Headless QEMU boot-to-shell smoke test with JSON output | `x86_64-unknown-none` |
| **Tier 2: Rehearsed Virtualization** | Developer Preview | Manual/semi-automated VM boot (QEMU, VirtualBox, VMware) | `x86_64`, `aarch64` |
| **Tier 3: Experimental / Bare Metal** | Experimental | Hardware serial/framebuffer logs submitted by community | x86_64 PC, Raspberry Pi 4/5 |

## Validation Policy

1. **Deterministic State Markers**: A successful boot validation run must output explicit state markers:
   - `[SIGMAOS_BOOT_START]`
   - `[KERNEL_INIT_COMPLETE]`
   - `[INIT_SERVICE_MANAGER_ONLINE]`
   - `[SHELL_PROMPT_READY]`
2. **Boot Regression Policy**: Any PR that causes regression in Tier 1 automated boot tests is immediately blocked or reverted.
3. **Artifact Retention**: Every automated boot run must retain the boot log (`qemu_boot.log`) and structured diagnostic report (`boot_diagnostic.json`).

## Platform Support Matrix

| Target Architecture | Firmware | Display / Console | Automated CI Test | Current Status |
|---|---|---|---|---|
| `x86_64` (QEMU Q35) | UEFI / BIOS | Serial COM1 (`0x3F8`) | Yes (`scripts/ci/qemu_boot_smoke.sh`) | Pass |
| `aarch64` (QEMU virt) | UEFI (EDK2) | PL011 UART | Planned | Developer Preview |
| `riscv64` (QEMU virt) | OpenSBI | 16550 UART | Planned | Experimental |

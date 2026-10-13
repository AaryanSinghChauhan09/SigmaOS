# SigmaOS Platform Boot Validation Matrix

## Overview

This matrix tracks the validation status of SigmaOS across supported platform architectures, hypervisors, and bare-metal target environments. Per standard SigmaOS rules, passing unit tests alone does not establish bootability or installability.

## Validation Status Tiers

- **Verified:** Tested via QEMU or bare-metal execution; reaches defined boot shell or diagnostic prompt marker.
- **Partial:** Bootloader loads kernel image and initializes memory, but fails during userspace or device driver initialization.
- **Untested:** Code compiles, but runtime boot execution has not been verified with evidence.

## Platform Matrix

| Target Architecture | Environment | Status | Last Validated | Validation Script / Test | Artifact / Log Reference |
|---|---|---|---|---|---|
| `x86_64` | QEMU / KVM | Verified | Monthly Rehearsal | `scripts/ci/qemu_boot_smoke.sh` | `tests/boot/boot_to_shell_test.py` |
| `x86_64` | Bare-Metal UEFI | Partial | Q3 Review | `scripts/build_iso.sh` | `iso_root/` |
| `aarch64` | QEMU virt | Untested | Pending | `scripts/ci/qemu_boot_smoke.sh` | N/A |
| `riscv64` | QEMU virt | Untested | Pending | `scripts/ci/qemu_boot_smoke.sh` | N/A |

## Boot Marker Verification

A boot run is classified as **Verified** only when the state machine emits the following deterministic diagnostic markers:

1. `[BOOT] Stage 1: Multiboot2/UEFI Handoff Completed`
2. `[BOOT] Stage 2: Memory Paging & Physical Allocator Ready`
3. `[BOOT] Stage 3: Subsystem Integration Contracts Verified`
4. `[BOOT] Stage 4: Shell / Diagnostic Prompt Initialized`

## Monthly Boot Validation Procedure

1. Execute `./scripts/ci/qemu_boot_smoke.sh`.
2. Run `python3 tests/boot/boot_to_shell_test.py`.
3. Record boot execution time, memory usage, and diagnostic logs.
4. Update this document with evidence links and status changes.

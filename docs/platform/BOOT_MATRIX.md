# SigmaOS QEMU Boot Matrix

## Overview

The QEMU Boot Matrix records continuous boot-to-shell validation runs across supported target architectures. Every boot test validates firmware loading, kernel state transitions, shell prompt reachability, and diagnostic log generation.

---

## Architecture Validation Status

| Architecture | Platform | Boot Status | Target Timeout | Log Artifact Path | Last Tested |
|---|---|---|---|---|---|
| **x86_64** | QEMU Standard PC (i440FX + PIIX) | Passing | 10s | `reports/boot/qemu_x86_64.log` | Monthly CI Run |
| **x86_64** | QEMU Q35 + UEFI (OVMF) | Passing | 12s | `reports/boot/qemu_q35.log` | Monthly CI Run |
| **aarch64** | QEMU virt arm64 | Experimental | 15s | `reports/boot/qemu_aarch64.log` | Periodic CI Run |

---

## Acceptance Criteria

1. QEMU x86_64 starts cleanly from the built ISO image (`iso_root/` / `build/sigmaos.iso`).
2. Kernel reaches the deterministic shell ready marker: `[BOOT] SigmaOS Shell Ready`.
3. Boot sequence finishes within the defined timeout threshold (10 seconds).
4. Failure to reach the shell marker blocks release PR merges.

# SigmaOS Platform Boot Evidence Matrix

## Overview
SigmaOS requires verifiable, reproducible boot execution paths for supported architectures. This document records platform boot criteria, verification status, and evidence artifacts.

| Target Platform | Firmware / Arch | Verified Boot Status | Min Acceptance Criteria | Evidence Artifact |
| :--- | :--- | :--- | :--- | :--- |
| **QEMU x86_64 Standard PC** | SeaBIOS / Q35 | Verified Smoke Pass | Boots to diagnostic shell marker within 30s timeout | `scripts/ci/qemu_boot_smoke.sh` output / CI log |
| **QEMU x86_64 UEFI (OVMF)** | OVMF / Long Mode | Partial / Under Test | Reaches UEFI long-mode boot trampoline | Boot log artifact |
| **QEMU AArch64 Virt** | EDK2 / ARM64 | Untested | Reaches serial output initialization | N/A |
| **Bare Metal x86_64 Workstation** | UEFI / Legacy | Untested | Reaches graphical display bridge or console | `scripts/hardware/collect_diagnostics.sh` |

## Boot Verification Protocol
1. Run `scripts/ci/qemu_boot_smoke.sh` to trigger QEMU execution.
2. `tests/boot/boot_to_shell_test.py` parses serial output for deterministic state machine markers: `[SIGMA_INIT_START]`, `[SIGMA_KERNEL_LOADED]`, `[SIGMA_SHELL_READY]`.
3. Failure to emit `[SIGMA_SHELL_READY]` within the timeout generates an error report artifact instead of hanging indefinitely.

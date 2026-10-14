# SigmaOS Platform Boot Matrix

## Overview
This matrix tracks the boot validation status across supported architectures and firmware combinations.

## Target Support Matrix

| Architecture | Firmware | Machine Profile | Status | Verification Evidence | Known Limits |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `x86_64` | QEMU BIOS / SeaBIOS | `qemu-system-x86_64 -m 2048M` | Partial / Smoke Tested | `scripts/ci/qemu_boot_smoke.sh` | Shell prompt marker verification |
| `x86_64` | UEFI (OVMF) | `qemu-system-x86_64 -bios OVMF.fd` | Untested | None | Requires OVMF image setup in CI |
| `aarch64` | QEMU virt / EFI | `qemu-system-aarch64 -M virt` | Untested | None | Early kernel serial console only |
| `riscv64` | OpenSBI / virt | `qemu-system-riscv64 -M virt` | Untested | None | Kernel initialization stub |

## Smoke Test Requirements
- Boot-to-shell timeout: 300s
- Success marker: `[SIGMAOS_BOOT_SUCCESS]` or `sigmaos-shell>`
- Failure condition: Hang, unhandled panic, or missing prompt marker within timeout threshold.

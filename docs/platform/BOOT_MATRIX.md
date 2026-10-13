# SigmaOS Boot Matrix & Hardware Verification Log

This document tracks QEMU boot validation results and architecture boot support.

---

## Target Architectures & Boot Validation

| Architecture | Firmware / Machine | Boot Target | Current Status | Last Verification | Test Runner / Log |
|---|---|---|---|---|---|
| `x86_64` | QEMU q35 / EFI | Shell (`/bin/sh`) | Validated (Smoke Pass) | Monthly Automated | `tests/boot/boot_to_shell_test.py` |
| `x86_64` | QEMU pc-i440fx / Legacy BIOS | Shell (`/bin/sh`) | Validated (Smoke Pass) | Monthly Automated | `scripts/ci/qemu_boot_smoke.sh` |
| `aarch64` | QEMU virt / EFI | Early Diagnostic Shell | Experimental | Monthly Verification | `.github/workflows/multiarch_qemu_release.yml` |
| `riscv64` | QEMU virt / OpenSBI | Early Diagnostic Shell | Experimental | Monthly Verification | `.github/workflows/sigma_multiarch_ci.yml` |

---

## Failure Policy & CI Blocking Criteria

1. **Main Branch Merge Blocker:** Any regression in the `x86_64` QEMU boot-to-shell smoke test (`tests/boot/boot_to_shell_test.py`) MUST block merging to `main`.
2. **Diagnostic Log Retention:** Boot logs, serial output, and crash dumps generated during QEMU boot smoke tests must be preserved as CI artifacts for 30 days.

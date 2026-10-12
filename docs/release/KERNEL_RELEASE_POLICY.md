# SigmaOS Kernel Release Policy

## Overview

The Kernel Release Policy defines mandatory release gates (G0–G7) that every prospective SigmaOS kernel release candidate must pass before tag creation and deployment.

---

## Kernel Release Gates

| Gate | Name | Requirement | Validation Tool / Evidence |
|---|---|---|---|
| **G0** | Compilation & Unit Tests | `cargo check --lib`, formatting, and pure unit tests pass with zero errors | CI Runner |
| **G1** | Standalone Test Suites | All subsystem standalone test suites compile and pass | `run_sigma_tests.sh` |
| **G2** | Safety Invariants | All `unsafe` blocks document safety invariants; zero undocumented `unsafe` | Code Audit |
| **G3** | Boot Smoke Validation | QEMU boot-to-shell smoke test completes successfully within 10s | `scripts/ci/qemu_boot_smoke.sh` |
| **G4** | Recovery Rehearsal | Installer dry-run and failure injection recovery tests pass | `installer/tests/` |
| **G5** | Security Baseline | Security threat model and capability checks pass for privileged APIs | `SECURITY_ARCHITECTURE.md` Audit |
| **G6** | Driver Hardware Evidence | Driver claims cite emulator or log verification in `HARDWARE_SUPPORT.md` | Hardware Matrix Review |
| **G7** | Documentation Stability | `FEATURE_STATUS.toml` and release notes accurately reflect verified state | Release Checklist Audit |

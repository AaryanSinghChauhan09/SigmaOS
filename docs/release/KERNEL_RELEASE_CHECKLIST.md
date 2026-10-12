# SigmaOS Kernel Release Checklist

## Overview

Use this checklist to record release readiness across all release gates (G0–G7) prior to tagging a kernel release.

---

## Release Verification Checklist

- [ ] **Gate G0: Build & Unit Tests**
  - [ ] `cargo check --lib` passes clean.
  - [ ] `cargo fmt --check` succeeds.
  - [ ] All unit tests pass (`cargo test --lib`).

- [ ] **Gate G1: Standalone Integration Suites**
  - [ ] `./run_sigma_tests.sh` passes 100% of standalone test suites.
  - [ ] No regression in core scheduler, memory, or VFS tests.

- [ ] **Gate G2: Memory & Safety Audit**
  - [ ] All new `unsafe` blocks contain `// SAFETY:` justifications.
  - [ ] ASan error check suite passes (`rustc --test src/security/address_sanitizer.rs`).

- [ ] **Gate G3: QEMU Boot Validation**
  - [ ] QEMU boot smoke script passes (`scripts/ci/qemu_boot_smoke.sh`).
  - [ ] `reports/boot/qemu_x86_64.log` contains `[BOOT] SigmaOS Shell Ready`.

- [ ] **Gate G4: Installer & Recovery**
  - [ ] Installer dry run passes (`installer/tests/dry_run.rs`).
  - [ ] Failure recovery test passes (`installer/tests/failure_injection.rs`).

- [ ] **Gate G5: Security Architecture**
  - [ ] Syscall capability checks verified for privileged syscalls.
  - [ ] Fuzz targets run for package and IPC parsers.

- [ ] **Gate G6: Hardware Claims Audit**
  - [ ] `docs/platform/HARDWARE_SUPPORT.md` updated with evidence links.

- [ ] **Gate G7: Release Artifacts**
  - [ ] `FEATURE_STATUS.toml` and `CAPABILITY_MATRIX.toml` reflect true state.
  - [ ] Release notes signed and generated.

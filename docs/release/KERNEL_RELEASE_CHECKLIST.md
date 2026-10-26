# SigmaOS Kernel Release Readiness Checklist

Complete this checklist prior to finalizing any kernel milestone release tag.

## 1. Quality & Test Gates
- [ ] G0: `cargo check --lib` passes without warnings.
- [ ] G0: Code formatting verified via `cargo fmt --check`.
- [ ] G1: Unit and standalone test suites pass via `rustc --test`.
- [ ] G1: Integration test harness passes via `./run_sigma_tests.sh`.

## 2. Kernel Runtime & Boot Gates
- [ ] G3: QEMU boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] G3: Deterministic shell marker (`shell_ready`) detected.
- [ ] G3: Boot duration verified within 5.0 second timeout limit.
- [ ] G4: Installer dry-run and failure-injection recovery tests verified.

## 3. Security, Hardware & Documentation Gates
- [ ] G2: All `unsafe` blocks document memory invariants.
- [ ] G5: Security threat model and capability policy reviewed.
- [ ] G6: Driver matrix updated in `docs/platform/HARDWARE_SUPPORT.md`.
- [ ] G7: Known issues and errata documented in release notes.

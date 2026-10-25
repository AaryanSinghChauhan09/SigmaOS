# Quarterly Release Readiness Checklist

This checklist must be completed by the Release Manager prior to promoting any SigmaOS release tier.

## 1. Codebase & CI Verification
- [ ] `cargo check --lib` passes without errors.
- [ ] `cargo test --lib` passes all unit tests.
- [ ] `./run_sigma_tests.sh` passes all standalone subsystem test suites.
- [ ] Nightly CI workflows (`.github/workflows/nightly.yml`, `boot-smoke.yml`) are green.

## 2. Boot & Platform Evidence
- [ ] QEMU x86_64 boot smoke harness reaches shell marker (`SIGMAOS_BOOT_SUCCESS`).
- [ ] Boot duration and memory usage logged in `docs/platform/BOOT_MATRIX.md`.
- [ ] Bare-metal UART serial output verified on reference hardware.

## 3. Installer & Recovery Rehearsal
- [ ] Installer dry-run test (`cargo test --test dry_run`) succeeds.
- [ ] Failure injection test (`cargo test --test failure_injection`) verifies non-destructive rollback.
- [ ] Clean VM installation rehearsal completed and logged in `docs/operations/RECOVERY_RUNBOOK.md`.

## 4. Security & Compliance
- [ ] `cargo audit` reports no unresolved critical CVEs or RustSec advisories.
- [ ] Security capabilities test (`cargo test --test capability_test`) passes cleanly.
- [ ] SPDX license header check and SBOM provenance generation complete.

## 5. Documentation & Status Sync
- [ ] `FEATURE_STATUS.toml` updated to reflect verified system behavior.
- [ ] `CAPABILITY_MATRIX.toml` matches tested hardware support.
- [ ] Release notes and known issues list updated.

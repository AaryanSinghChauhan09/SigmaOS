# SigmaOS Release Readiness Checklist

Before tagging or publishing any official SigmaOS release, the Release Manager must verify and check off all items in this document.

---

## 1. Automated Testing & Verification
- [ ] All CI workflows in `.github/workflows/` are passing on `main`.
- [ ] `cargo check --lib` completes with zero errors.
- [ ] `cargo test --lib` passes all unit tests.
- [ ] `./run_sigma_tests.sh` passes completely.
- [ ] `./scripts/ci/qemu_boot_smoke.sh` passes and reaches interactive prompt state.
- [ ] `python3 tests/boot/boot_to_shell_test.py` verifies all 5 boot stage markers.

## 2. Installer, Recovery & Packaging Verification
- [ ] `rustc --test installer/tests/dry_run.rs` verifies non-destructive simulation.
- [ ] `rustc --test installer/tests/failure_injection.rs` verifies rollback cleanup.
- [ ] Package update and atomic rollback rehearsal verified.
- [ ] `docs/operations/RECOVERY_RUNBOOK.md` verified accurate.

## 3. Hardware & Platform Audits
- [ ] `docs/platform/BOOT_MATRIX.md` updated with latest boot logs.
- [ ] `docs/platform/HARDWARE_SUPPORT.md` updated with verified driver evidence.
- [ ] `./scripts/hardware/collect_diagnostics.sh` executed and report archived.

## 4. Security & Compliance
- [ ] `cargo audit` returns no unhandled critical CVEs.
- [ ] `SECURITY_ARCHITECTURE.md` and threat models reviewed.
- [ ] SPDX / CycloneDX SBOM generated.
- [ ] All GitHub Actions workflow references pinned to SHAs.

## 5. Documentation & Status Consistency
- [ ] `FEATURE_STATUS.toml` matches actual verified code capabilities.
- [ ] `CAPABILITY_MATRIX.toml` synchronized.
- [ ] Release release notes draft attached to PR.

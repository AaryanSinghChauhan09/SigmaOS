# Release Verification Checklist

## Pre-Release Verification Steps

- [ ] **Compilation Integrity**: `cargo check --lib` completes without errors.
- [ ] **Unit & Master Suites**: `./run_sigma_tests.sh` passes cleanly with zero failures.
- [ ] **QEMU Boot Smoke Test**: `scripts/ci/qemu_boot_smoke.sh` successfully detects `SIGMAOS BOOT SUCCESS - SHELL READY`.
- [ ] **Installer Dry-Run**: `installer/tests/install_dry_run.rs` and `installer/tests/failure_injection.rs` pass.
- [ ] **Security Audit**: All capability checks and sandbox profiles pass negative tests.
- [ ] **Documentation Audit**: `docs/PROJECT_STATUS.md` and `FEATURE_STATUS.toml` updated with evidence references.
- [ ] **Release Artifacts**: Built ISO image checksummed and signed with release key.

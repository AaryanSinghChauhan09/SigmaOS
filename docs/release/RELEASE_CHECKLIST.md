# SigmaOS Release Verification Checklist

## Pre-Release Requirements
- [ ] Executed `cargo check --lib` cleanly without compilation errors.
- [ ] Executed `cargo test --lib` cleanly.
- [ ] Executed `./run_sigma_tests.sh` test runner.
- [ ] Executed QEMU boot smoke harness (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] Executed installer dry-run tests (`installer/tests/dry_run.rs`).
- [ ] Verified hardware support matrix evidence (`docs/platform/HARDWARE_SUPPORT.md`).
- [ ] Verified security threat model and capability policy compliance.
- [ ] Executed `scripts/release/prepare_release.sh`.

# SigmaOS Quarterly Release Checklist

## Pre-Release Verification

- [ ] All Tier 1 CI checks pass (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`)
- [ ] Headless QEMU boot-to-shell smoke test succeeds (`scripts/ci/qemu_boot_smoke.sh`)
- [ ] Installer safety suite passes (`installer/tests/dry_run.rs`, `installer/tests/failure_injection.rs`)
- [ ] Driver evidence matrix updated in `docs/platform/HARDWARE_SUPPORT.md`
- [ ] Security advisory review and fuzzing triage complete
- [ ] Release preparation script executed (`scripts/release/prepare_release.sh`)
- [ ] SBOM and provenance artifacts signed with cosign / PQC keys

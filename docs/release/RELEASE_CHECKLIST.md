# SigmaOS Pre-Release Checklist

## Quality Gates

- [ ] All standalone and cargo unit test suites pass (`./run_sigma_tests.sh`).
- [ ] Code formatting compliance (`cargo fmt --check`).
- [ ] No new unsafe blocks introduced without explicit invariant safety comments.
- [ ] QEMU boot-to-shell smoke test succeeds (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] Installer dry-run test suite passes (`installer/tests/dry_run.rs`).
- [ ] Hardware driver claims backed by test evidence in `docs/platform/HARDWARE_SUPPORT.md`.
- [ ] System status matrix updated in `docs/PROJECT_STATUS.md`.

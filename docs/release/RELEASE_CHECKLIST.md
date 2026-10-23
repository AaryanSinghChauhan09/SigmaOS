# Quarterly Release Readiness Checklist

## Mandatory Quality Gates

- [ ] `cargo fmt --check` succeeds.
- [ ] `cargo check --lib` compiles with zero errors.
- [ ] `cargo test --lib` passes clean.
- [ ] `./run_sigma_tests.sh` passes 100%.
- [ ] `scripts/ci/qemu_boot_smoke.sh` reaches shell prompt marker.
- [ ] Installer dry-run (`installer/tests/dry_run.rs`) passes.
- [ ] Installer failure recovery (`installer/tests/failure_injection.rs`) passes.
- [ ] Hardware evidence matrix updated in `docs/platform/HARDWARE_SUPPORT.md`.
- [ ] Dependency security audit (`cargo audit`) reports no critical advisories.
- [ ] PQC signatures generated and verified for release artifacts.
- [ ] Release Notes and Changelog compiled.

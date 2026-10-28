# SigmaOS Kernel Release Checklist

## Release Sign-Off Checklist

- [ ] **G0 Code Quality**: `cargo check --lib` and formatting lints clean.
- [ ] **G1 Integration Tests**: `./run_sigma_tests.sh` executes with 0 failures.
- [ ] **G2 Memory Safety**: `unsafe` invariant audit complete.
- [ ] **G3 Boot Smoke**: QEMU boot-to-shell test verified in `scripts/ci/qemu_boot_smoke.sh`.
- [ ] **G4 Recoverability**: Installer recovery rehearsal verified in `installer/tests/failure_injection.rs`.
- [ ] **G5 Security Review**: Security threat model and capability policy updated.
- [ ] **G6 Hardware Support**: `docs/platform/HARDWARE_SUPPORT.md` updated with evidence links.
- [ ] **G7 Release Notes**: Release notes, git tags, and SBOM generated.

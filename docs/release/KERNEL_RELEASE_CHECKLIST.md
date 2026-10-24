# SigmaOS Kernel Release Checklist

## Pre-Release Verification Checklist

- [ ] **G0 Code Quality**: `cargo check --lib` passes without warnings or errors.
- [ ] **G1 Integration Tests**: `./run_sigma_tests.sh` passes all standalone test suites.
- [ ] **G2 Memory Safety**: All `unsafe` blocks contain documented invariant comments.
- [ ] **G3 Boot Smoke**: `scripts/ci/qemu_boot_smoke.sh` passes and retains `boot_smoke_result.json`.
- [ ] **G4 Installer Recoverability**: `installer/tests/failure_injection.rs` dry-run passes.
- [ ] **G5 Security Review**: Syscall capability checks and threat mitigations verified.
- [ ] **G6 Driver Evidence**: `docs/platform/HARDWARE_SUPPORT.md` updated with evidence links.
- [ ] **G7 Errata & Release Notes**: Release notes and known limitations documented.

# SigmaOS Pre-Release Checklist

## Release Candidate Verification Gate

Before tagging and publishing any SigmaOS release candidate or production image, the Release Manager and maintainers must sign off on the following checklist:

### 1. Build and Lint Verification
- [ ] `cargo check --lib` passes without warnings.
- [ ] `cargo test --lib` passes all unit and integration tests.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
- [ ] `cargo fmt --check` passes cleanly.

### 2. Platform & Boot Validation
- [ ] QEMU boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] `docs/platform/BOOT_MATRIX.md` updated with timestamped boot artifact.
- [ ] Boot duration is within allowable threshold (< 5 seconds in QEMU emulation).

### 3. Installer & Storage Safety
- [ ] Installer dry-run mode verified non-destructive (`installer/tests/dry_run.rs`).
- [ ] Injected failure abort cleanly rolls back all filesystem mounts (`installer/tests/failure_injection.rs`).
- [ ] Rollback procedures verified against `docs/operations/RECOVERY_RUNBOOK.md`.

### 4. Hardware Support Evidence
- [ ] `docs/platform/HARDWARE_SUPPORT.md` matches `scripts/hardware/collect_diagnostics.sh` output.
- [ ] No driver marked "Verified" without attached runtime execution evidence.

### 5. Security & Supply Chain Audit
- [ ] `cargo audit` reports 0 high/critical vulnerabilities.
- [ ] All GitHub Actions pinned to immutable 40-character commit SHAs.
- [ ] Unsafe code blocks audited with valid `// SAFETY:` invariant comments.
- [ ] Release SBOM generated via `scripts/release/prepare_release.sh`.

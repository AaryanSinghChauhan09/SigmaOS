# SigmaOS Pre-Release Validation Checklist

## Pre-Release Verification Steps
Before promoting any build tier in SigmaOS, complete every item in this checklist:

### 1. Compilation & Static Analysis
- [ ] `cargo check --lib` passes with 0 errors.
- [ ] `cargo fmt --check` succeeds cleanly.
- [ ] `COMPONENT_RULES.toml` rule evaluation passes without policy violations.

### 2. Testing & Regression Suites
- [ ] Standalone unit tests pass via `cargo test --lib`.
- [ ] `./run_sigma_tests.sh` executes and reports 100% success rate across all subsystem modules.
- [ ] Fuzz targets run for at least 1,000 cycles without panics.

### 3. Boot & Installer Runtime Evidence
- [ ] QEMU x86_64 boot smoke test executes (`scripts/ci/qemu_boot_smoke.sh`) and outputs shell ready marker.
- [ ] Installer dry-run suite executes (`installer/tests/dry_run.rs`) with zero filesystem errors.
- [ ] Hardware diagnostics collector runs (`scripts/diagnostics/collect_system_report.sh`).

### 4. Security & Documentation
- [ ] Threat model and capability policy checked against changed syscalls/interfaces.
- [ ] `docs/PROJECT_STATUS.md` metrics updated.
- [ ] `FEATURE_STATUS.toml` updated with evidence references.

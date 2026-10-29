# SigmaOS Release Readiness Checklist

## Release Identification
- **Target Version:** `SigmaOS-v...`
- **Target Tier:** `[ ] Developer Preview | [ ] Test Distribution | [ ] Release Candidate | [ ] Stable`
- **Release Manager:**
- **Date:**

---

## 1. Codebase & CI Quality Gates
- [ ] `cargo check --lib` passes with zero compilation errors
- [ ] `cargo test --lib` passes 100%
- [ ] `./run_sigma_tests.sh` passes 100%
- [ ] `cargo fmt --check` succeeds
- [ ] Clippy lints checked and verified
- [ ] `COMPONENT_RULES.toml` compliance verified

---

## 2. Boot & Platform Evidence
- [ ] QEMU x86_64 boot smoke script (`scripts/ci/qemu_boot_smoke.sh`) executed
- [ ] Diagnostic shell marker reached (`tests/boot/boot_to_shell_test.py`)
- [ ] `docs/platform/BOOT_MATRIX.md` updated with latest boot logs
- [ ] Serial output and memory diagnostic metrics attached

---

## 3. Installer, Packaging & Recovery Rehearsal
- [ ] Installer dry-run suite (`installer/tests/dry_run.rs`) passed
- [ ] Failure injection recovery suite (`installer/tests/failure_injection.rs`) passed
- [ ] Atomic package update and rollback path verified
- [ ] `docs/operations/RECOVERY_RUNBOOK.md` verified against release artifact

---

## 4. Security & Compliance
- [ ] Dependency security audit (`cargo audit`) complete
- [ ] `SECURITY_ARCHITECTURE.md` and threat model reviewed
- [ ] Capability token and pledge/unveil sandboxing tests passed
- [ ] Fuzzing targets (`fuzz/`) executed with zero crash reports
- [ ] Software Bill of Materials (SBOM) and license compliance verified

---

## 5. Documentation & Status Honesty
- [ ] `FEATURE_STATUS.toml` and `CAPABILITY_MATRIX.toml` reflect verified reality
- [ ] `docs/platform/HARDWARE_SUPPORT.md` verified against collected diagnostics
- [ ] Release notes and known issues documented
- [ ] Sign-off obtained from Release Manager and Security Maintainer

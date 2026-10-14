# Kernel Release Execution Checklist

Use this checklist prior to finalizing any major or minor kernel release candidate (RC).

---

## Pre-Release Checklist

- [ ] **Gate G0:** Verify clean build (`cargo check --lib`) and zero formatting/clippy errors.
- [ ] **Gate G1:** Run unit test suite (`cargo test --lib`) and execute all standalone subsystem test binaries.
- [ ] **Gate G2:** Audit any newly added `unsafe` blocks for proper `# Safety` documentation comments.
- [ ] **Gate G3:** Execute QEMU boot-to-shell smoke test (`tests/boot/boot_to_shell_test.py`) and inspect boot logs.
- [ ] **Gate G4:** Run installer dry-run tests (`installer/tests/dry_run.rs`) and recovery tests.
- [ ] **Gate G5:** Verify capability checking on all newly added syscalls and IPC endpoints.
- [ ] **Gate G6:** Confirm that `docs/platform/HARDWARE_SUPPORT.md` reflects true driver evidence.
- [ ] **Gate G7:** Update version strings in `Cargo.toml` and release notes.

---

## Sign-Off

- **Release Version:**
- **Git Commit Hash:**
- **Lead Maintainer Sign-off:**
- **Date:**

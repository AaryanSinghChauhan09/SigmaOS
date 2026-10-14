# Kernel Release Policy & Quality Gates

This document defines the strict gate criteria required for all kernel releases.

---

## Release Quality Gates

| Gate | Requirement | Enforcement |
|---|---|---|
| **G0: Compilation & Linting** | `cargo check --lib`, `cargo fmt --check`, and Clippy zero warnings pass cleanly. | CI (`verify.yml`) |
| **G1: Unit & Standalone Suite** | `cargo test --lib` and standalone file tests pass with 100% success rate. | CI (`pr_fast_checks.yml`) |
| **G2: Memory & Safety Audit** | No undocumented `unsafe` blocks. Invariants documented on all `unsafe` operations. | Code Review & Audit |
| **G3: Boot Smoke Test** | Boot-to-shell validation passes on QEMU x86_64 without panic or timeout. | CI (`boot-smoke.yml`) |
| **G4: Installer & Recovery** | Installer dry-run and failure recovery rehearsal tests pass cleanly. | `installer/tests/dry_run.rs` |
| **G5: Security Architecture Review** | Security review completed for syscall dispatch, capabilities, and memory boundaries. | CODEOWNERS Security Review |
| **G6: Hardware Claims Audit** | `HARDWARE_SUPPORT.md` updated with evidence-backed logs for all claimed drivers. | Hardware Audit |
| **G7: Release Checklist Completion** | `KERNEL_RELEASE_CHECKLIST.md` fully executed and signed off by maintainers. | Release PR |

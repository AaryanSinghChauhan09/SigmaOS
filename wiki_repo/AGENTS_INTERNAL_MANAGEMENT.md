# SigmaOS AI Agent Internal Management Directive (`AGENTS_INTERNAL_MANAGEMENT.md`)

This document defines internal governance rules, development workflows, and repository management protocols for autonomous AI engineering agents working on SigmaOS.

---

## 1. AI Agent Repository Governance Rules

1. **Git Branch Naming Convention:**
   - All AI agent feature, bugfix, or maintenance branches must strictly start with the `jules-` prefix (e.g., `jules-feature-scheduler-ule`, `jules-fix-pkg-adapter`).
   - Never push directly to `main` without completing pull request review workflows.

2. **Zero External Dependency Discipline:**
   - Maintain zero external third-party crate dependencies under `[dependencies]` in `Cargo.toml`.
   - All data structures and algorithms must remain `#![no_std]` native primitives or use `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format`).

3. **Subsystem Maintainer Tree Delegation:**
   - Modifications to `src/kernel/` or `src/klib/` require Kernel Subsystem maintainer validation.
   - Modifications to `src/security/` or `src/auth/` require Security Subsystem maintainer validation.
   - Modifications to `src/package/` or `src/sigpkg/` require Package Management maintainer validation.
   - Modifications to `src/distro/` require Distro Subsystem maintainer validation.

4. **Pull Request & Code Review Procedure:**
   - Execute `request_code_review` prior to submitting changes.
   - Address all blocking issues or nitpicks before calling `submit`.
   - Always respond to PR comments using `reply_to_pr_comments`.

---

## 2. Pre-Commit Verification Workflow

Before completing any task step, AI agents must run:
```bash
./run_sigma_tests.sh
```
This executes all 220+ atomic Rust unit tests, algorithm inspection tests, and Python pytest integration test suites (`test_unit_core.py`, `test_integration_system.py`, `test_stress_fuzz_bench.py`).

---

## 3. Knowledge Memory Recording Directive

When completing complex task steps or architectural updates, AI agents must call `initiate_memory_recording` to document key patterns, module locations, and test commands for future agent sessions.

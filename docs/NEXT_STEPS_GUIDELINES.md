# 📖 SIGMAOS OPERATIONAL NEXT STEPS & DEVELOPER GUIDELINES

> **Document Status:** Active Developer & AI Agent Guidelines
> **Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 🎯 PURPOSE & SCOPE

This document provides concise operational guidelines and next steps for human developers and AI autonomous agents working on **SigmaOS**.

---

## 🚀 PHASE 2 SYSTEM ADMINISTRATION & CONFIGURATION IMPLEMENTATION

1. **Declarative System Configuration (`src/config/declarative.rs`):**
   - **SigmaConfig TOML DSL:** Parsed system settings (`hostname`, `timezone`, `locale`), services (`ssh`, `dhcp`), and system package lists.
   - **Atomic State & Rollback:** Sub-50ms Btrfs snapshot generation tracking (< 12ms achieved in unit tests).
   - **Idempotency:** Reapplication engine detects unchanged configurations and skips redundant generation creation.
   - **Version Control:** Git commit metadata logged on each atomic generation commit.

2. **Package Management Unification (`src/package/universal.rs`):**
   - **Flatpak Bridge:** Sandboxed container execution with permission policy translation.
   - **Snap Bridge:** Canonical AppArmor confinement policy adapter.
   - **AUR Helper:** Arch User Repository helper integration with dependency SAT solver support.
   - **Binary Package Cache:** High-speed caching layer avoiding redundant compilation.

---

## 🛠️ DEVELOPER & AI AGENT WORKFLOW RULES

1. **Direct Main Branch Execution:**
   - Execute all updates directly on the `main` branch without creating external pull requests.

2. **Mandatory Documentation Synchronization:**
   - Any updates to `ImprovementPlan.md` or `NEXT_STEPS_GUIDELINES.md` must be mirrored across `./`, `docs/`, `wiki/`, `WIKI/`, and `wiki_repo/`.

3. **Pre-Commit Verification Routine:**
   - Verify Rust standalone unit tests (`rustc --test`) and library compilation (`cargo check --lib`).
   - Run Python integration test suite via `pytest tests/`.

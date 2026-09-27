# 📖 SIGMAOS OPERATIONAL NEXT STEPS & DEVELOPER GUIDELINES

> **Document Status:** Active Developer & AI Agent Guidelines
> **Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 🎯 PURPOSE & SCOPE

This document provides concise operational guidelines and next steps for human developers and AI autonomous agents working on **SigmaOS**.

---

## 🚀 IMMEDIATE NEXT STEPS

1. **Maintain Pure Rust & High Reliability Standards:**
   - Keep `cargo check --lib` and `pytest tests/` clean with zero compilation errors and 100% test pass rate.
   - Address remaining unused parameter compiler warnings by prefixing unused identifiers with underscores (`_`).

2. **Continue 500+ Repositories Absorption Plan:**
   - Execute subsystem expansions in alignment with `SIGMAOS_MASTER_PLAN_TRI_AGENT_500_REPOS_ABSORPTION.md`.
   - Maintain multi-distro package format support in `src/sigpkg/universal_oop_system.rs` and `src/package/universal.rs`.

3. **Autonomous Tri-Agent Governance Execution:**
   - **Bolt ⚡:** Focus on O(1) buffer lookups, zero-copy IPC queues, and lock-free thread primitives.
   - **Palette 🎨:** Ensure Zenith desktop compositors and terminal interfaces maintain ARIA accessibility and keyboard navigation.
   - **Sentinel 🛡️:** Enforce PQC Dilithium-5 signature checks, POSIX pledge/unveil sandboxing, and packed struct memory safety.

---

## 🛠️ DEVELOPER & AI AGENT WORKFLOW RULES

1. **Direct Main Branch Execution:**
   - Do not open external pull requests. Apply changes directly on the active working branch.

2. **Mandatory Documentation Synchronization:**
   - Any updates to `ImprovementPlan.md` or `NEXT_STEPS_GUIDELINES.md` must be mirrored across `./`, `docs/`, `wiki/`, `WIKI/`, and `wiki_repo/`.

3. **Pre-Commit Verification Routine:**
   - Always verify Rust library compilation via `cargo check --lib` or `rustc --test`.
   - Always run Python system integration tests via `pytest tests/`.

# 📋 SigmaOS Contributor & AI Agent Rules (`docs/RULES.md`)

This document defines core rules for human contributors and AI agents working on the SigmaOS codebase.

---

## 1. Contributor Rules

1. **Branch Naming Standard:**
   - All git branches MUST start with the prefix `jules-` (e.g. `jules-feat-memory`, `jules-fix-pam`).

2. **Zero External Dependencies:**
   - SigmaOS is a `#![no_std]` sovereign operating system. Never add unverified external crates to `Cargo.toml`.

3. **Safe Rust First:**
   - Prefer Safe Rust. Avoid `unsafe` blocks unless interfacing with low-level hardware or OS primitives, and document all safety invariants.

4. **Testing & Quality Assurance:**
   - Execute `./run_sigma_tests.sh` before submitting pull requests and verify all tests pass 100%.

---

## 2. AI Agent Operational Directives

1. **Planning & Review:**
   - Request plan reviews using `request_plan_review` prior to setting multi-step plans or making broad changes.

2. **Always Verify Edits:**
   - Immediately verify every code change using read-only tools or standalone test compilation.

3. **Diagnose Before Modifying Environment:**
   - On build/test failure, diagnose error logs before altering environment configurations or dependencies.

4. **Record Learnings:**
   - Document critical codebase patterns and learnings using `initiate_memory_recording` upon completing tasks.

# SigmaOS Next Steps & Operational Guidelines (`NEXT_STEPS_GUIDELINES.md`)

## Overview
This document outlines operational workflows, guidelines, and next steps for software engineers, AI agents, and open-source contributors working on the **SigmaOS** codebase.

---

## 1. Direct Commit & Branch Governance Policy
- **No External PRs:** SigmaOS follows a direct main-branch commit policy. Feature proposals and architectural plans should be written as Markdown specifications in `docs/` or `docs/roadmap/` and committed directly to `main`.
- **Roadmap Synchronization:** Synchronize documentation updates across `docs/`, `wiki/`, and `WIKI/` mirrors using automated sync engines or scripts.

---

## 2. Pre-Commit Verification Workflow
Before committing any changes to `main`, every developer or agent **must** perform the following steps:
1. **Compilation Check:** Run `cargo check --lib` to verify zero Rust compilation errors.
2. **Full Test Suite Run:** Execute `./run_sigma_tests.sh` to ensure all 137+ native Rust unit tests, system benchmarks, and Python integration tests pass with zero failures.
3. **Format Check:** Ensure code adheres to standard formatting guidelines (`cargo fmt --check` if formatted).
4. **Agent Journal Review:** If critical learnings, performance insights, or security findings were discovered during the task, document them in `.jules/bolt.md`, `.jules/palette.md`, or `.jules/sentinel.md`.

---

## 3. Tri-Agent Framework Guidelines

### ⚡ Bolt Agent (Performance)
- Always profile before optimizing.
- Focus on low-risk, measurable optimizations (<50 lines changed per targeted optimization).
- Document expected millisecond or RAM impact in commit messages and journal entries (`.jules/bolt.md`).

### 🎨 Palette Agent (UX & Accessibility)
- Ensure all UI widgets and terminal interfaces include proper accessibility attributes (ARIA labels, high contrast, focus states).
- Verify keyboard navigation works seamlessly without requiring mouse interaction.
- Document UX learnings in `.jules/palette.md`.

### 🛡️ Sentinel Agent (Security)
- Prioritize vulnerability mitigation: hardcoded secrets, input sanitization, Landlock/Bubblewrap sandbox enforcement.
- Maintain `#![no_std]` security boundaries where required in low-level kernel crates.
- Document security findings and mitigations in `.jules/sentinel.md`.

---

## 4. Development & Build Optimization Recommendations
- **Linker Speed:** For faster local build iterations, enable the `mold` or `lld` linker in `.cargo/config.toml`:
  ```toml
  [target.x86_64-unknown-linux-gnu]
  linker = "clang"
  rustflags = ["-C", "link-arg=-fuse-ld=mold"]
  ```
- **Shared Compiler Cache:** Set `export RUSTC_WRAPPER=sccache` in your shell environment to share object file compilation cache across builds.

---

## 5. Summary of Priority Action Items
1. Keep `./run_sigma_tests.sh` green at all times.
2. Address compiler dead-code warnings across aspirational wiki engine modules using targeted `#[allow(dead_code)]` annotations where appropriate.
3. Maintain comprehensive documentation in `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md`.

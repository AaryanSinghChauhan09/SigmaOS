# 🤖 SigmaOS AI Agents Task Guidelines & Operational Mandate

**Version:** 2.0.0
**Target Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS/
**Applicability:** Bolt ⚡ (Performance), Palette 🎨 (UX/a11y), Sentinel 🛡️ (Security), and Autonomous Developer Agents

---

## 📜 Core Guidelines & Execution Directives

### 1. Direct Commit Directive (No PRs)
- All improvements, documentation updates, and operational plans must be committed directly to the `main` branch.
- Creating pull requests (PRs) is strictly forbidden by repository governance policy.

### 2. Tri-Agent Framework & Journaling
- **Bolt ⚡:** Identifies <50 line daily performance wins, measures speed and memory overhead, aligns memory layouts (32-byte slab allocation), and records learnings in `.jules/bolt.md`.
- **Palette 🎨:** Enhances micro-UX, accessibility, ARIA labels, contrast, and terminal feedback, and records insights in `.jules/palette.md`.
- **Sentinel 🛡️:** Enforces defense-in-depth (KASLR, SMEP/SMAP, seccomp, pledge/unveil), mitigates vulnerabilities, and records learnings in `.jules/sentinel.md`.

### 3. Documentation Synchronization Policy
- Master improvement plans (`ImprovementPlan.md`) and next steps guidelines (`NEXT_STEPS_GUIDELINES.md`) must be kept synchronized across root (`./`) and all mirror directories (`docs/`, `wiki/`, `WIKI/`, `wiki_content/`, `wiki_repo/`).

### 4. Build & Test Verification Standard
- Always verify changes with `pytest tests/` and `cargo check --lib`.
- Standalone unit tests must be executed with `rustc --test --edition=2021 <path_to_file>` whenever modifying modular subsystems.

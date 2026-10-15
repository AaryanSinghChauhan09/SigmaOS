# SigmaOS Next Steps Guidelines & Operational Roadmap (`NEXT_STEPS_GUIDELINES.md`)

## Overview
This document provides clear operational guidelines, repository policies, and step-by-step procedures for future development on **SigmaOS** (Sovereign Operating System).

---

## 1. Direct Main-Branch Commit & Submission Policy
- **No External Pull Requests:** Changes must be committed directly to the `main` branch.
- **Proposal & Specification Documents:** Feature specifications and PR proposals should be saved as Markdown files in `docs/roadmap/` or `docs/` and synchronized across `wiki/` and `WIKI/` documentation mirrors.

---

## 2. Daily Workflow & Agent Roles

### ⚡ Bolt Agent (Performance)
1. **Profile:** Identify performance bottlenecks or dynamic memory re-allocation overheads.
2. **Optimize:** Implement small, low-risk optimizations (<50 lines) with clear performance comments.
3. **Verify:** Run `./run_sigma_tests.sh` to confirm zero regressions.
4. **Journal:** Document critical performance learnings in `.jules/bolt.md`.

### 🎨 Palette Agent (UX & Accessibility)
1. **Observe:** Inspect user interface components (`web_ui/`, `zenith_desktop.css`, `index.html`) for accessibility or visual polish gaps.
2. **Paint:** Add ARIA attributes, keyboard focus states, hover feedback, or loading spinners.
3. **Journal:** Record reusable UX/a11y insights in `.jules/palette.md`.

### 🛡️ Sentinel Agent (Security & Compliance)
1. **Scan:** Hunt for unvalidated user inputs, hardcoded credentials, or missing permission checks.
2. **Secure:** Apply defensive validation, input sanitization, or permission checks.
3. **Journal:** Log critical security findings and prevention patterns in `.jules/sentinel.md`.

---

## 3. Testing & Verification Requirements
- **Run Native Tests:** Run `./run_sigma_tests.sh` before finalizing any commit. All 137 unit test suites must pass cleanly.
- **Python Verification:** If touching installer or scripts, run `pytest tests/`.
- **Code Formatting:** Run `cargo fmt -- --check` to ensure formatting compliance.

---

## 4. Documentation Maintenance
- Maintain single-source-of-truth documentation in `docs/`.
- Ensure new architecture specifications are mirrored in `wiki/` and `WIKI/` directories.
- Un-ignore required markdown files in `.gitignore` if added to repository root using negation rules (e.g., `!ImprovementPlan.md`).

# 📖 SigmaOS Developer & AI Agent Next Steps Guidelines

Welcome to the **SigmaOS Developer & AI Agent Operational Guidelines**. This handbook provides actionable instructions for maintaining, testing, and expanding the SigmaOS repository in accordance with maintainer standards and the Tri-Agent Framework.

---

## 🎯 Primary Operational Directives

### 1. **Commit Policy & No-Unrequested-PR Rule**
- When executing tasks under direct maintenance instructions, commit updates to the main working branch.
- Do not create external Pull Requests unless explicitly requested by the maintainer.

### 2. **Zero Dependency Philosophy (`#![no_std]`)**
- Kernel code, system drivers, and core utilities must remain free of third-party external crates.
- Use core Rust and `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::boxed::Box`, `alloc::format`).

### 3. **Tri-Agent Framework Governance**
- **Bolt ⚡ (Performance):** Identifies focused performance optimizations (<50 lines). Journal key learnings in `.jules/bolt.md`.
- **Palette 🎨 (UX & A11y):** Implements intuitive visual and keyboard accessibility enhancements (<50 lines). Journal key learnings in `.jules/palette.md`.
- **Sentinel 🛡️ (Security & Hardening):** Fixes vulnerabilities and enforces hardware/memory boundary security (<50 lines). Journal key learnings in `.jules/sentinel.md`.

---

## 🛠️ Verification & Pre-Commit Workflow

Before completing any development step or committing changes:

1. **Verify Standalone Compilation:**
   ```bash
   rustc --edition=2021 --test <modified_file_path> --cfg 'feature="standalone_test"'
   ```

2. **Execute Core Test Suites:**
   ```bash
   ./run_sigma_tests.sh
   pytest tests/
   ```

3. **Check Code Formatting:**
   ```bash
   cargo fmt --check
   ```

4. **Synchronize Documentation Mirrors:**
   When updating documentation or master guidelines, copy updated files across:
   - `./` (Root)
   - `docs/`
   - `wiki/`
   - `WIKI/`
   - `wiki_repo/`

---

## 📋 Recommended Immediate Tasks

1. **Subsystem Warning Cleanup:** Prefix unused function parameters with `_` in `src/syscall/dispatcher.rs` and `src/functions/tuning.rs`.
2. **Driver Expansion Testing:** Extend standalone test coverage for network and storage drivers under `src/drivers/`.
3. **Tri-Agent Journal Tracking:** Maintain daily entries in `.jules/bolt.md`, `.jules/palette.md`, and `.jules/sentinel.md` following critical discoveries.

---
*For further architectural insights, see `README.md` and `ARCHITECTURE.md`.*

---

## AI Agent Maintenance Instructions

**Purpose:** This page provides operational directives and pre-commit workflow instructions for AI agents and human contributors working on SigmaOS.

**Maintenance Guidelines:**
1. **Update Frequency:** Update whenever the commit policy, dependency rules, or verification workflow changes.
2. **Operational Directives:** The "Primary Operational Directives" section must reflect the current governance model. When adding new agents or roles, update the Tri-Agent Framework Governance section with the agent's name, focus area, and journal file path.
3. **Verification Workflow:** The "Verification & Pre-Commit Workflow" section must list all required verification steps in the correct order. Update commands when the build system or test infrastructure changes.
4. **Zero Dependency Rule:** The "Zero Dependency Philosophy" section must be updated if the allowed dependency list changes. Currently only `core` and `alloc` are allowed.
5. **Immediate Tasks:** The "Recommended Immediate Tasks" section should be reviewed weekly. Move completed tasks to a "Completed Tasks" section at the bottom with completion dates.
6. **Mirror Synchronization:** The "Synchronize Documentation Mirrors" step must list all active mirror directories. Remove references to deleted mirrors (e.g., `WIKI/` and `wiki_repo/` after consolidation).
7. **Sync Requirement:** After updating this file, propagate changes to `WIKI/` and `wiki_repo/` mirrors (if they still exist) and update the GitHub Wiki page via `gh api`.

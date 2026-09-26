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

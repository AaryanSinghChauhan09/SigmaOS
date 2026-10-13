# Sovereign OS Next Steps Guidelines (`NEXT_STEPS_GUIDELINES.md`)

**Date:** October 13, 2025
**Repository:** [SigmaOS (AaryanSinghChauhan09/SigmaOS)](https://github.com/AaryanSinghChauhan09/SigmaOS/)
**Target Branch:** `main`
**Policy:** Direct main-branch commit policy. PR proposals and specifications are prepared as Markdown documents in `docs/` and synchronized across `wiki/`, `WIKI/`, and `wiki_repo/` mirrors rather than opening external GitHub PRs.

---

## 1. Operational Execution Guidelines

### Main-Branch Direct Execution
1. **Direct Commit Policy:**
   * All bug fixes, optimizations, documentation updates, and feature additions must be committed directly to the `main` branch.
   * PR proposal specifications (e.g., `PR_SPEC_*.md`) are written as Markdown documents inside `docs/roadmap/` and mirrored to `wiki/` and `WIKI/`.

2. **Pre-Commit Verification Invariant:**
   * Before committing, developers and agents must execute `./run_sigma_tests.sh` to ensure all native test modules and integration test suites pass with 0 failures.
   * Format code using `cargo fmt` where applicable to keep formatting clean.

---

## 2. Developer Onboarding & Workflow Standards

### Getting Started
1. **Toolchain Requirements:**
   * Rust stable toolchain (`rust-toolchain.toml`).
   * Standard build tools (`gcc`, `make`, `python3`, `bash`).

2. **Running Test Suites:**
   ```bash
   # Run all native Rust unit test suites and integration tests
   ./run_sigma_tests.sh

   # Run a specific standalone test file
   rustc --test tests/test_md_wiki_ideas_verification.rs --edition=2021 -o build/test_md_wiki_ideas_verification && ./build/test_md_wiki_ideas_verification
   ```

3. **Documentation Mirroring:**
   * When modifying or creating new documentation in `docs/`, synchronize copies to `wiki/` and `WIKI/` following the repository synchronization guidelines.

---

## 3. Agent Framework Principles & Tri-Agent Roles

* ⚡ **Bolt Agent (Performance):** Identifies micro-optimizations (<50 lines) that yield measurable speedups or eliminate dynamic memory allocations in hot execution loops. Maintains `.jules/bolt.md`.
* 🎨 **Palette Agent (UX & Accessibility):** Focuses on micro-UX enhancements, keyboard navigation, ARIA accessibility standards, and visual polish. Maintains `.jules/palette.md`.
* 🛡️ **Sentinel Agent (Security & Sandboxing):** Scans for security risks, input validation, memory safety, and compliance guardrails. Maintains `.jules/sentinel.md`.

---

## 4. Immediate Next Steps Roadmap

1. **Maintain Zero-Failure CI/CD Test Pipeline:**
   * Continuously run `./run_sigma_tests.sh` during all maintenance cycles.
2. **Refactor Repetitive Package Dispatchers:**
   * Apply macro-based dispatchers to reduce duplication in package adapter suites.
3. **Keep Wiki & Master Plan synchronized:**
   * Ensure master specifications in `docs/` remain mirrored across `wiki/` and `WIKI/`.

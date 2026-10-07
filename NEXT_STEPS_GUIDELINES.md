# SigmaOS Operational Next Steps Guidelines

## Executive Summary
This document outlines the operational execution guidelines, roadmap priorities, pre-commit workflows, and repository governance rules for developers and autonomous agents working on **SigmaOS**.

---

## Direct Main Branch Policy & PR Guidelines
1. **No External Pull Requests:** Changes must be committed directly to the `main` branch following thorough local verification and test suite pass confirmation.
2. **Pull Request Proposal Format:** Any aspirational features, distro innovations, or multi-repo absorption proposals must be prepared as structured Markdown documents in `docs/` and synchronized across `wiki/` and `WIKI/` mirrors.
3. **Branch Hygiene:** Feature work should be conducted in dedicated local/remote branches before direct merge into `main`. Stale remote branches must be regularly pruned.

---

## Operational Execution Guidelines

### 1. Code Modification & Testing Protocol
- **Always Test Before Commit:** Run `./run_sigma_tests.sh` to execute all 157+ native Rust unit test suites and verify zero test failures.
- **Python Integration Tests:** Run `pytest tests/` (or `python3 -m unittest discover tests`) to confirm Python script compatibility.
- **Compile Verification:** Run `cargo check --tests` to identify and resolve compiler warnings (dead code, unused variables, non-standard naming conventions).
- **Rustfmt & Formatting:** Maintain clean code style by executing `cargo fmt --check` prior to submission.

### 2. Tri-Agent Framework Workflows
- **Bolt ⚡ (Performance Agent):**
  - Identify micro-bottlenecks in hot paths (e.g., string searching, vector allocations).
  - Apply zero-copy slice iteration where possible.
  - Document learnings in `.jules/bolt.md`.
- **Palette 🎨 (UX & Accessibility Agent):**
  - Verify WCAG 2.1 AA accessibility (ARIA roles, focus rings, status feedback).
  - Ensure intuitive user feedback for asynchronous desktop/web operations.
  - Document learnings in `.jules/palette.md`.
- **Sentinel 🛡️ (Security Agent):**
  - Audit capability gates, syscall dispatchers, and memory buffer bounds.
  - Eliminate hardcoded secrets and unvetted unsafe code blocks.
  - Document learnings in `.jules/sentinel.md`.

---

## Pre-Commit Checklist & Verification Workflow
Before committing changes to the `main` branch, ensure the following steps are completed:
1. **Test Execution:** Verify `./run_sigma_tests.sh` finishes with `test result: ok` across all suites.
2. **Documentation Mirror Synchronization:** Ensure changes to core specifications in `docs/` are reflected in `wiki/` and `WIKI/` mirrors.
3. **Journal Updates:** Ensure `.jules/bolt.md`, `.jules/palette.md`, and `.jules/sentinel.md` are updated with critical findings if applicable.
4. **Clean Status:** Verify `git status` shows no stray build artifacts or temporary test files.

---

## Roadmap Priorities & Release Milestones
- **Phase 1 (Immediate):** Address dead-code compiler warnings in `src/wiki_unimplemented_ideas.rs` and `src/distro/wiki_ideas_implementation.rs`.
- **Phase 2 (Short-Term):** Expand UDF microarch package optimization rules across additional Linux and BSD distribution adapters.
- **Phase 3 (Medium-Term):** Standardize POSIX syscall dispatching and seL4 IPC performance benchmarks.
- **Phase 4 (Long-Term):** Complete full absorption of 500+ open-source repositories as specified in `docs/SIGMAOS_500_REPOS_TRI_AGENT_ABSORPTION_MASTER_PLAN.md`.

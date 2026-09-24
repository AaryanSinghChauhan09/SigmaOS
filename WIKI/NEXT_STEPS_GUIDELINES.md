# SigmaOS Next Steps Guidelines & Operational Execution Handbook

## Executive Mandate
This handbook outlines the mandatory operational standards, testing protocols, security guidelines, design principles, and synchronization rules for developers and autonomous agents working on **SigmaOS**.

All updates, bug fixes, features, and documentation enhancements must be committed **directly to the `main` branch**. Under no circumstances should pull requests (PRs) be created for codebase updates.

---

## 🛠️ Developer & Agent Operational Rules

### 1. Direct Commit Policy (Zero-PR Mandate)
- **Rule:** Never create a pull request (PR) for any changes in this repository.
- **Action:** Push changes directly to `main` after verifying that all tests pass and code quality checks are satisfied.

### 2. User Mode (Ring 3) & TSS Standard
- **Rule:** Enforce Ring 0 / Ring 3 privilege level transitions via `TaskStateSegment64` in `src/arch/cpu_sys.rs`. Ensure kernel stack pointers (`rsp0`) are updated before switching context to Ring 3 user mode.

### 3. Linux & BSD 50% Rule Governance Standard
- **Rule:** Implement and enforce the 50% resource threshold rule (`FiftyPercentRuleEngine` in `src/access/mod.rs`) across memory swap watermarks, CPU cgroup caps, page cache reclaim, overcommit limits, anonymous sessions, and process migration.

---

## 🔄 Mirror Directory Synchronization Standard

Whenever `ImprovementPlan.md` or `NEXT_STEPS_GUIDELINES.md` (or any core architecture documentation) is updated, it **MUST** be synchronously copied across all mirror locations in the repository:

1. `./` (Root directory)
2. `docs/`
3. `wiki/`
4. `WIKI/`
5. `wiki_content/`
6. `wiki_repo/`

---

## 🎯 Immediate Tactical Execution Checklist
- [x] Implement `TaskStateSegment64` and Ring 3 transition routines in `src/arch/cpu_sys.rs` (5 unit tests passing).
- [x] Enable and verify unit tests in `src/memory/segmentation_paging.rs` (3 unit tests passing).
- [x] Implement `FiftyPercentRuleEngine` and enable unit tests in `src/access/mod.rs` (8 unit tests passing).
- [x] Verify Python (`pytest tests/`) and Rust unit test suites (`./run_sigma_tests.sh`).
- [x] Synchronize master plans across all six mirror directories.
- [x] Execute pre-commit verification steps.
- [x] Commit and push directly to `main`.

# SigmaOS Next Steps Guidelines & Development Procedures 🚀

**Target Branch:** `main`
**Governance Policy:** Direct Main Branch Execution (No Pull Requests Required)

---

## Overview

This guide outlines the mandatory guidelines, testing standards, developer workflows, and execution protocols for contributing directly to the `main` branch of **SigmaOS**.

---

## 1. General Development Principles

1. **Direct Main Branch Execution:** All code improvements, bug fixes, performance optimizations, and documentation updates are committed directly to the `main` branch.
2. **Zero Test Regressions:** Never commit changes that break existing unit or integration tests. Run `./run_sigma_tests.sh` prior to every commit.
3. **No Artifact Bloat:** Build output binaries in `build/` or `target/` must remain ignored by `.gitignore`.
4. **Documentation Sync:** Essential root markdown files (`ImprovementPlan.md`, `NEXT_STEPS_GUIDELINES.md`, `.jules/*.md`) are explicitly unignored in `.gitignore` to allow direct main-branch tracking.

---

## 2. Agent Framework Standards (Bolt ⚡, Palette 🎨, Sentinel 🛡️)

### ⚡ Bolt (Performance Agent)
- Focus on small (<50 lines), high-impact performance enhancements.
- Measure and document performance impacts in `.jules/bolt.md`.
- Prioritize zero-copy allocations, bitwise routing, and lockless data structures.

### 🎨 Palette (UX & Accessibility Agent)
- Maintain high accessibility standards (WCAG 2.1 AA compliant focus states and ARIA labels).
- Document reusable UX patterns in `.jules/palette.md`.

### 🛡️ Sentinel (Security Agent)
- Enforce strict input validation, PQC signature checks, and UDF scriptlet sandboxing.
- Document security learnings in `.jules/sentinel.md`.

---

## 3. Step-by-Step Developer Workflow

```bash
# Step 1: Verify current main branch status
git status

# Step 2: Make required code, test, or documentation edits
# ... edit source files ...

# Step 3: Run the full test suite
./run_sigma_tests.sh

# Step 4: Verify git status to ensure untracked artifacts are clean
git status

# Step 5: Commit changes directly with standard descriptive commit message
git add .
git commit -m "feat(subsystem): describe implementation cleanly"
```

---

## 4. Code Quality & Refactoring Standards

- **Warning Cleanliness:** Run `cargo check --lib` and resolve any unused imports or unmutated variables.
- **OOP Design Patterns:** Apply Object-Oriented Principles (Encapsulation, Strategy, Factory, Observer, Abstraction) when refactoring complex procedural code blocks.
- **Error Handling:** Use explicit `Result` types instead of `unwrap()` or `panic!`.

---

## 5. Contact & Support

For issues, questions, or architectural discussions, consult `ImprovementPlan.md` or consult the project maintainers.

# 📖 SigmaOS Next Steps Guidelines & Operational Handbook

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Branch:** `main`
> **Status:** Active Operational Handbook & Developer/Agent Guidelines

---

## 🎯 Purpose of This Guide

This document provides developer and AI agent operational guidelines for working on the **SigmaOS** codebase. It outlines core workflows, testing commands, coding standards, tri-agent governance protocols (⚡ Bolt, 🎨 Palette, 🛡️ Sentinel), and step-by-step procedures for contributing directly on the `main` branch without creating unnecessary pull requests when instructed.

---

## 📋 1. Standard Developer & Agent Workflows

### 1.1 Local Environment & Verification Commands
Before committing any changes, developers and AI agents must execute the standard diagnostic and test verification commands:

```bash
# 1. Check Rust library compilation and warnings
cargo check --lib

# 2. Run Python integration test harness
pytest tests/

# 3. Run all standalone Rust subsystem test suites (137 tests)
./run_sigma_tests.sh
```

### 1.2 Direct Branch Policy & PR Controls
- **No PR Directive:** When instructed to work directly on the `main` branch, **do not create pull requests**.
- All commits should be atomic, well-formatted, and verified via `./run_sigma_tests.sh` prior to committing.

---

## ⚡ 2. Tri-Agent Governance Framework Guidelines

SigmaOS employs a continuous tri-agent governance framework. Each agent follows specific operational boundaries and maintains persistent journals under `.jules/`:

### 2.1 Bolt ⚡ (Performance & Optimization)
- **Goal:** Implement micro-optimizations (<50 lines) that make SigmaOS measurably faster.
- **Boundaries:** Measure before optimizing; do not sacrifice readability for micro-optimizations.
- **Journal File:** `.jules/bolt.md`

### 2.2 Palette 🎨 (UX & Accessibility)
- **Goal:** Enhance interface accessibility, keyboard focus states, ARIA roles, and contrast.
- **Boundaries:** Ensure full keyboard navigation support (Tab / Shift+Tab) and screen reader friendliness.
- **Journal File:** `.jules/palette.md`

### 2.3 Sentinel 🛡️ (Security & Compliance)
- **Goal:** Detect and resolve security risks, hardcoded secrets, input validation gaps, and permission flaws.
- **Boundaries:** Fail securely, sanitize inputs, and enforce least privilege.
- **Journal File:** `.jules/sentinel.md`

---

## 🏗️ 3. Architectural & OOP Principles in Rust

### 3.1 `#![no_std]` Kernel & Alloc Scoping
- Kernel modules placed under kernel boundaries must strictly observe `#![no_std]` rules.
- When `standalone_test` mode is enabled, ensure `alloc::*` or `core::*` types are imported properly.

### 3.2 Object-Oriented Design Patterns in Rust
- **Encapsulation:** Keep data fields private and expose controlled constructors (`new()`) and methods.
- **Polymorphism & Strategy Pattern:** Use Rust traits (`PackageFormatStrategy`, `UniversalPackageASTVisitor`) for modular component swapping.
- **Behavioral Patterns:** Utilize Mediators, Mementos, Visitors, and Caretakers (`UniversalDistroPackageMediator`, `SystemStateCaretaker`) to handle cross-subsystem messaging and state rollbacks without coupling.

---

## 🚀 4. Step-by-Step Task Checklist for Contributors

1. [ ] **Pull Latest Main:** Ensure local working tree is up to date on `main`.
2. [ ] **Diagnose First:** Run `cargo check --lib` and `./run_sigma_tests.sh` to confirm baseline health.
3. [ ] **Apply Changes:** Make targeted code, test, or documentation modifications.
4. [ ] **Verify Outcome:** Confirm that all unit tests pass with zero regressions.
5. [ ] **Update Documentation:** Update `ImprovementPlan.md` or domain spec docs if architecture changes.
6. [ ] **Pre-Commit Checks:** Complete pre-commit verification steps before submitting.

---

*End of SigmaOS Next Steps Guidelines & Operational Handbook.*

# 📖 SigmaOS Next Steps Guidelines & Operational Handbook

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Branch:** `main`
> **Status:** Active Operational Handbook & Guidelines

---

## 🎯 Purpose of This Guide

This document provides developer and AI agent operational guidelines for working on the **SigmaOS** codebase. It outlines core workflows, testing commands, coding standards, tri-agent governance protocols, and step-by-step procedures for contributing directly on the `main` branch without creating unnecessary pull requests when instructed.

---

## 📋 1. Standard Developer & Agent Workflows

### 1.1 Local Environment & Verification Commands
Before making or committing any changes, developers and agents must verify the environment using the following standard commands:

```bash
# 1. Verify Rust library code & check for warnings
cargo check --lib

# 2. Run Python integration test suite
pytest tests/

# 3. Run all standalone Rust subsystem test suites
./run_sigma_tests.sh
```

### 1.2 Direct Branch Policy & PR Controls
- When instructed to work directly on the `main` branch, **do not create pull requests**.
- All commits should be atomic, well-formatted, and verified via `./run_sigma_tests.sh` prior to committing.

---

## ⚡ 2. Tri-Agent Continuous Governance Guidelines

SigmaOS employs a continuous tri-agent governance framework (Bolt ⚡, Palette 🎨, Sentinel 🛡️). Each agent follows specific operational boundaries and maintains persistent journals under `.jules/`:

### 2.1 Bolt ⚡ (Performance & Optimization)
- **Goal:** Implement micro-optimizations (<50 lines) that make SigmaOS measurably faster.
- **Boundaries:** Measure before optimizing; do not sacrifice readability for micro-optimizations.
- **Journal File:** `.jules/bolt.md`

### 2.2 Palette 🎨 (UX & Accessibility)
- **Goal:** Enhance interface accessibility, keyboard focus states, ARIA roles, and contrast.
- **Boundaries:** Ensure full keyboard navigation support and screen reader friendliness.
- **Journal File:** `.jules/palette.md`

### 2.3 Sentinel 🛡️ (Security & Compliance)
- **Goal:** Detect and resolve security risks, hardcoded secrets, input validation gaps, and permission flaws.
- **Boundaries:** Fail securely, sanitize inputs, and enforce least privilege.
- **Journal File:** `.jules/sentinel.md`

---

## 🏗️ 3. Architectural & Coding Principles

### 3.1 `#![no_std]` Kernel & Alloc Scoping
- Kernel modules placed under kernel boundaries must strictly observe `#![no_std]` rules.
- When `standalone_test` or test mode is enabled, ensure `alloc::*` or `core::*` types are imported properly.

### 3.2 Object-Oriented Principles (OOP) in Rust
- **Encapsulation:** Keep data fields private and expose controlled constructors (`new()`) and methods.
- **Polymorphism & Strategy Pattern:** Use Rust traits (`PackageFormatStrategy`, `UniversalPackageASTVisitor`) for modular component swapping.
- **Behavioral Patterns:** Utilize Mediators, Mementos, and Visitors to handle cross-subsystem messaging and transactions without high coupling.

---

## 🚀 4. Step-by-Step Task Checklist for Contributors

1. [ ] **Pull Latest Main:** Ensure local working tree is up to date on `main`.
2. [ ] **Diagnose First:** Run `cargo check --lib` and `./run_sigma_tests.sh` to confirm baseline health.
3. [ ] **Apply Changes:** Make targeted code, test, or documentation modifications.
4. [ ] **Verify Outcome:** Confirm that all unit tests pass with zero regressions.
5. [ ] **Update Documentation:** Update `ImprovementPlan.md` or domain spec docs if architecture changes.
6. [ ] **Pre-Commit Checks:** Complete pre-commit verification steps before submitting.

---

*End of SigmaOS Next Steps Guidelines.*

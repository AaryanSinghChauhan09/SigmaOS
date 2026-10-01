# 📖 SigmaOS Next Steps Guidelines & Operational Handbook

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Branch:** `main`
> **Status:** Active Operational Handbook & Developer/Agent Guidelines

---

## 🎯 Purpose of This Guide

This document provides developer and AI agent operational guidelines for working on the **SigmaOS** codebase. It outlines core workflows, testing commands, coding standards, tri-agent governance protocols (⚡ Bolt, 🎨 Palette, 🛡️ Sentinel), and step-by-step procedures for contributing directly on the `main` branch without creating unnecessary pull requests when instructed.

---

## 📋 1. Standard Developer & Agent Verification Workflows

### 1.1 Local Environment & Verification Commands
Before committing any changes, developers and AI agents must execute the standard diagnostic and test verification commands:

```bash
# 1. Check Rust library compilation and warnings
cargo check --lib

# 2. Run Python integration test harness
pytest tests/

# 3. Run all standalone Rust subsystem test suites (137 tests)
./run_sigma_tests.sh

# 3. Run Python integration test suite
pytest tests/
```

### 1.2 Direct Branch Policy & PR Controls
- **No PR Directive:** When instructed to work directly on the `main` branch, **do not create pull requests**.
- All commits should be atomic, well-formatted, and verified via `./run_sigma_tests.sh` prior to committing.

---

## ⚡ 2. Tri-Agent Governance Framework Guidelines

SigmaOS employs a continuous tri-agent governance framework. Each agent follows specific operational boundaries and maintains persistent journals under `.jules/`:

### 2.1 Bolt ⚡ (Performance Agent)
- **Focus:** Micro-optimizations (<50 lines) targeting execution latency, memory allocation, and build speed.
- **Rule:** Measure before optimizing. Maintain `.jules/bolt.md` with critical performance learnings.

### 2.2 Palette 🎨 (UX & Accessibility)
- **Goal:** Enhance interface accessibility, keyboard focus states, ARIA roles, and contrast.
- **Boundaries:** Ensure full keyboard navigation support (Tab / Shift+Tab) and screen reader friendliness.
- **Journal File:** `.jules/palette.md`

### 2.3 Sentinel 🛡️ (Security Agent)
- **Focus:** Security hardening, CVE remediation, parameter sanitization, and input validation.
- **Rule:** Fail securely, sanitize inputs at subsystem boundaries, and maintain `.jules/sentinel.md`.

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

## 🚀 4. Step-by-Step Execution Checklist for Next Steps

1. [ ] **Clean Compiler Warnings:** Run `cargo fix --lib -p sigmaos --allow-dirty` to auto-clean unused import warnings.
2. [ ] **Decompose Large Modules:** Split monolithic files like `src/open_source_os_gap_closure.rs` into submodules.
3. [ ] **Maintain Document Parity:** Ensure `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` are mirrored across `docs/`, `wiki/`, and `WIKI/`.
4. [ ] **Verify Test Harnesses:** Execute `./run_sigma_tests.sh` and `pytest tests/` before finalizing any changes.

---

*End of SigmaOS Next Steps Guidelines & Operational Handbook.*

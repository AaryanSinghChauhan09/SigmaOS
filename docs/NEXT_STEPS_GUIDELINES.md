# 📖 SigmaOS Next Steps Guidelines & Operational Handbook

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Branch:** `main`
> **Status:** Active Operational Handbook & Developer Guidelines

---

## 🎯 Purpose of This Guide

This document serves as the operational handbook for developers and AI agents working on **SigmaOS**. It details daily workflows, testing commands, code quality guidelines, continuous tri-agent governance (Bolt ⚡, Palette 🎨, Sentinel 🛡️), and step-by-step procedures for contributing directly on the `main` branch.

---

## 📋 1. Standard Developer & Agent Verification Workflows

Before committing any changes, developers and agents MUST verify system stability using the following standard commands:

```bash
# 1. Verify Rust library compilation & check for warnings
cargo check --lib

# 2. Run all standalone Rust subsystem test suites
./run_sigma_tests.sh

# 3. Run Python integration test suite
pytest tests/
```

### 1.1 Direct Branch Policy (No PR Directives)
- When instructed to work directly on the `main` branch, **do not create pull requests**.
- Ensure all commits are atomic, self-contained, well-documented, and fully verified prior to pushing.

---

## ⚡ 2. Tri-Agent Governance Framework

SigmaOS utilizes a tri-agent governance model to ensure continuous optimization, user experience polish, and security hardening:

### 2.1 Bolt ⚡ (Performance Agent)
- **Focus:** Micro-optimizations (<50 lines) targeting execution latency, memory allocation, and build speed.
- **Rule:** Measure before optimizing. Maintain `.jules/bolt.md` with critical performance learnings.

### 2.2 Palette 🎨 (UX & Accessibility Agent)
- **Focus:** Micro-UX enhancements, ARIA accessibility attributes, contrast ratios, and keyboard navigation.
- **Rule:** Ensure WCAG 2.1 AAA compliance and full keyboard focus accessibility.

### 2.3 Sentinel 🛡️ (Security Agent)
- **Focus:** Security hardening, CVE remediation, parameter sanitization, and input validation.
- **Rule:** Fail securely, sanitize inputs at subsystem boundaries, and maintain `.jules/sentinel.md`.

---

## 🏛️ 3. OOP & Software Engineering Standards

### 3.1 Object-Oriented Principles in Rust
1. **Encapsulation:** Keep struct data private (`pub(crate)` or private) and expose constructors (`new()`) and accessor methods.
2. **Polymorphism:** Utilize traits (`PackageFormatStrategy`, `UniversalPackageASTVisitor`) to define behavioral interfaces.
3. **Abstraction:** Simplify complex subsystem operations behind unified engines (e.g., `SovereignCompilerToolchainEngine`).
4. **Design Patterns:** Apply Mediators for subsystem messaging, Mementos for transaction rollback, and Visitors for AST processing.

### 3.2 `#![no_std]` Kernel Scoping
- Kernel modules placed under kernel boundaries must adhere to `#![no_std]` constraints.
- Use `core::*` and `alloc::*` imports; avoid `std::*` in kernel space.

---

## 🚀 4. Step-by-Step Execution Checklist for Next Steps

1. [ ] **Clean Compiler Warnings:** Run `cargo fix --lib -p sigmaos --allow-dirty` to auto-clean unused import warnings.
2. [ ] **Decompose Large Modules:** Split monolithic files like `src/open_source_os_gap_closure.rs` into submodules.
3. [ ] **Maintain Document Parity:** Ensure `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` are mirrored across `docs/`, `wiki/`, and `WIKI/`.
4. [ ] **Verify Test Harnesses:** Execute `./run_sigma_tests.sh` and `pytest tests/` before finalizing any changes.

---

*End of SigmaOS Next Steps Guidelines.*

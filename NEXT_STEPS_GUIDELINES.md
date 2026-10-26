# SigmaOS Next Steps Guidelines & Development Roadmap

## Overview
This document provides actionable guidelines and step-by-step procedures for core contributors, developers, and autonomous AI agents (⚡ Bolt, 🎨 Palette, 🛡️ Sentinel) working on **SigmaOS**.

---

## 1. Development Guidelines

### Codebase Conventions
- **Rust Toolchain**: Use stable Rust channel as configured in `rust-toolchain.toml`.
- **No-Std Capability**: Core kernel and embedded modules (`src/kernel/`, `src/memory/`, `src/creative/`) must maintain `#![no_std]` compliance where required.
- **Safety**: Minimize `unsafe` blocks. Every `unsafe` block must be accompanied by a `// SAFETY:` explanatory comment.
- **Error Handling**: Use explicit `Result<T, E>` types. Avoid `.unwrap()` or `.expect()` in non-test production code.

### Commit & Direct-to-Main Guidelines
- When instructed to work directly on the `main` branch without pull requests:
  1. Always verify `.gitignore` explicit negations if adding root markdown or `.jules/` files.
  2. Run `./run_sigma_tests.sh` to ensure 100% test pass rate across all 760+ test cases.
  3. Format all Rust code using `cargo fmt`.
  4. Write descriptive, git-agnostic commit messages summarizing functional changes.

---

## 2. Tri-Agent Execution Guidelines

### ⚡ Bolt Agent (Performance)
- **Focus**: Latency reduction, zero-copy buffers, loop vectorization, allocation pruning.
- **Limit**: Keep daily optimizations focused (< 50 lines of code change where possible).
- **Journal**: Document unexpected bottlenecks or rejected optimizations in `.jules/bolt.md`.

### 🎨 Palette Agent (UX & Accessibility)
- **Focus**: Terminal prompt feedback, error suggestion clarity, ARIA attributes in web/UI components, keyboard shortcuts.
- **Limit**: Ensure visual and interaction changes build smoothly across Zenith Desktop and Shell.
- **Journal**: Document UI patterns and screen reader accessibility insights in `.jules/palette.md`.

### 🛡️ Sentinel Agent (Security)
- **Focus**: Sandbox isolation (Landlock, pledge/unveil), memory bounds checks, cryptographic attestation, compliance.
- **Limit**: Prioritize critical and high vulnerabilities over minor security cleanup.
- **Journal**: Document security edge cases and sandbox constraint discoveries in `.jules/sentinel.md`.

---

## 3. Immediate Next Steps Checklist

- [x] **Shell Maintenance Documentation**: Maintained `docs/shell/SHELL_MAINTENANCE.md` to satisfy shell test lifecycle runner.
- [x] **Agent Journals**: Created `.jules/bolt.md`, `.jules/palette.md`, and `.jules/sentinel.md`.
- [x] **Comprehensive Audit**: Published findings across 8 core domains in `ImprovementPlan.md`.
- [ ] **Static Mutable Refactoring**: Replace remaining `static mut` instances in `src/creative/` with thread-safe atomic or mutex containers.
- [ ] **SAT DPLL Fuzz Testing**: Add property-based fuzz tests to `tests/test_package_advancements_v74.rs`.
- [ ] **Release Gate Validation**: Run `./scripts/release_gate_mint_omarchy_migration.sh` to verify Mint/Omarchy ecosystem migration paths.

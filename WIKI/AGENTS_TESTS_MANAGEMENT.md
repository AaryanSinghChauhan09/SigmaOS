# SigmaOS AI Agent Tests Management Guidelines

## 1. Overview
SigmaOS incorporates automated test execution and continuous verification frameworks operated by AI testing agents (such as `TestExecutionGovernor`, `ModularPythonTestRunner`, `InspectionTestSuiteRunner`, and `StressFuzzTestMatrix`). These guidelines define native Rust unit testing, standalone `rustc --test` compilation rules, Python integration pytest matrices, algorithm/subsystem inspection suites, and stress/fuzzing matrices in SigmaOS.

## 2. Core Tests Management Principles

### 2.1 Native `./run_sigma_tests.sh` Master Test Runner
- **Master Test Runner**: All OS code modifications must pass `./run_sigma_tests.sh` without failures.
- **Suite Components**:
  1. Python integration test suite (`pytest tests/`).
  2. Standalone `rustc --test` binary builds (`build/test_cache`, `build/test_universal_adapter`, `build/test_unimplemented_features`, `build/test_unimplemented_tools`, `/tmp/test_gap`, `/tmp/test_wiki`, `/tmp/test_arch`, `/tmp/test_boot`, `/tmp/test_fedora`).
  3. UI/UX accessibility test scripts (`scripts/uiux_accessibility_test.sh`).
  4. Core Rust lib unit tests (`cargo test --lib`).
  5. Security input validation test suite (`build/input_val_test`).

### 2.2 Standalone `rustc --test` Rules & `klib` Imports
- **Explicit Imports**: Standalone test files compiled directly with `rustc --test` outside Cargo harnesses require explicit `extern crate core;` and `extern crate alloc;` directives at top of file.
- **HashMap Fallback**: Direct `rustc --test` files must include `#[cfg(test)] use std::collections::HashMap;` fallback aliases.

### 2.3 Modular Python & Inspection Test Matrices
- **Modular Pytest Suite**: `tests/test_unit_core.py`, `tests/test_integration_system.py`, and `tests/test_stress_fuzz_bench.py` validate system integration, universal package manager CLI simulations, and performance benchmarks.
- **Dedicated Inspection Suites**: `tests/algorithm_inspection_tests.rs` and `tests/linux_bsd_inspection_tests.rs` verify ML algorithms (K-Means, PCA, Local LLM), security policies (Unveil, SELinux), hypervisor loops (KVM vCPU, VirtIO), and Linux/BSD subsystem mechanisms.

---
*Maintained by the SigmaOS QA, Testing & Quality Assurance Steering Committee.*

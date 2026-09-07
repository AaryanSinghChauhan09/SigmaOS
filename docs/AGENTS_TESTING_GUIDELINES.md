# SigmaOS AI Agent Testing Guidelines & Verification Architecture

This document specifies mandatory testing procedures, standalone unit test compilation patterns, and verification standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Master Testing Philosophy & Zero-Regression Rule
- **Mandatory 100% Pass Rate**: Every code modification made by an AI agent must achieve a 100% pass rate across all Rust, Python, and C++ test suites before submission.
- **Proactive Testing**: AI agents must write inline unit tests for every newly added struct, enum, function, or module in `#![no_std]` or standard Rust crates.

## 2. Test Suite Execution Frameworks
### 2.1 Primary Master Test Runner (`./run_sigma_tests.sh`)
- Orchestrates the full verification pipeline:
  1. Rust unit tests across core kernel, HAL, packaging, and security modules.
  2. Security input validation test suite (IPv4/v6, path traversal, null byte, overflow checks).
  3. Python modular test suite (`tests/test_unit_core.py`, `tests/test_integration_system.py`, `tests/test_stress_fuzz_bench.py`).
  4. Universal package format adapter tests (`test_universal_adapter`).
  5. Unimplemented features and tools standalone tests.

### 2.2 Standalone Unit Test Compilation Pattern (`rustc --test`)
- To isolate and rapidly test specific modules without building the entire crate graph, AI agents should execute standalone tests:
  ```bash
  # Standalone test compilation for specific modules
  rustc --test --edition 2021 src/memory/pmm_vmm.rs -o build/test_pmm_vmm && ./build/test_pmm_vmm
  rustc --test --edition 2021 src/hal/multi_arch.rs -o build/test_multi_arch && ./build/test_multi_arch
  rustc --test --edition 2021 src/unimplemented_features.rs -o build/test_unimplemented_features && ./build/test_unimplemented_features
  rustc --test --edition 2021 src/unimplemented_tools.rs -o build/test_unimplemented_tools && ./build/test_unimplemented_tools
  ```

## 3. AI Agent Testing Directives
1. **Never Skip or Disable Failing Tests**:
   - AI agents must fix the underlying logic rather than commenting out or ignoring failing test assertions.
2. **Handle Standalone `#[cfg(test)]` Imports**:
   - Ensure module imports use `#[cfg(any(feature = "standalone_test", test))]` to support both `cargo test` and `rustc --test`.
3. **Validate Boundary Conditions**:
   - Test zero lengths, NULL pointer equivalents, maximum array/slice bounds, and integer overflow conditions.

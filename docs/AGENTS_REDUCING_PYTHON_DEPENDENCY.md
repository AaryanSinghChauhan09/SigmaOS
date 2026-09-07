# Sovereign AI Agent Python Dependency Reduction Specification

This document specifies mandatory rules, migration strategies, Rust-first toolchain directives, and zero-dependency runtime standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) tasked with reducing and eliminating Python runtime dependencies across the SigmaOS codebase (`src/tools/`, `scripts/`, `tests/`).

---

## 1. Rationale for Reducing Python Dependency

While Python is widely used for scripting and test harnesses, relying on an external Python interpreter within SigmaOS introduces several core challenges:

1. **Bare-Metal & Microkernel Incompatibility**:
   - The Safe-Rust microkernel, 12 core shards, and bootloader stages operate in `#![no_std]` environments without a heavy CPython interpreter or dynamic C library runtime (`libc.so.6`).
2. **Non-Deterministic Build & CI Environments**:
   - Python package dependencies (`pip`, `setuptools`, virtual environments) introduce non-reproducible build states, version drift, and slow CI pipeline execution.
3. **Execution Latency & Memory Footprint**:
   - Python startup overhead (~50–100ms) and high RAM usage (~30–50MB per process) violate the SigmaOS 3-second boot-to-web and sub-millisecond execution latency benchmarks.

---

## 2. Core Python Reduction Directives

1. **Native Safe-Rust Tooling First**:
   - All system utilities, benchmark generators, package transpilers, and test drivers must be written in zero-dependency Safe-Rust (`src/tools/`, `src/klib/`) or POSIX-compliant POSIX shell scripts (`scripts/*.sh`).
2. **Replacing Python Test Suites**:
   - Legacy Python test runners (`tests/*.py`) must be replaced by native Rust unit and integration test runners compiled via `rustc --test` or integrated into `./run_sigma_tests.sh`.
3. **Replacing Python Build Automation**:
   - Python-based build drivers (`scripts/merge_all_branches.py`, `scripts/generate-benchmark-report.py`) must be ported to standalone Rust binary tools (`src/tools/`) or pure POSIX shell utilities.
4. **Wasm / WebAssembly for Dynamic Scripts**:
   - Where dynamic scripting is required within userland or browser-shell PWAs, agents must utilize WebAssembly (Wasm) micro-runtimes or embedded Wasm components rather than invoking Python host binaries.

---

## 3. Migration Roadmap & Conversion Guidelines

| Python Tool / Script | Replacement Strategy | Native Target Component |
| :--- | :--- | :--- |
| `scripts/merge_all_branches.py` | Port to standalone Rust binary tool or shell script | `src/tools/git_merge_engine.rs` / `scripts/sync_wiki.sh` |
| `scripts/generate-benchmark-report.py` | Native Rust benchmark reporter | `PhoronixTestSuiteRunner` in `src/unimplemented_features.rs` |
| `tests/test_unit_core.py` | Rust standalone unit test runner | `rustc --test` / `./run_sigma_tests.sh` |
| `tests/test_stress_fuzz_bench.py` | Rust cargo-fuzz & native fuzzing harness | `src/testing/fuzzing.rs` |

---

## 4. AI Agent Python Reduction Directives Summary

1. **Prohibit New Python Additions**: Do not create new `.py` scripts or introduce Python runtime requirements into core build pipelines.
2. **Prioritize Native Rust Binaries**: Implement system tools as zero-dependency Rust executables in `src/tools/`.
3. **Keep CI Pipelines Hermetic**: Ensure all testing and build verification commands run natively using `rustc` and POSIX shell scripts without requiring `python3`.

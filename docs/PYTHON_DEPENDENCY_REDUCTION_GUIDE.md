# SigmaOS — Python Dependency Elimination & Native Rust Architecture Guide

This document outlines the architectural strategy and technical directives for eliminating Python runtime dependencies in **SigmaOS**.

---

## 🚀 Why Eliminate Python Runtime Dependencies?

1. **Eliminate Runtime Initialization Overhead**:
   A standard Python 3 interpreter startup incurs 30–300 ms latency and 15–30 MB baseline RSS RAM footprint.
2. **Prevent Version Incompatibilities**:
   Eliminates script breakages across system Python upgrades (`python3.10` vs `3.12`).
3. **Enhance Security & Capability Sandboxing**:
   Python interpreters execute with ambient root privileges. Native Rust tools enforce strict OpenBSD `pledge()` and `unveil()` path restrictions.
4. **Enable Deterministic Hermetic Builds**:
   Guarantees reproducible compilation outputs without external Python pip/wheels package pollution.

---

## 🛠️ Native Rust Alternatives in SigmaOS

| Legacy Python Tool | Native Rust Replacement in SigmaOS | Module Location |
| :--- | :--- | :--- |
| `scikit-learn` / `numpy` | `SovereignDataTransformer` / `SovereignPipelineEstimator` | `src/ml/sovereign_data_workspace.rs` |
| `pandas` DataFrames | `SovereignDataframeExporter` (Apache Arrow / CSV) | `src/ml/sovereign_data_workspace.rs` |
| `neofetch` / `fastfetch` | `FastfetchSystemInfoEngine` | `src/tools/powertoys.rs` |
| `pytest` / Test Runners | `cargo test --lib` & `rustc --test` | `src/testing/` |
| Python Build Scripts | `DeveloperToolkitConverter::convert_python_to_rust` | `src/distro/transformation_engine.rs` |
| Python Package Managers (`pip`, `poetry`) | `sigpkg` Universal Format Adapter | `src/sigpkg/universal_adapter.rs` |

---

## 💡 AI Agent Operational Guidelines

AI agents interacting with the SigmaOS codebase MUST follow these rules:
1. **Never Spawning `python3` Subprocesses**: Use native `sigma` subcommands or Rust utilities.
2. **Hermetic Test Execution**: Run unit tests directly via `cargo test` or `rustc --test`.
3. **Automated Python Transpilation**: Pass legacy Python utility scripts through `DeveloperToolkitConverter::convert_python_to_rust()`.

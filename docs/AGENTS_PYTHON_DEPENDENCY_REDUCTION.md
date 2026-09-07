# AI Agent Guidelines for Python Language Dependency Reduction in SigmaOS

## 1. Overview
SigmaOS enforces a strict policy reducing external Python runtime dependencies in favor of native Rust tools (`src/tools/sigma_cli.rs`), zero-allocation AI data science modules (`src/ai/`), and native Rust inspection test suites (`tests/algorithm_inspection_tests.rs`).

## 2. Guidelines for AI Agents

### 2.1 Native Rust System Tools
- **Avoid External Python Scripts**: AI agents executing system tasks must invoke `sigma` subcommands (`sigma build`, `sigma run`, `sigma attest`, `sigma publish`) rather than calling `python3 scripts/*.py`.
- **WASM Fast-Paths**: System automation tasks run via WASM hostcalls inside `sigma_cli` with zero python interpreter startup overhead.

### 2.2 Embedded Data Science & Local AI
- **Rust ML Primitives**: Data analysis tasks utilize native zero-allocation Rust algorithms (`KMeansClustering`, `PrincipalComponentAnalysis` in `src/ai/sigma_data.rs`) and embedded GGUF local LLMs (`src/ai/local_llm.rs`).

### 2.3 Rust Inspection Test Harnesses
- **Pure Rust Testing**: Testing agents prioritize native Rust test harnesses (`cargo test --lib`, `cargo test --tests`, `./run_sigma_tests.sh`) over Python pytest runners.

---
*Maintained by the SigmaOS Architecture Steering Committee.*

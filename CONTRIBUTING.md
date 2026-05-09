# Contributing to SigmaOS

Join the meritocratic Sovereign Council and help build the future of computing.

## 🏁 Coding Standards
*   **Language**: C++20 for core shards, Rust for industrial tooling.
*   **Patterns**: OOP Singleton for shard state, RAII for resource management.
*   **Linter**: Must pass `clang-tidy` and `reproducible_build.ps1` validation.

## 🔄 PR Workflow
1.  **Fork & Branch**: Create a feature branch (e.g., `feat/new-driver`).
2.  **Lint & Test**: Run `tests/run_all.ps1`.
3.  **Submit**: Open a PR with a detailed shard manifest entry.
4.  **Audit**: Pass security review and IP audit (`SovereignIPAuditor`).

## ⚖️ Code of Conduct
Respect, Transparency, and Sovereignty. No harassment or proprietary "dark" shards allowed.

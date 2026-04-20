# Contributing to Σ SigmaOS

We welcome contributions to the Sovereign Lattice! As a project focused on low-level purity and high-level snappiness, we follow these strict guidelines.

## 🔒 Purity First
1. **Zero-Dependency**: Every shard must be written in pure C11 or Assembly. No standard HLL libraries (stdio, stdlib, etc.) are permitted inside the kernel suites.
2. **Namespace Sovereignty**: Do not pollute the global namespace. Use suite-prefixed symbols (e.g., `s01_shard_init`).
3. **Modular Sharding**: Every feature must be its own shard within a suite.

## 🛠 Submission Process
1. **Fork & Branch**: Create a feature branch (e.g., `feature/pwa-support`).
2. **Lint & Test**: Ensure `make lint` and `make diagnostics` pass on your machine.
3. **Document**: Every new shard requires a doc block (Competitive USPs Absorbed) for the `sovereign_wiki_builder`.
4. **Pull Request**: Use our [PR Template](.github/PULL_REQUEST_TEMPLATE.md).

## 🏢 Formatting
- Use 4-space indentation.
- Follow the directory structure: `kernel/suites/S[NUM]_[NAME]/shards/[FILE].c`.

---
*Thank you for helping us sculpt the future of silicon.*

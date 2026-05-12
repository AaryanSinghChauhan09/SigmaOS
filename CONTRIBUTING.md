# Contributing to SigmaOS

Thank you for your interest in contributing to the Sovereign Lattice! As an industrial-grade project, we maintain high standards for code quality, modularity, and security.

## 🏛 Shard-First Architecture
SigmaOS is built as a collection of **shards**. Every new feature should be implemented as an isolated shard with:
1.  **Zero External Dependencies**: Use only `SovereignLibC` and `SigmaOOP` primitives.
2.  **Strict Namespacing**: Follow the `SigmaOS::Kernel::<Subsystem>` pattern.
3.  **C-Bridge Integration**: Expose functions via `extern "C"` for kernel/userland interoperability.

## 🛠 Development Workflow
1.  **Fork & Clone**: Pull the latest `main` branch.
2.  **Modularize**: Ensure your changes don't tightly couple shards.
3.  **Audit**: Run `sigma-cli pqc-audit` and `forensic-scan` if applicable.
4.  **Lint**: Ensure Markdown files follow the repo standards (no MD025, MD060).
5.  **PR**: Submit your PR with a clear explanation of the "Industrial USP" your change provides.

## 📝 Coding Style
- **Naming**: `CamelCase` for classes, `snake_case` for functions and variables.
- **Macros**: Prefix with `SIGMA_` or `LOG_`.
- **Comments**: Every shard must have a header comment explaining its Purpose, USP, and Features.

## 🛡 Security Requirements
- All data-handling logic must consider Post-Quantum Cryptography (PQC) attestation.
- Avoid raw pointers where possible; use the `SigmaObject` lifecycle.
- Zero buffer overflows: use `sigma_strncpy` and `sigma_memcpy` with explicit bounds.

---
*By contributing, you agree to license your work under the Sovereign Lattice License.*

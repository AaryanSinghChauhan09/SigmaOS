# 🔄 SigmaOS Release Governance and Automated CI/CD Specification

This document defines the release engineering cadence, versioning discipline, and automated regression testing pipeline for **SigmaOS**.

***

## 🏗️ 1. Rolling Release Cadence (The Sovereign Channel)

SigmaOS adopts a strict **rolling-release model** inspired by Arch Linux:

*   **Core Kernel & Submodules:** Updated continuously under semver `major.minor.patch` versioning discipline.
*   **`Pacman` and `makepkg` Engine:** Backed by source-compilable recipes allowing community developers to pull and build the newest rolling releases with one unified command.
*   **Reproducible ISO Builds:** Signed using cryptographic PKI verifiers (`CryptoVerifier` in `src/sigpkg/verifier.rs`), ensuring 100% binary reproducibility.

***

## 🧪 2. Regression Testing Framework (CI/CD)

Every code commit triggers the automated regression testing pipeline:

1.  **Compilation Check:** Compiles targets cleanly on Hosted/CI environments.
2.  **Standard Linting (`clippy`):** Runs `cargo clippy --all-targets -- -D warnings` to enforce zero warnings.
3.  **Automated Unit Testing:** Executes the entire 202 test suite checking memory, IPC, paging, and shell REPL correctness.
4.  **Smoke-Test Automation (`scripts/smoke-test.sh`):** Executes binary checks, filesystem hierarchy validations, and code formatting checks (`cargo fmt --check`).

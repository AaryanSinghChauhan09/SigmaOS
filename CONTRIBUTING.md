# Contributing to SigmaOS

We welcome contributions to SigmaOS! Please review the guidelines below.

## 1. Development Setup
- **Rust Toolchain**: Use the `nightly` Rust compiler (required for `#![no_std]` features).
- **QEMU**: Used for system emulation and debugging.
- Run `cargo build` and `cargo test` before submitting changes.

## 2. Code Style
- Use `rustfmt` standard guidelines (`cargo fmt`).
- Avoid `unsafe` unless absolutely necessary for hardware interaction. Document all `unsafe` blocks with safety invariants.

## 3. Testing Requirements
- New features must include unit tests.
- Kernel subsystems and drivers must be tested in `integration_test.rs`.

## 4. Branch Naming Conventions
- We use trunk-based development. All changes are merged directly into `main`.
- Please ensure your PR is rebased against the latest `main`.

## 5. Pull Request Process
1. Fork the repository and create a branch.
2. Implement your feature and write tests.
3. Open a Pull Request using the provided PR template.
4. Pass all CI checks (cargo check, clippy, fmt, and tests).
5. A maintainer will review and merge.

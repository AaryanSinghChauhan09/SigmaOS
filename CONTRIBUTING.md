# Contributing to SigmaOS

SigmaOS follows an Ubuntu-inspired governance model, but with strict technical constraints to guarantee sovereignty. 

## Technical Mandates

1. **Rust Only**: C and C++ are strictly prohibited in SigmaOS. All components (kernel, drivers, shell, UI, apps) must be written in Rust.
2. **`no_std` and `no_builtins`**: All code must compile in a `#![no_std]` environment. You must not rely on `std`, `libc`, POSIX, or any external standard libraries.
3. **No Third-Party Dependencies**: No external crates or third-party submodules are allowed. If you need a feature (e.g., parsing, cryptography, specific data structures), you must implement it from scratch in the SigmaOS repository using our native Rust SDKs.
4. **Object-Oriented Architecture**: Use Rust Structs and Traits to mirror OOP principles (encapsulation, abstraction, polymorphism).

## Contribution Workflow

1. **Fork & Branch**: Create a feature branch off `main`.
2. **Format**: Run `cargo fmt` to adhere to standard Rust formatting.
3. **Lint**: Ensure `cargo clippy --target x86_64-unknown-none` passes with no warnings (`-D warnings`).
4. **Testing**: All PRs must maintain 100% existing test pass rate (`cargo test`).
5. **Review**: 2 core maintainer approvals required for kernel changes.
6. **CI**: All 9 CI jobs (3 profiles × 3 targets) must pass green.

## Licensing
All contributions must include the `// SPDX-License-Identifier: GPL-2.0-or-later` header.

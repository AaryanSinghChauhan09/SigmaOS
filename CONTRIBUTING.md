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

## Documentation Guidelines

**Before creating a new doc, check [docs/README.md](docs/README.md) for the canonical file list.**

- New documentation → `docs/<Name>.md`
- New wiki page → `wiki_repo/<Name>.md`
- Do not create duplicates of the canonical files in the table.
- If you want to add a competitive analysis note → edit `docs/Competitive_Analysis.md`.
- If you want to add a development idea → add to `docs/IDEAS_1000.md`.
- If you studied an OSS project for cleanroom reference → add to `docs/OSS_Reference_Map.md`.

Doc sprawl hurts contributor onboarding and trust. One canonical file per topic.

## Cleanroom Rule

SigmaOS is MIT/BSD licensed. When drawing inspiration from GPL projects (Linux, Mesa, etc.):

1. **Never copy GPL source code** into any SigmaOS file.
2. Study architecture, interfaces, and patterns only.
3. Document what you studied in `docs/OSS_Reference_Map.md`.
4. Implement independently, without referencing GPL source during coding.

See [docs/License_Map.md](docs/License_Map.md) and
[wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md](wiki_repo/CANONICAL_CLEANROOM_ABSORPTION.md).

## SPDX Headers

Every source file must begin with:

```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
```

Missing headers fail `make check-spdx` in CI.

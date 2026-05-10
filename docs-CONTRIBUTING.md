# Contribution Guide

## Coding Standards

<<<<<<< HEAD

- **C/C++**: We enforce strict `clang-format` compliance and zero-warnings via `clang-tidy`. OOP patterns must be maintained.
- **No Global State**: All drivers and services must be encapsulated inside Sovereign classes.
=======

## 2. Technical Requirements

* **Language Proficiency**: We utilize C++11/14 for core shards. Strict OOP Singleton patterns are mandatory.
* **Zero-Trust Encapsulation**: Global state is prohibited. All services must be encapsulated within Sovereign classes.
* **Header-First Design**: Declare interfaces in `.hpp` before implementation in `.cpp`.
* **Linting & Hygiene**: Zero-warning policy. All code must pass `clang-tidy` and `markdownlint`.

## 3. Workflow & Orchestration

* **Branching**: Use `feature/shard-name` for new components and `fix/remediation` for technical debt.
* **Commit Messages**: Follow Semantic Commits (e.g., `feat(S100): add Vulkan silicon probe`).
* **Validation**: Every contribution must compile via the `reproducible_build.ps1` and pass QEMU sanity checks.
>>>>>>> a7188b091 (Î£ SigmaOS: Cleaning repository by migrating documentation shards to wiki for industrial parity.)

- **Linting**: Ensure all Markdown files comply with standard Markdown linting rules.

## Branching Strategy

- `main`: The stable core. Do not push directly to `main`.
- `feature/*`: For new additions.

- `fix/*`: For bug fixes and refactors.

## CI/CD Pipeline

- **Automated Tests**: All PRs must pass `make test` before merging.
- **Commit Messages**: Use semantic commits (`feat:`, `fix:`, `docs:`).

## How to Submit

1. Fork the repo and create a branch.
2. Implement your changes.

3. Run `make rebuild` and ensure it compiles via QEMU.
4. Submit a Pull Request.

# Contribution Guide

## Coding Standards


* **C/C++**: We enforce strict `clang-format` compliance and zero-warnings via `clang-tidy`. OOP patterns must be maintained.
* **No Global State**: All drivers and services must be encapsulated inside Sovereign classes.

* **Linting**: Ensure all Markdown files comply with standard Markdown linting rules.

## Branching Strategy


* `main`: The stable core. Do not push directly to `main`.
* `feature/*`: For new additions.

* `fix/*`: For bug fixes and refactors.

## CI/CD Pipeline


* **Automated Tests**: All PRs must pass `make test` before merging.
* **Commit Messages**: Use semantic commits (`feat:`, `fix:`, `docs:`).

## How to Submit


1. Fork the repo and create a branch.
2. Implement your changes.

3. Run `make rebuild` and ensure it compiles via QEMU.
4. Submit a Pull Request.

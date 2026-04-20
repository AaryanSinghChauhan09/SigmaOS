# Contributing to SigmaOS

First off, thank you for considering contributing to SigmaOS! It's people like you that make SigmaOS such a great sovereign OS.

## Developer Quickstart

1. **Setup Toolchain:** Ensure you have native C11 and Assembly compilers.
   ```bash
   make toolchain
   ```
2. **Build the Kernel:**
   ```bash
   make kernel
   ```
3. **Run Sandbox Tests:**
   ```bash
   make verify
   ```

## Development Guidelines

- **Strict C11/Assembly:** SigmaOS is 100% dependency-free. Do not introduce Python, JS, or external standard libraries into the core kernel lattice (`S01` to `S12`).
- **Sovereign Object-Oriented Foundation (SOOF):** All new kernel data structures MUST inherit from `sigma_obj_t` for uniform tracking and lifecycle management.
- **Architectural Purity:** Avoid global state unless strictly defined under `SovereignCommon.h`. Use the `sigma_` prefix for all types and APIs.

## Pull Request Process

1. Fork the repo and create your branch from `main`.
2. Write unit tests for your subsystem.
3. If you've modified APIs or architectural structures, update the Wiki.
4. Ensure the test suite passes (`make test` / CI).
5. Ensure your code passes Clang-Tidy and any linting checks.
6. Submit a PR filling out the provided PR template.

## Bug Reports

Please use the provided GitHub Issue Template for bugs. Include logs, your platform, and a minimum reproducible example.

# Contributing to SigmaOS

Thank you for your interest in contributing to the SigmaOS Sovereign Lattice!

## 🛡️ Core Development Philosophy
1. **Zero-Dependency:** Do not introduce external libraries or dependencies.
2. **Silicon-Native:** All drivers must communicate directly with S-HAL.
3. **Type-Safety First:** Use strict typing and avoid unsafe memory operations.

## 🌿 Branching Strategy
- `main`: The stable, production-ready zenith release.
- `lattice-dev`: The active development branch.
- Feature branches should be named `feature/<shard-name>` or `fix/<component-name>`.

## 🛠️ Submitting a Pull Request
1. Fork the repository and create your feature branch.
2. Ensure your code strictly adheres to the 600-shard modular architecture.
3. Run the automated linting and IDE checks.
4. Submit a PR against `lattice-dev` and tag a maintainer.

## 🐞 Reporting Bugs
If you encounter an "off" issue or a kernel-level bug:
1. **Log it:** Open a GitHub Issue with the label `bug`.
2. **Reproduce:** Provide clear steps to reproduce the issue (e.g., QEMU command and boot logs).
3. **Trace:** Include serial output logs from Step 3 debugging.

We label beginner-friendly issues as `good first issue` and `help wanted`.

# Contributing to SigmaOS

We welcome contributions to SigmaOS, the post-quantum, sovereign, microkernel-based operating system.

## 1. Coding Style (C/C++)

* **Indentation**: 4 spaces, no tabs.
* **Brace Placement**: K&R style. Open brace on the same line.
* **Naming Conventions**:
  * Classes/Namespaces: `PascalCase` (e.g., `SovereignBootEngine`)
  * Functions/Methods: `camelCase` or `snake_case` (e.g., `init()`, `probe_bus()`)
  * Variables/Fields: `snake_case` with `m_` prefix for member variables (e.g., `m_device_count`)
* **Headers**: Use standard header guards `#ifndef SIGMA_...` and include paths relative to the file.

## 2. Patch Submission Process

1. Fork the repository and create a feature branch (`git checkout -b feature/your-feature-name`).
2. Write clear, descriptive commit messages.
3. Ensure all code conforms to the zero-dependency, silicon-direct architecture (no standard library usage in the kernel).
4. Run all local regression tests and ensure the build succeeds.
5. Submit a Pull Request (PR) to the `main` branch.

## 3. Code Review

* All PRs must pass automated CI/CD checks.
* At least one core maintainer must approve the PR before it can be merged.
* Security-related patches (especially in S-ARMOR or S-CRYPT) require rigorous review and cryptographic attestation.

## 4. Module API

When adding new kernel modules, ensure they inherit from `SigmaOS::SigmaObject` and utilize the `SigmaSingleton` pattern where global access is required. Use hardware-direct APIs and avoid legacy OS abstractions.

Thank you for contributing to the future of sovereign computing!

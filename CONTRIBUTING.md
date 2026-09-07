# 🤝 Contributing to SigmaOS

Thank you for contributing to **SigmaOS**! This document provides task guidelines inspired by Linux & BSD distribution contributor standards.

---

## Task Guidelines & Contribution Rules

### 1. **Branch Naming & Workflow**
- All git branches MUST start with `jules-` (e.g. `jules-feature-scheduler`, `jules-fix-pam`).
- Keep commits granular, logical, and focused on single task objectives.

### 2. **Distro Parity & Zero Dependencies**
- All kernel and userspace components must maintain strict `#![no_std]` zero external dependency design.
- Contributions taking inspiration from Linux & BSD distributions (Arch Linux ALPM, Debian sbuild, Fedora DNF, FreeBSD Ports, OpenBSD Pledge/Unveil, NixOS Flakes) must include unit tests.

### 3. **Testing & Code Review**
- Run standalone tests on modified files (`rustc --edition=2021 --test <file>`).
- Execute `./run_sigma_tests.sh` to ensure all 13 native test stages pass cleanly.
- Pull requests require double maintainer code review before merging.

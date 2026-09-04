# Contributing to SigmaOS

Thank you for your interest in contributing to **SigmaOS**! SigmaOS is a from-scratch, zero-dependency, zero-trust, bare-metal operating system. We welcome contributions from developers, researchers, and open-source enthusiasts.

---

## 📜 Principles & Standards

SigmaOS enforces strict engineering standards inspired by Arch Linux, FreeBSD, and OpenBSD:

1. **Zero External Unverified Dependencies**: Core kernel and userspace modules are written in standard-library-free (`#![no_std]`), memory-safe Rust with explicit capability bounds.
2. **PQC Cryptographic Signing**: All driver modules, package recipes, and security advisories must be signed using post-quantum Kyber-1024 or Dilithium-5 signatures.
3. **Capability-Gated Isolation**: Every new feature or driver shard must run in userland under `pledge` and `unveil` sandboxing primitives.
4. **Comprehensive Verification**: All pull requests must pass the atomic test suite (`./run_sigma_tests.sh`) and pass quality gates (`./scripts/sigma_quality_check.sh`).

---

## 📑 Contributor Guidelines Outline

### 🔹 1. Code of Conduct
- Maintain respectful, professional, and inclusive communication across issues, pull requests, and GitHub Discussions.
- Zero tolerance for harassment, discrimination, or abusive behavior.
- Assume good intent, focus on pragmatic technical solutions, and adhere to open-source governance rules.

### 🔹 2. Getting Started
- **Fork & Clone**: Fork the repository on GitHub and clone your fork locally.
- **Toolchain Setup**: Ensure Rust nightly with `x86_64-unknown-none` target support is installed.
- **Build & Test**: Run `cargo build` and `./run_sigma_tests.sh` to confirm baseline stability before making changes.
- **Explore Architecture**: Review `ARCHITECTURE*.md`, `FUTURE-DEVELOPMENT-ROADMAP.md`, and `docs/MAINTAINERS.md`.

### 🔹 3. Contribution Workflow
1. **Issue First**: Open an issue or comment on an existing issue to discuss proposed changes before starting large implementations.
2. **Feature Branches**: Create isolated feature branches (`feature/<short-description>`).
3. **Developer Certificate of Origin (DCO)**: Include a `Signed-off-by: Name <email>` line in all commit messages.
4. **Pull Requests**: Open a PR against `main` with a clear description, linked issues, and verification logs.

### 🔹 4. Coding Standards
- **Formatting & Linting**: Format code with `cargo fmt` and check for warnings with `cargo clippy`.
- **Bare-Metal Restrictions**: Strictly avoid `std::` primitives in core kernel/driver layers.
- **Documentation**: Document all public structs, traits, enums, and functions using Rustdoc comments (`///`).
- **Security First**: Avoid `unsafe` code blocks unless strictly necessary for raw hardware register/DMA memory access, and document all safety invariants.

### 🔹 5. Core Areas to Contribute
- **Kernel / Systems**: Microkernel scheduler, SovereignVMM demand paging, IPC mechanisms, and syscalls.
- **Device Drivers**: Storage (NVMe), USB (xHCI), Networking (E1000/RTL8139), and KMS/DRM framebuffer stubs.
- **Userland & Package Management**: `sigma_pkg` multi-distro adapters, universal CLI utilities, and shell REPL.
- **Zenith Desktop**: Tiling window manager, bare-metal compositor, and declarative theme engines.
- **Documentation**: Man pages (`docs/man/`), wiki updates, API references, and architecture guides.

### 🔹 6. Community Engagement & Governance
- Join **GitHub Discussions** for technical RFCs and feature brainstorming.
- Participate in community development sprints and hackathons.
- Help review pull requests, triage issues, and mentor new contributors.

---

## 🛡️ Security Disclosures

Please report security vulnerabilities directly to the security team following `.github/SECURITY.md`.

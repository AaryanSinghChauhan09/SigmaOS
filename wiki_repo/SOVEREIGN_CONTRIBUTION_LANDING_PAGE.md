# SOVEREIGN CONTRIBUTION LANDING PAGE & DEVELOPER MASTER MANUAL

Welcome to the **SigmaOS Sovereign Contribution Landing Page**. This master manual provides open-source contributors, kernel engineers, package maintainers, and security researchers with complete guidance for contributing to the SigmaOS ecosystem, inspired by the contributor portals of Arch Linux, FreeBSD, Debian, Gentoo, and OpenBSD.

***

## 🚀 Quick Navigation & Developer Portals

| Portal / Module | Scope & Objectives | Inspiration Source |
| :--- | :--- | :--- |
| **Kernel & Driver Subsystem** | `#![no_std]` Rust kernel, interactive hybrid schedulers, zero-copy IPC | Linux Kernel & FreeBSD ULE |
| **Universal Package Manager (`SigmaPkg`)** | `pacman-contrib`, AUR integration, PKGBUILD security audits, `.deb`/`.rpm`/`.apk` translation | Arch Pacman, FreeBSD Pkg & Portage |
| **Security & Verification** | OpenBSD signify, PQC Dilithium-5, Linux `Signed-off-by` DCO enforcement | OpenBSD Signify & Linux DCO |
| **Zenith Desktop Compositor** | Adaptive visual overlays, bare-metal compositor synthesis | Cinnamon, KDE Plasma & Wayland |
| **Supreme Court Governance** | Judicial review of architectural changes, dispute resolution, statutory compliance | Debian Constitution & Apache Foundation |

***

## 📜 Developer Certificate of Origin (DCO) & Commit Signoff

SigmaOS strictly enforces the Developer Certificate of Origin (DCO) for all commits across kernel, userland, and documentation modules.

### DCO Statement

By making a contribution to this project, I certify that:

1.  The contribution was created in whole or in part by me and I have the right to submit it under the open-source license indicated in the file; or
2.  The contribution is based upon previous work that, to the best of my knowledge, is covered under an appropriate open-source license; or
3.  The contribution was provided directly to me by a person who certified (1) or (2) and I have not modified it.

### Required Commit Format

All commit messages must conclude with an explicit `Signed-off-by` tag:

```text
type(scope): concise summary under 50 characters

Detailed description of changes, rationale, and test results.

Signed-off-by: Developer Name <developer@example.com>
```

***

## 🔧 Build, Test & Verification Workflows

### 1. Building Kernel & Userland

```bash
# Verify kernel and userland library compilation
cargo check --lib

# Run Ninja userland targets
ninja userland
```

### 2. Executing Comprehensive Test Suites

```bash
# Run all 11+ SigmaOS inspection test suites
./run_sigma_tests.sh

# Run SPDX license header verification
./scripts/check-spdx.sh
```

### 3. Testing Standalone Modules

```bash
# Example: Standalone verification test execution
mkdir -p build
rustc --edition 2021 --test src/sigpkg/verifier.rs -o build/test_verifier && ./build/test_verifier
```

***

## 🏛️ Contribution Rules & Supreme Court Review Process

1.  **Bare-Metal Zero-Dependency Imperative**: External crate dependencies are prohibited in core `#![no_std]` crates without Supreme Court approval.
2.  **Proactive Testing**: Every feature addition or bug fix must include corresponding unit tests.
3.  **Documentation Parity**: Architectural updates must update corresponding documentation in `WIKI/` and be synchronized using `./scripts/sync_wiki.sh`.

# Contributing to SigmaOS & Contributor Charter

Welcome to **SigmaOS**! We welcome contributions from developers, security researchers, accessibility advocates, and institutions building the post-Linux sovereign operating system.

---

## 📜 Contributor Charter & Governance Principles

All contributions to SigmaOS must uphold the core principles defined in the **SigmaOS Contributor Charter** (`docs/GOVERNANCE_CHARTER.md`):

1. **Sovereignty**: Code contributions must remain transparent and independent from proprietary binary blobs or opaque vendor lock-in.
2. **Clarity**: Code structure, Rust traits, and documentation must be explicit, declarative, and self-documenting.
3. **Resilience**: Modifications to kernel shards, VFS storage, or package managers must preserve $O(1)$ state rollback guarantees.
4. **Security**: All code must enforce Safe-Rust memory safety, Post-Quantum Cryptography (Dilithium-5/Kyber) verifications, and least-privilege capability sandboxing (`pledge`/`unveil`).

---

## 🛠️ How to Contribute

### 1. Development Workflow
- Fork the repository and create a feature branch (`jules-<feature-name>`).
- Follow strict `#![no_std]` bare-metal zero-dependency Rust coding standards.
- Run `cargo check --lib` and `./run_sigma_tests.sh` to ensure all tests pass with zero warnings or errors.

### 2. Contributor Roles
- **Core Shard Maintainers**: Oversee the 12 microkernel shards (Media, Networking, Storage, AI, Compositor, Drivers, Security, Virtualization, System, Package, IPC, Hardware).
- **Community Contributors**: Submit bug fixes, documentation updates, translations, and accessibility improvements.
- **Institutional Partners**: Academic labs and sovereign entities deploying cluster-native hardware nodes.

---

## 📰 Official Project Resources & Documents
- **Public Launch Announcement**: `docs/LAUNCH_ANNOUNCEMENT.md`
- **Technical Whitepaper**: `docs/WHITEPAPER.md`
- **Media & Press Kit**: `docs/PRESS_KIT.md`
- **Governance Charter**: `docs/GOVERNANCE_CHARTER.md`

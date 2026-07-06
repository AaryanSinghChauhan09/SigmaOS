# Phase 5: Ecosystem & Developer Tools

## Overview

Phase 5 focuses on making SigmaOS approachable for external developers. It delivers comprehensive documentation, cross-platform SDKs, and contribution infrastructure modeled after Ubuntu's open-source governance — while remaining entirely sovereign.

---

## SigmaOS SDKs

### Kernel & Driver SDK (Rust)

The **SigmaOS Kernel SDK** provides everything a driver or kernel-extension developer needs, entirely in `no_std` Rust:

```
sdk/
├── kernel/
│   ├── mod.rs                  # Root kernel SDK module

│   ├── types.rs                # Core sovereign types

│   ├── oop.rs                  # Sovereign OOP traits (SigmaObject, SigmaSingleton)

│   └── hal.rs                  # HAL interface — drivers implement this trait

└── examples/
    └── hello_driver.rs         # Minimal Rust driver skeleton

```

### App / Userspace SDK (Rust)

The **SigmaOS App SDK** enables native sovereign applications:

- **Sovereign ABI**: Apps talk to the kernel via the Sovereign Syscall Gate — no POSIX shim required.

- **Rust UI bindings**: `sdk::app::zenith::Widget` trait and Zenith Desktop bindings.

---

## Cross-Compilation Toolchain

Build SigmaOS targets from Linux, macOS, or Windows:

```bash

# Install the SigmaOS cross-toolchain (x86_64-sigmaos-elf)

sigpkg install sigma-toolchain-x86_64

# Or build from source

cmake -B toolchain-build \
  -DSIGMA_TOOLCHAIN_TARGET=x86_64-sigmaos-elf \
  -DSIGMA_SYSROOT=/opt/sigmaos-sysroot
ninja -C toolchain-build
```

Supported host environments:

| Host OS | Cross-Target | Status |
|---------|-------------|--------|
| Ubuntu 22.04+ | x86_64-sigmaos-elf | ✅ Stable |
| macOS 14+ | x86_64-sigmaos-elf | 🔄 Beta |
| Windows 11 (WSL2) | x86_64-sigmaos-elf | ✅ Stable |

---

## Documentation Hub

| Resource | Location |
|----------|---------|
| Wiki (this site) | [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) |
| Architecture Overview | [Architecture-Overview](Architecture-Overview) |
| Driver Development Guide | [Driver-Development](Driver-Development) |
| API Reference | [API-Reference](API-Reference) |
| Getting Started | [Getting-Started](Getting-Started) |
| FAQ | [FAQ](FAQ) |

---

## Contribution Guidelines

SigmaOS follows an Ubuntu-inspired governance model:

1. **Fork & Branch**: Create a feature branch off `main`.

2. **Code Style**: C++17, SPDX headers required, `clang-format` enforced.

3. **Testing**: All PRs must maintain 100% existing test pass rate.

4. **Review**: 2 core maintainer approvals required for kernel changes.

5. **CI**: All 9 CI jobs (3 profiles × 3 targets) must pass green.

See [CONTRIBUTING.md](CONTRIBUTING) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT) for full details.

---

## Release Governance

| Role | Responsibility |
|------|---------------|
| **Core Maintainers** | Kernel architecture, security, release signing |
| **Module Owners** | Own a specific subsystem (net, fs, drivers) |
| **Community Contributors** | Bug fixes, documentation, new drivers |
| **Security Team** | CVE triage, PQC hardening, audit |

---

## 🔗 Related Pages

- [Phase 4: CI/CD & Testing](Phase-4-CICD-And-Testing)

- [Phase 6: Long-Term Vision](Phase-6-Long-Term-Vision)

- [Contributor Guidelines](Contributor-Guidelines)

- [API Reference](API-Reference)

- [Getting Started](Getting-Started)

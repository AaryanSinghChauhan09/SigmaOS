# AI Agent Fedora Linux Parity Component Maintenance & Development Guide

This document provides operational directives, architectural guidelines, and verification rules for AI agents maintaining and extending Fedora Linux compatibility and infrastructure components in SigmaOS.

---

## 1. Overview of Fedora Infrastructure Components

Fedora compatibility in SigmaOS is implemented as zero-dependency, `#![no_std]` Rust modules located across the following files:

| Component | File Path | Scope & Responsibilities |
|---|---|---|
| **Fedora Missing Infrastructure** | `src/compatibility/fedora_missing_components.rs` | Koji Build System, Bodhi Updates, Pagure Forge, COPR Build Gateway, Rootless Podman, Mock Chroot Builder, DNF5 Engine, Anaconda Kickstart, SSSD / FreeIPA |
| **Fedora Immutable Core & Provisioning** | `src/compatibility/fedora.rs` | Greenboot Health Monitoring, OSBuild Pipeline, Ignition/Butane Provisioning, Bootupd Bootloader Engine, rpm-ostree Compose Engine, Offline Updates |
| **Fedora Parity Extensions** | `src/distro/fedora_parity.rs` | Package staging, update triggers, security policies |

---

## 2. Core Operational Directives for AI Agents

1. **Zero-Dependency Rule**: All Fedora compatibility engines MUST remain 100% Rust-native without calling external C/C++ libraries, `libc`, or third-party crates in `#![no_std]` mode. Use `alloc::` for heap collections (`BTreeMap`, `Vec`, `String`).
2. **Standalone Verification**: Every modified file must be verifiable using standalone `rustc --test`:
   ```bash
   rustc --test src/compatibility/fedora_missing_components.rs --edition=2021 -o build/fedora_missing_test && ./build/fedora_missing_test
   rustc --test src/compatibility/fedora.rs --edition=2021 -o build/fedora_test && ./build/fedora_test
   ```
3. **Master Suite Integration**: Any new Fedora subsystem must be registered inside `SovereignFedoraEcosystemSuite` in `src/compatibility/fedora_missing_components.rs`.

---

## 3. Maintenance Procedures by Engine

### A. Mock Chroot Builder (`FedoraMockChrootBuilder`)
- Maintain chroot target configurations (`fedora-40-x86_64`, `fedora-rawhide-aarch64`).
- Ensure SRPM build invocations isolate buildroot dependencies and produce valid output `.rpm` names.

### B. DNF5 Next-Gen Package Engine (`FedoraDnf5PackageEngine`)
- Implement DNF5 transaction solver parity.
- Support advisory ID filtering (`FEDORA-2026-*`) and group installations (`install_group("workstation-product")`).

### C. Anaconda Kickstart Auto-Installer (`FedoraAnacondaKickstartEngine`)
- Parse `%packages`, `%post`, `%pre`, and `timezone` declarations.
- Generate Btrfs subvolume layouts (`@root`, `@home`, `@swap`).

### D. SSSD & FreeIPA Identity Engine (`FedoraSssdFreeIpaEngine`)
- Manage domain join configurations (`join_freeipa_domain`).
- Support Kerberos `kinit` ticket cache lifetime validation.

---

## 4. Verification Checklist

Before submitting PRs touching Fedora components:
- [ ] Run standalone unit tests: `rustc --test src/compatibility/fedora_missing_components.rs --edition=2021 -o build/fedora_missing_test && ./build/fedora_missing_test`
- [ ] Run system test runner: `./run_sigma_tests.sh`
- [ ] Verify `#![no_std]` compliance.

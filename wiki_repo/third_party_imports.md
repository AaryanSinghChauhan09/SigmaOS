# Third-Party Imports Documentation

This document tracks all third-party components, drivers, tools, and features imported into SigmaOS from upstream sources including Linux kernel, Fedora, Arch Linux, Debian/Ubuntu, NixOS, Kali, and Qubes.

## Integration Principles

1. **Prefer upstream kernel for drivers**; maintain SigmaOS branch for local patches
2. **Convert packaging (RPM/PKG/DEB/Nix) into sigpkg** with SBOM and GPG signing
3. **Reimplement critical tooling in Rust/C** where feasible; keep high-level languages only for non-critical tooling with migration plans
4. **Document every import** in this file and mirror to the Wiki
5. **Track license compliance** for each imported component and add attribution in LICENSES.md

## Source Distributions

| Distro | Strength | Repo Types | Why Useful for SigmaOS |
| --- | --- | --- | --- |
| Linux kernel | Upstream drivers, core subsystems | Kernel tree, drivers | Canonical source for drivers and kernel features |
| Fedora | Upstream integration, SELinux, packaging | RPM specs, kernel modules, tooling | Strong driver packaging, security policies, CI patterns |
| Arch Linux | Fast adoption, PKGBUILDs, userland tooling | PKGBUILDs, AUR helpers | Rapid packaging patterns, bleeding-edge modules |
| Debian/Ubuntu | Stability, wide hardware support | Debian packages, kernel configs | Long-term driver support and packaging best practices |
| NixOS | Declarative, reproducible builds | nixpkgs, modules | Reproducible packaging and declarative system configs |
| Kali | Security tooling, hardware for pentesting | Tool bundles, drivers | Curated security toolsets and device support |
| Qubes | Compartmentalization, microVM patterns | Templates, isolation tooling | Proven compartmentalization and policy models |

## Import Status

### Quick Wins (0-6 weeks) - High Priority

- [ ] Item 38: SBOM generation pipeline (NixOS/Fedora)
- [ ] Item 39: Signed package repository tooling (Fedora)
- [ ] Item 56: Calamares installer modules (Arch/Fedora)
- [ ] Item 57: Encrypted home defaults (Ubuntu/Fedora)
- [ ] Item 71: Wayland compositor improvements (Fedora)
- [ ] Item 73: PipeWire audio stack defaults (Fedora)
- [ ] Item 40: Package rollback hooks (NixOS/OSTree)
- [ ] Item 36: Binary delta update algorithm (Fedora/OSTree)
- [ ] Items 1-5, 61: HCL and driver CI tests for top 10 devices

### Medium Priority (1-3 months)

- [ ] Item 91: Firecracker microVM integration (Qubes/Firecracker)
- [ ] Item 45: Reproducible build farm orchestration (NixOS/Fedora)
- [ ] Item 59: Persona bundle installer (Fedora/Ubuntu)
- [ ] Item 94: TPM attestation workflows (Fedora)
- [ ] Item 1: Mainline Intel GPU driver stack (torvalds/linux)
- [ ] Item 2: AMDGPU updates (torvalds/linux)
- [ ] Item 3: NVIDIA Nouveau improvements (torvalds/linux/mesa)
- [ ] Item 4: Broadcom Wi-Fi driver fixes (Debian/Fedora patches)
- [ ] Item 5: Realtek Wi-Fi driver backports (Arch/Fedora)

### Long Term (3-12 months) - Low Priority

- [ ] Item 31: Nix-style declarative package modules (NixOS)
- [ ] Item 37: OSTree/Immutable OS model (Fedora Silverblue)
- [ ] Item 112: Signed model marketplace (community repos)

## Detailed Import Records

### Item 38: SBOM Generation Pipeline

**Source**: NixOS/Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Integrate SBOM output in CI pipeline for all packages

### Item 39: Signed Package Repository Tooling

**Source**: Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Implement GPG signing and verification for sigpkg repository

### Item 56: Calamares Installer Modules

**Source**: Arch/Fedora
**License**: GPL-3.0-or-later
**Status**: Pending
**Integration Notes**: Adopt modular installer flows for SigmaOS

### Item 57: Encrypted Home Defaults

**Source**: Ubuntu/Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Implement encrypted home and LUKS by default

### Item 71: Wayland Compositor Improvements

**Source**: Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Integrate compositor patches for low latency

### Item 73: PipeWire Audio Stack Defaults

**Source**: Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Low-latency audio and sandboxed streams

### Item 40: Package Rollback Hooks

**Source**: NixOS/OSTree
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Atomic rollback support in sigpkg

### Item 36: Binary Delta Update Algorithm

**Source**: Fedora/OSTree
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Implement delta updates for sigpkg to reduce download sizes

### Item 1: Mainline Intel GPU Driver Stack

**Source**: torvalds/linux
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Integrate i915 improvements; test on HCL

### Item 2: AMDGPU Updates

**Source**: torvalds/linux
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Absorb latest DRM patches and firmware packaging

### Item 3: NVIDIA Nouveau Improvements

**Source**: torvalds/linux/mesa
**License**: GPL-2.0-or-later / MIT
**Status**: Pending
**Integration Notes**: Upstream patches and packaging

### Item 4: Broadcom Wi-Fi Driver Fixes

**Source**: Debian/Fedora patches
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Package signed firmware and drivers

### Item 5: Realtek Wi-Fi Driver Backports

**Source**: Arch/Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Backport stable fixes for common laptops

### Item 61: Automated HCL Hardware Detection

**Source**: Arch/Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Preselect drivers during install

### Item 91: Firecracker MicroVM Integration

**Source**: Qubes/Firecracker
**License**: Apache-2.0
**Status**: Pending
**Integration Notes**: Per-app microVM sandboxing

### Item 45: Reproducible Build Farm Orchestration

**Source**: NixOS/Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Reproducible builder orchestration

### Item 59: Persona Bundle Installer

**Source**: Fedora/Ubuntu
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Preconfigured bundles for Dev/Data/Security

### Item 94: TPM Attestation Workflows

**Source**: Fedora
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Device attestation and sealed secrets

### Item 31: Nix-Style Declarative Package Modules

**Source**: NixOS
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Adopt declarative system manifests

### Item 37: OSTree/Immutable OS Model

**Source**: Fedora Silverblue
**License**: GPL-2.0-or-later
**Status**: Pending
**Integration Notes**: Optional immutable root with layered packages

### Item 112: Signed Model Marketplace

**Source**: Community repos
**License**: Various (MIT, Apache-2.0)
**Status**: Pending
**Integration Notes**: Offline model marketplace with signed models

## License Compliance

All imported components must have their licenses tracked in this document and attributed in the root LICENSES.md file. Components with incompatible licenses will be reimplemented in Rust/C where feasible.

## Implementation Checklist

- [x] Create docs/third_party_imports.md documentation structure
- [ ] Open issues for each item with labels `area/*` and `priority/*`
- [ ] Create feature branches `work/<area>/<short-desc>` for each implementation
- [ ] Add CI jobs: build, reproducibility, SBOM, CVE scan, hardware driver tests
- [ ] Mirror docs to Wiki via script that syncs docs/ to Wiki on merged PRs
- [ ] Track license compliance for each imported component
- [ ] Add attribution in LICENSES.md

---

**Document Version**: 1.1
**Last Updated**: 2026-07-08
**Status**: Active Tracking

# SigmaOS Linux & BSD Production Roadmap & Architecture Guidelines

Based on proven design patterns from kernel.org, FreeBSD, OpenBSD, Debian, Arch Linux, and Nix/Guix.

## 1. Documentation Model (kernel.org / man(7) Standard)
- **Canonical Tree:** All core documentation converges into `docs/` (`docs/ARCHITECTURE.md`, `docs/ROADMAP.md`, `docs/SECURITY.md`).
- **BSD Man Pages:** In-tree man pages shipped in `docs/man/` (`sigma-sh(1)` for command shell, `sigma-pkg(8)` for package manager).
- **Clean Root:** Marketing & competitive analysis files housed in wiki or external repositories.

## 2. Release Engineering & Cadence (Debian / OpenBSD Model)
- **Branch Strategy:** `main` for active development; `release/X.Y` cut on fixed 6-month cycles (OpenBSD model).
- **Security Errata:** GPG/PQC-signed tags for errata patches and point releases.
- **Reproducible Determinism:** Automatic publishing of bit-for-bit build SHA256 hashes (`build-provenance`).

## 3. Package Management Strategy (Content-Addressed Binary Cache)
- **Declarative Nix/Guix CAS Model:** Content-Addressed Store with reproducible build recipes.
- **Binary Substituter Network:** Sub-second binary downloads via P2P/MirrorManager 2 substituter nodes.
- **Transpilation Pipeline:** Cross-distro transpilation for `.deb`, `.rpm`, `.pkg.tar.zst`, and `.apk`.

## 4. Kernel & Driver Stability Discipline (OpenBSD Audit Model)
- **Security Audit Process:** Continuous security audit tracking and public vulnerability disclosure policy (`SECURITY.md`).
- **Capability Isolation:** Microkernel pledge/unveil access restriction gating syscalls.

## 5. Subsystem Maintainer Governance (Linux Kernel Model)
- **Maintainer Hierarchy:** Independent maintainers for `kernel/`, `drivers/`, `zenith_desktop/`, `userland/`, and `sigpkg/`.
- **Pull Request Review Gate:** Automated CI review and required maintainer signoff per subsystem.

## 6. Scriptable Netinst & Live Installer (FreeBSD bsdinstall / Arch Model)
- **Minimal Text-Based Installer:** Scriptable `bsdinstall`-style terminal installer operating without GUI compositor dependencies.
- **Automated Provisioning:** Kickstart/declarative config JSON for headless deployments.

## 7. Formalized Test Harness (Linux kselftest / FreeBSD Kyua Model)
- **In-Tree Test Suite:** `./run_sigma_tests.sh` executing unit and subsystem inspection tests.
- **CI Gatekeeping:** Automated GitHub Actions workflows gating PR merges.

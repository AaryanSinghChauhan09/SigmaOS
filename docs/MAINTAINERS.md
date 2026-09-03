# SigmaOS Maintainers & Subsystem Governance Tree

This document describes the subsystem maintainer hierarchy and governance process for SigmaOS, modeled after the Linux Kernel maintainer tree (`MAINTAINERS`) and FreeBSD committer guidelines.

---

## Governance Principles

1. **Subsystem Ownership:** Every core directory (`src/kernel/`, `src/driver/`, `src/security/`, `src/sigpkg/`, `src/desktop/`) is owned by dedicated maintainers responsible for code quality, architectural integrity, and review.
2. **Pull Request Workflow:** All changes must pass automated CI checks (`./run_sigma_tests.sh`) and receive explicit signoff from at least one subsystem maintainer before merging into `main`.
3. **Developer Certificate of Origin (DCO):** All commits must include a `Signed-off-by: Author <email>` trailer certifying compliance with open source licensing.

---

## Subsystem Maintainers Tree

### KERNEL & MEMORY MANAGEMENT
- **Path:** `src/kernel/`, `src/klib/`
- **Scope:** Hybrid microkernel, process scheduler, buddy memory allocator, paging, interrupt handling.
- **Maintainers:** Kernel Subsystem Team <kernel@sigmaos.org>

### DRIVER FRAMEWORK & HARDWARE
- **Path:** `src/driver/`
- **Scope:** xHCI USB, storage controllers, network interfaces, power management.
- **Maintainers:** Driver Subsystem Team <drivers@sigmaos.org>

### SECURITY & POST-QUANTUM CRYPTOGRAPHY
- **Path:** `src/security/`
- **Scope:** SELinux/Capsicum MAC sandboxing, OpenBSD Pledge/Unveil, TPM 2.0 PCR attestation, PQC Dilithium-5/Kyber-1024 cryptography.
- **Maintainers:** Security Subsystem Team <security@sigmaos.org>

### PACKAGE MANAGEMENT & BUILD REPRODUCIBILITY
- **Path:** `src/package/`, `src/sigpkg/`
- **Scope:** `sigma-pkg` binary manager, universal package translator, AUR/Portage integration, hermetic chroot builds.
- **Maintainers:** Package Management Team <pkg@sigmaos.org>

### ZENITH DESKTOP & USERLAND
- **Path:** `src/desktop/`, `src/shell/`
- **Scope:** Zenith Desktop overlays, `sigma-sh` terminal emulator, UI compositing.
- **Maintainers:** Desktop & Userland Team <desktop@sigmaos.org>

---

## Merge Requirements
- **CI Certification:** All 3 core test suites (`cache`, `unimplemented_features`, `unimplemented_tools`) must pass 100%.
- **Review Approval:** At least 1 approving review from the designated subsystem maintainer.
- **Commit Format:** Conventional Commit messages (`feat:`, `fix:`, `perf:`, `docs:`, `refactor:`).

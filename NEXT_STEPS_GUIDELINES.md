# SigmaOS Next Steps Guidelines & Multi-OS Distro Integration Roadmap

## Executive Summary
This document provides concrete execution guidelines and an architectural roadmap for developers and maintainers contributing to **SigmaOS**. It integrates multi-OS inspirations from Linux (Arch, Gentoo, Void, NixOS, Alpine, Ubuntu, Debian, Fedora) and BSD (FreeBSD, OpenBSD, NetBSD) ecosystems, focusing on enhancing **Kernel Scheduling**, **Userland Capabilities**, **System Supervision**, the **SigmaOS User Repository (AUR / Sovereign AUR)**, the **SigmaOS Arch Build System / Protocol (ASP / ABS in `src/sigpkg/arch_pacman_engine.rs`)**, the **SigmaOS Sovereign Installer (`installer/sigma-installer.rs` & `installer/bsdinstall_netinst.rs`)**, the **SigmaOS Web Interface (`web_ui/`)**, **Package Repository Infrastructure (`src/sigpkg/repository_manager.rs`)**, **Forgejo OCI Image Registry Infrastructure (`src/container/oci_orchestrator.rs`)**, **Firmitas System Integrity & Immutability Engine (`src/security/firmitas.rs`)**, **Fedora Kernel Subsystem Integration (`src/kernel/subsystems/sovereign_modules.rs`)**, **Universal Foreign Distro Package Bridge (`src/package/universal.rs`)**, **Release Engineering Infrastructure (`src/release/mod.rs`)**, **In-Tree Kyua/kselftest Harness (`tests/kyua_kselftest_harness.rs`)**, and **System Manual Pages (`docs/man/`)**.

---

## 🏛️ Master Linux & BSD Production Models Adopted

1. **Documentation — kernel.org / man(7) Model**: Consolidate documentation into canonical guides (`ARCHITECTURE.md`, `ROADMAP.md`, `SECURITY.md`) and author semantic `mdoc(7)` manual pages (`docs/man/man1/sigma-sh.1`, `docs/man/man8/sigma-pkg.8`).
2. **Release Engineering — Formal Release Cadence**: Debian stable/testing/unstable branches and OpenBSD 6-month release cadence with GPG/Dilithium-5 signed tags, reproducible build hashes, and errata advisories (`src/release/mod.rs`).
3. **Package Management — Dual Content-Addressed & Universal Bridge Model**: Nix/Guix content-addressed store paths (`/sigma/store/`) combined with FreeBSD ports / Arch AUR build recipes (`src/sigpkg/aurweb.rs`) and multi-distro package conversion (`src/package/universal.rs`).
4. **Driver/Kernel Stability — OpenBSD Security-Audit Discipline**: Continuous code audits, small maintainer committer set, explicit disclosure policy (`SECURITY.md`), and in-tree security audit logs.
5. **Governance — Linux Maintainer-Tree Model**: Hierarchical subsystem maintainers (`kernel/`, `drivers/`, `zenith_desktop/`, `userland/`) managing subtree merges (`CODEOWNERS`).
6. **Installer & Live Media — bsdinstall & Netinst Engine**: Text-based, scriptable, no-GUI-dependency minimal installer supporting Root-on-ZFS (`installer/bsdinstall_netinst.rs`).
7. **Testing — Linux kselftest / FreeBSD Kyua Harness**: In-tree subsystem test harness (`tests/kyua_kselftest_harness.rs`) gating merges across kernel, drivers, security, and userland.

---

## 1. System Manual Pages & Documentation Guidelines (`docs/man/`)

### A. BSD `mdoc(7)` Macro Format Standard
- **Guideline**: Author all system manual pages using semantic `mdoc(7)` macro syntax (`.Dd`, `.Dt`, `.Sh NAME`, `.Sh SYNOPSIS`, `.Sh DESCRIPTION`, `.Sh EXAMPLES`, `.Sh EXIT STATUS`).
- **Implementation**: Maintain `mdoc` source files under `docs/man/man1/` (e.g. `sigma-sh.1`) and `docs/man/man8/` (e.g. `sigma-pkg.8`).

### B. OpenBSD `mandoc -Tlint` CI Quality Gate
- **Guideline**: Enforce static manual page linting during continuous integration.
- **Implementation**: Run `mandoc -Tlint` across all `docs/man/` files in CI to catch formatting warnings and macro syntax errors.

---

## 2. Universal Foreign Distro Package Bridge Architecture (`src/package/universal.rs`)

### A. Multi-Format Foreign Package Conversion
- **Guideline**: Convert packages from Linux (.deb, .rpm, .pkg.tar.zst, .apk, .xbps, .ebuild, .eopkg) and BSD (.pkg, .ports, .pkgsrc) formats directly into native SigmaPkg objects.
- **Implementation**: Utilize `SovereignUniversalDistroBridgeEngine::convert_foreign_package_bytes_to_sigpkg` in `src/package/universal.rs` to write packages into `/sigma/store/<hash>-<name>-<version>`.

### B. Virtual Dependency Name Normalization
- **Guideline**: Normalize distro-specific library and compiler dependency names into unified SigmaPkg virtual dependency tokens.
- **Implementation**: Utilize `UniversalPackageTranslator::normalize_dependency_name` to map `libc6`/`glibc`/`musl` -> `sovereign-libc` and `libssl-dev`/`openssl-devel` -> `sovereign-openssl`.

---

## 3. Release Engineering Guidelines (`src/release/mod.rs`)

### A. Formal Release Branch Cadence
- **Guideline**: Maintain `main` for active development, cut `release/vX.Y` branches for stable cycles, and publish errata advisories for security fixes.
- **Implementation**: Utilize `ReleaseEngineeringEngine::cut_release_branch` to register release tags and track stable/testing/unstable cadences.

### B. Signed Tags & Reproducible Build Hash Verification
- **Guideline**: Every official release tag must be cryptographically signed with GPG or Dilithium-5 post-quantum keys, and publish reproducible build SHA256 hashes.
- **Implementation**: Verify reproducible build hashes via `verify_reproducible_build_hash` before distribution.

---

## 4. FreeBSD `bsdinstall` & Netinst Text Installer Guidelines (`installer/bsdinstall_netinst.rs`)

### A. Text-Based Scriptable Installation
- **Guideline**: Provide a lightweight, text-based installer independent of desktop GUI compositors for headless server, cloud, and live ISO installations.
- **Implementation**: Utilize `BsdinstallNetinstEngine` (`installer/bsdinstall_netinst.rs`) supporting unattended script generation (`install.conf`).

### B. Root-on-ZFS & Partitioning
- **Guideline**: Automatically create ZFS storage pools (`zroot`) with boot environment datasets (`zroot/ROOT/default`, `zroot/var`, `zroot/home`).
- **Implementation**: Utilize `partition_disk_zfs` to set up Root-on-ZFS topologies.

---

## 5. In-Tree Kyua / kselftest Subsystem Test Harness Guidelines (`tests/kyua_kselftest_harness.rs`)

### A. Subsystem Test Discovery & Execution
- **Guideline**: Maintain in-tree test suites per subsystem (Kernel, Security, Network, Filesystem, Drivers) that gate code merges.
- **Implementation**: Utilize `KyuaKselftestHarness::register_test_suite` and `run_all_subsystem_tests` in `tests/kyua_kselftest_harness.rs`.

### B. TAP & JUnit XML Test Reporting
- **Guideline**: Output standardized Test Anything Protocol (TAP) and JUnit XML reports for CI pipeline consumption.
- **Implementation**: Export test execution logs via `generate_junit_tap_report`.

---

## 6. General Engineering & Quality Guidelines

### A. Code Quality & Type Safety
- **Rust Atomic Enum Transmutes**: Ensure all enums backed by atomic store operations are marked with `#[repr(usize)]` or `#[repr(u32)]`.
- **Linting & Warnings**: Fix unused variables and unneeded `mut` annotations in `src/sigpkg/` and `src/driver/`.

### B. Tri-Agent Autonomous Principles
- **Bolt ⚡ (Performance)**: Prioritize zero-copy allocations, SLUB slab caches, lock-free atomic swaps, and zero-copy byte slice iteration (`input.as_bytes()`).
- **Palette 🎨 (UX & Accessibility)**: Enforce ARIA labels (`aria-label`, `aria-checked`), keyboard focus navigation (`focus-visible:ring-2`), and high-contrast desktop themes.
- **Sentinel 🛡️ (Security & Compliance)**: Enforce strict input validation, zero hardcoded secrets, Firmitas system immutability, OCI post-quantum image signatures, and compliance with GDPR, HIPAA, WCAG 2.1 AA, and ISO 27001 standards.

---

## 7. Recommended Phased Implementation Sequence

1. **Phase 1: Release Engineering & Signed Tags**: Deploy `ReleaseEngineeringEngine` for managing stable release branches and reproducible build hash validation.
2. **Phase 2: Universal Foreign Distro Package Bridge**: Integrate `SovereignUniversalDistroBridgeEngine` for automated foreign package translation (.deb, .rpm, .pkg.tar.zst, .apk, .xbps, .pkg).
3. **Phase 3: In-Tree Kyua / kselftest Test Harness**: Integrate `KyuaKselftestHarness` into CI pipelines for subsystem merge gating.
4. **Phase 4: Text-Based bsdinstall Netinst Engine**: Integrate `BsdinstallNetinstEngine` into live ISO media for Root-on-ZFS text installs.
5. **Phase 5: System Manual Page Standardization**: Author additional `mdoc(7)` manual pages in `docs/man/` (`sigma-init.8`, `sigma-ctl.8`).
6. **Phase 6: Sovereign AUR Clean Chroot Sandboxing**: Mandate `poudriere` chroot and `unveil` path isolation for all package builds.
7. **Phase 7: Firmitas System Integrity & Immutability**: Integrate `FirmitasEngine` read-only root mounts and A/B atomic boot deployment slots.
8. **Phase 8: Fedora Kernel Subsystem Integration**: Integrate `FedoraSubsystemIntegrationEngine` systemd unit dependencies and cgroup v2 controllers.
9. **Phase 9: Fedora Forgejo OCI Container Image Registry**: Integrate `ForgejoOciImageEngine` into `src/container/` for zero-trust OCI container deployments.

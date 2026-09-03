# SigmaOS Next Steps Guidelines & Multi-OS Distro Integration Roadmap

## Executive Summary
This document provides concrete execution guidelines and an architectural roadmap for developers and maintainers contributing to **SigmaOS**. It integrates multi-OS inspirations from Linux (Arch, Gentoo, Void, NixOS, Alpine, Ubuntu, Debian, Fedora) and BSD (FreeBSD, OpenBSD, NetBSD) ecosystems, focusing on enhancing **Kernel Scheduling**, **Userland Capabilities**, **System Supervision**, the **SigmaOS User Repository (AUR / Sovereign AUR)**, the **SigmaOS Arch Build System / Protocol (ASP / ABS in `src/sigpkg/arch_pacman_engine.rs`)**, the **SigmaOS Sovereign Installer (`installer/sigma-installer.rs`)**, the **SigmaOS Web Interface (`web_ui/`)**, **Package Repository Infrastructure (`src/sigpkg/repository_manager.rs`)**, **Forgejo OCI Image Registry Infrastructure (`src/container/oci_orchestrator.rs`)**, and **System Manual Pages (`docs/man/`)**.

---

## 1. Master Multi-OS Parity Execution Guidelines for Missing Capabilities

Developers implementing missing Linux & BSD capabilities in SigmaOS must follow these architecture guidelines:

### A. Linux `sched_ext` Extensible BPF Schedulers
- **Guideline**: Allow dynamic, pluggable BPF scheduling policies without kernel rebuilds.
- **Implementation**: Hook BPF scheduler policies into `PolicyAdaptiveEventScheduler` (`src/distro/sovereign_system_innovations.rs`).

### B. FreeBSD Capsicum & Casper Capabilities
- **Guideline**: Transition processes into capability mode where raw ambient syscall access is denied and only capability file descriptors are permitted.
- **Implementation**: Wrap system operations in `CapsicumSandbox` capabilities inside `src/security/`.

### C. Void/Gentoo Process Supervision & OpenRC
- **Guideline**: Run lightweight, fast process supervisors maintaining process dependency graphs and automatic daemon restart.
- **Implementation**: Integrate a zero-dependency process supervisor in `src/process/`.

---

## 2. Multi-OS Distro Inspired System Manual Page Guidelines (`docs/man/`)

To evolve system manual pages in `docs/man/` into clear, machine-readable reference guides, documentation maintainers must follow these guidelines:

### A. BSD `mdoc(7)` Macro Format Standard
- **Guideline**: Author all manual pages using semantic `mdoc(7)` macro syntax (`.Dd`, `.Dt`, `.Sh NAME`, `.Sh SYNOPSIS`, `.Sh DESCRIPTION`, `.Sh EXAMPLES`, `.Sh EXIT STATUS`) rather than plain presentation troff macros.
- **Implementation**: Maintain `mdoc` source files under `docs/man/man1/`, `docs/man/man5/`, and `docs/man/man8/`.

### B. OpenBSD `mandoc -Tlint` CI Quality Gate
- **Guideline**: Enforce static manual page linting during continuous integration.
- **Implementation**: Run `mandoc -Tlint` across all `docs/man/` files in CI to catch formatting warnings and macro syntax errors.

### C. Arch Linux `man-db` Binary Indexing
- **Guideline**: Enable fast keyword and `apropos` search indexing for system command utilities.
- **Implementation**: Pre-render `mandb` binary index databases during system image building.

---

## 3. Multi-OS Distro Inspired ASP / ABS Build Tree Guidelines (`src/sigpkg/arch_pacman_engine.rs`)

To evolve `ArchBuildSystem` in `src/sigpkg/arch_pacman_engine.rs` into a high-performance source checkout and package build framework, maintainers must follow these guidelines:

### A. Arch Linux ASP Git-Backed Source Tree Checkout
- **Guideline**: Support checking out PKGBUILD source trees directly from Git mirrors without downloading full tarballs (`asp checkout <package>`).
- **Implementation**: Parse `.SRCINFO` metadata directly from shallow git clones of package source repositories.

### B. FreeBSD Ports Tree Structured Hierarchy
- **Guideline**: Maintain a local hierarchical ports tree structure in `/sigma/ports/<category>/<package>`.
- **Implementation**: Support MAKE variables (`CFLAGS`, `LDFLAGS`, `WITH_DEBUG`) and automated checksum verification against `distinfo`.

### C. OpenBSD `dpb` Distributed Parallel Build Scheduling
- **Guideline**: Accelerate bulk compilation via distributed multi-node compile job distribution.
- **Implementation**: Schedule build jobs across local CPU cores and remote build worker nodes using lock-free job queues.

### D. Void Linux `xbps-src` Unprivileged Container Sandboxing
- **Guideline**: Ensure build scripts run inside unprivileged user namespaces and isolated temp roots.
- **Implementation**: Enforce non-root build privileges inside clean container namespaces during `makepkg` execution.

---

## 4. Multi-OS Distro Inspired Package Repository Infrastructure Guidelines

To evolve `registry_config.json` and `src/sigpkg/repository_manager.rs` into a global, zero-trust distribution network, repository maintainers must follow these guidelines:

### A. FreeBSD DNS SRV Record Auto-Discovery
- **Guideline**: Implement dynamic mirror discovery using DNS SRV records to eliminate hardcoded mirror lists.
- **Implementation**: Query `_https._tcp.repo.sigmaos.org` to dynamically resolve geographical mirror hosts with automatic fallback on timeout.

### B. Nix Cryptographically Signed Binary Caches
- **Guideline**: All pre-compiled binary packages and store objects must be signed with Ed25519 cryptographic signatures.
- **Implementation**: Enforce signature checking before extracting binary archives, storing trusted public keys in `registry_config.json`.

### C. Linux Mint Automated Mirror Speed & Latency Benchmarks
- **Guideline**: Automatically measure mirror latency and throughput before bulk updates.
- **Implementation**: Expand `MirrorBenchmarkEngine` (`src/sigpkg/repository_manager.rs`) to benchmark mirror endpoints and rank active sources automatically.

### D. Ubuntu/Debian PPA Snippets & GPG Verification
- **Guideline**: Allow modular third-party repository additions via `PpaRepository`.
- **Implementation**: Automatically fetch and verify GPG key fingerprints for custom repository entries added to `/etc/sigma/sources.list.d/`.

---

## 5. Multi-OS Distro Inspired Web UI Architecture Guidelines (`web_ui/`)

To evolve `web_ui/index.html` and `web_ui/styles/style.css` into an accessible, responsive, zero-jank web interface, front-end maintainers must adhere to the following guidelines:

### A. OpenBSD Zero-JavaScript Progressive Enhancement
- **Guideline**: Ensure all critical information (release notes, ISO download mirrors, installation steps) remains fully functional when JavaScript is disabled or when rendered in text-based user agents (`lynx`, `w3m`, `links`).
- **Implementation**: Form elements and installer steppers must rely on standard semantic `<form>` actions with server fallback routes alongside client-side JS.

### B. NixOS Interactive Option & Package Search
- **Guideline**: Implement instant client-side package and configuration searching directly in the web UI.
- **Implementation**: Embed a lightweight, zero-dependency client-side fuzzy search index (`web_ui/index.js`) for searching kernel modules, package names, and configuration parameters.

### C. FreeBSD SSG Documentation & Static Mirroring
- **Guideline**: Build self-contained static documentation bundles that can be served offline from local ISO media.
- **Implementation**: Compile wiki pages and specifications into offline static HTML bundles stored in `docs/` and accessible directly from the live ISO installer interface.

### D. Linux Mint Responsive Glassmorphism Design System
- **Guideline**: Maintain a modern, accessible glassmorphism visual aesthetic with full dark mode support and WCAG 2.1 AA contrast compliance.
- **Implementation**: Standardize CSS custom variables (`--bg-glass`, `--accent-sig`, `--text-primary`) in `web_ui/styles/style.css`, enforcing `focus-visible:ring-2` keyboard outline rings across all interactive buttons.

---

## 6. Multi-OS Distro Inspired Installer Architecture Guidelines

To evolve `installer/sigma-installer.rs` into a high-reliability installer engine, developers must follow these architectural guidelines:

### A. Calamares-Inspired Plugin Modularization
- **Guideline**: Decouple monolithic installer routines into modular, isolated steps (Language, Timezone, DiskPartition, UserAccount, PackageSelection, BootloaderInstall, PostInstallHooks).
- **Implementation**: Define a Rust `InstallerPlugin` trait with `prepare()`, `validate()`, and `execute()` callbacks.

### B. FreeBSD `bsdinstall` Root-on-ZFS & Boot Environments
- **Guideline**: Support automatic ZFS pool creation with Boot Environments (`bectl`/`beadm`).
- **Implementation**: Allow user selection of `FilesystemType::ZFS`, automatically generating zpool root datasets (`zroot/ROOT/default`, `zroot/home`, `zroot/var`).

### C. OpenBSD Autoinstall (`install.conf`) & Debian Preseed
- **Guideline**: Support non-interactive headless PXE/HTTP automated installations.
- **Implementation**: Expand `preseed_file` parsing to accept OpenBSD-style key-value answer files (`install.conf`) or JSON/YAML unattended install scripts.

### D. Ubuntu Subiquity Cloud-Init & Network Provisioning
- **Guideline**: Integrate declarative network and cloud-init post-installation provisioners.
- **Implementation**: Automatically output netplan/NetworkManager YAML files and cloud-init metadata during stage 2 target disk chroot setup.

---

## 7. Multi-OS Distro Inspired AUR Architecture Guidelines

To elevate the SigmaOS User Repository (AUR) into a world-class, sovereign package ecosystem, maintainers must adhere to the following architectural guidelines:

### A. FreeBSD `poudriere` Clean Chroot & FLAVORS
- **Guideline**: Never compile untrusted user build recipes directly on the host root filesystem.
- **Implementation**: Utilize `AurBuildSandbox` (`src/sigpkg/aurweb.rs`) to spawn isolated clean chroot containers. Implement FLAVORS support allowing users to build variants (e.g., `pkg-nox`, `pkg-qt6`, `pkg-gtk4`).

### B. OpenBSD `pledge(2)` and `unveil(2)` Security Restrictions
- **Guideline**: Restrict system call access and filesystem path visibility during package build steps.
- **Implementation**: Enforce `pledge` rules (`stdio rpath wpath cpath inet`) and `unveil` restrictions (limiting write access strictly to `/tmp/sigma_aur_builds`).

### C. Gentoo Portage USE Flags & EBUILD Conditional Compilation
- **Guideline**: Provide fine-grained feature toggles for package dependencies and compilation options.
- **Implementation**: Integrate `PortageUseFlagPipeline` (`src/sigpkg/universal_oop_system.rs`) into PKGBUILD processing, allowing flags like `+wayland`, `-x11`, `+cuda`.

### D. Nix Pure Functional Store Paths & Atomic Rollbacks
- **Guideline**: Ensure zero dependency conflicts through content-addressed store paths.
- **Implementation**: Package binaries output to `/sigma/store/<hash>-<name>-<version>` before symlinking into system profiles, enabling instant $O(1)$ rollback capability.

### E. Arch Linux `namcap` & Security Audit Linting
- **Guideline**: Perform automated static analysis on all user-submitted package recipes prior to repository index publication.
- **Implementation**: Run `NamcapSecurityAuditor` (`src/sigpkg/aurweb.rs`) to verify file permissions, missing dependencies, redundant library linkages, and hardcoded path vulnerabilities.

---

## 8. Fedora Linux Inspired Forgejo OCI Container Image Guidelines (`src/container/oci_orchestrator.rs`)

To elevate container registry and OCI image management in SigmaOS, developers must adhere to the following Fedora CoreOS / Fedora OCI guidelines:

### A. Fedora CoreOS OSTree Layering & OCI v1.1 Manifests
- **Guideline**: Support OSTree immutable base OS layers (`application/vnd.fedora.ostree.layer.v1+tar`) alongside standard OCI v1.1 layer specs.
- **Implementation**: Utilize `ForgejoOciImageEngine` (`src/container/oci_orchestrator.rs`) to register OSTree layers and generate valid OCI v2 manifest JSON.

### B. Cosign & Dilithium-5 Post-Quantum Image Signatures
- **Guideline**: All OCI container image layers published to Forgejo registries must be signed with Dilithium-5 post-quantum signatures.
- **Implementation**: Enforce `sign_image_dilithium5` signature verification before extracting image layers into container runtime roots.

### C. SLSA Level 3 Build Provenance Metadata
- **Guideline**: Embed cryptographic build provenance metadata into container images to prevent supply-chain tampering.
- **Implementation**: Attach `SlsaBuildProvenance` metadata (builder ID, source repo URL, commit SHA) to OCI tags.

---

## 9. General Engineering & Quality Guidelines

### A. Code Quality & Type Safety
- **Rust Atomic Enum Transmutes**: Ensure all enums backed by atomic store operations are marked with `#[repr(usize)]` or `#[repr(u32)]` to match platform word sizes and eliminate transmute size mismatches.
- **Linting & Warnings**: Fix unused variables and unneeded `mut` annotations in `src/sigpkg/` and `src/driver/`.

### B. Tri-Agent Autonomous Principles
- **Bolt ⚡ (Performance)**: Prioritize zero-copy allocations, SLUB slab caches, lock-free atomic swaps, and zero-copy byte slice iteration (`input.as_bytes()`).
- **Palette 🎨 (UX & Accessibility)**: Enforce ARIA labels (`aria-label`, `aria-checked`), keyboard focus navigation (`focus-visible:ring-2`), and high-contrast desktop themes.
- **Sentinel 🛡️ (Security & Compliance)**: Enforce strict input validation, zero hardcoded secrets, OCI post-quantum image signatures, and compliance with GDPR, HIPAA, WCAG 2.1 AA, and ISO 27001 standards.

---

## 10. Recommended Phased Implementation Sequence

1. **Phase 1: Compiler & Transmute Hardening**: Fix Rust atomic transmutation mismatches across `src/package/`.
2. **Phase 2: Sovereign AUR Sandbox Expansion**: Mandate `poudriere` chroot and `unveil` path isolation for all package builds.
3. **Phase 3: Extensible BPF Scheduling & Capsicum Sandbox**: Integrate `sched_ext` BPF hooks and Capsicum fd capability sandboxes.
4. **Phase 4: ASP / ABS Source Tree Checkout**: Integrate Git-backed `.SRCINFO` PKGBUILD checkout routines in `src/sigpkg/arch_pacman_engine.rs`.
5. **Phase 5: Fedora Forgejo OCI Container Image Registry**: Integrate `ForgejoOciImageEngine` into `src/container/` for zero-trust OCI container deployments.
6. **Phase 6: Calamares-style Installer Plugin Modularization**: Refactor `installer/sigma-installer.rs` into modular Rust plugin modules.
7. **Phase 7: Web UI Zero-JS Progressive Enhancement & Search**: Enhance `web_ui/index.html` with OpenBSD-style zero-JS fallbacks and client-side package option search.
8. **Phase 8: System Manual Page Standardization**: Author system tool man pages in `docs/man/` using `mdoc(7)` macro syntax with `mandoc -Tlint` CI validation.
9. **Phase 9: Repository Infrastructure Geo-Routing & Signed Caches**: Enable DNS SRV auto-discovery and Ed25519 binary cache verification in `src/sigpkg/repository_manager.rs`.
10. **Phase 10: Multi-OS Package Translators**: Enable seamless conversion between `.pkg.tar.zst`, `.deb`, `.rpm`, `.apk`, `.xbps`, and FreeBSD `.pkg` formats.

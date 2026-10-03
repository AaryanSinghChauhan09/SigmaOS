# SigmaOS AI Agent Roadmap: Universal Package Management & Distro Format Absorption

## Overview & Package Supremacy Architecture
This document details the AI engineering agent roadmap for `SigmaPkg` and the universal package interoperability layer (`SovereignUniversalPackageInteropEngine`). The goal is total absorption and zero-dependency translation of 29+ Linux, BSD, and container package formats into native, cryptographic `SigmaPkg` archives with DPLL SAT dependency resolution and OpenBSD pledge/unveil scriptlet sandboxing.

---

## 1. Multi-Format Absorption Engine (`SigmaPkg Interop`)

### Targeted Package Formats
- **Arch Linux:** Pacman `.pkg.tar.zst`, `PKGBUILD` scripts, and AUR metadata ingestion.
- **Debian / Ubuntu:** Apt `.deb` control files and `dpkg` maintainer script transpilation.
- **Red Hat / Fedora:** Dnf / Rpm `.rpm` spec parsing and CPIO archive extraction.
- **NixOS / Guix:** Nix Flakes declaratively-evaluated store path mapping (`/nix/store/` -> `/sigma/store/`).
- **Gentoo:** Portage EAPI 8 ebuild bash AST transpilation to Rust execution graphs.
- **Alpine Linux:** APK v3 `.apk` index parsing and tarball extraction.
- **Void Linux:** XBPS `.xbps` binary package ingestion.
- **FreeBSD / OpenBSD / NetBSD:** FreeBSD `pkg` `.pkg`, OpenBSD ports `PLIST`, NetBSD pkgsrc `buildlink3`.
- **Containers & Sandboxed Formats:** Flatpak, Snap, AppImage, OCI image layers.

### AI Agent Execution Directives
1. **Zero-Allocation Dependency Normalization:**
   - Optimize package name and version comparison functions in `src/package/universal.rs` and `src/package/sovereign_distro_package_advancements_v7.rs` using `eq_ignore_ascii_case` or zero-allocation ASCII window matching (`contains_ignore_case`).
2. **DPLL SAT Solver Engine:**
   - Enforce fast, non-backtracking SAT dependency graph resolution in `src/package/resolver.rs` supporting conflict resolution, virtual provides, and version constraints.
3. **Pledge/Unveil Scriptlet Sandboxing:**
   - Execute pre-install/post-install package scriptlets inside `SovereignPortableSandboxEngine` restricted by OpenBSD pledge promises (`stdio rpath wpath cpath`) to prevent rogue package scripts from corrupting system state.

---

## 2. Mirrorlist Benchmarking & Repository Synchronization

### Distro Inspiration Sources
- **Arch Linux `pacman-mirrors` / `reflector`:** Parallel HTTP/HTTPS speed benchmarking and latency-based mirror list sorting.
- **Gentoo `mirrorselect`:** Automated rsync/HTTP mirror probing and geographical routing.

### AI Agent Execution Directives
1. **Parallel Mirror Probing:**
   - Maintain `PacmanMirrorManager` (`src/package/pacman_mirrors.rs`) for asynchronous latency scoring, GPG/Signify key verification, and repo prioritization (`[core]`, `[extra]`, `[multilib]`).
2. **PQC Dilithium-5 Metadata Signatures:**
   - Verify repository sync manifests (`repository_manager.rs`) using hybrid Dilithium-5 and Signify keyrings before initiating system upgrades.

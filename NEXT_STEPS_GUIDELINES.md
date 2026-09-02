# SigmaOS Next Steps Guidelines & Multi-OS Distro Integration Roadmap

## Executive Summary
This document provides concrete execution guidelines and an architectural roadmap for developers and maintainers contributing to **SigmaOS**. It integrates multi-OS inspirations from Linux (Arch, Gentoo, Void, NixOS, Alpine) and BSD (FreeBSD, OpenBSD, NetBSD) ecosystems, focusing heavily on enhancing the **SigmaOS User Repository (AUR / Sovereign AUR)**.

---

## 1. Multi-OS Distro Inspired AUR Architecture Guidelines

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

## 2. General Engineering & Quality Guidelines

### A. Code Quality & Type Safety
- **Rust Atomic Enum Transmutes**: Ensure all enums backed by atomic store operations are marked with `#[repr(usize)]` or `#[repr(u32)]` to match platform word sizes and eliminate transmute size mismatches.
- **Linting & Warnings**: Fix unused variables and unneeded `mut` annotations in `src/sigpkg/` and `src/driver/`.

### B. Tri-Agent Autonomous Principles
- **Bolt ⚡ (Performance)**: Prioritize zero-copy allocations, SLUB slab caches, and lock-free atomic swaps.
- **Palette 🎨 (UX & Accessibility)**: Enforce ARIA labels (`aria-label`, `aria-checked`), keyboard focus navigation (`focus-visible:ring-2`), and high-contrast desktop themes.
- **Sentinel 🛡️ (Security & Compliance)**: Enforce strict input validation, zero hardcoded secrets, and compliance with GDPR, HIPAA, WCAG 2.1 AA, and ISO 27001 standards.

---

## 3. Recommended Phased Implementation Sequence

1. **Phase 1: Compiler & Transmute Hardening**: Fix Rust atomic transmutation mismatches across `src/package/`.
2. **Phase 2: Sovereign AUR Sandbox Expansion**: Mandate `poudriere` chroot and `unveil` path isolation for all package builds.
3. **Phase 3: Multi-OS Package Translators**: Enable seamless conversion between `.pkg.tar.zst`, `.deb`, `.rpm`, `.apk`, `.xbps`, and FreeBSD `.pkg` formats.
4. **Phase 4: Multi-Seat Desktop & Driver Management**: Integrate PAM/BSD-auth multi-seat controls and NVIDIA PRIME hybrid graphics profile switching.

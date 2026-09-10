# AI Agent Gentoo Parity Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing Gentoo Linux compatibility components within **SigmaOS**.

---

## 1. Overview & Gentoo Parity Subsystem

SigmaOS implements a clean-room, zero-external-dependency Gentoo Linux parity subsystem across `src/distro/gentoo.rs`, `src/distro/gentoo_inspirations.rs`, `src/sigpkg/gentoo_use_flags.rs`, and `src/sigpkg/portage.rs`.

Key components managed by AI agents:

1. **Portage USE-Flag Governor & Solver (`PortageUseFlagGovernor`)**:
   - Parses global and package-specific USE flags (`/etc/portage/make.conf`, `/etc/portage/package.use`), resolving conditional dependencies (`USE="wayland -X"`).
2. **Ebuild Specification Parser & Metadata Extractor (`GentooEbuildMetadata`)**:
   - Parses Gentoo `.ebuild` files (`EAPI`, `DESCRIPTION`, `HOMEPAGE`, `SRC_URI`, `LICENSE`, `SLOT`, `KEYWORDS`, `IUSE`, `RDEPEND`, `DEPEND`).
3. **Portage Package Masking & Unmasking Engine (`PortagePackageMaskEngine`)**:
   - Evaluates package masks (`package.mask`, `package.unmask`, `package.accept_keywords`) to restrict experimental or unstable software builds.
4. **Gentoo Portage Slotting & Dual-Version Coexistence (`PortageSlotEngine`)**:
   - Manages library and application slots (e.g. `SLOT="0/30"`, `SLOT="python3.11"`), allowing multiple version branches to coexist without path collisions.

---

## 2. Rules for AI Agents Developing Gentoo Parity Modules

1. **Zero External Dependencies**:
   - All ebuild parsing and USE flag dependency constraint solvers must use safe Rust or `klib` primitives.
2. **USE Flag Dependency Semantics**:
   - Support conditional dependency expressions (`use_flag? ( dep )`, `!use_flag? ( dep )`, `|| ( dep1 dep2 )`).
3. **Reproducible Ebuild Environment**:
   - Sandbox build phases (`src_unpack`, `src_prepare`, `src_configure`, `src_compile`, `src_install`) within capability zones.

---

## 3. Verification Commands

AI agents must verify Gentoo compatibility changes using:

```bash
rustc --edition=2021 --crate-type=lib src/lib.rs -o libsigmaos.rlib
```

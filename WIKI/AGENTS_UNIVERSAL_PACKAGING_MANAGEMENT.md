# AI Agent Universal Packaging Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing the Universal Packaging System within **SigmaOS**.

---

## 1. Overview & Universal Packaging System

SigmaOS implements a clean-room, zero-external-dependency universal multi-format packaging system across `src/sigpkg/universal_oop_system.rs`, `src/sigpkg/universal_engine.rs`, `src/sigpkg/spec.rs`, `src/package/universal.rs`, and `src/sigpkg/universal_adapter.rs`.

Key components managed by AI agents:

1. **29+ Universal Package Format Adapters (`PackageFormatAdapter` / `UniversalPackageAdapter`)**:
   - Parses, validates, and translates foreign package formats including `.air`, `.bottle`, `.ipa`, `.ports`, `.pkg`, `.aab`, `.apk`, AppImage, `.eopkg`, `.nixpkg`, `.portage`, `.deb`, `.tar.gz`, `.xz`, `.rpm`, `.ebuild`, `.pkg.tar.xz`, Flatpak, `.app`, `.hap`, `.PiSi`, `.tgz`, `.superdeb`, `.lzm`, pup, `.snap`, pacman, `.tar`, `.pet`, etc.
2. **Canonical Dependency Name Mapper (`UniversalDependencyMapper`)**:
   - Converts distro-specific dependency names (`libffi-dev`, `glib2-devel`, `libpcre2-dev`, `libuv-devel`, `net-misc/openssh`, `media-libs/mesa`, `dev-vcs/git`, `dev-build/cmake`) into standardized SigmaOS package names (`libffi`, `glib`, `pcre`, `libuv`, `openssh`, `mesa`, `git`, `cmake`).
3. **Foreign Package Manager Command Dispatcher (`UniversalPmCommandDispatcher`)**:
   - Translates foreign CLI commands (`apt install`, `pacman -S`, `dnf remove`, `microdnf`, `apk add`, `pkg_add`, `yay`, `emerge`) into unified `DispatchedPmAction` operations (`Install`, `Remove`, `Upgrade`, `Search`, `QueryInfo`, `CleanCache`).
4. **Universal Package Bridge Engine (`SigPkgUniversalBridgeEngine`)**:
   - Converts foreign packages directly into native `Package` structures with Content-Addressed Storage (CAS) deduplication and registers them into `UniversalPackageManager`.
5. **Dry-Run Installation Simulator (`UniversalDryRunSimulator`)**:
   - Simulates package installations, resolving dependencies and permissions without committing disk state transitions.

---

## 2. Rules for AI Agents Developing Packaging Modules

1. **Zero External Dependencies**:
   - All format parsers, dependency mappers, and bridge engines must use safe Rust or `klib` primitives.
2. **Extension & Magic Byte Auto-Detection**:
   - Always support both extension matching (`detect_format_by_extension`) and header signature magic byte inspection (`detect_format_by_header`).
3. **Atomic Rollbacks**:
   - Foreign package state transitions must trigger Btrfs CoW or ZFS subvolume snapshots for instant rollback.

---

## 3. Verification Commands

AI agents must verify universal packaging changes using:

```bash
# Run universal adapter integration tests
rustc --edition=2021 --crate-type=lib src/lib.rs -o libsigmaos.rlib
rustc --edition=2021 --test tests/test_universal_adapter.rs --extern sigmaos=libsigmaos.rlib -o build/test_adapter && ./build/test_adapter
```

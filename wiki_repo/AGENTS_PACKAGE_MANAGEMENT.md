# AI Agent Package Management Specification for SigmaOS

This document provides guidelines and specifications for AI agents developing, parsing, translating, and managing software packages within the **SigmaOS** ecosystem.

---

## 1. Overview & Universal Package Subsystem

SigmaOS implements a sovereign, multi-format Universal Package Manager capable of absorbing, translating, and executing packages across all major Linux and BSD distribution formats:

- **Debian / Ubuntu**: `.deb`, `.udeb`, `.superdeb` (APT)
- **Red Hat / Fedora / SUSE**: `.rpm`, `.drpm`, `.spec`, `.zypper` (DNF/Yum)
- **Arch Linux**: `.pkg.tar.zst`, `.pkg.tar.xz`, `PKGBUILD` (Pacman/AUR)
- **Gentoo**: `.ebuild`, `.portage` (Portage USE flags)
- **Alpine Linux**: `.apk`, `APKINDEX` (APK v3)
- **Void Linux**: `.xbps` (XBPS)
- **Solus**: `.eopkg`, `.moss` (eopkg/Moss)
- **Containerized Apps**: `.snap`, `.flatpak`, `.appimage`
- **BSD Distributions**: `.pkg` (FreeBSD UCL), `+CONTENTS` (OpenBSD), `pkgsrc` (NetBSD), `.ports`
- **Haiku OS**: `.hpkg`
- **SigmaOS Native**: `.sigpkg`, `.sigma`

---

## 2. Core OOP Design Patterns for Package Management

AI agents extending the package subsystem in `src/sigpkg/universal_oop_system.rs` and `src/sigpkg/universal_adapter.rs` must adhere to the following design patterns:

1. **Strategy Pattern (`IPackageParser`)**:
   - Every package format implements `IPackageParser` with `can_parse()`, `parse()`, and `serialize()`.

2. **Factory Pattern (`PackageParserFactory`)**:
   - Uses `auto_detect_parser()` via file extension or header magic byte inspection to instantiate the correct parser.

3. **Adapter Pattern (`BaseAdapter` & `UniversalFormatAdapterRouter`)**:
   - Translates foreign package metadata into canonical `Package` and `PackageMetadata` structures.

4. **Decorator Pattern (`SandboxedPackageDecorator`, `AuditedPackageDecorator`, `PqcSignedPackageDecorator`)**:
   - Enhances packages dynamically with security audits, post-quantum signatures (Dilithium-5), and OpenBSD pledge/unveil sandbox rules.

5. **Observer Pattern (`PackageEventManager` & `IPackageObserver`)**:
   - Dispatches `PackageEvent` notifications (`Installed`, `Removed`, `FileDiverted`, `AlternativeSwitched`) to registered listeners.

6. **Command Pattern (`IPackageCommand` & `TransactionRollbackExecutor`)**:
   - Enables transactional, atomic, and fully reversible package installations and rollbacks.

---

## 3. Dependency Canonicalization & Scriptlet Transpilation

1. **Canonical Dependency Mapping**:
   - AI agents must map foreign dependency names to canonical Sigma-pkg dependency names using `UniversalDependencyMapper::to_canonical_name()`:
     - `libssl-dev` / `openssl-devel` / `dev-libs/openssl` -> `openssl`
     - `libc6` / `glibc` / `musl-dev` -> `libc`
     - `zlib1g-dev` / `zlib-devel` -> `zlib`

2. **Scriptlet Hook Transpilation**:
   - Transpile foreign maintainer scripts into native `SigmaPkgHookType` lifecycle hooks (`PreInstall`, `PostInstall`, `PreRemove`, `PostRemove`) using `UniversalScriptletConverter`.

3. **Capability Mapping**:
   - Map container permissions (Plugs, Finish-args) directly into SigmaOS Capability Gate Permissions (`Permission::NetworkTcp`, `Permission::FileRead`, `Permission::Ipc`, `Permission::ProcessExec`).

---

## 4. Testing & Verification

When adding or updating package management components, verify via:

```bash
# Run universal package adapter standalone test
rustc --test --edition 2021 tests/test_universal_adapter.rs -o build/test_universal_adapter && ./build/test_universal_adapter

# Run full test runner
./run_sigma_tests.sh
```

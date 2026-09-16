# AI Agent Guidelines for SigmaOS Universal Package Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, maintaining, or extending the **SigmaOS Universal Package Management Subsystem**.

---

## 1. System Architecture & Subsystem Layout

SigmaOS implements a universal, cross-distro package management architecture across two primary Rust subsystems:

1. **`src/package/` (Core System Package Manager)**
   - `src/package/universal.rs`: Universal package models (`UnifiedPackage`), format enums (`PackageFormat`), strategy pattern handlers, OOP package capability decorators (`HardwareOptimizationDecorator`, `ResourceLimitDecorator`, `PqcSignedDecorator`), trigger registry (`PackageTriggerRegistry`), and filename extension detection (`PackageFormat::from_filename`).
   - `src/package/manager.rs`: High-level package manager coordinator and repository synchronization.
   - `src/package/sigma_pkg.rs`: Native `.spkg` archive reader, installer, and verification engine.
   - `src/package/debian.rs` & `debian_apt.rs`: Debconf, statoverride, usrmerge, and Debian package translation engines.
   - `src/package/bsd_linux_package_innovations.rs`: Package innovations inspired by FreeBSD Ports, Void XBPS, Alpine APK, Gentoo Portage, Haiku HPKG, openSUSE Zypper, Solus Moss, and Slackware Pkgtool.

2. **`src/sigpkg/` (Universal Package Adapter & OOP Engine)**
   - `src/sigpkg/universal_adapter.rs`: Manifest parsers, extension auto-detection (`detect_format_by_extension`), magic byte header verification (`detect_format_by_header`), foreign CLI command dispatchers (`UniversalPmCommandDispatcher`), and canonical dependency mapping (`UniversalDependencyMapper`).
   - `src/sigpkg/universal_engine.rs`: High-level multi-distro bridge engine (`SigPkgUniversalBridgeEngine`) and Content-Addressed Store (CAS) storage governor.
   - `src/sigpkg/universal_oop_system.rs`: OOP design pattern architecture (Factory, Strategy, Observer, Decorator, Command, UDF pipeline, Boolean dependency solver, package delta patch engine, software alternatives manager) and `PackageFormat` auto-detection.

---

## 2. Universal Package Format Matrix

SigmaOS supports **64+ package formats** across Linux, BSD, Unix, macOS, mobile, HPC, and language ecosystems. AI agents modifying package handling must update all primary package system modules:
1. `src/package/universal.rs`
2. `src/sigpkg/universal_adapter.rs`
3. `src/sigpkg/universal_engine.rs`
4. `src/sigpkg/universal_oop_system.rs`

### Format Magic Headers (`detect_format_by_header`)

When extending binary signature parsing, register the magic byte header signature in `src/sigpkg/universal_adapter.rs`:

| Format | File Extension | Magic Header Bytes | Primary Subsystem / Ecosystem |
| :--- | :--- | :--- | :--- |
| **Apt / Deb** | `.deb`, `.udeb` | `!<arch>\n` | Debian / Ubuntu |
| **Yum / RPM** | `.rpm`, `.drpm` | `0xED 0xAB 0xEE 0xDB` / `DRPM` | RHEL / Fedora / openSUSE |
| **OpenWrt IPK** | `.ipk` | `IPK!` | OpenWrt / opkg / Entware |
| **Yocto OPKG** | `.opkg` | `OPKG` | Yocto Project / OpenEmbedded |
| **Solaris IPS** | `.p5p`, `.ips` | `P5P!` | Solaris / Illumos |
| **Nix / Guix NAR** | `.nar` | `NARS` | NixOS / GNU Guix |
| **OpenBSD PKG** | `.openbsd.tgz` | `OBSD` | OpenBSD `pkg_add` |
| **HPC Spack** | `.spack` | `SPAK` | Spack HPC Package Manager |
| **C/C++ Conan** | `.conan` | `CONA` | Conan C/C++ Package Manager |
| **Python Wheel** | `.whl` | `WHEL` | Python PyPI |
| **Cargo Crate** | `.crate` | `CRAT` | Rust Crates.io |
| **RubyGem** | `.gem` | `GEMS` | RubyGems |
| **NuGet** | `.nupkg` | `NUPK` | .NET NuGet |
| **Vcpkg** | `.vcpkg` | `VCPK` | Microsoft Vcpkg |
| **Nix NarInfo** | `.narinfo` | `NARI` | Nix / Guix Substituter Manifest |
| **Haiku HPKG** | `.hpkg` | `hpkg` | Haiku OS |
| **Solus Moss** | `.moss` | `MOSS` | Solus OS |
| **SquashFS / TCZ** | `.tcz`, `.sfs` | `hsqs` / `sqsh` | Tiny Core / Puppy / Slax |
| **Apple DMG** | `.dmg` | `koly` | macOS Disk Image |
| **Bedrock Stratum** | `.stratum` | `BRLK` | Bedrock Linux |
| **Slackware** | `.slackbuild`, `.txz`, `.tlz`, `.tbz` | `SLAK` | Slackware Linux |
| **Void Linux** | `.xbps` | `XBPS` | Void Linux |
| **Tiny Core Linux** | `.tcz` | `sqsh` | Tiny Core Linux |
| **NetBSD** | `.pkgsrc` | `PKG3` | NetBSD / pkgsrc |
| **FreeBSD / DragonFly** | `.ports`, `.dports` | `PORT` | FreeBSD / DragonFly BSD |

---

## 3. OOP Design Patterns & User-Defined Functions (UDFs)

AI agents maintaining or adding package management capabilities must adhere to the following Object-Oriented Programming (OOP) design patterns in `src/package/universal.rs` and `src/sigpkg/universal_oop_system.rs`:

1. **Strategy Pattern (`InstallStrategy`, `IPackageDeltaStrategy`):**
   - Encapsulates package installation, verification, removal, and delta patch computation into format-specific strategies (`DebInstallStrategy`, `RpmInstallStrategy`, `DnfDeltaRpmStrategy`, `ZstdChunkedDeltaStrategy`, etc.).
2. **Adapter Pattern (`PackageMetadataAdapter`, `IPackageParser`):**
   - Translates foreign package metadata formats into standard `UnifiedPackage` and `IPackage` traits (`SerpentMossAdapter`, `FreeBsdPkgAdapter`, `AndroidAabApkAdapter`, `SystemdSysupdateAdapter`).
3. **Decorator Pattern (`PackageCapability`, `IPackageDecorator`):**
   - Dynamically adds features like sandboxing, hardware optimizations, resource limits, and Post-Quantum Cryptographic (PQC) Dilithium-5 signatures (`SandboxDecorator`, `HardwareOptimizationDecorator`, `ResourceLimitDecorator`, `PqcSignedDecorator`).
4. **Factory Pattern (`PackageFactory`, `PackageParserFactory`):**
   - Instantiates correct strategies, metadata adapters, and parsers based on `PackageFormat` or auto-detection.
5. **Observer Pattern (`PackageObserver`, `PackageTriggerRegistry`, `PackageEventManager`):**
   - Dispatches events and notifies observers when package state transitions occur during download, installation, or removal.
6. **User-Defined Function (UDF) Pipelines (`UserDefinedFunctionPipeline`, `UserDefinedPackageTransformPipeline`):**
   - Allows users and AI agents to register custom closure hooks and transform scriptlets executed during build/install lifecycle phases (`UserDefinedScriptletHook`).
7. **Boolean Dependency Solver (`SovereignBooleanDependencySolver`, `BooleanDependencyExpr`):**
   - Solves rich boolean dependency expressions (`and`, `or`, `if`, `unless`, `not`) for modern distribution formats like DNF5 and APT 2.9.
8. **Alternatives & Diverter Manager (`SovereignUniversalAlternativesManager`):**
   - Unified facade combining Debian file diversions (`DebianDiverterEngine`), update-alternatives (`SovereignAlternativesEngine`), and Gentoo eselect module switching.

---

## 4. Package Manager CLI Dispatcher (`UniversalPmCommandDispatcher`)

`UniversalPmCommandDispatcher` in `src/sigpkg/universal_adapter.rs` maps foreign PM command invocations into canonical SigmaOS actions (`UniversalPmOperation`):

- **Supported PM Binaries:** `apt`, `apt-get`, `dpkg`, `pacman`, `dnf`, `yum`, `zypper`, `apk`, `opkg`, `ipkg`, `pkg`, `pkg_add`, `pkgsend`, `spack`, `conan`, `pip`, `cargo`, `gem`, `nuget`, `vcpkg`, `xbps-install`, `xbps-remove`, `xbps-query`.
- **Action Mapping:** `Install`, `Remove`, `Upgrade`, `Search`, `QueryInfo`, `CleanCache`.

---

## 5. Testing & Verification Protocol for AI Agents

When making changes to package system source files, AI agents must run the following validation commands in order:

### 1. Standalone Module Test Execution
Automated standalone compilation verifies that modified Rust source files compile and pass their unit test suites independently:

```bash
# Test src/package/universal.rs
rustc --test --cfg 'feature="standalone_test"' src/package/universal.rs --edition=2021 -o /tmp/package_test && /tmp/package_test

# Test src/sigpkg/universal_oop_system.rs
rustc --test src/sigpkg/universal_oop_system.rs --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_sigpkg_oop && /tmp/test_sigpkg_oop

# Test src/sigpkg/universal_adapter.rs
rustc --test --edition=2021 src/sigpkg/universal_adapter.rs -o /tmp/test_adapter && /tmp/test_adapter
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate native C++ test runners, inspection test binaries, Python test suites, and core kernel/package subsystems:

```bash
./run_sigma_tests.sh
```

---

## 6. Coding Standards & Import Conventions

- **`#![no_std]` Compatibility:** When editing core kernel or gap closure modules (e.g. `src/open_source_os_gap_closure.rs`), use `extern crate alloc;` and import types from `alloc::` (e.g. `alloc::string::String`, `alloc::vec::Vec`, `alloc::collections::BTreeMap`) rather than `std::`.
- **`PackageFormat` Default Trait Implementation:** `PackageFormat` implements `Default` manually via `impl Default for PackageFormat { fn default() -> Self { Self::Deb } }` to avoid `derive(Default)` attribute collision issues across crates.
- **Standalone Test Conditional Imports:** For standalone `rustc --test` compilation in package modules, import `HashMap` conditionally:
  ```rust
  #[cfg(any(feature = "standalone_test", test))]
  use std::collections::HashMap;
  ```
- **Verification Rule:** Always use `read_file` or `list_files` after modifying codebase files to confirm that all edits were correctly applied.

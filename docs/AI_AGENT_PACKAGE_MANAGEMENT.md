# AI Agent Guidelines for SigmaOS Universal Package Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending the **SigmaOS Universal Package Management Subsystem**.

---

## 1. System Architecture & Subsystem Layout

SigmaOS implements a universal, cross-distro package management architecture across two primary Rust subsystems:

1. **`src/package/` (Core System Package Manager)**
   - `src/package/universal.rs`: Universal package models (`UnifiedPackage`), format enums (`PackageFormat`), strategy pattern handlers, and filename extension detection (`PackageFormat::from_filename`).
   - `src/package/manager.rs`: High-level package manager coordinator and repository synchronization.
   - `src/package/sigma_pkg.rs`: Native `.spkg` archive reader, installer, and verification engine.
   - `src/package/debian.rs` & `debian_apt.rs`: Debconf, statoverride, usrmerge, and Debian package translation engines.
   - `src/package/bsd_linux_package_innovations.rs`: Package innovations inspired by FreeBSD Ports, Void XBPS, Alpine APK, Gentoo Portage, Haiku HPKG, openSUSE Zypper, Solus Moss, and Slackware Pkgtool.

2. **`src/sigpkg/` (Universal Package Adapter & OOP Engine)**
   - `src/sigpkg/universal_adapter.rs`: Manifest parsers, extension auto-detection (`detect_format_by_extension`), magic byte header verification (`detect_format_by_header`), foreign CLI command dispatchers (`UniversalPmCommandDispatcher`), and canonical dependency mapping (`UniversalDependencyMapper`).
   - `src/sigpkg/universal_engine.rs`: High-level multi-distro bridge engine (`SigPkgUniversalBridgeEngine`) and Content-Addressed Store (CAS) storage governor.
   - `src/sigpkg/universal_oop_system.rs`: OOP design pattern architecture (Factory, Strategy, Observer, Decorator, Command, UDF pipeline) and `PackageFormat` auto-detection.

---

## 2. Universal Package Format Matrix

SigmaOS supports **64+ package formats** across Linux, BSD, Unix, macOS, mobile, HPC, and language ecosystems. AI agents modifying package handling must update all four primary files:
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
| **Slackware** | `.slackbuild` | `SLAK` | Slackware Linux |

---

## 3. Package Manager CLI Dispatcher (`UniversalPmCommandDispatcher`)

`UniversalPmCommandDispatcher` in `src/sigpkg/universal_adapter.rs` maps foreign PM command invocations into canonical SigmaOS actions (`UniversalPmOperation`):

- **Supported PM Binaries:** `apt`, `apt-get`, `dpkg`, `pacman`, `dnf`, `yum`, `zypper`, `apk`, `opkg`, `ipkg`, `pkg`, `pkg_add`, `pkgsend`, `spack`, `conan`, `pip`, `cargo`, `gem`, `nuget`, `vcpkg`, `xbps-install`, `xbps-remove`, `xbps-query`.
- **Action Mapping:** `Install`, `Remove`, `Upgrade`, `Search`, `QueryInfo`, `CleanCache`.

---

## 4. Testing & Verification Protocol for AI Agents

When making changes to package system source files, AI agents must run the following validation commands in order:

### 1. Standalone Module Test Execution
Automated standalone compilation verifies that modified Rust source files compile and pass their unit test suites independently:

```bash
./scripts/changed_files_rustc_tests.sh
```

Alternatively, test specific package modules directly:

```bash
rustc --test --edition=2021 src/sigpkg/universal_adapter.rs -o build/test_adapter && ./build/test_adapter
rustc --test --edition=2021 src/sigpkg/universal_oop_system.rs -o build/test_oop && ./build/test_oop
rustc --test --edition=2021 src/package/bsd_linux_package_innovations.rs -o build/test_innovations && ./build/test_innovations
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate native C++ test runners, inspection test binaries, Python test suites, and core kernel/package subsystems:

```bash
./run_sigma_tests.sh
```

---

## 5. Coding Standards & Import Conventions

- **`#![no_std]` Compatibility:** When editing core kernel or gap closure modules (e.g. `src/open_source_os_gap_closure.rs`), use `extern crate alloc;` and import types from `alloc::` (e.g. `alloc::string::String`, `alloc::vec::Vec`, `alloc::collections::BTreeMap`) rather than `std::`.
- **Standalone Test Conditional Imports:** For standalone `rustc --test` compilation in package modules, import `HashMap` conditionally:
  ```rust
  #[cfg(any(feature = "standalone_test", test))]
  use std::collections::HashMap;
  ```
- **Verification Rule:** Always use `read_file` or `list_files` after modifying codebase files to confirm that all edits were correctly applied.

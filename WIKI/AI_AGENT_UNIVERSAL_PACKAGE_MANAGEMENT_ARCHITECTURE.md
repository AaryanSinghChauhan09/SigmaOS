# AI Agent Universal Package Management Architecture

## Executive Overview

Universal Package Management in SigmaOS allows native `Sigma-pkg` (`src/bin/sigpkg.rs`) to absorb, parse, translate, and execute foreign package formats from Linux and BSD distributions without requiring native distribution package managers or external runtimes. Implemented across `src/sigpkg/universal_adapter.rs`, `src/sigpkg/universal_oop_system.rs`, and `src/sigpkg/universal_engine.rs`, SigmaOS natively handles Debian `.deb` / `apt`, RedHat `.rpm` / `dnf` / `zypper`, Arch Linux `PKGBUILD` / `.PKGINFO` / `pacman`, Gentoo `.ebuild` / `portage`, Alpine `.apk`, Void `.xbps`, Haiku `.hpkg`, FreeBSD `pkg` UCL, OpenBSD `pkg_add`, NetBSD `pkgsrc`, Snap `snapcraft.yaml`, Flatpak JSON manifests, and Nix flakes with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents inspecting, extending, or maintaining foreign package bridges and Universal PM command dispatchers in SigmaOS.

---

## Subsystem Architecture & Foreign Package Bridge Pipeline

```
                                +-----------------------------------+
                                |    Foreign Command / Package      |
                                | (apt, pacman, dnf, apk, .deb, .rpm)|
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |   UniversalPmCommandDispatcher    |
                                |  (src/sigpkg/universal_adapter)   |
                                +-----------------------------------+
                                 /                |                \
                                /                 |                 \
            +-----------------------+   +-------------------+   +-----------------------+
            | UniversalFormatConv   |   | SigPkgUniversal    |  | SandboxCapabilityMat  |
            | parse_apt_control()   |   | BridgeEngine      |  | Snap Plugs/Flatpak    |
            | parse_pacman_pkgbuild |   | convert_to_sigpkg |  | -> SigmaOS Permission |
            +-----------------------+   +-------------------+   +-----------------------+
                                \                 |                 /
                                 \                |                /
                                  v               v               v
                                +-----------------------------------+
                                | UniversalDependencyMapper         |
                                | to_canonical_name("libssl-dev")   |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                | Native Sigma-pkg Package Model    |
                                +-----------------------------------+
```

### Core Packaging Components

1. **`UniversalPackageAdapter` & Parsers (`src/sigpkg/universal_adapter.rs`)**:
   - Format Auto-Detection: `detect_format_by_extension` and `detect_format_by_header` recognize header magic bytes (e.g., `!<arch>\n` for `.deb`, `0xEDABEEDB` for `.rpm`, `hpkg` for Haiku, `SPKG` for SigPkg).
   - Manifest Parsers: Dedicated zero-alloc line parsers for Debian `control`, Arch `PKGBUILD` / `.PKGINFO`, RedHat `.spec`, Gentoo `.ebuild`, Alpine `APKINDEX`, Void `XBPS`, FreeBSD UCL `+MANIFEST`, OpenBSD `+CONTENTS`, NetBSD `pkgsrc`, Snap `snapcraft.yaml`, and Flatpak JSON manifests.

2. **`SigPkgUniversalBridgeEngine`**:
   - `convert_to_sigpkg(filename, raw_data)`: Automatically detects format, parses metadata, canonicalizes dependencies, and maps permissions into a native `Package` instance.
   - `absorb_and_register(filename, raw_data)`: Registers converted foreign packages directly into `UniversalPackageManager`.

3. **Dependency & Scriptlet Transpilation**:
   - `UniversalDependencyMapper`: Maps distro-specific dependency names (`libssl-dev`, `openssl-devel`, `dev-libs/openssl`, `security/openssl`) to the canonical SigmaOS package name (`openssl`).
   - `UniversalScriptletConverter`: Transpiles foreign maintainer scripts (`postinst`, `%post`, `post-install`, `pkg_postinst`) into `SigmaPkgHookType::PostInstall` hooks.
   - `UniversalSandboxCapabilityMatrix`: Translates Snap plugs (`network`, `home`, `audio`) and Flatpak finish-args (`--share=network`, `--filesystem=home`) into native SigmaOS Capability permissions (`Permission::NetworkTcp`, `Permission::FileRead`).

4. **Foreign PM Command Dispatcher (`UniversalPmCommandDispatcher`)**:
   - Translates foreign CLI commands (`apt install`, `pacman -Syu`, `dnf remove`, `apk add`, `pkg install`, `emerge`, `nix-env`) into canonical `DispatchedPmAction` operations (`Install`, `Remove`, `Upgrade`, `Search`, `QueryInfo`).

---

## Zero-Allocation Guardrails

AI agents modifying packaging bridges must enforce these constraints:
- Header detection checks slice byte prefix signatures (`data.starts_with(...)`) in $O(1)$ time without heap reallocation.
- Text line tokenization uses borrow-based iterators (`text.lines()`) without allocating intermediary string vectors.

---

## Related Architectural References
- `src/sigpkg/universal_adapter.rs` - Primary universal package adapter engine.
- `src/sigpkg/universal_oop_system.rs` - Object-oriented packaging models.
- `src/bin/sigpkg.rs` - Master CLI executable binary.

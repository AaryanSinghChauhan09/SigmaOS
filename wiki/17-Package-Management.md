# Package Management

SigmaOS provides a universal package management subsystem (`sigpkg`) that natively supports all major Linux and BSD package formats through a unified foreign-PM translation layer and a suite of advancement modules (V3–V10).

## Overview

The package management stack consists of:

- **sigpkg** — The native SigmaOS package manager (`src/sigpkg/`)
- **Universal PM Layer** — Translates foreign package manager commands to sigpkg operations
- **Advancement Suites V3–V10** — Iterative feature additions per version cohort
- **BsdPkg / OpenBsdPorts support** — Full BSD package format support added in the Oct 2026 session

## sigpkg CLI

```bash
# Install a package
sigpkg install <package>

# Remove a package
sigpkg remove <package>

# Search for a package
sigpkg search <keyword>

# Upgrade all packages
sigpkg upgrade

# Query package info
sigpkg info <package>

# Clean package cache
sigpkg clean-cache

# Show dependency tree
sigpkg deptree <package>
```

## Universal Package Manager (Universal PM)

The Universal PM layer (`src/package/sovereign_universal_pm_pr_bridge.rs`) translates foreign PM commands from any supported distro into canonical sigpkg operations. It supports all formats in `UniversalDistroPackageFormat`.

### Supported Package Formats

| Format | Distro/System | File Type |
|--------|--------------|-----------|
| `AptDeb` | Debian, Ubuntu, Mint | `.deb` |
| `PacmanPkg` | Arch, Manjaro, CachyOS | `.pkg.tar.zst` / PKGBUILD |
| `DnfRpm` | Fedora, RHEL, CentOS | `.rpm` / `.spec` |
| `ZypperDeltaRpm` | openSUSE | `.drpm` |
| `AlpineApk` | Alpine Linux | `.apk` / APKBUILD |
| `VoidXbps` | Void Linux | `.xbps` / template |
| `GentooEbuild` | Gentoo | `.ebuild` |
| `FreeBsdPkg` | FreeBSD | `+MANIFEST` / ports |
| `BsdPkg` | NetBSD, DragonFly | `+MANIFEST` / pkg-plist |
| `OpenBsdPkg` | OpenBSD | `+CONTENTS` |
| `OpenBsdPorts` | OpenBSD | Makefile / ports tree |
| `NetBsdPkgsrc` | NetBSD | Makefile (pkgsrc) |
| `NixFlake` | NixOS | `flake.nix` / derivation |
| `GuixScheme` | GNU Guix | Scheme / nar |
| `FlatpakApp` | All distros | `.flatpakref` |
| `SnapApp` | Ubuntu-based | `snapcraft.yaml` / `.snap` |
| `AppImage` | All Linux | `.AppImage` |
| `SlackwareTxz` | Slackware | `.txz` / SlackBuild |
| `SolusEopkg` | Solus | `pspec.xml` / `.eopkg` |
| `HaikuHpkg` | Haiku OS | `.hpkg` |
| `SwupdBundle` | Clear Linux | bundle / manifest |
| `HomebrewBottle` | macOS / Linux | `.bottle.tar.gz` / Formula |
| `OciContainer` | All | OCI container tarball |
| `CargoCrate` | Rust ecosystem | `.crate` / Cargo.toml |
| `NativeSigPkg` | SigmaOS | `.sigpkg` |

### Foreign Command Translation

The `UniversalPmCommandDispatcher` parses foreign PM invocations and dispatches to sigpkg:

```rust
let dispatcher = UniversalPmCommandDispatcher::new();
let action = dispatcher.dispatch_command("apt install nginx")?;
// action.operation == UniversalPmOperation::Install
// action.target_packages == ["nginx"]
// action.source_pm == "apt"
```

Supported foreign PMs: `apt`, `pacman`, `dnf`, `apk`, `pkg` (FreeBSD), `zypper`, `xbps-install`, `emerge`, `flatpak`, `brew`.

### Dry-Run Simulation

```rust
let simulator = UniversalDryRunSimulator::new();
let result = simulator.simulate_install("nginx");
// Returns estimated disk usage, dependency count, conflicts
```

## Advancement Suites

Each suite version adds capabilities on top of previous versions:

### V3 — Foundation (`sovereign_distro_package_advancements_v3.rs`)
- Base package resolution and metadata normalization
- Cross-distro package name mapping

### V4 — Dependency Graph (`sovereign_distro_package_advancements_v4.rs`)
- SAT-based dependency resolution
- Conflict detection

### V5 — Signing & Provenance (`sovereign_distro_package_advancements_v5.rs`)
- PGP/PQC package signature verification
- Build provenance attestation (SLSA level 3)

### V6 — Optimization (`sovereign_distro_package_advancements_v6.rs`)
- PGO/FDO build flag injection (`-fprofile-use`, `-fprofile-sample-use`, LTO, BOLT)
- Arch CachyOS microarchitecture optimization engine
- Reproducible build auditor (SOURCE_DATE_EPOCH, ELF build-id)

### V7 — BSD Parity (`sovereign_distro_package_advancements_v7.rs`)
- FreeBSD ports flavours and VUXML security advisory integration
- NetBSD pkgsrc options framework
- OpenBSD signify binary integrity engine
- DragonFly dports Hammer2 snapshot engine

### V8 — Sandbox & Mirror (`sovereign_distro_package_advancements_v8.rs`)
- MicroVM hermetic package sandbox (`SovereignMicrovmHermeticPackageSandboxEngine`)
- PQC multi-keyring trust governor
- AI-optimized mirror ranking
- Atomic boot environment snapshot
- Cross-distro SONAME ABI verifier

`SovereignDistroPackageAdvancementsSuiteV8` fields:

| Field | Type | Purpose |
|-------|------|---------|
| `sandbox_engine` | `SovereignMicrovmHermeticPackageSandboxEngine` | MicroVM install isolation |
| `trust_governor` | `SovereignPqcMultiKeyringPackageTrustGovernor` | PQC multi-keyring trust |
| `mirror_governor` | `SovereignAiOptimizedMirrorRankingGovernor` | AI mirror selection |
| `boot_snapshot_engine` | `SovereignAtomicBootEnvironmentPackageSnapshotEngine` | Boot snapshots |
| `soname_verifier` | `SovereignCrossDistroSonameAbiVerifierEngine` | ABI compat check |
| `sat_resolver` | `SovereignUniversalSatDependencyResolver` | SAT dep resolution |
| `sig_verifier` | `SovereignUniversalPackageSignatureVerifier` | Signature verification |
| `trigger_engine` | `SovereignUniversalSystemTriggerIntegratorEngine` | ldconfig/desktop triggers |

### V9 — Universal PR Bridge (`sovereign_distro_package_advancements_v9.rs`)
- Pull-request–style package workflow for universal PM
- Cross-distro PR gateway (`SovereignUniversalPrGatewayEngine`)
- Universal PM PR bridge (`SovereignUniversalPmPrBridgeEngine`)

### V10 — Foreign Package Transpiler (`sovereign_distro_package_advancements_v10.rs`)
- `UniversalForeignPackageFormat` enum for autodetecting manifest format from text
- `SovereignDistroPackageAdvancementsSuiteV10` — master orchestrator for V10 features
- Manifest transpilation: converts any foreign format to `.sigpkg`

```rust
let mut suite = SovereignDistroPackageAdvancementsSuiteV10::new();
let format = suite.autodetect_format("pkgname=nginx\npkgver=1.25.3\n");
// format == UniversalForeignPackageFormat::ArchPacman
```

## BSD Package Support (BsdPkg)

Added in the October 2026 session. `UniversalDistroPackageFormat` now includes:
- `BsdPkg` — NetBSD/DragonFly pkg format with `+MANIFEST` and `pkg-plist`
- `OpenBsdPorts` — OpenBSD ports tree Makefile format

Security scoring for BSD formats uses Capsicum capability confinement level 3 (full confinement), the highest tier in sigpkg's security matrix.

## Package Snapshot & Rollback

```bash
# Take package state snapshot
sigpkg snapshot create my-snapshot

# List snapshots
sigpkg snapshot list

# Rollback to snapshot
sigpkg snapshot rollback my-snapshot

# Diff two snapshots
sigpkg snapshot diff snap1 snap2
```

Backed by `SovereignPackageSnapshotRollbackEngine` in `src/sigpkg/package_snapshot_rollback.rs`.

## Zero-Allocation Resolver

For embedded/bare-metal contexts, sigpkg provides a `no_std` zero-allocation dependency resolver:

```rust
use sigmaos::sigpkg::{PackageDependencyResolver, MAX_RECIPE_DEPENDENCIES};

let mut resolver = PackageDependencyResolver::new();
resolver.add_package("nginx", &["libc", "libssl"]);
let order = resolver.resolve()?;
```

`MAX_RECIPE_DEPENDENCIES` is the compile-time maximum dependency count (default: 256).

## Maintenance

> **For AI agents maintaining this page:**
>
> - When a new `sovereign_distro_package_advancements_vN.rs` file is added to `src/package/`, add it to the Advancement Suites table above with its key struct and feature summary.
> - When new variants are added to `UniversalDistroPackageFormat`, add them to the Supported Package Formats table.
> - Keep the `sigpkg` CLI section synchronized with `src/bin/sigma_pkg.rs` and `src/bin/sigpkg.rs`.
> - **Bolt ⚡**: Document any zero-allocation path improvements in the Zero-Allocation Resolver section.
> - **Sentinel 🛡️**: Verify BSD format security tier classifications match `sovereign_universal_pm_pr_bridge.rs` `security_isolation_score()` implementation.
> - **Palette 🎨**: Maintain Arch Linux wiki style — use tables for format lists, code blocks for CLI examples.
> - If `src/package/mod.rs` is modified to add/remove modules, update this page accordingly.

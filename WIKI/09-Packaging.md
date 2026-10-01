# Packaging

SigmaPkg is SigmaOS's universal package manager supporting 110+ Linux/BSD package formats and cross-distribution ecosystems via an advanced Object-Oriented Programming (OOP) architecture, User-Defined Function (UDF) scripting engines, and multi-distro adapters.

## Package Management

### Basic Operations

Update package database:

```bash
sigpkg update
```

Upgrade installed packages:

```bash
sigpkg upgrade
```

Search for packages:

```bash
sigpkg search package-name
```

Install packages:

```bash
sigpkg install package-name
```

Remove packages:

```bash
sigpkg remove package-name
```

List installed packages:

```bash
sigpkg list
```

### Package Information

Show package details:

```bash
sigpkg info package-name
```

Show package dependencies:

```bash
sigpkg deps package-name
```

Show package files:

```bash
sigpkg files package-name
```

## Universal Package System Parity Engine

SigmaPkg (`src/sigpkg/universal_oop_system.rs` and `src/package/universal.rs`) implements a complete **Universal Package System Parity Engine** allowing full interoperability with all major Linux distributions, BSD operating systems, containerized application bundles, and language-specific package managers.

### Ingested Linux & BSD Distribution Paradigms

| Linux / BSD Distribution | Native Packaging Format & Subsystem | SigmaOS Parity Engine & Implementation Component |
| :--- | :--- | :--- |
| **Arch Linux / CachyOS** | `.pkg.tar.zst`, ALPM Hooks, Makepkg, AUR RPC | `PacmanAdapter`, `PacmanZstdV2Adapter`, `CachyOSMicroarchAdapter`, `ArchAlpmHookTransactionEngine` |
| **Debian / Ubuntu / Deepin** | `.deb`, `.superdeb`, `apt`, `dpkg` triggers, debconf | `DebAdapter`, `SuperdebAdapter`, `DebianTriggerManager`, `DebianAptPinningEngine` |
| **Fedora / RHEL / openSUSE** | `.rpm`, `.drpm`, DNF5 SQLite metadata, Zypper YaST | `RpmAdapter`, `Dnf5SQLiteAdapter`, `ZypperYastRpmDeltaPackageAdapter`, `DnfDeltaRpmStrategy` |
| **Gentoo Linux** | `.ebuild`, EAPI 8, Portage slots, USE_EXPAND flags | `EbuildAdapter`, `PortageEbuildV2Adapter`, `PortagePackage`, `PortageSlotResolver`, `GentooUseFlagManager` |
| **Alpine Linux** | `.apk` (v2 & v3), apk-index, signed tarballs | `ApkAdapter`, `Apk3SignatureAdapter`, `AlpineApkCachePeerSyncEngine` |
| **Void Linux** | `.xbps` (Zstd compressed archives), xbps-src | `XbpsAdapter`, `XbpsZstdAdapter`, `XbpsSonameAndOrphanEngine` |
| **NixOS & GNU Guix** | `.nix`, `.scm`, `.nar`, Flake lockfiles, pure CAS store | `NixAdapter`, `GuixAdapter`, `NixFlakeLockAdapter`, `SovereignProfileManager`, `NixStoreGcEngine` |
| **FreeBSD / OpenBSD / NetBSD** | `.pkg`, `pkg_add` Signify, Ports VuXML, NetBSD pkgsrc | `BsdPkgPortsAdapter`, `FreeBsdVuXmlPoudriereAuditAdapter`, `OpenBsdPledgeUnveilSandboxScriptletEngine` |
| **Containerized Apps & Bundles** | Flatpak, Snap, AppImage, Clear Linux Swupd | `FlatpakAdapter`, `SnapAdapter`, `AppImageAdapter`, `SwupdBundleAdapter`, `FlatpakManifest` |
| **Subsystem Overlays** | Bedrock Linux Strata, Distrobox OCI, systemd sysext | `BedrockStratumAdapter`, `DistroboxOciAdapter`, `SystemdSysextAdapter` |

### Object-Oriented Programming (OOP) Design Patterns

The package engine leverages **14 fundamental OOP Design Patterns** for extreme modularity and reliability:

1. **Mediator Pattern (`UniversalDistroPackageMediator`)**: Centralizes communication between parsers, validators, build pipelines, sandboxes, and repository managers (`MediatorEvent`).
2. **Visitor Pattern (`IPackageVisitor`, `SecurityAuditVisitor`, `LicenseScannerVisitor`, `FootprintMetricsVisitor`)**: Allows AST traversal and deep inspection (security audits, license compliance, disk footprint) across heterogeneous package structures without mutating package classes.
3. **Memento Pattern (`PackageTransactionMemento` & `SystemStateCaretaker`)**: Captures immutable system state snapshots before package operations, enabling atomic multi-step transaction rollbacks, undo, and redo.
4. **State Pattern (`PackageLifecycleState` & `PackageStateMachine`)**: Enforces explicit state machine transitions (`Uninstalled` -> `ResolvingDependencies` -> `Downloading` -> `Validating` -> `Installing` -> `Installed` / `Broken`).
5. **Chain of Responsibility Pattern (`PackageValidationHandlerChain`)**: Chains validation handlers (`ChecksumValidationHandler`, `PqcSignatureValidationHandler`, `DependencyIntegrityHandler`, `LicenseComplianceHandler`) sequentially.
6. **Command Pattern (`IPackageCommand` & `TransactionRollbackExecutor`)**: Encapsulates install, remove, and upgrade operations into reversible command objects.
7. **Observer Pattern (`PackageEventManager` & `IPackageObserver`)**: Broadcasts package lifecycle events (`PackageEvent::Installed`, `FileDiverted`, `AlternativeSwitched`) to registered listeners.
8. **Strategy Pattern (`IPackageParser`, `IPackageDeltaStrategy`, `IPackageFetchStrategy`)**: Encapsulates format parsing, delta patch reconstruction (DRPM, MOSS, Zstd), and package fetching protocols (HTTP/3, P2P BitTorrent, local file store).
9. **Template Method Pattern (`AbstractPackageBuildTemplate` & `StandardPackageBuildPipeline`)**: Defines the invariant skeleton of the package build algorithm (`prepare` -> `configure` -> `compile` -> `test` -> `install_files` -> `clean`) while allowing customizable hook overrides.
10. **Adapter Pattern (`PackageFormatAdapter` & Distro Adapters)**: Converts divergent distro package structures into unified `IPackage` and `UnifiedPackage` abstractions.
11. **Flyweight Pattern (`PackageMetadataFlyweightFactory` & `SharedMetadataFlyweight`)**: Reuses shared immutable metadata blocks (common license texts, maintainer groups, target CPU architectures) to optimize memory footprint when managing thousands of package records.
12. **Proxy Pattern (`LazyPackageLoadProxy`)**: Delays expensive archive extraction and network metadata fetches until package properties are accessed.
13. **Decorator Pattern (`SandboxedPackageDecorator`, `AuditedPackageDecorator`, `PqcSignedPackageDecorator`)**: Dynamically adds sandboxing promises (OpenBSD Pledge/Unveil, FreeBSD Capsicum, Linux Landlock), security audit status, and post-quantum Dilithium signatures.
14. **Builder Pattern (`UniversalPackageBuilder`)**: Provides a fluent API for assembling complex package metadata, dependencies, and formats with validation before building.
15. **Factory Pattern (`PackageParserFactory` & `PackageFactory`)**: Auto-detects and instantiates parser adapters and installation strategies based on payload headers or filename extensions.

### User-Defined Function (UDF) Scripting Engines

SigmaPkg exposes 7 **User-Defined Function (UDF) Engines** for dynamic package customization:

1. **UDF Custom Scriptlet Execution Engine (`UdfCustomScriptletEngine`)**: Executes custom pre/post install and pre/post remove scriptlets safely inside isolated sandboxes.
2. **UDF Package Conflict Resolver Engine (`UdfPackageConflictResolverEngine`)**: Evaluates user-defined functions to resolve complex multi-distro package conflicts dynamically.
3. **UDF Package Archive Transformer Engine (`UdfPackageArchiveTransformerEngine`)**: Applies user-defined payload transformations (e.g. re-compressing tar.xz to zstd or stripping debug ELF symbols).
4. **UDF Dependency Rewriter Engine (`UdfDependencyRewriterEngine`)**: Maps legacy distro package dependencies (e.g., `libssl-dev`, `openssl-devel`) to canonical sovereign system capabilities (`sovereign-openssl`).
5. **UDF Post-Extract File Filter (`UdfPostExtractFileFilter`)**: Filters extracted file lists post-installation (e.g. stripping debug symbols or unwanted documentation).
6. **UDF Lifecycle Hook Registry (`UdfLifecycleHookRegistry`)**: Executes user closures registered for specific package lifecycle events (`PreParse`, `PostParse`, `PreBuild`, `PostBuild`, `PreInstall`, `PostInstall`).
7. **UDF Constraint Solver Filter (`UdfCustomConstraintSolverFilter`)**: Applies custom rule filters to DPLL SAT dependency resolution.

## Package Formats

### Supported Formats

SigmaPkg supports 110+ package formats spanning Linux, BSD, Unix, containers, and language package managers:

- `.deb` / `.udeb` / `.superdeb` (Debian/Ubuntu/Deepin)
- `.rpm` / `.drpm` (Fedora/RHEL/openSUSE)
- `.apk` (Alpine v2 & v3)
- `.pkg.tar.zst` / `.pacman` (Arch Linux / CachyOS)
- `.ebuild` (Gentoo Linux)
- `.xbps` (Void Linux)
- `.txz` / `.tgz` (Slackware)
- `.ipk` / `.opkg` (OpenWrt / Yocto)
- `.nix` / `.nar` / `.guix` (NixOS / GNU Guix)
- `.flatpakref` / `.snap` / `.appimage` (Flatpak / Snap / AppImage)
- `.sigpkg` (SigmaOS native)

### Cross-Distro Packages

Install packages from other distributions:

```bash
# Install Debian package
sigpkg install package.deb

# Install RPM package
sigpkg install package.rpm

# Install Arch package
sigpkg install package.pkg.tar.zst
```

## Repository Configuration

### Add Repository

Add package repository:

```toml
# /etc/sigmaos/sigpkg.toml
[repositories]
official = "https://packages.sigmaos.org"
community = "https://community.sigmaos.org"
testing = "https://testing.sigmaos.org"
```

### Enable Repository

Enable repository:

```bash
sigpkg repo enable repository-name
```

## Package Development

### Creating Packages

Create SigmaPkg packages:

```bash
# Create package skeleton
sigpkg create package-name

# Build package
sigpkg build package-name

# Install local package
sigpkg install package-name.sigpkg
```

### Package Metadata

Package metadata in `SIGPKG.toml`:

```toml
[package]
name = "example-package"
version = "1.0.0"
description = "Example package description"
maintainer = "maintainer@example.com"

[dependencies]
required = ["dependency1", "dependency2"]
optional = ["optional-dependency"]

[files]
# List of files to include
```

## Package Signing

### Sign Packages

Sign packages for verification:

```bash
# Generate signing key
sigpkg keygen

# Sign package
sigpkg sign package.sigpkg

# Verify signature
sigpkg verify package.sigpkg
```

## Package Updates

### Auto Updates

Configure automatic updates:

```toml
# /etc/sigmaos/sigpkg.toml
[package]
auto_update = true
auto_cleanup = true
keep_old_versions = 3
```

### Manual Updates

Update specific packages:

```bash
# Update package
sigpkg update package-name

# Update all packages
sigpkg upgrade --all
```

## Package Cleanup

### Remove Unused Packages

Remove unused dependencies:

```bash
# Remove orphan packages
sigpkg autoremove

# Clean package cache
sigpkg clean
```

### Old Versions

Remove old package versions:

```bash
# Remove old versions
sigpkg remove --old-versions
```

## Next Steps

- [Development](10-Development.md) - Development tools and building
- [Configuration](03-Configuration.md) - System configuration
- [Security](07-Security.md) - Package security and signing

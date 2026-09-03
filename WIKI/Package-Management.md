# Package Management (Universal Package System)

SigmaOS's native package manager **SigmaPkg** / **UniversalPackageManager** supports **18 major Linux/BSD/mobile/container distribution formats** without any external tools or dynamic library dependencies.

## Supported Formats (18 Major Distribution Formats)

| Format | Distribution / Target | Subsystem Adapter |
|---|---|---|
| `Deb` | Debian, Ubuntu, Mint, Parrot OS | `DebMetadataAdapter` / `DebInstallStrategy` |
| `Rpm` | Fedora, RedHat, CentOS, Zypper | `RpmMetadataAdapter` / `RpmInstallStrategy` |
| `Pacman` | Arch Linux, Manjaro, EndeavourOS | `PacmanMetadataAdapter` / `PacmanInstallStrategy` |
| `Ebuild` | Gentoo Linux | `EbuildMetadataAdapter` / `EbuildInstallStrategy` |
| `Apk` | Alpine Linux | `ApkMetadataAdapter` / `ApkInstallStrategy` |
| `Nix` | NixOS / Nix Flakes | `NixMetadataAdapter` / `NixInstallStrategy` |
| `Flatpak` | Desktop Containerized Apps | `FlatpakMetadataAdapter` / `FlatpakInstallStrategy` |
| `Snap` | Canonical Isolated Snaps | `SnapMetadataAdapter` / `SnapInstallStrategy` |
| `AppImage` | Portable Runtime Desktop Bundles | `AppImageMetadataAdapter` / `AppImageInstallStrategy` |
| `Xbps` | Void Linux | `XbpsMetadataAdapter` / `XbpsInstallStrategy` |
| `Txz` | Slackware Slackpkg | `TxzMetadataAdapter` / `TxzInstallStrategy` |
| `Eopkg` | Solus Linux | `EopkgMetadataAdapter` / `EopkgInstallStrategy` |
| `Zypper` | openSUSE libsolv / zypp | `ZypperMetadataAdapter` / `ZypperInstallStrategy` |
| `Guix` | GNU Guix System | `GuixMetadataAdapter` / `GuixInstallStrategy` |
| `CachyOS` | Optimized CachyOS Arch Variant | `CachyOSMetadataAdapter` / `CachyOSInstallStrategy` |
| `Swupd` | Intel Swupd Stateless Model | `SwupdMetadataAdapter` / `SwupdInstallStrategy` |
| `Starling` | Post-Quantum Micro-Packages | `StarlingMetadataAdapter` / `StarlingInstallStrategy` |
| `SigmaPkg` | Sovereign SigmaOS Native Format | `SigmaPkgMetadataAdapter` / `SigmaPkgInstallStrategy` |

## Advanced OOP Architecture & Design Patterns

The universal packaging engine in `src/package/universal.rs` is designed using strict Object-Oriented Principles (OOP):

1. **Strategy Pattern (`InstallStrategy`)**: Decouples package installation, verification, and uninstallation actions per distribution format (`DebInstallStrategy`, `RpmInstallStrategy`, `PacmanInstallStrategy`, etc.).
2. **Adapter Pattern (`PackageMetadataAdapter`)**: Translates raw metadata text files (Debian `control`, RPM spec, Pacman `PKGBUILD`) into unified `UnifiedPackage` instances.
3. **State Pattern (`PackageState`)**: Tracks explicit package lifecycle transitions (`Uninstalled` ➔ `Downloading` ➔ `Installing` ➔ `Installed`).
4. **Decorator Pattern (`PackageCapability`)**: Wraps runtime sandbox limits (`SandboxDecorator`) and network access controls (`NetworkRestrictionDecorator`) around base packages.
5. **Factory Pattern (`PackageFactory`)**: Dynamically instantiates matching strategies and adapters based on `PackageFormat`.
6. **Observer Pattern & User-Defined Functions (UDFs)**: Dispatches lifecycle state change notifications to registered `PackageObserver`s and executes closure-based pre/post installation triggers (`PackageUdfHook`).

```
User Request / CLI Invoke
           ↓
UniversalPackageManager (Facade)
           ↓
PackageFactory (Strategy & Adapter Factory)
           ↓
PackageMetadataAdapter (Normalizes metadata to UnifiedPackage)
           ↓
DependencyResolver & Conflict Resolution
           ↓
PackageTriggerRegistry (Executes Pre-Install UDF Hooks)
           ↓
InstallStrategy (Executes format-specific installation steps)
           ↓
PackageState Observer Notification (Dispatches state updates)
           ↓
PackageCapability Decorators (Enforces Sandbox & Network Restrictions)
```

## Key Code Locations

- `src/package/universal.rs` — Core Universal Package Manager engine, 18 formats, OOP design patterns, UDF triggers, and transactional checkpoints.
- `src/sigpkg/spec.rs` — Native SigmaPkg format specification.
- `src/sigpkg/transaction.rs` — Transactional history and rollback mechanisms.
- `src/sigpkg/resolver.rs` — Dependency resolution engine.

## Usage Example

```rust
use crate::package::universal::*;
use std::sync::Arc;

let mut manager = UniversalPackageManager::new();

// Register a User-Defined Function (UDF) hook
manager.triggers.register_pre_install(Arc::new(|pkg| {
    println!("Executing custom pre-install hook for package: {}", pkg.name);
    Ok(())
}));

// Add a package supporting Pacman format
let pkg = UnifiedPackage::new("firefox".to_string(), "120.0.0".to_string())
    .with_format(PackageFormat::Pacman);

manager.add_package(pkg);
manager.install("firefox").unwrap();
```

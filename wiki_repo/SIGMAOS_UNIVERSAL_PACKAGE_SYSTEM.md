# SigmaOS Universal Package System

## Overview

The SigmaOS Universal Package System (SigPkg) is an OOP-based package management system designed to support all Linux distro package formats, making it easier to run Linux-based applications in SigmaOS.

## Key Features

### OOP Design Patterns

The package system implements several OOP design patterns:

- **Strategy Pattern**: Different parsing strategies for each package format (deb, rpm, pacman, etc.)
- **Adapter Pattern**: Adapters for converting between different package formats
- **Factory Pattern**: Package parser factory for creating format-specific parsers
- **Facade Pattern**: Universal package manager providing a simple interface for all operations

### Supported Package Formats

- **Debian/Ubuntu** (.deb) - via `DebAdapter`
- **Fedora/RHEL** (.rpm) - via `RpmAdapter`
- **Arch Linux** (pacman) - via `PacmanAdapter`
- **Gentoo** (ebuild) - via `EbuildAdapter`
- **Alpine** (apk) - via `ApkAdapter`
- **Nix** (nix expressions) - via `NixAdapter`
- **Flatpak** - via `FlatpakAdapter`
- **Snap** - via `SnapAdapter`
- **AppImage** - via `AppImageAdapter`
- **Void Linux** (xbps) - via `XbpsAdapter`
- **Slackware** (txz) - via `TxzAdapter`
- **Solus** (eopkg) - via `EopkgAdapter`
- **OpenSUSE** (zypper) - via `ZypperAdapter`
- **Guix** - via `GuixAdapter`
- **SigmaOS Native** (sigpkg) - via native format

### User-Defined Functions (UDF)

The system supports user-defined hooks for custom package processing:

```rust
pub trait UserDefinedHook: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, package: &mut dyn IPackage) -> Result<(), HookError>;
}
```

### Core Abstractions

#### IPackage Trait

The core package interface:

```rust
pub trait IPackage: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &Version;
    fn dependencies(&self) -> &[Dependency];
    fn format(&self) -> PackageFormat;
    fn metadata(&self) -> &PackageMetadata;
}
```

#### IPackageParser Trait

The parser interface for different formats:

```rust
pub trait IPackageParser: Send + Sync {
    fn format(&self) -> PackageFormat;
    fn can_parse(&self, data: &[u8]) -> bool;
    fn parse(&self, data: &[u8]) -> Result<Box<dyn IPackage>, ParseError>;
    fn serialize(&self, package: &dyn IPackage) -> Result<Vec<u8>, ParseError>;
}
```

## Usage Examples

### Parsing a Package

```rust
use sigpkg::UniversalPackageManager;

let manager = UniversalPackageManager::new();

// Auto-detect format and parse
let package = manager.parse_package(deb_data)?;

// Or parse with specific format
let package = manager.parse_package_with_format(PackageFormat::Rpm, rpm_data)?;
```

### Installing a Package

```rust
let mut manager = UniversalPackageManager::new();

let package = manager.parse_package(package_data)?;
manager.install_package(package)?;
```

### Adding Custom Hooks

```rust
let mut adapter = DebAdapter::new();

adapter.add_hook(Arc::new(CustomHook::new()));
```

### Registering Custom Parsers

```rust
let mut manager = UniversalPackageManager::new();

manager.register_parser(Box::new(CustomAdapter::new()));
```

## Architecture

### Module Structure

- `src/sigpkg/mod.rs` - Main module exports
- `src/sigpkg/universal_oop_system.rs` - OOP-based universal package system
- `src/sigpkg/universal_adapter.rs` - Package format adapters
- `src/sigpkg/universal_engine.rs` - Universal package manager engine with UDF support
- `src/sigpkg/recipe.rs` - Package recipe definitions
- `src/sigpkg/arch_compat.rs` - Arch Linux compatibility layer
- `src/sigpkg/rpm_compat.rs` - RPM compatibility layer
- `src/sigpkg/spec.rs` - Package specification
- `src/sigpkg/resolver.rs` - Dependency resolver
- `src/sigpkg/store.rs` - Content-addressed storage
- `src/sigpkg/transaction.rs` - Transaction management
- `src/sigpkg/verifier.rs` - Cryptographic verification
- `src/sigpkg/zero_alloc_resolver.rs` - Zero-allocation dependency resolver

## Post-Quantum Security

The package system incorporates NIST-validated post-quantum cryptography:

- **Kyber-1024** key encapsulation (KEM)
- **Dilithium-5** digital signatures
- **SHA3-256** hashing

## Content-Addressed Storage

Packages are stored using content-addressed storage (CAS) for:

- Immutable package storage
- Efficient deduplication
- O(1) rollback capability
- Cryptographic verification

## Future Enhancements

- [ ] Additional distro format support
- [ ] Enhanced dependency resolution algorithms
- [ ] Binary cache integration
- [ ] Container-based package isolation
- [ ] Package signing infrastructure
- [ ] Repository management tools
- [ ] Package search and discovery
- [ ] Automatic updates and security patches

## Related Documentation

- [Package Management Spec](Package-Management-Spec.md)
- [SigmaPkg Defragmentation and Parity](SigmaPkg-Defragmentation-And-Parity.md)
- [OOP Development Plan](OOP_Development_Plan.md)
- [Linux Distro Absorption Spec](LINUX_DISTRO_ABSORPTION_SPEC.md)

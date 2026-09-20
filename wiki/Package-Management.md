# SigmaOS Package Management

This page consolidates all package management documentation for SigmaOS.

## Overview

SigmaOS implements a universal package management system that supports all major Linux and BSD package formats through a unified interface with OOP design patterns and User-Defined Functions (UDFs).

## Supported Package Formats

### Linux Package Formats
- **Deb** - apt/dpkg (Debian, Ubuntu, Linux Mint, Pop!_OS)
- **Rpm** - yum/dnf/zypper (Fedora, RHEL, openSUSE, Mageia)
- **Pacman** - pacman/pkgbuild (Arch Linux, Manjaro, EndeavourOS, Garuda)
- **Snap** - snap/squashfs (Ubuntu universal packages)
- **Flatpak** - flatpak sandbox (cross-distro applications)
- **AppImage** - AppImage single-file container
- **Apk** - apk (Alpine Linux, postmarketOS)
- **Xbps** - xbps (Void Linux)
- **Ebuild** - portage (Gentoo, Funtoo)
- **Tcz** - Tiny Core Modules (Tiny Core Linux)
- **Sol** - eopkg (Solus)
- **Opkg** - OpenWrt package format
- **Nix** - Nix store paths (NixOS, Guix)

### BSD Package Formats
- **Pkg** - FreeBSD pkg
- **Tgz** - Slackware pkgtool
- **IPS** - Solaris Image Packaging System (Solaris, Illumos, OpenIndiana, SmartOS)
- **Sx** - OpenBSD packages
- **Nar** - Guix Nix archives

### Container Formats
- **Docker** - Docker image format
- **LXC** - Linux container format
- **AppC** - Application Container format
- **OCI** - Open Container Initiative

## Universal Package System Architecture

### Core Components

#### UniversalPackageManager
The central package manager that orchestrates all package formats through adapters:

```rust
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, Box<dyn PackageAdapter>>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, PackageState>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub user_hooks: Vec<Arc<dyn PackageHook>>,
    pub node_distro_engine: NodeBinaryDistroEngine,
    pub distro_repo_sync: DistroRepoSyncEngine,
    pub triggers: PackageTriggerRegistry,
}
```

#### Package Adapters
Each package format has a dedicated adapter implementing the `PackageAdapter` trait:

- `DebAdapter` - Debian/dpkg operations
- `RpmAdapter` - RPM operations
- `PacmanAdapter` - Arch pacman operations
- `SnapAdapter` - Snap operations
- `FlatpakAdapter` - Flatpak operations
- `ApkAdapter` - Alpine apk operations
- `XbpsAdapter` - Void xbps operations
- `PortageAdapter` - Gentoo portage operations
- `FreeBsdPkgAdapter` - FreeBSD pkg operations
- `OpenBsdPkgAdapter` - OpenBSD pkg operations
- `SolarisIpsAdapter` - Solaris IPS operations

#### Dependency Resolution
- **SAT Solver** - Boolean satisfiability for complex dependencies
- **O(1) Lookup** - Optimized conflict detection with HashMap hoisting
- **Transaction Rollback** - Sub-1ms state restoration with differential snapshots

### Package States

```rust
pub enum PackageState {
    Uninstalled,
    Downloading,
    Installing,
    Installed,
    BrokenDependency,
    Available,
    Updating,
    Corrupted,
}
```

## Security Features

### Cryptographic Signing
- **Post-Quantum Cryptography** - Dilithium-5 and Kyber-1024 for package signatures
- **Ed25519** - Digital signature verification
- **SHA-256/512** - Package integrity checksums
- **Secure Boot** - Measured boot TPM PCR measurements

### Sandboxing
- **Flatpak Sandboxing** - Application isolation
- **Snap Confinement** - Strict mode confinement
- **Capsicum Rights** - FreeBSD capability-based security
- **Landlock v5** - Linux filesystem access control

### Audit & Verification
- **Package Signoff** - QA quorum checks (qa_tested, build_reproducible, security_audited)
- **Vulnerability Tracking** - SecurityAdvisoryTracker with classification
- **Livepatch Verification** - KernelPatchVerificationEngine for patch trampolines

## Advanced Features

### Cross-Distro Package Translation
- **Format Conversion** - Convert between package formats for compatibility
- **Dependency Mapping** - Map distro-specific dependencies to universal equivalents
- **Repository Sync** - DistroRepoSyncEngine for multi-repo management

### Transaction Management
- **Atomic Operations** - All-or-nothing package operations
- **Rollback Support** - Automatic rollback on failure
- **Checkpoint System** - State snapshots before major operations

### UDF System
- **Custom Hooks** - User-defined functions for package lifecycle events
- **Pre/Post Install Hooks** - Execute custom scripts
- **Observer Pattern** - Monitor package operations

## Package Submission Guidelines

### Repository Requirements
- Reproducible builds
- Source code availability
- License compliance
- Security audit
- QA testing

### Package Metadata
- **Name** - Unique package identifier
- **Version** - Semantic versioning
- **Description** - Package description
- **Dependencies** - Required packages
- **Conflicts** - Incompatible packages
- **Architecture** - Supported architectures
- **Checksums** - File integrity verification

## Integration with SigmaOS

### Subsystem Compatibility
The universal package system integrates with SigmaOS subsystems:
- **144 Subsystem Modes** - Support for all Linux and BSD distributions
- **Cross-Subsystem Dispatch** - Unified package operations across subsystems
- **Distro Innovation Synthesis** - Package management innovations from each distro

### AI Agent Integration
- **Automated Dependency Resolution** - AI-driven package recommendations
- **Security Scanning** - Automated vulnerability detection
- **Performance Optimization** - Cache optimization and parallel operations

## Documentation References

For detailed implementation specifications:
- [Architecture](ARCHITECTURE.md)
- [Security](SECURITY.md)
- [Roadmap](ROADMAP.md)
- [Kernel](Kernel.md)

## Contributing

Package management development follows the SigmaOS agent guidelines:
- **Sentinel**: Security vulnerability remediation
- **Bolt**: Performance optimization (O(1) algorithms, lock-free structures)
- **Palette**: UX enhancements for package management interface

---

*This page consolidates the following individual package management documents:*
- AGENTS_PACKAGE_MANAGEMENT.md
- AI_AGENT_PACKAGE_MANAGEMENT.md
- ai_agents_package_management.md
- AI_AGENTS_PACKAGE_MANAGEMENT_SPEC.md
- AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT_ARCHITECTURE.md
- AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT_GUIDELINES.md
- AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT.md
- Category:Package-Management.md
- Package-Management-and-Sigpkg.md
- PACKAGE_MANAGEMENT.md
- package-manager.md
- Package-Submission-Guidelines.md
- Post-Quantum-Cryptography-Package-Distribution.md
- UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN.md

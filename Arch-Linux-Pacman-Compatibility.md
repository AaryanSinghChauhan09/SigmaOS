# Arch Linux Pacman Compatibility in SigmaOS

## Overview

SigmaOS includes a zero-dependency, clean-room subsystem providing comprehensive compatibility with **Arch Linux** and its **Pacman** package manager. This subsystem allows Arch packages, PKGBUILD files, and AUR (Arch User Repository) packages to be parsed, resolved, and managed natively on SigmaOS.

---

## Key Modules

- [`src/sigpkg/arch_pacman_engine.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/arch_pacman_engine.rs): Pacman package manager, PKGBUILD parser, and AUR helper
- [`src/sigpkg/mod.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/mod.rs): Unified package orchestrator integrating native `.spkg`, Arch `.pkg.tar`, and other formats

---

## Features

| Feature | SigmaOS Implementation | Notes |
|---------|------------------------|-------|
| **Pacman Database** | Native package database emulation | Compatible with pacman database format |
| **PKGBUILD Parsing** | Native PKGBUILD parser | Extracts package metadata without makepkg |
| **AUR Integration** | AUR helper for community packages | Access to Arch User Repository |
| **ABS Support** | Arch Build System compatibility | Build packages from source |
| **Repository Management** | Core, extra, community, multilib | Full Arch repository support |
| **Dependency Resolution** | Recursive dependency graph | Handles complex dependency trees |

---

## Architecture Flow

```
Arch Linux Repositories
       │ (Download Packages)
       ▼
[Pacman Database] ──────> Manages package metadata
       │
       ▼
[Dependency Resolver] ───> Resolves Depends/MakeDepends/OptDepends
       │
       ▼
[Arch Build System] ─────> Parses PKGBUILD files
       │
       ▼
[AUR Helper] ────────────> Manages AUR packages
       │
       ▼
[SigmaPkg Native Index] ───────> Integrates with native package system
```

---

## CLI Usage

```bash
# Search Arch packages
sigma-pkg pacman-search nginx

# Install from Arch repository
sigma-pkg pacman-install nginx

# Update Arch repositories
sigma-pacman -Sy

# Query package information
sigma-pacman -Qi nginx

# List installed packages
sigma-pacman -Q

# Remove a package
sigma-pacman -R nginx
```

---

## Implementation Details

### Pacman Package Structure

```rust
pub struct ArchPacmanPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub architecture: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub installed_size: u64,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
}
```

### Pacman Database Configuration

```rust
pub struct PacmanDatabase {
    pub packages: Vec<ArchPacmanPackage>,
    pub local_packages: Vec<ArchPacmanPackage>,
    pub sync_databases: Vec<String>,
}
```

### Repository Configuration

Arch Linux repositories are automatically configured:
- **core**: Essential packages
- **extra**: Additional packages
- **community**: Community-maintained packages
- **multilib**: 32-bit compatibility libraries

### Dependency Fields

- **depends**: Required dependencies
- **makedepends**: Build-time dependencies
- **checkdepends**: Test dependencies
- **optdepends**: Optional dependencies
- **provides**: Virtual packages provided
- **conflicts**: Conflicting packages
- **replaces**: Packages this replaces

---

## PKGBUILD Support

### PKGBUILD Format

The PKGBUILD parser handles standard Arch PKGBUILD files:

```bash
pkgname=test-package
pkgver=1.0.0
pkgrel=1
pkgdesc="Test package for SigmaOS"
arch=('x86_64')
license=('MIT')
depends=('glibc')
makedepends=('gcc')
source=("https://example.com/${pkgname}-${pkgver}.tar.gz")
sha256sums=('SKIP')
```

### Arch Build System (ABS)

```rust
pub struct ArchBuildSystem {
    pub pkgbuild: String,
    pub srcinfo: String,
}
```

The ABS provides:
- PKGBUILD parsing
- SRCINFO extraction
- Package building capabilities
- Source integration

---

## AUR Integration

### AUR Helper

```rust
pub struct AURHelper {
    pub aur_packages: Vec<ArchPacmanPackage>,
}
```

### AUR Features

- **Package Search**: Search AUR for packages
- **Package Information**: Get detailed AUR package info
- **Package Installation**: Install AUR packages with dependencies
- **PKGBUILD Integration**: Clone and build from AUR

### AUR Workflow

```bash
# Search AUR
sigma-pkg aur-search yay

# Get AUR package info
sigma-pkg aur-info yay

# Install AUR package
sigma-pkg aur-install yay
```

---

## Integration with SigmaOS

The Arch Pacman engine integrates seamlessly with:
- **SigmaPkg**: Native package manager
- **Repository System**: Cross-distro repository management
- **Filesystem**: Arch-standard filesystem layout
- **Service Management**: Systemd integration

---

## Benefits

1. **Zero-Dependency**: No external Pacman tools required
2. **Arch Ecosystem**: Access to vast Arch package ecosystem
3. **Rolling Release**: Latest software versions
4. **AUR Access**: Community-driven package repository
5. **ABS Support**: Build packages from source
6. **Minimal Overhead**: Efficient package management

---

## Examples

### Web Server Installation

```bash
# Install nginx from Arch repository
sigma-pkg pacman-install nginx

# Install Apache with dependencies
sigma-pkg pacman-install apache

# Query package details
sigma-pacman -Qi nginx
```

### Development Tools

```bash
# Install development tools
sigma-pkg pacman-install base-devel
sigma-pkg pacman-install git
sigma-pkg pacman-install cmake
```

### AUR Package Installation

```bash
# Install yay AUR helper
sigma-pkg aur-install yay

# Install other AUR packages
yay -S google-chrome
```

### System Updates

```bash
# Update system packages
sigma-pacman -Syu

# Update only repositories
sigma-pacman -Sy

# Update specific package
sigma-pacman -S nginx
```

---

## Comparison with Original Pacman

| Feature | Original Pacman | SigmaOS Implementation |
|---------|----------------|------------------------|
| **Dependency Resolution** | Libalpm | Native recursive resolver |
| **Package Database** | /var/lib/pacman | Native HashMap database |
| **Repository Cache** | /var/cache/pacman/pkg | Native cache system |
| **Configuration** | /etc/pacman.conf | Native configuration system |
| **Tool Dependency** | pacman, makepkg | Zero external tools |

---

## Repository Mirrors

SigmaOS supports standard Arch Linux mirror configuration:
- Official Arch mirrors
- Custom mirror configuration
- Mirror ranking and selection
- Geographic mirror optimization

---

## Security Features

- **Package Signing**: PGP signature verification
- **SHA256 Checksums**: Package integrity verification
- **Dependency Validation**: Secure dependency resolution
- **Build Verification**: Reproducible builds

---

**Generated:** August 24, 2026  
**Repository:** [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
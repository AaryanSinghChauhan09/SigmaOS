# SigmaOS Package Management Guide

## Table of Contents

1.  [Introduction to SigmaPkg](#introduction-to-sigmapkg)
2.  [Basic Package Operations](#basic-package-operations)
3.  [Repository Management](#repository-management)
4.  [Package Building](#package-building)
5.  [Package Compatibility](#package-compatibility)
6.  [Advanced Features](#advanced-features)
7.  [Troubleshooting](#troubleshooting)

## Introduction to SigmaPkg

SigmaPkg is the native package manager for SigmaOS, inspired by Arch Linux's Pacman and Debian's APT. It provides:

*   Content-addressed storage (Nix-inspired)
*   Declarative build system (Bazel/Nix-inspired)
*   Package ratings and reviews
*   Multiple package format support
*   Transaction rollback capabilities

### Package Formats Supported

*   **SigmaPackage (.spkg)**: Native SigmaOS format
*   **Debian (.deb)**: Full compatibility via translation layer
*   **Arch (.pkg.tar.xz)**: PKGBUILD support
*   **RPM (.rpm)**: Red Hat compatibility
*   **Flatpak**: Containerized applications
*   **AppImage**: Portable applications

## Basic Package Operations

### Update Package Database

```bash
# Update repository metadata
sigpkg update

# Check for updates
sigpkg check-updates
```

### Install Packages

```bash
# Install a package
sigpkg install package-name

# Install multiple packages
sigpkg install package1 package2 package3

# Install with dependencies
sigpkg install --with-deps package-name

# Install from local file
sigpkg install ./package.spkg
```

### Remove Packages

```bash
# Remove a package
sigpkg remove package-name

# Remove with dependencies
sigpkg remove --with-deps package-name

# Remove configuration files
sigpkg remove --purge package-name

# Remove orphan packages
sigpkg remove-orphans
```

### Upgrade Packages

```bash
# Upgrade all packages
sigpkg upgrade

# Upgrade specific package
sigpkg upgrade package-name

# Upgrade from specific version
sigpkg upgrade --from version package-name
```

### Search Packages

```bash
# Search by name
sigpkg search package-name

# Search by description
sigpkg search --desc "search term"

# Search installed packages
sigpkg search --installed

# Search for updates
sigpkg search --updates
```

### Package Information

```bash
# Show package information
sigpkg info package-name

# Show package files
sigpkg files package-name

# Show package dependencies
sigpkg deps package-name

# Show package dependencies tree
sigpkg deps --tree package-name
```

## Repository Management

### Repository Configuration

```bash
# List repositories
sigpkg repo list

# Add repository
sigpkg repo add repo-name https://repo.sigmaos.org

# Remove repository
sigpkg repo remove repo-name

# Enable repository
sigpkg repo enable repo-name

# Disable repository
sigpkg repo disable repo-name
```

### Repository Mirrors

```bash
# List mirrors
sigpkg mirror list

# Add mirror
sigpkg mirror add repo-name https://mirror.sigmaos.org

# Select best mirror
sigpkg mirror select repo-name

# Rank mirrors by speed
sigpkg mirror rank repo-name
```

### Repository Components

```bash
# Add repository component
sigpkg repo component add repo-name component-name

# Remove repository component
sigpkg repo component remove repo-name component-name
```

## Package Building

### PKGBUILD System (Arch Inspiration)

Create PKGBUILD files for custom packages:

```bash
# Create PKGBUILD
sigpkg build create package-name

# Edit PKGBUILD
sigpkg build edit package-name

# Build package
sigpkg build package-name

# Build and install
sigpkg build --install package-name
```

### PKGBUILD Example

```bash
# PKGBUILD template
pkgname="example-package"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="Example package description"
arch=("x86_64")
url="https://example.com"
license=("MIT")
depends=("dependency1" "dependency2")

source=("https://example.com/example-1.0.0.tar.gz")
sha256sums=("abcdef1234567890")

build() {
    cd "$pkgname-$pkgver"
    ./configure --prefix=/usr
    make
}

package() {
    cd "$pkgname-$pkgver"
    make DESTDIR="$pkgdir" install
}
```

### Declarative Build System (Nix/Bazel Inspiration)

Use declarative build definitions:

```nix
# Nix derivation example
{ stdenv, fetchurl, ... }:
stdenv.mkDerivation {
  name = "example-1.0.0";
  src = fetchurl {
    url = "https://example.com/example-1.0.0.tar.gz";
    sha256 = "abcdef1234567890";
  };
  buildInputs = [ dependency1 dependency2 ];
  configurePhase = "./configure --prefix=$out";
  buildPhase = "make";
  installPhase = "make install";
}
```

```python
# Bazel target example
py_binary(
    name = "example",
    srcs = ["example.py"],
    deps = [
        "//path/to/dependency1",
        "//path/to/dependency2",
    ],
)
```

## Package Compatibility

### Debian Package Support

```bash
# Install Debian package
sigpkg install ./package.deb

# Convert Debian to SigmaPackage
sigpkg convert deb ./package.deb output.spkg

# Import Debian repository
sigpkg import deb http://archive.debian.org/debian stable main
```

### Arch Package Support

```bash
# Install Arch package
sigpkg install ./package.pkg.tar.xz

# Convert Arch to SigmaPackage
sigpkg convert arch ./package.pkg.tar.xz output.spkg

# Import AUR package
sigpkg aur install package-name
```

### RPM Package Support

```bash
# Install RPM package
sigpkg install ./package.rpm

# Convert RPM to SigmaPackage
sigpkg convert rpm ./package.rpm output.spkg

# Import RPM repository
sigpkg import rpm http://mirror.centos.org/centos/8/os/x86_64/
```

### Flatpak Support

```bash
# Install Flatpak
sigpkg flatpak install app-id

# List Flatpak remotes
sigpkg flatpak remotes

# Add Flatpak remote
sigpkg flatpak remote-add flathub https://flathub.org/repo/flathub.flatpakrepo
```

## Advanced Features

### Transaction Management

```bash
# Begin transaction
sigpkg transaction begin

# Add operations to transaction
sigpkg transaction add install package1
sigpkg transaction add remove package2

# Commit transaction
sigpkg transaction commit

# Rollback transaction
sigpkg transaction rollback

# View transaction history
sigpkg transaction history
```

### Package Ratings and Reviews

```bash
# View package ratings
sigpkg rating package-name

# Rate package
sigpkg rate package-name 5

# Write review
sigpkg review package-name "Great package!"

# View reviews
sigpkg reviews package-name
```

### Dependency Resolution

```bash
# SAT solver-based dependency resolution
sigpkg resolve package-name

# Show dependency tree
sigpkg deps --tree package-name

# Check for conflicts
sigpkg conflicts package-name

# Resolve conflicts
sigpkg resolve-conflicts package-name
```

### Package Verification

```bash
# Verify package signature
sigpkg verify package-name

# Verify package checksum
sigpkg checksum package-name

# Check package integrity
sigpkg integrity package-name
```

### Package Cache Management

```bash
# Clean package cache
sigpkg clean

# Clean old packages only
sigpkg clean --old

# Clean uninstalled packages
sigpkg clean --uninstalled

# View cache size
sigpkg cache size
```

## Troubleshooting

### Package Installation Fails

```bash
# Check for conflicts
sigpkg conflicts package-name

# Resolve dependencies
sigpkg resolve package-name

# Force installation
sigpkg install --force package-name

# Check logs
sigpkg log package-name
```

### Dependency Issues

```bash
# Show dependency tree
sigpkg deps --tree package-name

# Check for circular dependencies
sigpkg deps --circular package-name

# Force dependency resolution
sigpkg resolve --force package-name
```

### Repository Issues

```bash
# Update repository metadata
sigpkg update

# Check repository status
sigpkg repo status

# Switch to different mirror
sigpkg mirror select repo-name

# Clear repository cache
sigpkg repo clean
```

### Build Failures

```bash
# Check build logs
sigpkg build log package-name

# Clean build directory
sigpkg build clean package-name

# Rebuild from scratch
sigpkg build --rebuild package-name

# Check build dependencies
sigpkg build deps package-name
```

## Best Practices

### System Updates

```bash
# Regular updates
sigpkg update && sigpkg upgrade

# Check for updates before upgrading
sigpkg check-updates

# Backup system before major updates
sigpkg backup
```

### Package Selection

```bash
# Check package ratings before installing
sigpkg rating package-name

# Read reviews
sigpkg reviews package-name

# Check dependencies
sigpkg deps package-name
```

### System Maintenance

```bash
# Remove orphan packages
sigpkg remove-orphans

# Clean package cache
sigpkg clean

# Check for security updates
sigpkg security check
```

## Additional Resources

*   [Installation Guide](./INSTALLATION)
*   [Configuration Guide](./CONFIGURATION)
*   [Security Hardening Guide](./SECURITY)
*   [Development Guide](./DEVELOPMENT)
*   [SigmaPkg API Documentation](../src/sigpkg/)

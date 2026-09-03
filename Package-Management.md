# SigmaOS Package Manager (sigpkg)

sigpkg is SigmaOS's universal, multi-format package manager. It can install, build, and manage packages from all major Linux and BSD distribution formats.

***

## Table of Contents

1.  [Quick Start](#quick-start)
2.  [Package Formats](#package-formats)
3.  [Command Reference](#command-reference)
4.  [Package Store](#package-store)
5.  [Dependency Resolution](#dependency-resolution)
6.  [PKGBUILD Recipes](#pkgbuild-recipes)
7.  [Atomic Upgrades and Rollback](#atomic-upgrades-and-rollback)
8.  [Reproducible Builds](#reproducible-builds)
9.  [AUR Bridge](#aur-bridge)
10. [Distro Bridges](#distro-bridges)
11. [Creating Packages](#creating-packages)
12. [Configuration](#configuration)

***

## Quick Start

```bash
# Install a package
sigpkg install nginx

# Search for packages
sigpkg search "web server"

# Update all packages
sigpkg upgrade

# Remove a package
sigpkg remove nginx

# Rollback to previous generation
sigpkg rollback

# List installed packages
sigpkg list --installed
```

***

## Package Formats

sigpkg handles all major Linux/BSD package formats natively:

| Format | Extension | Origin | Status |
|--------|-----------|--------|--------|
| Sigma native | `.spkg` | SigmaOS | ✅ Default |
| Arch pacman | `.pkg.tar.zst` | Arch Linux | ✅ Full |
| Debian | `.deb` | Debian/Ubuntu | ✅ Full |
| RPM | `.rpm` | Fedora/RHEL/SUSE | ✅ Full |
| Alpine | `.apk` | Alpine Linux | ✅ Full |
| Gentoo ebuild | `ebuild` | Gentoo | ✅ Full |
| Nix expression | `.nix` | NixOS | ✅ Full |
| FreeBSD port | `Makefile` | FreeBSD | ✅ Full |
| Flatpak | `.flatpakref` | Universal | 🔧 In progress |
| Snap | `.snap` | Ubuntu | 🔧 In progress |
| AppImage | `.AppImage` | Universal | 🗓 Planned |

### Format Auto-Detection

sigpkg automatically detects format from file extension and magic bytes:

```bash
# Install from any format
sigpkg install package.pkg.tar.zst
sigpkg install package.deb
sigpkg install package.rpm
sigpkg install package.apk
```

***

## Command Reference

### Installation

```bash
# Install from default repositories
sigpkg install <package>

# Install with specific version
sigpkg install nginx==1.25.3

# Install from local file
sigpkg install ./mypackage.spkg

# Install from URL
sigpkg install https://example.com/package.spkg

# Install multiple packages
sigpkg install nginx curl wget git

# Install without dependencies (dangerous)
sigpkg install --nodeps nginx

# Dry run (show what would be installed)
sigpkg install --dry-run nginx
```

### Querying

```bash
# Search packages
sigpkg search "text editor"
sigpkg search --name vim

# Show package info
sigpkg info nginx

# List all installed packages
sigpkg list
sigpkg list --installed
sigpkg list --available

# Show files installed by package
sigpkg files nginx

# Check which package owns a file
sigpkg owns /usr/bin/nginx

# Show package dependencies
sigpkg deps nginx
sigpkg deps --reverse nginx  # What depends on nginx?
```

### Upgrade & Maintenance

```bash
# Update repository index
sigpkg update

# Upgrade all packages
sigpkg upgrade

# Upgrade specific package
sigpkg upgrade nginx

# Clean package cache
sigpkg clean
sigpkg clean --all  # Remove all cached packages

# Verify installed packages
sigpkg verify
sigpkg verify nginx  # Verify specific package
```

### Package Management

```bash
# Remove package
sigpkg remove nginx

# Remove with dependencies (orphan removal)
sigpkg remove --recursive nginx

# Hold package at current version
sigpkg hold nginx

# Unhold package
sigpkg unhold nginx

# Show held packages
sigpkg list --held
```

***

## Package Store

sigpkg uses a **content-addressed store** at `/sigma/store/`:

    /sigma/store/
    └── sha256-abc123def456.../    ← package hash is its path
        ├── bin/
        ├── lib/
        ├── share/
        └── .sigma-meta/
            ├── manifest.toml
            ├── files.sha256
            └── signature.ed25519

### Store Properties

*   **Immutable**: installed packages are never modified
*   **Deduplicated**: identical files share storage via hard links
*   **Atomic**: installation either fully succeeds or fails (no partial state)
*   **Parallel**: multiple package versions coexist in the store

### Generations

The **active generation** is a symlink at `/sigma/current` pointing to a store path. Switching generations is atomic:

```bash
# Show all generations
sigpkg generations list

# Activate a previous generation
sigpkg generations activate 42

# Delete old generations
sigpkg generations gc --keep-last 5
```

***

## Dependency Resolution

sigpkg uses a **SAT (Boolean Satisfiability) solver** for dependency resolution:

1.  Parse all package dependency expressions into SAT clauses
2.  Add user-requested packages as unit clauses (must be true)
3.  Solve the SAT problem (DPLL algorithm with conflict-driven clause learning)
4.  Translate solution back to package installation plan

This guarantees:

*   Complete dependency resolution (no missing dependencies)
*   Conflict detection before installation begins
*   Minimal installation (only what's needed)

### Dependency Syntax

```toml
# In package manifest
[dependencies]
"libssl >= 3.0" = {}
"curl != 7.0" = {}
"python3 >= 3.10 < 4.0" = {}
"(vim | neovim)" = {}  # Alternative packages
"!java" = {}           # Conflict
```

***

## PKGBUILD Recipes

Build packages from source using Arch-Linux-style PKGBUILD recipes:

```bash
# SigmaOS PKGBUILD example
pkgname=myapp
pkgver=1.0.0
pkgrel=1
pkgdesc="My application"
arch=('x86_64')
url="https://example.com/myapp"
license=('MIT')
depends=('openssl' 'zlib')
makedepends=('cmake' 'ninja')
source=("https://example.com/myapp-$pkgver.tar.gz")
sha256sums=('abc123...')

build() {
    cmake -B build -DCMAKE_BUILD_TYPE=Release
    cmake --build build
}

package() {
    DESTDIR="$pkgdir" cmake --install build
}
```

Build with sigpkg:

```bash
sigpkg makepkg PKGBUILD
sigpkg install myapp-1.0.0-1-x86_64.spkg
```

### Sandboxed Builds

All package builds run in an isolated environment:

*   Read-only access to system libraries
*   Network blocked during compilation (for reproducibility)
*   Separate build user with no write access outside build directory

***

## Atomic Upgrades and Rollback

sigpkg's **two-phase commit** ensures upgrades never leave the system in a broken state:

    Phase 1 (Prepare):
      ├── Download all packages
      ├── Verify signatures
      ├── Check disk space
      └── Build new generation in /sigma/store/

    Phase 2 (Commit — atomic):
      └── Atomically update /sigma/current symlink

    Rollback (instant):
      └── Atomically revert /sigma/current symlink to previous generation

```bash
# Upgrade
sigpkg upgrade

# If something breaks, instant rollback:
sigpkg rollback

# Rollback to specific generation:
sigpkg rollback --generation 41
```

***

## Reproducible Builds

sigpkg guarantees reproducibility:

1.  **Hermetic build environment** — build dependencies exactly pinned
2.  **No network during build** — all sources fetched before build starts
3.  **Deterministic timestamps** — SOURCE\_DATE\_EPOCH set to 0
4.  **Path normalisation** — absolute paths stripped from binaries
5.  **Content addressing** — package hash is computed from inputs, not outputs

Verify reproducibility:

```bash
# Build the same package twice and compare
sigpkg makepkg --verify-reproducible PKGBUILD
```

***

## AUR Bridge

Connect to the Arch User Repository:

```bash
# Search AUR
sigpkg aur search "cool-tool"

# Install from AUR (builds from PKGBUILD)
sigpkg aur install cool-tool

# Update AUR packages
sigpkg aur upgrade
```

Source: `src/sigpkg/arch_compat.rs`

***

## Distro Bridges

sigpkg can pull packages from distro repositories directly:

```bash
# Use Fedora repository
sigpkg --repo fedora install dnf-plugin-core

# Use Debian repository
sigpkg --repo debian install apt-utils

# Use Alpine repository
sigpkg --repo alpine install musl-dev
```

Configured in `/etc/sigma/sigpkg.toml`:

```toml
[[repositories]]
name = "sigma-main"
url = "https://pkg.sigmaos.dev/main"
type = "sigma"
priority = 100

[[repositories]]
name = "arch-core"
url = "https://geo.mirror.pkgbuild.com"
type = "arch"
priority = 50

[[repositories]]
name = "fedora-40"
url = "https://dl.fedoraproject.org/pub/fedora/linux/releases/40"
type = "rpm"
priority = 30
```

***

## Creating Packages

### Native .spkg Format

```toml
# manifest.toml
[package]
name = "myapp"
version = "1.0.0"
description = "My application"
license = "MIT"
homepage = "https://example.com"

[dependencies]
openssl = ">=3.0"
zlib = ">=1.2"

[files]
"/usr/bin/myapp" = { source = "bin/myapp", mode = "0755" }
"/usr/share/myapp/" = { source = "share/", recursive = true }
"/etc/myapp/config.toml" = { source = "config.toml", mode = "0644", conffile = true }

[scripts]
post-install = "scripts/post-install.sh"
pre-remove = "scripts/pre-remove.sh"
```

Build and sign:

```bash
sigpkg build manifest.toml
sigpkg sign myapp-1.0.0.spkg --key /etc/sigma/signing.key
```

***

## Configuration

`/etc/sigma/sigpkg.toml`:

```toml
[general]
# Package store location
store = "/sigma/store"
# Maximum parallel downloads
parallel_downloads = 4
# Signature verification (always/optional/never)
signature_check = "always"
# Keep old generations
keep_generations = 5

[cache]
# Download cache directory
dir = "/var/cache/sigpkg"
# Maximum cache size
max_size = "10G"

[build]
# Number of parallel compilation jobs
jobs = 8
# Build flags
cflags = "-O2 -pipe"
# Sandboxed builds
sandbox = true
```

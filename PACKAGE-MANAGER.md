# sigma-pkg: SigmaOS Package Manager

## Overview

`sigma-pkg` is the unified package manager for SigmaOS, combining ideas from pacman, DNF, APT, and Nix into a single coherent tool.

## Quick Reference

| Operation | Command |
|-----------|--------|
| Install package | `sigma-pkg install <pkg>` |
| Remove package | `sigma-pkg remove <pkg>` |
| Update all | `sigma-pkg update` |
| Search | `sigma-pkg search <query>` |
| Show info | `sigma-pkg info <pkg>` |
| List installed | `sigma-pkg list` |
| Clean cache | `sigma-pkg clean` |
| Install AUR pkg | `sigma-pkg aur <pkg>` |
| Install Flatpak | `sigma-pkg flatpak <pkg>` |
| Build from src | `sigma-pkg build <PKGBUILD>` |
| Rollback | `sigma-pkg rollback` |
| Lock version | `sigma-pkg pin <pkg>@<version>` |

## Package Sources

### 1. Sigma Official Repository

Curated, security-audited packages maintained by the SigmaOS team.

```bash
# Enable/disable repos
sigma-pkg repo list
sigma-pkg repo enable extra
sigma-pkg repo disable testing
```

### 2. AUR (Arch User Repository)

Compatibility layer for Arch Linux AUR packages.

```bash
# Install from AUR
sigma-pkg aur yay
sigma-pkg aur visual-studio-code-bin
```

### 3. Flatpak

Sandboxed application delivery from Flathub.

```bash
# Install Flatpak app
sigma-pkg flatpak org.gnome.Builder
sigma-pkg flatpak com.spotify.Client
```

### 4. AppImage

Portable application bundles.

```bash
# Install AppImage
sigma-pkg appimage https://example.com/app.AppImage
```

### 5. Nix Packages (experimental)

Access to the Nix package ecosystem.

```bash
# Enable Nix integration
sigma-pkg nix enable
sigma-pkg nix install nixpkgs#hello
```

## PKGBUILD Compatibility

SigmaOS natively supports Arch Linux PKGBUILD format:

```bash
# Build and install from PKGBUILD
git clone https://aur.archlinux.org/package.git
cd package
sigma-pkg build .
```

## Sigma Package Format (.spkg)

Native SigmaOS package format with enhanced metadata:

```toml
[package]
name = "example"
version = "1.0.0"
arch = ["x86_64", "aarch64", "riscv64"]
license = "MIT"
description = "Example package"

[dependencies]
required = ["glibc", "openssl"]
optional = ["curl"]

[security]
signature = "ed25519:..."
sbom = true  # Software Bill of Materials

[install]
script = "install.sh"
post-install = "post-install.sh"
```

## Dependency Resolution

`sigma-pkg` uses a SAT-solver based dependency resolver inspired by DNF/libsolv:

*   Handles conflicts automatically
*   Suggests alternatives for conflicting packages
*   Supports version constraints and ranges
*   Generates dependency graph visualization: `sigma-pkg graph <pkg>`

## Atomic Transactions

All package operations are atomic:

*   If installation fails, system is rolled back
*   Transaction log at `/var/log/sigma-pkg/transactions.log`
*   Rollback: `sigma-pkg rollback [--to=<date>]`

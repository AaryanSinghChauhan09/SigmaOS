# SigmaOS — Package Manager Guide (`sigpkg`)

## Overview

`sigpkg` is the native SigmaOS package manager. It is inspired by Arch's pacman, Gentoo's portage, and NixOS's nix, combining the best of all three while maintaining full compatibility with `.deb`, `.rpm`, `.pkg.tar.zst`, and AppImage formats.

***

## Quick Reference

```bash
# Install a package
sigpkg install <name>

# Remove a package
sigpkg remove <name>

# Search for packages
sigpkg search <query>

# Update all packages
sigpkg update

# Full system upgrade
sigpkg upgrade

# List installed packages
sigpkg list

# Show package info
sigpkg info <name>

# Build from source recipe
sigpkg build ./RECIPE

# Add a repository
sigpkg repo add <url>
```

***

## Package Formats Supported

| Format | Distro Origin | Status |
|--------|--------------|--------|
| `.sig` (native) | SigmaOS | ✅ Native |
| `.pkg.tar.zst` | Arch Linux | ✅ Full support |
| `.deb` | Debian/Ubuntu | ✅ Full support |
| `.rpm` | Fedora/RHEL | ✅ Full support |
| `.apk` | Alpine Linux | 🔄 Partial |
| AppImage | Cross-platform | ✅ Full support |
| Flatpak | Cross-platform | ✅ Full support |
| Snap | Ubuntu | 🔄 Partial |

***

## Architecture

    sigpkg CLI
        │
        ├── src/package/mod.rs       — Core package management logic
        ├── src/package/universal.rs — Universal cross-format adapter
        ├── src/package/store.rs     — Local package database
        │
        ├── src/sigpkg/mod.rs        — Native .sig package format
        ├── src/sigpkg/recipe.rs     — Build recipe DSL (PKGBUILD-like)
        ├── src/sigpkg/makepkg.rs    — Build system
        ├── src/sigpkg/spec.rs       — USE flags and feature spec
        ├── src/sigpkg/store.rs      — Package store (hash-addressed)
        └── src/sigpkg/universal_adapter.rs — .deb/.rpm/.pkg adapter

***

## Native Package Recipe Format

SigmaOS recipes (`.recipe`) are similar to Arch's `PKGBUILD`:

```bash
# Example: firefox.recipe
name="firefox"
version="125.0"
description="Mozilla Firefox web browser"
url="https://www.mozilla.org/"
license="MPL-2.0"

# Dependencies
depends=("libgtk" "libdbus" "libasound")
makedepends=("rust" "python3" "nodejs")

# Optional feature flags (USE flags, Gentoo-inspired)
use_flags=("wayland" "pulseaudio" "webrtc" "bluetooth")

# Source archive
source="https://releases.mozilla.org/pub/firefox/releases/${version}/source/firefox-${version}.source.tar.xz"
sha256="abc123..."

build() {
    ./mach build
}

package() {
    ./mach install DESTDIR="${pkgdir}"
}

# Post-install
post_install() {
    sigma-update-desktop-db
}
```

***

## USE Flags (Gentoo-inspired)

Fine-grained compile-time feature selection:

```bash
# Enable USE flags globally
echo 'USE="wayland pulseaudio bluetooth"' >> /etc/sigpkg/make.conf

# Enable USE flags per-package
echo 'firefox: USE="wayland -pulseaudio"' >> /etc/sigpkg/package.use

# List available USE flags for a package
sigpkg info --use-flags firefox

# Build with specific USE flags
USE="wayland pulseaudio" sigpkg build firefox.recipe
```

***

## Package Store (NixOS-inspired)

SigmaOS uses a content-addressable package store:

    /sigma/store/
    ├── abc123def456-firefox-125.0/     # Hash-addressed install
    ├── 789xyz012abc-gtk-4.12.0/
    └── 345mno678pqr-libdbus-1.16.2/

Benefits:

*   **Multiple versions** of the same package coexist
*   **Atomic rollbacks** — switch to any previous generation
*   **No dependency conflicts** — each package has its own deps
*   **Reproducible builds** — same inputs always produce same output

***

## Repositories

### Official Repositories

| Repo | Description | Priority |
|------|-------------|----------|
| `sigma-core` | Essential system packages | 1 (highest) |
| `sigma-extra` | Extended community packages | 2 |
| `sigma-community` | User-maintained packages | 3 |
| `sigma-aur` | Arch-compatible recipes | 4 |

### Adding a Repository

```bash
# Add official extra repo
sigpkg repo add https://pkg.sigmaos.dev/extra

# Add a community repo
sigpkg repo add https://example.com/my-repo

# List configured repos
sigpkg repo list

# Update repo databases
sigpkg repo update
```

***

## Security & Signing

All packages in official repos are signed with Dilithium-5 (post-quantum):

```bash
# Verify package signature manually
sigpkg verify --pkg firefox-125.0.sig

# Import a maintainer key
sigpkg key import --id ABC123DEF456

# List trusted keys
sigpkg key list
```

***

## Build Farm

SigmaOS has a distributed build farm (`src/buildfarm/`) similar to Fedora's Koji:

*   All packages built in isolated containers
*   Reproducible build verification
*   Binary transparency log
*   Multi-architecture (x86\_64, aarch64, riscv64)

***

## Rollbacks & Generations

```bash
# List system generations
sigpkg gen list

# Roll back to previous generation
sigpkg gen rollback

# Roll back to specific generation
sigpkg gen switch 42

# Delete old generations
sigpkg gen gc --keep 3
```

***

*Last updated: 2026-08-23 | SigmaOS Package Management Team*

# SigmaOS Package Manager (SigmaPkg)

## Overview

SigmaPkg is SigmaOS's native package manager, a clean-room, zero-dependency implementation inspired by the best of Fedora's DNF, Arch's Pacman, Debian's APT, and Nix's purely functional model. It is fully written in Rust with no reliance on system package managers.

---

## Key Features

| Feature | Description |
|---------|-------------|
| **Atomic upgrades** | All-or-nothing transactions (Nix-inspired) |
| **Parallel downloads** | Multi-threaded fetch with io_uring backend |
| **GPG verification** | Ed25519 signature validation on all packages |
| **Dependency solver** | Optimized recursive resolver with cycle detection |
| **Delta packages** | Binary delta updates (bsdiff-inspired) |
| **Multiple repos** | Rolling, stable, and security channels |
| **Rollback** | Snapshot-based system rollback |
| **AUR-compatible** | Build-from-source packages like Arch AUR |

---

## Architecture

```
sigpkg/
├── mod.rs           # Core package manager
├── arch_compat.rs   # Arch Linux PKGBUILD compatibility
└── resolver/        # Dependency resolution engine
    ├── fedora.rs    # DNF/RPM-style resolver
    └── debian.rs    # APT-style resolver
```

### Dependency Resolution Algorithm

The dependency resolver uses a **topological sort with circular dependency detection**:

```rust
pub fn resolve_and_install(&mut self, name: &str) -> Result<Vec<String>, String> {
    // Pre-allocate for performance
    let mut install_order = Vec::with_capacity(self.packages.len().min(32));
    let mut visited = HashMap::with_capacity(self.packages.len().min(32));
    
    self.resolve_deps_recursive(name, &mut install_order, &mut visited)?;
    Ok(install_order)
}
```

**Complexity:** O(V + E) where V = packages, E = dependency edges

**Optimizations** (from `feature/optimize-dependency-resolvers`):
- Pre-allocated capacity avoids heap reallocations
- Reference-based deduplication vs. string cloning
- In-progress tracking prevents cycle traversal

---

## Repository Configuration

### sigma-rolling.toml
```toml
[repo.sigma-core]
url = "https://pkg.sigmaos.io/rolling/core"
gpg_key = "SIGMA_CORE_GPG_KEY"
priority = 100

[repo.sigma-extra]
url = "https://pkg.sigmaos.io/rolling/extra"
gpg_key = "SIGMA_EXTRA_GPG_KEY"
priority = 90
```

### sigma-stable.toml
```toml
[repo.sigma-stable]
url = "https://pkg.sigmaos.io/stable"
gpg_key = "SIGMA_STABLE_GPG_KEY"
```

---

## Package Format

SigmaOS packages use `.spkg` format (Sigma Package):

```
package.spkg
├── PKGINFO          # Package metadata (name, version, deps)
├── INSTALL          # Pre/post install scripts
├── data.tar.zst     # Compressed payload (zstd)
└── signature.ed25519 # Ed25519 signature over data.tar.zst
```

### PKGINFO Format
```
pkgname = firefox
pkgver = 128.0-1
pkgdesc = Mozilla Firefox web browser
arch = x86_64
depends = (libgtk glibc-sigma dbus-sigma)
makedepends = (rust cargo nodejs)
maintainer = SigmaOS Ports Team
```

---

## CLI Reference

```bash
# Install packages
sigma-pkg install firefox

# Search packages
sigma-pkg search "web browser"

# Update all packages (atomic upgrade)
sigma-pkg upgrade

# Rollback to previous snapshot
sigma-pkg rollback 1

# Build from source (AUR-compatible)
sigma-pkg build-src PKGBUILD

# Show dependency tree
sigma-pkg deptree firefox

# Verify package signatures
sigma-pkg verify firefox

# Remove package and orphans
sigma-pkg remove --orphans firefox
```

---

## Fedora/DNF Parity Module

**Module:** `src/compatibility/fedora.rs`

Implements DNF/RPM compatibility for running Fedora packages on SigmaOS:
- RPM package registration and metadata
- GPG signature verification
- Mock chroot builder for clean package builds
- SELinux context propagation
- systemd preset compatibility

```rust
let mut resolver = DnfPackageResolver::new();
resolver.sync_repodata();
resolver.register_rpm("firefox", vec!["libgtk", "dbus"]);
let install_order = resolver.resolve_and_install("firefox")?;
```

---

## Arch Linux Parity Module

**Module:** `src/sigpkg/arch_compat.rs`

PKGBUILD processing and makepkg compatibility:
- Parse and execute PKGBUILD scripts
- Source tarball download + verification
- Custom build environment (clean chroot)
- AUR helpers integration

---

## Nix-Inspired Features

### Atomic Transactions
Every package operation is atomic:
1. Download + verify all packages
2. Prepare new system generation
3. Atomically switch symlink to new generation
4. Rollback on failure

### Content-Addressable Store
Packages stored by content hash — identical files shared automatically.

---

*See also:*
- [DEPENDENCY_REDUCTION_GUIDE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DEPENDENCY_REDUCTION_GUIDE.md)
- [KLIB_REFERENCE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/KLIB_REFERENCE.md)
- [ARCH_LINUX_PARITY_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ARCH_LINUX_PARITY_ROADMAP.md)

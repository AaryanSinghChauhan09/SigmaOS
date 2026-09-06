# SigmaOS Package Manager (sigma-pkg)

## Overview

`sigma-pkg` is the SigmaOS native package manager. It supports:
- Native `.spkg` (SigmaOS Package) format
- Multi-distro compatibility: APT/dpkg, RPM/DNF, ALPM/pacman, APK, pkg, ports
- AUR-compatible user repository
- Cryptographic package verification (Ed25519 + Dilithium-5 PQC)
- Atomic transactions with rollback
- Declarative package lists (NixOS-style)

## Quick Reference

```bash
sigma-pkg install vim git curl           # Install packages
sigma-pkg remove vim                     # Remove package
sigma-pkg update                         # Update package lists
sigma-pkg upgrade                        # Upgrade all packages
sigma-pkg search rust                    # Search packages
sigma-pkg info vim                       # Package information
sigma-pkg list --installed               # List installed packages
sigma-pkg query /usr/bin/vim             # Which package owns this file?
sigma-pkg history                        # Transaction history
sigma-pkg rollback 3                     # Roll back to transaction #3
sigma-pkg verify vim                     # Verify package integrity
sigma-pkg clean                          # Clean cache
```

## Package Format (.spkg)

`.spkg` is a zstd-compressed tarball with metadata:

```
my-package-1.0.0-x86_64.spkg
├── METADATA/
│   ├── control.toml      ← Package metadata
│   ├── files.lst         ← File list with checksums
│   ├── pre-install.sh    ← Pre-install script
│   ├── post-install.sh   ← Post-install script
│   ├── pre-remove.sh     ← Pre-removal script
│   ├── post-remove.sh    ← Post-removal script
│   └── signature.ed25519 ← Package signature
└── data/                 ← File tree to install
    ├── usr/bin/my-tool
    ├── usr/lib/libmy.so
    └── usr/share/my-package/
```

### control.toml

```toml
[package]
name = "my-package"
version = "1.0.0"
release = 1
architecture = "x86_64"
description = "My example package for SigmaOS"
maintainer = "Alice <alice@example.com>"
license = "MIT"
url = "https://example.com/my-package"

[dependencies]
required = ["glibc >= 2.35", "openssl >= 3.0"]
optional = ["vim", "curl"]
build = ["cmake", "ninja"]
conflicts = ["old-my-package"]
provides = ["my-package-compat"]
replaces = ["my-package-legacy"]
```

For detailed specifications on version parsing, SemVer normalization algorithms, constraints, and automated testing procedures, see [Version Handling Documentation](version_handling.md).

## Repositories

```toml
# /etc/sigma/repositories.toml
[[repo]]
name = "sigma-official"
url = "https://pkg.sigmaos.dev/official"
priority = 100
signed = true
key = "/etc/sigma/keys/official.pub"

[[repo]]
name = "sigma-community"
url = "https://pkg.sigmaos.dev/community"
priority = 50
signed = true
key = "/etc/sigma/keys/community.pub"

[[repo]]
name = "sigma-compat-arch"
url = "https://pkg.sigmaos.dev/compat/arch"
priority = 30
backend = "pacman"

[[repo]]
name = "sigma-compat-debian"
url = "https://pkg.sigmaos.dev/compat/debian"
priority = 30
backend = "apt"
```

## Multi-Distro Compatibility

sigma-pkg can install packages from other distributions' repositories:

```bash
# Install from Arch AUR
sigma-pkg install --from aur yay

# Install Debian package
sigma-pkg install --from apt firefox

# Install RPM package
sigma-pkg install --from dnf firefox

# Install Alpine APK
sigma-pkg install --from apk musl-dev
```

## Declarative Packages (NixOS-style)

```toml
# /etc/sigma/packages.toml
[system]
packages = [
    "base",
    "sigma-kernel",
    "sigma-sh",
    "openssh",
    "vim",
    "git",
]

[desktop]
packages = [
    "sigma-desktop",
    "sigma-media",
    "firefox",
]

[development]
packages = [
    "rust",
    "gcc",
    "cmake",
    "lldb",
]
```

Apply declarative configuration:
```bash
sigma-pkg apply /etc/sigma/packages.toml
```

## Security

All packages are verified with:
1. **Ed25519 signature** — standard elliptic curve signature
2. **Dilithium-5** — post-quantum safe signature (future-proof)
3. **SHA-256 checksums** — per-file integrity verification

```bash
# Verify a downloaded package before installing
sigma-pkg verify-file my-package-1.0.0.spkg

# Import a new signing key
sigma-pkg key import /path/to/repo.pub

# List trusted keys
sigma-pkg key list
```

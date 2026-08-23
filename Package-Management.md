# 📦 Package Management in SigmaOS

SigmaOS features **sigma-pkg**, a universal package manager supporting multiple package formats with atomic operations and rollback.

---

## Overview

```
sigma-pkg
├── Native SigmaPkg (.spkg) — content-addressed, zero-dep
├── Arch ALPM/pacman compatible (.pkg.tar.zst)
├── Debian APT/deb compatible (.deb)
├── Fedora/RHEL RPM compatible (.rpm)
├── Gentoo Portage compatible (ebuilds)
└── Flatpak/AppImage (sandboxed)
```

---

## Basic Usage

### Install Packages
```bash
sigma-pkg install firefox
sigma-pkg install --aur firefox-wayland  # AUR-compatible
sigma-pkg install ./mypackage.spkg       # Local package
```

### Remove Packages
```bash
sigma-pkg remove firefox
sigma-pkg remove --purge firefox  # Remove config files too
```

### Update System
```bash
sigma-pkg update          # Update package database
sigma-pkg upgrade         # Upgrade all packages
sigma-pkg sync            # Full system sync
```

### Search
```bash
sigma-pkg search neovim
sigma-pkg info neovim
sigma-pkg files neovim    # List installed files
```

---

## Atomic Transactions & Rollback

SigmaOS uses Btrfs/ZFS-style snapshots for **atomic package operations**:

```bash
# List package snapshots
sigma-pkg snapshot list

# Roll back to previous state
sigma-pkg snapshot rollback

# Roll back to specific snapshot
sigma-pkg snapshot rollback --to snapshot-20260823-120000
```

### How It Works

```
sigma-pkg install nginx
    ↓
1. Dependency resolution (SAT solver)
    ↓
2. Package download + verification (Dilithium-5 sig check)
    ↓
3. Create filesystem snapshot (pre-install)
    ↓
4. Atomic staging area population
    ↓
5. Commit transaction (rename staging → live)
    ↓
6. Execute post-install hooks
    ↓
7. Record snapshot (post-install) for rollback
```

---

## Multi-Format Support

### Install from Different Formats
```bash
# Arch Linux AUR package
sigma-pkg install --format=alpm htop

# Debian package  
sigma-pkg install --format=deb ./htop_3.2.2_amd64.deb

# RPM package
sigma-pkg install --format=rpm ./htop-3.2.2.x86_64.rpm

# Gentoo ebuild (with USE flags)
sigma-pkg install --format=ebuild --use="ncurses X" sys-process/htop
```

---

## SigmaPkg Package Format

The native `.spkg` format uses content-addressed storage:

```toml
# PKGINFO format
name = "nginx"
version = "1.24.0"
arch = "x86_64"

[build]
source = "https://nginx.org/download/nginx-1.24.0.tar.gz"
sha256 = "..."
signature = "..."  # Dilithium-5 signature

[deps]
runtime = ["openssl", "pcre2", "zlib"]
build = ["gcc", "make", "perl"]

[hooks]
post_install = ["systemctl enable nginx"]
post_remove = ["systemctl disable nginx"]
```

---

## Repositories

```bash
# List configured repos
sigma-pkg repo list

# Add a repository
sigma-pkg repo add https://repo.example.com/sigmaos

# Remove a repository
sigma-pkg repo remove example-repo

# Sync repo databases
sigma-pkg repo sync
```

Default repositories:
- `core` — essential system packages
- `extra` — extended packages  
- `community` — community-maintained (AUR-style)
- `edge` — bleeding edge (rolling only)

---

*Related: [AUR Helper](AUR-Helper) | [Architecture Overview](Architecture-Overview) | [Components Master Table](Components-Master-Table)*

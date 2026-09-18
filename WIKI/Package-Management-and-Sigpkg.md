# 📦 Package Management & Universal Package Engine (`sigpkg`)

SigmaOS provides a unified, sovereign package management system called **`sigpkg`** (`src/bin/sigpkg.rs`, `src/sigpkg/universal_engine.rs`).

In addition to native, post-quantum signed `.sigpkg` packages, `sigpkg` seamlessly integrates **Arch Linux `PKGBUILD` parsing**, an **AUR Helper (`AurHelper`)**, **Reflector mirror ranking**, and **containerized multi-distro package adapters (`apx`)**.

---

## 🏗️ Architectural Overview

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                     UNIVERSAL PACKAGE ENGINE                           │
   ├────────────────────────────────────────────────────────────────────────┤
   │                         `sigpkg` CLI Facade                            │
   ├───────────────────┬────────────────────┬───────────────────────────────┤
   │ Native `.sigpkg`  │  Arch PKGBUILD     │ Multi-Distro Adapters (`apx`) │
   │ (Dilithium Signed)│  & AUR Helper      │ (.deb, .rpm, .apk, .xbps...)  │
   └───────────────────┴────────────────────┴───────────────────────────────┘
```

---

## 🛠️ `sigpkg` CLI Syntax & Operations

| Command | Action Description |
|---------|--------------------|
| `sigpkg install <pkg>` | Resolves dependencies via DPLL SAT solver and installs package |
| `sigpkg remove <pkg>` | Safely removes package and checks for orphan dependencies |
| `sigpkg search <query>` | Searches local database and remote repository indices |
| `sigpkg update` | Synchronizes package index metadata and updates system packages |
| `sigpkg rollback <id>` | Reverts system packages to snapshot `<id>` in < 1ms |
| `sigpkg rankmirrors` | Benchmarks remote mirrors via `ReflectorMirrorRanker` |
| `sigpkg clean` | Prunes cached package archives (`paccache_clean` parity) |

---

## 🏛️ Arch Linux Parity & AUR Build Engine (`AurHelper`)

SigmaOS provides 100% command and syntax parity with Arch Linux package tooling (`pacman`, `yay`, `paru`, `makepkg`, `namcap`).

### 1. Building Packages from `PKGBUILD`:
`sigpkg` parses standard Arch `PKGBUILD` manifests natively in Rust (`src/sigpkg/arch_pacman_engine.rs`):
```bash
# Clone AUR repository
git clone https://aur.archlinux.org/fastfetch.git
cd fastfetch

# Audit PKGBUILD diff AST for security risks
sigpkg aur audit PKGBUILD

# Compile and install package in isolated chroot container
sigpkg aur build -si
```

### 2. Yay/Paru Parity Helper Commands:
```bash
# Search and install from official repos + AUR
sigpkg aur search firefox

# Update all installed AUR packages
sigpkg aur update
```

---

## 🌐 Multi-Distro Adapter Pipeline (`apx`)

SigmaOS can install and manage packages from 60+ Linux & BSD package ecosystems without binary incompatibility or dependency hell:

### Supported Foreign Package Extensions:
- **Debian / Ubuntu**: `.deb` (parsed via `AptDebManifest`)
- **Arch / Manjaro**: `PKGBUILD`, `.pkg.tar.zst` (parsed via `ArchPkgInfoManifest`)
- **Fedora / RHEL**: `.rpm` (parsed via `FedoraRpmEngine`)
- **Alpine Linux**: `.apk` (parsed via `ApkIndexManifest`)
- **Void Linux**: `.xbps` (parsed via `XbpsManifest`)
- **FreeBSD**: `.txz`, `+MANIFEST` (parsed via `FreeBsdUclManifest`)
- **Haiku**: `.hpkg` (parsed via `HaikuHpkgManifest`)
- **Flatpak / Snap**: OCI sandboxed container applications

### Multi-Distro Installation Examples:
```bash
# Install Debian package via containerized adapter wrapper
sigpkg install --apt ./ripgrep_14.1.0_amd64.deb

# Install Fedora RPM package
sigpkg install --dnf ./htop-3.3.0-1.fc40.x86_64.rpm

# Install Alpine APK package
sigpkg install --apk htop-3.3.0-r0.apk
```

---

## 🧠 Dependency Resolution (DPLL SAT Solver)

`sigpkg` employs a **Davis-Putnam-Logemann-Loveland (DPLL) Boolean Satisfiability (SAT) Solver** (`src/package/dpll_solver.rs`) to evaluate package dependency graphs.

This guarantees:
- Zero circular dependency deadlocks.
- Deterministic conflict resolution for multi-version libraries.
- Optimal transactional order during bulk updates.

---

## 🔐 Post-Quantum Package Signing & Integrity

Every `.sigpkg` package is cryptographically verified using **Dilithium-5 post-quantum signatures** before installation.

Package manifests contain SHA-256 digests and post-quantum attestation tokens, protecting against supply-chain tampering and signature forgery.

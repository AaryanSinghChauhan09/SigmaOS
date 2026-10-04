# Package Management

SigmaOS features a sovereign package management stack built entirely in Rust. It provides a universal package manager (`sigpkg`), a source-based build system, AUR/ABS compatibility, Flatpak integration, and AI-powered dependency resolution — surpassing APT (Mint), pacman (Omarchy/Arch), and DNF in speed, safety, and extensibility.

---

## Architecture Overview

```
 ┌──────────────────────────────────────────────────────┐
 │                sigma-pkg  (CLI)                       │
 │  install │ remove │ upgrade │ search │ build │ audit  │
 └──────────────────────┬───────────────────────────────┘
                        │
 ┌──────────────────────▼───────────────────────────────┐
 │            SigmaPkg Core (src/sigpkg/ + src/package/)│
 │  DependencyResolver │ PackageDB │ DownloadManager     │
 │  SignatureVerifier  │ SandboxedInstall │ Rollback      │
 └──────────────────────┬───────────────────────────────┘
                        │
          ┌─────────────┼─────────────┐
          ▼             ▼             ▼
    SigmaRepo      AUR Bridge     Flatpak
    (native)       (compat)       (portal)
```

---

## Package Format

SigmaOS packages use the `.spkg` format (Sigma Package):

```
package.spkg  (zstd-compressed tar)
├── PKGINFO          # name, version, arch, deps, provides
├── INSTALL          # pre/post install hooks (Rust WASM)
├── VERIFY           # SHA256 manifest + Ed25519 sig
└── data/            # package files
    ├── usr/bin/...
    └── usr/lib/...
```

### PKGINFO Example
```toml
name = "sigma-browser"
version = "120.0.1"
arch = "x86_64"
depends = ["sigma-gtk4 >= 4.12", "sigma-webengine"]
provides = ["web-browser"]
conflicts = ["firefox"]
install_size_kb = 85000
checksum = "sha256:abc123..."
signature = "ed25519:xyz..."
```

---

## Dependency Resolver

- **SAT solver** (DPLL-based in Rust) — no version conflicts
- **Backtracking**: tries multiple versions, reports why each fails
- **Virtual packages**: `provides = ["web-browser"]` resolved transparently
- **AI-augmented**: flags packages with known regressions or CVEs

---

## Signature Verification

Every package is verified before installation:
1. Ed25519 signature over SHA256 manifest
2. Transparency log entry check (Sigma PKI ledger)
3. Build reproducibility verification (optional, for critical packages)

```
sigma-pkg verify firefox-120.spkg
✅ Signature valid (Sigma Official Key 2025)
✅ Build reproducible (hash match: sigmaos-build-farm/job-4421)
✅ No known CVEs in this version
```

---

## Repository Structure

```
https://repo.sigmaos.org/
├── stable/          # Curated, tested packages
├── testing/         # Pre-release packages
├── community/       # Community-maintained
└── aur-mirror/      # AUR packages auto-built for SigmaOS
```

---

## Compatibility

### Arch (pacman/AUR) Compatibility (`src/sigpkg/arch_compat.rs`)
- PKGBUILD parsing and execution in isolated sandbox
- `makepkg` equivalent in Rust
- AUR helpers: direct `sigma-pkg aur install <pkg>`
- Arch binary compatibility: `.pkg.tar.zst` installable

### Flatpak Integration (`src/sigpkg/flatpak_bridge.rs`)
- Flathub browsing and installation via `sigma-pkg flatpak install`
- Sandboxed via Sigma container runtime
- XDG portal integration for seamless desktop access
- Auto-updates alongside native packages

### Debian/RPM (`src/compatibility/`)
- `.deb` and `.rpm` extraction and conversion to `.spkg`
- Dependency mapping via translation layer
- Not recommended for production; use native packages

---

## Update Model

| Channel | Update Frequency | Stability |
|---------|-----------------|-----------|
| `stable` | Monthly snapshots | Production-ready |
| `testing` | Weekly | Pre-release testing |
| `rolling` | Daily | Cutting-edge, higher risk |

- Delta updates: only changed blocks downloaded (bsdiff-based)
- Atomic transactions: all-or-nothing (Btrfs snapshots)
- Rollback: `sigma-pkg rollback <package>` restores previous version

---

## Comparison vs APT / pacman / DNF

| Feature | APT | pacman | DNF | **sigma-pkg** |
|---------|-----|--------|-----|---------------|
| Language | C/Perl | C | Python | **Rust** |
| SAT solver | ✅ (aptcc) | ❌ | ✅ | ✅ DPLL |
| Atomic install | ❌ | ❌ | ❌ | ✅ Btrfs |
| Ed25519 sigs | ❌ | ✅ | ❌ | ✅ |
| Reproducibility | ❌ | ✅ partial | ❌ | ✅ |
| AI CVE alerts | ❌ | ❌ | ❌ | ✅ |
| Flatpak native | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/sigpkg/` | Package manager core |
| `src/sigpkg/arch_compat.rs` | Arch/AUR compatibility |
| `src/package/` | Package format and DB |
| `src/compatibility/` | Debian/RPM translators |
| `src/buildfarm/` | Build farm integration |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/sigpkg/`, `src/package/`
> - Update repository URL when infrastructure changes
> - Add new compatibility layers as they are implemented
> - Keep CVE/AI section current with `src/ai/` integration

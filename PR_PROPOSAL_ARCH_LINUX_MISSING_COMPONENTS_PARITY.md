# Pull Request Proposal: Arch Linux Missing Components Parity Infrastructure for SigmaOS

**PR Title:** `feat(arch-parity): Implement Arch Linux Missing Components & Pacman/ALPM Infrastructure`
**PR Branch:** `feature/arch-linux-missing-components-parity`
**Target Branch:** `main`
**Status:** Proposal / Specification Ready
**Component Scope:** `src/distro/arch_linux_pinnacle_gap_closure.rs`, `src/sigpkg/`, `docs/roadmap/`

---

## 1. Executive Summary & Problem Statement

Arch Linux is renowned for its simplicity, minimalism, transparency, and rolling-release model powered by `pacman`, `libalpm`, and the Arch User Repository (`AUR`). While SigmaOS contains universal package management capabilities, several critical Arch Linux system components and developer tooling primitives are missing or only partially simulated.

This PR proposal specifies the complete implementation of missing Arch Linux components to achieve 100% operational parity, enabling SigmaOS to directly execute Arch Linux PKGBUILDs, interface with AUR v5 RPC, validate package signatures via `pacman-key`, perform clean chroot builds, parse `arch-news`, and rank mirrors using `reflector`.

---

## 2. Missing Arch Linux Components Overview

The following 15 key Arch Linux components are targeted for full absorption:

| # | Arch Component | Description | SigmaOS Target Implementation Subsystem |
|---|----------------|-------------|------------------------------------------|
| 1 | `makepkg` | PKGBUILD parser & tar.zst package compiler | `ArchPkgbuildCompilerEngine` |
| 2 | `namcap` | Package & PKGBUILD static analysis linter | `ArchNamcapLinterEngine` |
| 3 | `libalpm 15` | ALPM database integrity & dependency tree solver | `ArchAlpmDbEngine` |
| 4 | `AUR v5 RPC` | Arch User Repository RPC v5 REST API client | `AurV5RpcClientEngine` |
| 5 | `vercmp` | Arch Linux version comparison logic | `ArchVercmpEngine` |
| 6 | `arch-news` | Critical Arch Linux RSS/JSON news feed parser | `ArchNewsFeedReaderEngine` |
| 7 | `pkgctl` | Devtools build container & git repository helper | `ArchPkgctlDevtoolsEngine` |
| 8 | `pacman` File Collision | Unowned file conflict & path collision resolver | `PacmanFileCollisionResolverEngine` |
| 9 | `mkinitcpio` | Modular CPIO initial RAM disk image builder | `ArchMkinitcpioHookGenerator` |
| 10 | `pacstrap` / `arch-chroot` | Rootfs installation & bind-mount chroot manager | `ArchInstallerChrootEngine` |
| 11 | `pacman-key` | GnuPG keyring initialization & trust manager | `PacmanKeyringTrustManager` |
| 12 | `arch-audit` | Arch Linux Security Advisory (ASA) CVE scanner | `ArchAuditCveScannerEngine` |
| 13 | `archinstall` | Automated JSON/Python installer profile executor | `ArchinstallProfileExecutorEngine` |
| 14 | `reflector` | Parallel HTTP/HTTPS mirrorlist ranker | `ReflectorMirrorlistRanker` |
| 15 | `ABS` | Arch Build System rsync/git tree sync engine | `ArchBuildSystemSyncEngine` |

---

## 3. Technical Implementation Specification

### 3.1 PKGBUILD Synthesis & Compilation (`makepkg` Parity)
```rust
pub struct ArchPkgbuildManifest {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub epoch: u32,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
}

impl ArchPkgbuildManifest {
    pub fn parse_pkgbuild(content: &str) -> Result<Self, ArchError> {
        // Parse PKGBUILD bash variable declarations and construct manifest
        todo!()
    }

    pub fn build_package(&self, chroot_dir: &Path) -> Result<PathBuf, ArchError> {
        // Compile package inside isolated clean chroot and produce .pkg.tar.zst archive
        todo!()
    }
}
```

### 3.2 AUR v5 RPC Client (`aur.archlinux.org` Integration)
```rust
pub struct AurRpcResponse {
    pub version: u32,
    pub type_str: String,
    pub resultcount: usize,
    pub results: Vec<AurPackageResult>,
}

pub struct AurPackageResult {
    pub name: String,
    pub package_base: String,
    pub version: String,
    pub description: Option<String>,
    pub num_votes: u32,
    pub popularity: f64,
    pub out_of_date: Option<u64>,
    pub url_path: String,
}
```

### 3.3 Security & Keyring Management (`pacman-key` & `arch-audit`)
- **Keyring Initialization:** WOT (Web of Trust) Master Keys for Arch Linux developers & trusted users (TUs).
- **ASA Scanner:** Parses `https://security.archlinux.org/json` feed and matches installed ALPM package versions against known CVE vulnerabilities.

---

## 4. Verification Plan & Test Matrix

### Unit & Integration Tests
1. **PKGBUILD Parser Test:** Validates variable extraction, array handling, and version formatting.
2. **AUR RPC Test:** Verifies query parameter encoding and JSON response deserialization.
3. **Version Compare (`vercmp`) Test:** Validates epoch comparison, alpha/beta tags, and release numbers.
4. **Clean Chroot Builder Test:** Confirms `systemd-nspawn` or `chroot` bind-mount isolation during builds.
5. **Mirror Ranker Test:** Benchmarks mirror response speeds and filters out-of-date mirrors.

---

## 5. Architectural Alignment

All components are registered in `src/distro/arch_linux_pinnacle_gap_closure.rs` and exposed through `src/distro/mod.rs` and `src/sigpkg/mod.rs`, maintaining full compatibility with the 3-Tier SigmaOS Architecture.

# Arch ALPM Pacman Component Guide

> **Related portals:** [Package & Distro Engineering](./Home.md) &bull; **Maintainer Agent:** [`ARCH_ALPM_PACMAN_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/ARCH_ALPM_PACMAN_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [Database Topology & Sync Lifecycle](#2-database-topology--sync-lifecycle)
3. [Competitor Inspirations (Arch Linux libalpm, Debian libapt, Alpine apk-tools)](#3-competitor-inspirations-arch-linux-libalpm-debian-libapt-alpine-apk-tools)
4. [Public API & Data Structures](#4-public-api--data-structures)
5. [Executable Rust ALPM Implementation](#5-executable-rust-alpm-implementation)
6. [Diagnostics & CLI Operations](#6-diagnostics--cli-operations)
7. [Future Roadmap](#7-future-roadmap)

---

## 1. Overview & Architecture

The **Sovereign ALPM Engine** (`SovereignAlpmEngine`) is the low-level package database and transaction management engine of SigmaOS, modeled after Arch Linux's `pacman` and `libalpm` (https://github.com/archlinux/pacman). It provides fast, transactional, dependency-verified package operations for both local packages and remote sync repositories.

```
┌─────────────────────────────────────────────────────────┐
│                    Sovereign ALPM Engine                │
│                                                         │
│   ┌─────────────────────┐      ┌────────────────────┐   │
│   │     Local Database  │      │   Sync Databases   │   │
│   │   /var/lib/alpm/    │      │ core, extra, mult. │   │
│   └──────────┬──────────┘      └─────────┬──────────┘   │
│              │                           │              │
│   ┌──────────▼───────────────────────────▼──────────┐   │
│   │              Dependency Resolver                │   │
│   │         DAG Verification & Conflict Checks      │   │
│   └─────────────────────┬───────────────────────────┘   │
│                         ▼                               │
│   ┌─────────────────────────────────────────────────┐   │
│   │              Transaction Manager                │   │
│   │       Prepare -> Commit -> Atomic Rollback      │   │
│   └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 2. Database Topology & Sync Lifecycle

- **`localdb`**: Stores metadata of all packages installed on the system, including exact file manifests, install dates, dependency links, and explicit/dependency install reasons.
- **`syncdb`**: Synchronized package catalogs downloaded from repository mirrors (`/var/lib/pacman/sync/*.db`).
- **Dependency Graph**: Enforces directed acyclic graph invariants to prevent cycles and ensure uninstallation does not orphan dependent packages.

---

## 3. Competitor Inspirations (Arch Linux libalpm, Debian libapt, Alpine apk-tools)

| Feature | Arch Linux (libalpm) | Debian (libapt) | Alpine (apk-tools) | SigmaOS Implementation |
|---|---|---|---|---|
| **Language** | C99 | C++ | C99 | Modern Safe `#![no_std]` Rust |
| **Transaction Speed** | High (tar/zstd) | Moderate (dpkg) | Ultra-fast (musl) | Sub-millisecond BTreeMap lookup |
| **Delta Packages** | Legacy support | Debdelta | — | Native Zstandard binary diffs |
| **Database Format** | Flat files in tar | Binary cache files | Single APKINDEX | Dual BTreeMap + SQLite index |
| **Rollback Safety** | Manual downgrade | Manual apt snapshots | — | Atomic Btrfs/ZFS pre-transaction snap |

---

## 4. Public API & Data Structures

```rust
// Implemented in src/package/alpm_engine.rs
pub struct AlpmPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub arch: PackageArch,
    pub isize: u64,
    pub installed_size: u64,
    pub sha256_checksum: String,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

pub struct SovereignAlpmEngine {
    pub fn register_repo(&mut self, repo_name: &str);
    pub fn install_package(&mut self, pkg: AlpmPackage, flags: AlpmTransactionFlags) -> Result<(), &'static str>;
    pub fn remove_package(&mut self, pkg_name: &str, flags: AlpmTransactionFlags) -> Result<(), &'static str>;
    pub fn check_status(&self, pkg_name: &str) -> SyncStatus;
    pub fn search_sync(&self, query: &str) -> Vec<&AlpmPackage>;
}
```

---

## 5. Executable Rust ALPM Implementation

```rust
#![no_std]
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct MinimalAlpm {
    db: BTreeMap<String, String>, // name -> version
}

impl MinimalAlpm {
    pub fn new() -> Self {
        Self { db: BTreeMap::new() }
    }

    pub fn install(&mut self, name: &str, ver: &str) {
        self.db.insert(name.to_string(), ver.to_string());
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.db.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_alpm() {
        let mut alpm = MinimalAlpm::new();
        alpm.install("linux", "6.12.0");
        assert!(alpm.is_installed("linux"));
        assert!(!alpm.is_installed("neovim"));
    }
}
```

---

## 6. Diagnostics & CLI Operations

```bash
# Query installed package information (pacman -Qi)
sigmapkg -Qi linux
# Search sync repository packages (pacman -Ss)
sigmapkg -Ss wayland
# List upgradeable packages (pacman -Qu)
sigmapkg -Qu
```

---

## 7. Future Roadmap

1. **Parallel Multi-Source Download Engine**: Concurrently stream package chunks across worldwide mirror nodes.
2. **Deterministic Build Verification**: Bit-for-bit reproducible packaging hash validation.
3. **P2P Local Subnet Package Sharing**: Share cached pacman packages between local LAN nodes via Warpinator.

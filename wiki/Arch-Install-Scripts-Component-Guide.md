# Arch Install Scripts Component Guide

> **Related portals:** [Package & Distro Engineering](./Home.md) &bull; **Maintainer Agent:** [`ARCH_INSTALL_SCRIPTS_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/ARCH_INSTALL_SCRIPTS_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [The Bootstrap Triad: pacstrap, genfstab, arch-chroot](#2-the-bootstrap-triad-pacstrap-genfstab-arch-chroot)
3. [Competitor Inspirations (Arch install scripts, Debian debootstrap, Alpine setup-alpine)](#3-competitor-inspirations-arch-install-scripts-debian-debootstrap-alpine-setup-alpine)
4. [Public API & Data Structures](#4-public-api--data-structures)
5. [Executable Rust Arch Install Scripts Implementation](#5-executable-rust-arch-install-scripts-implementation)
6. [Diagnostics & Installation Walkthrough](#6-diagnostics--installation-walkthrough)
7. [Future Roadmap](#7-future-roadmap)

---

## 1. Overview & Architecture

The **Sovereign Arch Install Scripts Engine** (`SovereignArchInstallScripts`) delivers clean, reproducible installation primitives inspired by Arch Linux's `arch-install-scripts` (https://github.com/archlinux/arch-install-scripts). It provides three foundational utilities:
1. **`pacstrap`**: Bootstrapping an isolated system root directory from packages without host architecture pollution.
2. **`genfstab`**: Automatically discovering mount structures and formatting `/etc/fstab` with persistent UUIDs or volume LABELs.
3. **`arch-chroot`**: Creating a containerized mount namespace with pseudo-filesystems (`/proc`, `/sys`, `/dev`, `/run`, `/sys/firmware/efi/efivars`) bind-mounted and safely unmounted upon exit.

```
┌─────────────────────────────────────────────────────────┐
│               Sovereign Arch Install Engine             │
│                                                         │
│   Step 1: pacstrap /mnt base linux sigma-tools          │
│   ┌─────────────────────────────────────────────────┐   │
│   │   Provisions isolated filesystem root tree      │   │
│   └────────────────────────┬────────────────────────┘   │
│                            │                            │
│   Step 2: genfstab -U /mnt >> /mnt/etc/fstab            │
│   ┌────────────────────────▼────────────────────────┐   │
│   │   Generates UUID-based declarative mount tables │   │
│   └────────────────────────┬────────────────────────┘   │
│                            │                            │
│   Step 3: arch-chroot /mnt                              │
│   ┌────────────────────────▼────────────────────────┐   │
│   │   Enters namespace-isolated configuration shell │   │
│   └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 2. The Bootstrap Triad: pacstrap, genfstab, arch-chroot

- **Pacstrap Engine**: Bypasses host package state to deploy pristine binaries and configuration templates into `/mnt`.
- **Genfstab Parser**: Scans active mountpoints (`/proc/mounts`), correlates block device UUIDs, identifies Btrfs/ZFS subvolumes, and writes `/etc/fstab`.
- **Chroot Supervisor**: Automates bind mounts and handles process termination signals (`SIGINT`, `SIGTERM`) to guarantee zero orphaned mounts.

---

## 3. Competitor Inspirations (Arch install scripts, Debian debootstrap, Alpine setup-alpine)

| Tool | Arch Linux | Debian | Alpine | SigmaOS Sovereign |
|---|---|---|---|---|
| **Bootstrap Utility** | `pacstrap` | `debootstrap` | `apk --root` | `SovereignArchInstallScripts::pacstrap` |
| **Fstab Synthesizer** | `genfstab` | Manual fstab | `setup-disk` | `SovereignArchInstallScripts::genfstab` |
| **Chroot Manager** | `arch-chroot` | `chroot` (manual) | `chroot` | `SovereignArchInstallScripts::arch_chroot` |
| **EFI Vars Mount** | Automatic | Manual | — | Automatic `/sys/firmware/efi/efivars` |

---

## 4. Public API & Data Structures

```rust
// Implemented in src/installer/arch_install_scripts.rs
pub struct MountEntry {
    pub device: String,
    pub uuid: String,
    pub label: String,
    pub mountpoint: String,
    pub fstype: String,
    pub options: String,
    pub dump: u8,
    pub pass: u8,
}

pub struct SovereignArchInstallScripts {
    pub fn pacstrap(config: PacstrapConfig) -> Result<BootstrapResult, &'static str>;
    pub fn genfstab(mounts: &[MountEntry], mode: FstabIdMode) -> String;
    pub fn prepare_chroot_bind_mounts(target_dir: &str) -> Vec<String>;
}
```

---

## 5. Executable Rust Arch Install Scripts Implementation

```rust
#![no_std]
extern crate alloc;
use alloc::format;
use alloc::string::String;

pub struct SimpleFstabEntry {
    pub uuid: &'static str,
    pub mount: &'static str,
    pub fstype: &'static str,
}

impl SimpleFstabEntry {
    pub fn format_entry(&self) -> String {
        format!("UUID={} {} {} defaults 0 0\n", self.uuid, self.mount, self.fstype)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fstab_entry() {
        let entry = SimpleFstabEntry {
            uuid: "1234-abcd",
            mount: "/",
            fstype: "btrfs",
        };
        assert_eq!(entry.format_entry(), "UUID=1234-abcd / btrfs defaults 0 0\n");
    }
}
```

---

## 6. Diagnostics & Installation Walkthrough

```bash
# Bootstrap clean base system into /mnt
sigmactrl pacstrap /mnt base linux linux-firmware
# Generate persistent UUID fstab
sigmactrl genfstab -U /mnt > /mnt/etc/fstab
# Enter chroot environment for configuration
sigmactrl arch-chroot /mnt
```

---

## 7. Future Roadmap

1. **Declarative YAML System Recipes**: Single-file automated installation configs (like Nix/Alpine).
2. **Zstandard Image Direct-Stream**: Stream root filesystem directly from compressed container images.
3. **Automatic LUKS2 Keyslot Enrollment**: Enroll TPM2 auto-unlock keys during `pacstrap` installation.

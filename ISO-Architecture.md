# ISO Architecture — SigmaOS Sovereign Live ISO

## Overview

SigmaOS uses a layered, composable ISO architecture inspired by the best practices from:
- **Linux Mint** (Calamares GUI installer, overlayfs live system)
- **Arch Linux** (archinstall, limine, mkinitcpio)
- **Omarchy** (Hyprland-native, minimal opinionated live ISO)
- **Debian** (live-build, apt-based package manifests)
- **FreeBSD** (bsdinstall, ZFS support, capsicum security)
- **OpenBSD** (autoinstall, pledge/unveil hardening)
- **ReactOS** (BIOS/UEFI hybrid El Torito)

## Principles Applied

| Principle | How Applied |
|---|---|
| ABSTRACTION | `IsoLayer` trait abstracts all ISO components |
| LAYERED DESIGN | Boot → LiveFS → Installer → Packages → Signing |
| SEPARATION OF CONCERNS | Each layer has a single responsibility |
| MODULARITY | Layers are independently composable |
| COMPOSITION OVER INHERITANCE | Profile builds from layer instances |
| GALL'S LAW | Start with stub ISO, evolve incrementally |
| ZERO-COPY | Squashfs + overlayfs: no duplicate data in memory |
| MEMORY DISCIPLINE | tmpfs sized to ≤50% RAM by default |
| DEFENSIVE PROGRAMMING | All layers validate before `build()` |
| FAIL-FAST | Validation errors aggregate and report early |
| SECURE DEFAULTS | GPG signing, SHA256/512, SBOM enabled by default |
| PORTABILITY | x86_64, aarch64, riscv64 targets supported |
| BOOTLOADER INTEGRATION | GRUB2 + Limine + syslinux (BIOS+UEFI) |
| SOFTWARE SUPPORT PROGRAM | LTS / Standard / Rolling / Edge channels |
| SOFTWARE UPDATING PROGRAM | casync delta updates |
| PRISTINE SOURCES | GPG-signed ISO images + SBOM |

## Source Location

```
src/iso/
├── mod.rs                          # Module root
└── sovereign_iso_architecture.rs   # Full ISO layer system
```

## Key Types

### `IsoLayer` Trait

```rust
pub trait IsoLayer {
    fn name(&self) -> &str;
    fn validate(&self) -> Result<(), IsoError>;
    fn estimated_size_bytes(&self) -> u64;
    fn description(&self) -> &str;
}
```

### `SigmaIsoProfile`

Top-level composition of all layers:

```rust
pub struct SigmaIsoProfile {
    pub name: String,
    pub version: String,
    pub arch: ArchTarget,
    pub boot: BootLayer,
    pub live_fs: LiveFilesystemLayer,
    pub installer: InstallerLayer,
    pub packages: PackageManifestLayer,
    pub signing: SigningLayer,
    pub output_path: String,
}
```

### Creating Profiles

```rust
// Standard desktop ISO (Linux Mint style)
let desktop = SigmaIsoProfile::desktop_standard(ArchTarget::X86_64);

// Minimal server ISO (unattended install)
let server = SigmaIsoProfile::server_minimal(ArchTarget::Aarch64);

// Validate before building
desktop.validate().expect("Profile must be valid");
```

## Boot Configuration

### GRUB2 (Linux Mint / Debian style)

Generated via `BootLayer::generate_grub_cfg()`:

```
menuentry "SigmaOS Desktop (Live)"
menuentry "SigmaOS Desktop (Safe Mode)"
menuentry "SigmaOS Installer"
menuentry "Memory Test (memtest86+)"
menuentry "Boot from next device"
```

### Limine (Arch Linux / modern UEFI style)

Generated via `BootLayer::generate_limine_conf()`:

```
/SigmaOS Desktop (Live)
    PROTOCOL=linux
    KERNEL_PATH=boot:///boot/vmlinuz
    MODULE_PATH=boot:///boot/initrd.img
```

## Live Filesystem

Uses squashfs with overlayfs write layer:

| Option | Default | Notes |
|---|---|---|
| Compression | `zstd` | Fast decompress, good ratio |
| Block size | 128K | Optimal for sequential read |
| tmpfs size | 50% RAM | Write layer budget |
| Overlay backend | `overlayfs` | Kernel native |

## Installer

Supports multiple installer backends via `InstallerType`:

| Type | Inspired By | Use Case |
|---|---|---|
| `CalamaresGui` | Linux Mint | Desktop: friendly GUI |
| `ArchinstallTui` | Arch Linux | Power users: JSON config |
| `SigmaTuiNative` | OpenBSD | Rust TUI, no dependencies |
| `UnattendedAutomatic` | Ubuntu autoinstall | Cloud/server automation |

## Partition Strategies

| Strategy | Inspired By |
|---|---|
| `WholeDisc` | Linux Mint default |
| `BtrfsSubvolumes` | Arch/openSUSE `@root @home @snapshots` |
| `LvmEncrypted` | Fedora/Debian LUKS+LVM |
| `Zfs` | FreeBSD `zroot` pool |
| `ManualPartitioning` | Expert mode |

## Support Channels

```rust
pub enum SupportChannel {
    Lts,       // 60 months security updates
    Standard,  // 9 months
    Rolling,   // Indefinite (Arch/Omarchy style)
    Edge,      // Bleeding edge, no guarantees
}
```

## Delta Updates

```rust
let updater = IsoDeltaUpdater::new("1.0", "1.1", "https://updates.sigmaos.dev");
// Generates: FROM:1.0 TO:1.1 DELTA:https://... SIG:https://...
```

## Build Roadmap

- [x] `sovereign_iso_architecture.rs` — Types, traits, profiles
- [x] `scripts/build_iso.sh` — Full ISO builder
- [x] `scripts/build_stub_iso.sh` — Quick stub ISO (GRUB2)
- [ ] `scripts/build_delta.sh` — Delta update generator
- [ ] `src/installer/calamares_bridge.rs` — Calamares integration
- [ ] Multi-arch CI matrix (x86_64 + aarch64)
- [ ] SBOM generation in CI (SLSA Level 2)

# Linux Distro Inspirations in SigmaOS

## Overview

SigmaOS draws inspiration from many Linux distributions and incorporates their best ideas while implementing them natively in Rust. This document maps each inspiration source to its concrete implementation in the SigmaOS codebase.

---

## Table of Contents

1. [Arch Linux](#arch-linux)
2. [Fedora](#fedora)
3. [NixOS](#nixos)
4. [Tails](#tails)
5. [Kali Linux](#kali-linux)
6. [Alpine Linux](#alpine-linux)
7. [CachyOS](#cachyos)
8. [Garuda Linux](#garuda-linux)
9. [Debian](#debian)
10. [Manjaro](#manjaro)
11. [Tiny Core Linux](#tiny-core-linux)
12. [BSD Inspirations](#bsd-inspirations)
13. [Feature Comparison Matrix](#feature-comparison-matrix)

---

## Arch Linux

**Core inspiration:** Rolling release model, minimal base, user empowerment, AUR.

### What SigmaOS Borrows

| Arch Feature | SigmaOS Implementation | File |
|-------------|----------------------|------|
| Rolling release channels | `RollingReleaseChannel` struct | `src/distro/improvements.rs` |
| Minimal base install | `MinimalBaseInstaller` | `src/distro/improvements.rs` |
| AUR-style user repos | `UserPackageRepository` | `src/distro/improvements.rs` |
| Pacman package format | `PacmanPackageDriverTranslator` | `src/package/linux_translation.rs` |
| PKGBUILD-style recipes | `SigmaRecipe` | `src/sigpkg/recipe.rs` |
| Arch Linux compat layer | `ArchCompatLayer` | `src/sigpkg/arch_compat.rs` |

### Rolling Release Philosophy
SigmaOS's `sigma-rolling` channel provides continuous delivery of packages:

```toml
# sigma-rolling.toml
[channel]
name = "sigma-rolling"
update_interval = "daily"
pre_upgrade_snapshot = true
allow_downgrades = false
```

### AUR-Style Packaging

```toml
# example SigmaOS PKGBUILD equivalent
[package]
name = "my-app"
version = "1.0.0"
source = "https://example.com/my-app-1.0.0.tar.gz"
sha256 = "abc123..."

[build]
commands = ["cargo build --release"]

[install]
binaries = ["target/release/my-app"]
```

---

## Fedora

**Core inspiration:** Btrfs by default, system snapshots, leading-edge packages, flatpak integration.

### What SigmaOS Borrows

| Fedora Feature | SigmaOS Implementation | File |
|---------------|----------------------|------|
| Btrfs as default FS | `BtrfsVolumeManager` | `src/distro/improvements.rs` |
| System snapshots | `SystemSnapshotManager` | `src/distro/improvements.rs` |
| Snapshot on upgrade | `auto_snapshot_on_upgrade: true` | `src/distro/improvements.rs` |
| Flatpak integration | `FlatpakCompatLayer` | `src/tools/sigma_flatpak_compat.rs` |
| DNF/RPM compat | `RpmPackageDriverTranslator` | `src/package/linux_translation.rs` |

### Snapshot Workflow

```
Before upgrade:
    SystemSnapshotManager::create_snapshot("pre-upgrade", true)

After failed upgrade:
    SystemSnapshotManager::rollback_to(snapshot_id)

Manual snapshot:
    sigma-snap create --description "before config change"
```

### Btrfs Subvolume Layout

```
/dev/sda
├── @           → /             (compress=zstd:3, noatime)
├── @home       → /home         (compress=zstd:3, noatime)
├── @snapshots  → /.snapshots   (noatime)
├── @var/log    → /var/log      (nodatacow)
└── @tmp        → /tmp          (nodatacow)
```

---

## NixOS

**Core inspiration:** Declarative configuration, functional package management, atomic upgrades, reproducible builds.

### What SigmaOS Borrows

| NixOS Feature | SigmaOS Implementation | File |
|--------------|----------------------|------|
| `configuration.nix` | `DeclarativeSystemConfig` struct | `src/distro/improvements.rs` |
| Nix store paths | `NixStyleStorePath` | `src/distro/improvements.rs` |
| Atomic upgrades | `AtomicUpgradeEngine` | `src/distro/improvements.rs` |
| Generation rollback | `AtomicUpgradeEngine::rollback()` | `src/distro/improvements.rs` |
| Reproducible builds | `sigma_repro_build.sh` | `tools/sigma_repro_build.sh` |
| Store-based caching | `PackageStore` | `src/sigpkg/store.rs` |

### Declarative Configuration Example

```toml
# sigma.toml (declarative system configuration)
[system]
hostname = "my-sigma-machine"
timezone = "UTC"

[packages]
install = [
    "sigmaos-base",
    "sigma-desktop",
    "sigma-browser",
    "sigma-office",
]

[services]
enable = ["sigma-sshd", "sigma-firewall", "sigma-update-daemon"]

[users.alice]
groups = ["wheel", "sigma-admin"]
shell = "/usr/bin/sigma-sh"

[boot]
max_generations = 10
```

### Atomic Upgrade Process

```
Current Generation N
        │
        ▼
Stage new config ──→ AtomicUpgradeEngine::stage_upgrade(config)
        │
        ▼
Build new generation ──→ AtomicUpgradeEngine::commit_upgrade()
        │
        ├─ Success → Generation N+1 becomes active
        └─ Failure → AtomicUpgradeEngine::rollback() → Generation N
```

---

## Tails

**Core inspiration:** Amnesic sessions, privacy by default, Tor integration, live USB.

### What SigmaOS Borrows

| Tails Feature | SigmaOS Implementation | File |
|--------------|----------------------|------|
| Amnesic sessions | `EphemeralSessionManager` | `src/distro/improvements.rs` |
| Amnesic trait | `Amnesic` trait, `RamDisk` | `src/distro/improvements.rs` |
| Tor-only networking | `NetworkPrivacyMode::TorOnly` | `src/distro/improvements.rs` |
| Tor client | `TorClient` | `src/net/tor_client.rs` |
| Encrypted persistence | `EncryptedPersistentStorage` | `src/distro/improvements.rs` |
| RAM disk wipe on shutdown | `RamDisk::wipe()` | `src/distro/improvements.rs` |

### Ephemeral Session Configuration

```toml
# sigma-ephemeral.toml
[session]
amnesic = true
network = "tor"
persistent_storage = false

[privacy]
dns_over_tor = true
block_cleartext_dns = true
disable_swap = true
```

### Privacy Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| `TorOnly` | All traffic via Tor | Investigative journalism, activism |
| `Offline` | No network at all | Air-gapped forensics |
| `Direct` | Normal internet | Development |
| `Vpn` | VPN tunnel | Corporate privacy |

---

## Kali Linux

**Core inspiration:** Penetration testing toolkit, live forensics, security-focused.

### What SigmaOS Borrows

| Kali Feature | SigmaOS Implementation | File |
|-------------|----------------------|------|
| Tool registry | `PenTestToolRegistry` | `src/distro/improvements.rs` |
| Tool categories | `PenTestCategory` enum | `src/distro/improvements.rs` |
| Live forensics | `LiveForensicsSession` | `src/distro/improvements.rs` |
| Chain of custody | `CustodyEntry` | `src/distro/improvements.rs` |
| Pentest profile | Security profile `pentest` | `sigma.toml` |
| Vulnerability scanner | `VulnerabilityScanner` | `src/security/vulnerability.rs` |

### Penetration Testing Tool Categories

| Category | Example Tools |
|----------|--------------|
| `NetworkScanning` | sigma-nmap, sigma-masscan |
| `WebApplicationTesting` | sigma-nikto, sigma-burp-compat |
| `PasswordCracking` | sigma-hashcat, sigma-john |
| `WirelessAttacks` | sigma-aircrack, sigma-hcxtools |
| `ForensicsAndRecovery` | sigma-foremost, sigma-autopsy |
| `ExploitDevelopment` | sigma-metaframe, sigma-pwndbg |
| `ReverseEngineering` | sigma-ghidra, sigma-radare2 |

### Enabling Pentest Mode

```toml
# sigma.toml
[security]
profile = "pentest"
capability_overrides = ["CAP_SIGMA_PENTEST", "CAP_NET_RAW"]
```

---

## Alpine Linux

**Core inspiration:** musl libc, minimal footprint, OpenRC init, static linking, security-focused.

### What SigmaOS Borrows

| Alpine Feature | SigmaOS Implementation | File |
|--------------|----------------------|------|
| musl-style libc | `LibcBackend::SigmaMusl` | `src/distro/improvements.rs` |
| Minimal runtime | `MinimalRuntime::new_alpine_style()` | `src/distro/improvements.rs` |
| Static linking | `static_linking: true` | `src/distro/improvements.rs` |
| OpenRC-style init | `OpenRcStyleInit` | `src/distro/improvements.rs` |
| Runlevels | `Runlevel` enum | `src/distro/improvements.rs` |
| APK compat | `ApkCompatLayer` | `src/tools/sigma_apk_compat.rs` |
| musl shim | `sigma_musl_shim.cpp` | `userland/posix/` |

### Alpine Footprint Target

```
SigmaOS Base Install Target:
  Disk:   < 512 MB
  RAM:    < 64 MB (headless), < 256 MB (desktop)
  Kernel: musl-linked, stripped, ~8 MB
```

### OpenRC-Style Service Management

```bash
# Service management
sigma-rc start sshd
sigma-rc stop sshd
sigma-rc enable nginx
sigma-rc status --all
```

---

## CachyOS

**Core inspiration:** BORE scheduler, kernel optimizations for gaming/desktop, CPU-specific builds.

### What SigmaOS Borrows

| CachyOS Feature | SigmaOS Implementation | File |
|----------------|----------------------|------|
| BORE scheduler | `BoreScheduler` | `src/distro/improvements.rs` |
| BORE config | `BoreSchedulerConfig` | `src/distro/improvements.rs` |
| CPU-optimized kernel | `OptimizedKernelProfile` | `src/distro/improvements.rs` |
| Cache-aware placement | `cache_aware: true` | `src/distro/improvements.rs` |
| PGO kernel builds | `pgo_enabled: true` | `src/distro/improvements.rs` |
| Sovereign scheduler | `SigmaScheduler` | `src/scheduler/sovereign.rs` |

### BORE Scheduler Explained

BORE (Burst-Oriented Response Enhancer) extends CFS with a burst score:

```
Pick Next = min(vruntime + burst_score) across run queue

burst_score accumulates when a task uses its full time slice
burst_score decays when a task yields early (I/O-bound tasks get lower scores)
```

This makes interactive tasks (low burst score) preempt batch tasks (high burst score) more aggressively, improving desktop responsiveness.

### CPU Profile Selection

```toml
# sigma.toml
[kernel]
cpu_profile = "zen4"     # znver4 with AVX-512, BMI2, LTO+PGO
# options: generic, zen3, zen4, skylake, alderlake, native
```

---

## Garuda Linux

**Core inspiration:** Btrfs with zstd compression everywhere, Snapper integration, beautiful defaults.

### What SigmaOS Borrows

| Garuda Feature | SigmaOS Implementation | File |
|---------------|----------------------|------|
| Btrfs + zstd | `GarudaBtrfsLayout` | `src/distro/improvements.rs` |
| Snapper integration | `SnapperIntegration`, `SnapperConfig` | `src/distro/improvements.rs` |
| Timeline snapshots | `timeline_enabled: true` | `src/distro/improvements.rs` |
| `discard=async` | `discard_async: true` | `src/distro/improvements.rs` |
| Snapshot on pacman | Pre-upgrade snapshot hook | `src/sigpkg/spec.rs` |

### Default Mount Options Generated

```
compress=zstd:3,noatime,space_cache=v2,discard=async
```

### Snapper Timeline Policy (Default)

| Period | Keep |
|--------|------|
| Hourly | 5 |
| Daily | 7 |
| Weekly | 0 |
| Monthly | 0 |
| Yearly | 0 |

---

## Debian

**Core inspiration:** Stability, three-tier release model, social contract, policy compliance.

### What SigmaOS Borrows

| Debian Feature | SigmaOS Implementation | File |
|--------------|----------------------|------|
| Deb package format | `DebPackageDriverTranslator` | `src/package/linux_translation.rs` |
| Three-tier releases | `ThreeTierReleaseModel` | `src/distro/specialized.rs` |
| Policy enforcer | `DebianPolicyEnforcer` | `src/distro/specialized.rs` |
| Freeze stabilization | `FreezeBasedStabilization` | `src/distro/specialized.rs` |
| Multi-arch support | `DpkgMultiArch` | `src/distro/specialized.rs` |
| Social contract | `DebianSocialContract` | `src/distro/specialized.rs` |

---

## Manjaro

**Core inspiration:** User-friendly Arch, hardware detection, kernel manager.

### What SigmaOS Borrows

| Manjaro Feature | SigmaOS Implementation | File |
|---------------|----------------------|------|
| Hardware detection | `ManjaroHardwareDetection` | `src/distro/manjaro.rs` |
| Kernel switcher | `ManjaroKernelSwitcher` | `src/distro/manjaro.rs` |
| MHWD driver config | `MhwdDriverConfig` | `src/distro/manjaro.rs` |
| Pamac GUI | `PamacPackageManager` | `src/distro/manjaro.rs` |
| Mirror ranking | `PacmanMirror` | `src/distro/manjaro.rs` |

---

## Tiny Core Linux

**Core inspiration:** Ultra-minimal RAM-based OS, extension bundles.

### What SigmaOS Borrows

| TinyCore Feature | SigmaOS Implementation | File |
|----------------|----------------------|------|
| RAM-only mode | `TinyCoreRAMEngine` | `src/distro/tiny_core.rs` |
| Modes (Core/TinyCore/CorePlus) | `TinyCoreMode` | `src/distro/tiny_core.rs` |
| TCZ extension bundles | `TczExtensionManager` | `src/distro/tiny_core.rs` |
| Apps audit | `AppsAuditTool` | `src/distro/tiny_core.rs` |

---

## BSD Inspirations

| BSD Feature | Source | SigmaOS Implementation |
|-------------|--------|----------------------|
| Jail isolation | FreeBSD | `src/container/runtime.rs`, `sigma_jail_cli.cpp` |
| Pledge/Unveil | OpenBSD | Seccomp + Sigma MAC |
| ZFS snapshots | FreeBSD | Adapted to Btrfs in `SystemSnapshotManager` |
| Ports tree | FreeBSD | `UserPackageRepository` |
| Capsicum capabilities | FreeBSD | `CapabilitySet` extensions |

---

## Feature Comparison Matrix

| Feature | Arch | Fedora | NixOS | Tails | Kali | Alpine | CachyOS | Garuda | SigmaOS |
|---------|------|--------|-------|-------|------|--------|---------|--------|---------|
| Rolling Release | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Btrfs Default | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| System Snapshots | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Declarative Config | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Atomic Upgrades | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Ephemeral Sessions | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Tor Integration | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ |
| Pentest Tools | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ |
| musl libc | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ |
| BORE Scheduler | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ |
| CPU-Optimized Build | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Capability System | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ |
| zstd Compression | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Rust Kernel | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

# Linux Mint & Omarchy Ecosystem Supremacy

## Overview

SigmaOS integrates high-performance, memory-safe Rust implementations inspired by the best features of **Linux Mint** (https://github.com/linuxmint) and **Omarchy** (https://github.com/omacom/omarchy):
- **Linux Mint Ecosystem**:
  - `Warpinator`: Zero-configuration, high-throughput peer-to-peer local network transfer with PIN verification, ChaCha20-Poly1305 encryption, and Blake3 integrity hashing.
  - `Timeshift`: Automated, atomic Btrfs/ZFS snapshots with bootloader entry generation and instant failover rollback.
  - `MintUpdate`: Automated kernel regression watchdog with HWE channel selection and boot-failure auto-reversion.
  - `Cinnamon / Muffin`: Tear-free Wayland compositor pipeline with fractional scaling and adaptive sync.
- **Omarchy Architecture**:
  - `Omarchy Omakase Provisioner`: 60-second reproducible workstation bootstrap engine.
  - `Universal Theme Injector`: Dynamic palette propagation (Catppuccin Mocha, TokyoNight Storm, Gruvbox Dark, Nord Frost, RosePine Moon, Kanagawa Wave) across terminals (Ghostty, Alacritty), editors, compositors (Zenith / Hyprland), and status bars (Waybar).
  - `Web2App Sandbox`: Secure PWA/Web-app sandboxing with Landlock and Capsicum rights enforcement.
  - `Immutable Root with Atomic OverlayFS`: Fail-safe system updates with 30-second post-boot health checking.

## Architecture & Implementation

Source location: [`src/distro/sovereign_mint_omarchy_supremacy_suite.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_supremacy_suite.rs)

### 1. Sovereign Warpinator Engine
Provides encrypted local peer-to-peer file transfer without centralized servers:
- **Discovery**: Zero-configuration discovery across local subnets using mDNS / DNS-SD simulation.
- **Mutual Authentication**: Numeric PIN handshake to prevent unauthorized transfer requests.
- **Data Integrity**: Cryptographic Blake3 checksum verification per file transfer session.
- **Throughput**: Chunked streaming pipeline designed for 10GbE and Wi-Fi 7 saturation.

```rust
let mut warpinator = SovereignWarpinatorEngine::new("sigmaos-node", 42000);
warpinator.register_peer("peer-laptop", "mint-laptop", [192, 168, 1, 55], 42000, 1337);
warpinator.authenticate_peer("peer-laptop", 1337)?;
let session_id = warpinator.start_send("peer-laptop", "kernel.tar.zst", 1048576, checksum)?;
warpinator.stream_chunk(session_id, 65536)?;
```

### 2. Sovereign Timeshift Engine
Automates filesystem snapshotting for mission-critical reliability:
- **Storage Backends**: Native Btrfs subvolumes (`/.snapshots/@`), ZFS datasets (`rpool/ROOT/sigmaos@`), and hardlink rsync pools.
- **Atomic Rollback**: Restores previous system states in < 1 second.
- **Bootloader Integration**: Automatically synthesizes systemd-boot and GRUB menu options for instant booting directly into past snapshots.

```rust
let mut timeshift = SovereignTimeshiftEngine::new(SnapshotBackend::BtrfsSubvolume);
let snap_id = timeshift.create_pre_upgrade_snapshot("v0.1.0-stable", "Pre-kernel update");
let entries = timeshift.generate_bootloader_entries();
timeshift.rollback_to_snapshot(snap_id)?;
```

### 3. Sovereign Omarchy Provisioner & Universal Theming
Implements Omarchy's declarative workstation setup:
- **Workstation Bootstrap**: Automates provisioning of developer tooling (`neovim`, `ghostty`, `tmux`, `zsh`, `starship`, `git`, `ripgrep`, `fzf`, `bat`, `eza`, `btop`, `zenith-compositor`, `waybar`, `swaylock`) in under 60 seconds.
- **Synchronized Theming**: Generates real-time configuration files for Waybar, Ghostty, Alacritty, and Hyprland/Zenith with exact HEX color palettes.

Supported Presets:
- `CatppuccinMocha`
- `TokyoNightStorm`
- `GruvboxDark`
- `NordFrost`
- `RosePineMoon`
- `KanagawaWave`

```rust
let mut prov = SovereignOmarchyProvisioner::new(OmakaseThemePreset::CatppuccinMocha);
let configs = prov.apply_theme(OmakaseThemePreset::TokyoNightStorm);
let package_count = prov.bootstrap_workstation();
```

### 4. Sovereign Web2App Sandboxing
Isolates web-based client applications without heavy Electron runtimes:
- **Sandboxing Mechanisms**: Enforces Linux Landlock v4 and FreeBSD/OpenBSD Capsicum/Pledge rights.
- **Isolated User Profiles**: Stores state in dedicated `/var/lib/web2app/profiles/{app_id}` sandboxes.

```rust
let mut manager = SovereignWeb2AppManager::new();
manager.register_app(
    "signal-web",
    "Signal Messenger",
    "https://signal.org",
    SandboxIsolationLevel::StrictLandlock,
);
manager.launch("signal-web")?;
```

## AI Agent Maintenance Instructions
- Language constraints: Strictly Rust (`#![no_std]` capable with `alloc`), Zig, or Nim only. No C/C++ or Python dependencies.
- Maintain atomic snapshot integrity and zero-copy semantics across all storage backends.
- Ensure peer discovery and file streaming paths remain lock-free and memory-safe.
- Verify that `cargo check --lib` passes with 0 errors before committing updates.
- Keep unit test coverage intact for all peer handshakes, rollbacks, and sandbox launchers.

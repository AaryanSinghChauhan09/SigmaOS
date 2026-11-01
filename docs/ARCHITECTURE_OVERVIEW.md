# SigmaOS Architecture Overview

## Five-Layer Model

```
┌─────────────────────────────────────────────────────────────┐
│                     Applications Layer                       │
│  browser · office · media · terminal · settings · security  │
├─────────────────────────────────────────────────────────────┤
│                    Desktop Shell Layer                       │
│  panel · launcher · compositor · theming · notifications    │
├─────────────────────────────────────────────────────────────┤
│                   System Services Layer                      │
│  package mgr · update · network · audio · power · printing  │
├─────────────────────────────────────────────────────────────┤
│                      Kernel Layer                            │
│  scheduler · VFS · memory · drivers · IPC · syscall gate    │
├─────────────────────────────────────────────────────────────┤
│                   Firmware / Boot Layer                      │
│       UEFI → GRUB → initramfs → init → system services      │
└─────────────────────────────────────────────────────────────┘
```

## Layer 1: Firmware and Boot

**Purpose**: Get the kernel loaded and hand control to init.

| Component | Repo | Description |
|---|---|---|
| Bootloader | SigmaOS-bootloader | GRUB2 config + Secure Boot shim |
| initramfs | SigmaOS-kernel | mkinitramfs with crypto + fsck |
| Init system | SigmaOS-system | systemd-compatible init |
| Live session | SigmaOS-live-session | ISO live boot environment |

Boot sequence:
1. UEFI loads GRUB2 (signed shim for Secure Boot)
2. GRUB loads kernel + initramfs from /boot
3. initramfs mounts root (with LUKS unlock if encrypted)
4. systemd takes over as PID 1
5. System services start in dependency order
6. Display manager launches desktop session

## Layer 2: Kernel

**Purpose**: Hardware abstraction, process management, memory, filesystems, and drivers.

| Component | Repo | Description |
|---|---|---|
| Kernel | SigmaOS-kernel | Linux kernel with SigmaOS config patches |
| Drivers | SigmaOS-drivers | Out-of-tree drivers + firmware |
| GPU | SigmaOS-mesa3D | Mesa 3D + Vulkan (Intel/AMD) |
| NVIDIA | SigmaOS-NVIDIA | NVIDIA prime + proprietary driver wrapper |

Key kernel config choices:
- `CONFIG_SECURITY_APPARMOR=y` — AppArmor LSM enabled
- `CONFIG_LOCKDOWN_LSM=y` — Kernel lockdown support
- `CONFIG_RANDOMIZE_BASE=y` — KASLR enabled
- `CONFIG_STACKPROTECTOR_STRONG=y` — Stack canaries
- `CONFIG_FORTIFY_SOURCE=y` — Buffer overflow detection
- BPF JIT hardening enabled
- `/proc/kcore` and `/proc/kallsyms` restricted

## Layer 3: System Services

**Purpose**: Core OS services that the desktop and applications depend on.

| Component | Repo | Description |
|---|---|---|
| Package manager | SigmaOS-packages | apt-compatible + signed metadata |
| Update manager | SigmaOS-update | Atomic updates + pre-update snapshots |
| Network | SigmaOS-network-discovery | NetworkManager integration |
| Audio | SigmaOS-audio-tuner | PipeWire + WirePlumber |
| Power | SigmaOS-power-manager | TLP + UPower + systemd-logind |
| Printing | SigmaOS-simple-scan | CUPS + IPP Everywhere |
| Timeshift | SigmaOS-timeshift | BTRFS/RSYNC snapshots |
| Recovery | SigmaOS-we-can-fix-everything | Rescue mode + repair tools |

### IPC Model

System services communicate via:
- **D-Bus** (system bus): package manager, network manager, power manager, audio
- **Unix domain sockets**: low-latency kernel services, logging
- **Netlink**: kernel-to-userspace events (udev, network changes)
- **Shared memory**: high-throughput audio/video pipelines

## Layer 4: Desktop Shell

**Purpose**: User-visible session, window management, and compositor.

| Component | Repo | Description |
|---|---|---|
| Desktop env | SigmaOS-desktop-environment | Cinnamon-inspired Wayland shell |
| Window mgr | SigmaOS-window-management-library | Muffin/Hyprland compositor |
| Panel | SigmaOS-panel | Taskbar + system tray |
| Greeter | SigmaOS-greeter | Slick Greeter (LightDM) |
| Screensaver | SigmaOS-screen-saver | Lock screen |
| Theming | SigmaOS-theme-marketplace | GTK + icon + cursor themes |
| Settings | SigmaOS-control-center | System settings hub |
| Notifications | SigmaOS-notification | D-Bus notification daemon |

### Session Startup Sequence

```
LightDM → slick-greeter → user selects session
  → cinnamon-session (or Hyprland session)
    → panel + compositor start
    → system tray applets (network, audio, BT, power)
    → first-run wizard (first login only)
    → desktop ready
```

## Layer 5: Applications

**Purpose**: End-user apps shipped as part of the default bundle.

24 default V1 apps. See `docs/V1_SCOPE.md` for the full list.
All apps use XDG standards: desktop entries, MIME types, D-Bus activation.

## Package Model

- Format: `.deb` (apt-compatible)
- Signing: Ed25519 signatures on package metadata + individual packages
- Repos: main, security, updates, contrib (all signed)
- Updates: atomic — pre-update snapshot → apply → verify → commit or rollback
- Channels: stable (default), testing, security (auto-applied for CVEs)

## Security Architecture

```
App (sandboxed) ──AppArmor profile──► allowed syscalls only
      │
      ▼
D-Bus policy (per-app permissions)
      │
      ▼
System service (drops privileges after init)
      │
      ▼
Kernel (KASLR + lockdown + seccomp)
```

See `docs/wiki/Security-Policy.md` for full threat model.

## Update Model

```
1. sigma-update checks repos
2. Downloads packages + verifies signatures
3. Creates pre-update BTRFS snapshot
4. Applies packages atomically
5. Runs post-update health checks
6. On success: marks snapshot as "pre-update-YYYYMMDD"
7. On failure: auto-rollback to pre-update snapshot
8. User can manually rollback via: sigma rescue --rollback
```

## Component Repository Index

All component repos live under `https://github.com/AaryanSinghChauhan09/`.
The main repo (`SigmaOS`) coordinates all components via `COMPONENTS.toml`.
Run `make registry-audit` to check component sync status.

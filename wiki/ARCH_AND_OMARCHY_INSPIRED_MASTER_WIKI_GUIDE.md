# Arch & Omarchy Inspired Master Wiki Guide for SigmaOS

This master wiki guide combines the **technical rigor and completeness of the Arch Linux Wiki** with the **opinionated user experience and product polish of Omarchy Linux**.

---

## 1. Installation & Bootstrapping Guide

### 1.1 Booting the Live ISO
1. Download the official `sigmaos-x86_64-latest.iso`.
2. Boot under UEFI on hardware or QEMU:
   ```bash
   qemu-system-x86_64 -enable-kvm -m 4G -bios /usr/share/ovmf/OVMF.fd -cdrom sigmaos-x86_64-latest.iso
   ```
3. At the boot menu, select **SigmaOS Live Workstation**.

### 1.2 Guided Installation
Execute the automated 60-second installer:
```bash
sigma-install --target-disk /dev/nvme0n1 --hostname sovereign-box --username sovereign
```

The installer automatically:
- Formats `/dev/nvme0n1` with Btrfs subvolumes (`@`, `@home`, `@snapshots`).
- Installs the base system, systemd-boot, and Zenith desktop compositor.
- Configures user accounts and sets up Sudo Expiry Guard.

---

## 2. Zenith Desktop Shell & Navigation

Zenith is a unified, long-running Wayland compositor and desktop shell inspired by Hyprland and Omarchy Omakase.

### Core Shell Components
- **Top Bar**: Live CPU/RAM monitors, network tray, volume applet, and clock.
- **Application Launcher**: Walker fuzzy search launcher accessible via `SUPER + Space`.
- **Workspace Grid**: 10 dynamic workspaces with dwindle auto-tiling.
- **Notifications**: Mako notification daemon queue with priority filtering.

---

## 3. Interactive Keyboard Shortcuts & HUD

Press `SUPER + K` at any time to open the interactive keybinding HUD.

| Shortcut | Action | Description |
|---|---|---|
| `SUPER + Return` | Launch Terminal | Opens Alacritty / Kitty terminal |
| `SUPER + B` | Launch Browser | Opens Wayland-native Chromium |
| `SUPER + Space` | Application Launcher | Opens Walker fuzzy launcher |
| `SUPER + SHIFT + Space` | Cycle Theme | Switches next curated desktop theme |
| `SUPER + K` | Keybindings HUD | Shows keyboard shortcuts guide |
| `SUPER + 1..9` | Switch Workspace | Focuses workspace 1 through 9 |
| `SUPER + SHIFT + Q` | Quit Session | Log out of Zenith desktop |

---

## 4. Theme Studio & Visual Customization

SigmaOS includes built-in instant theme switching without requiring reboots.

### Curated Themes
1. **Tokyo Night**: `#1a1b26` background, `#7aa2f7` blue accent.
2. **Catppuccin Mocha**: `#1e1e2e` background, `#cba6f7` mauve accent.
3. **Gruvbox Dark**: `#282828` background, `#fe8019` orange accent.
4. **Nord**: `#2e3440` background, `#88c0d0` frost accent.
5. **Everforest**: `#2d353b` background, `#a7c080` green accent.
6. **Kanagawa**: `#1f1f28` background, `#7e9cd8` wave blue accent.

Switch themes via CLI:
```bash
sigomarchy theme set catppuccin
```

---

## 5. Package Management (`sigpkg` & Universal Adapters)

`sigpkg` is the native package manager for SigmaOS, utilizing Merkle Content-Addressed Storage (CAS) trees and cryptographic Ed25519/PQC signatures.

### Basic Commands
```bash
sigma-pkg search <query>       # Search repository packages
sigma-pkg install <package>    # Install package and dependencies
sigma-pkg remove <package>     # Remove package safely
sigma-pkg update               # Refresh repository indexes
sigma-pkg rollback             # Rollback package state
```

---

## 6. Atomic Updates & Automatic Rollback

SigmaOS uses A/B generation switching to guarantee system stability.

- Updates take effect on reboot via generation switching.
- Post-boot health validation verifies kernel startup, filesystem mounts, and Zenith desktop readiness.
- If health checks fail, the bootloader automatically rolls back to the previous known good generation.

Command surface:
```bash
sigma-system generations      # List available system generations
sigma-system rollback         # Manually rollback to previous generation
sigma-system pin <gen>        # Pin generation to prevent GC cleanup
```

---

## 7. Security Sandboxing & Permission Management

SigmaOS provides cross-platform security sandboxing translating declarative permissions into platform-native controls:
- **Linux**: Landlock v5 path and network TCP port isolation.
- **OpenBSD**: Pledge syscall restrictions and Unveil filesystem path masking.
- **FreeBSD**: Capsicum Capability Rights and VNET Jails.

---

## 8. Freestanding (`no_std`) vs Desktop (`std`) Boundary

SigmaOS enforces a strict architectural boundary:
- **Kernel, Boot, & Core Allocators**: Strict `#![no_std]` Rust static compilation for zero external dependencies and maximum security.
- **Zenith Desktop Shell & Userland Tooling**: Practical `std` runtime for rich Wayland graphics, audio, and user interfaces.

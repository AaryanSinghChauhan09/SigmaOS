# SigmaOS Hybrid Distro Roadmap & Best Practices Guide

This document outlines the hybrid Linux & BSD architecture and feature roadmap for SigmaOS.

## 1. Base Philosophy

- **Hybrid Model**: Combines Linux kernel hardware compatibility & package ecosystem with BSD simplicity, clean defaults, security discipline, and clean service/config separation.
- **Stability & Pace**: Debian-like base stability, Arch-like rolling updates for user applications, and Nix-style declarative/reproducible state management.

## 2. Core System Layers

- **Universal Package System (`SIGPKG`)**:
  - Signed package verification (Dilithium-5 PQC / Ed25519 / GPG).
  - Multi-channel repositories: `stable`, `testing`, `unstable`/`rolling`.
  - Canonical dependency resolution & atomic transaction rollback.
- **Init & Service Model**:
  - `SigmaInit` / `systemd` / OpenRC target unit state control.
  - Service health monitoring, automatic restart policies, and audit logs.
- **Boot, Storage & Filesystem**:
  - Standard FHS layout (`/etc`, `/var`, `/srv`, `/home`, `/usr`).
  - OpenZFS (`bectl`) & Btrfs (`Snapper`) transactional CoW snapshot boot environments.
  - Offline recovery tools & fallback boot entries.

## 3. Integrated Security Architecture

- **Default State**:
  - Firewall (`pf` / `nftables`) active by default with zero open incoming ports (except LocalSend port 53317).
  - OpenBSD `pledge` & `unveil` sandboxing alongside Linux AppArmor / Landlock v5 LSM rules.
  - Rate-limited SSHD daemon (`omarchy` / `SovereignSshDaemon`) and read-only system partitions.

## 4. Wayland-First Modern Desktop Stack

- **Zenith Wayland Compositor**:
  - Wayland 1.24+ Zenith HDR tone mapping & direct KMS scanout.
  - Consistent theming across GTK, Qt, Ghostty, Alacritty, and Neovim.
  - Integrated settings, lock screen, session controls, and workspace management.

## 5. Ecosystem & Development Workflows

- **Flatpak & AppImage Integration**: Native execution of desktop applications outside base repository.
- **Developer Tooling**: Preinstalled toolchains for Rust, Go, Python, Node, Java, and C/C++.
- **Tmux IDE Layouts**: Pre-configured functions (`tdl`, `tds`, `tdlm`, `tsl`) for instant multi-agent IDE sessions.

## 6. System Administration & Ergonomics

- **Dual GUI / CLI Parity**: Complete CLI equivalence for all GUI settings (`omarchy` spaced CLI router).
- **Network & Tailscale Mesh**: Wi-Fi band pinning (2.4GHz / 5GHz / 6GHz), QR code sharing, DNS profiles, and Taildrop file transfers to `~/Downloads`.

## 7. Implementation Phases

- **Phase 1: Foundation**: Package manager, repo signing, init system, boot environments, recovery ISO.
- **Phase 2: Desktop**: Wayland compositor, app launcher, settings, terminal suites, gestures.
- **Phase 3: Security**: Default firewall, Landlock/Pledge sandboxing, PQC update verification.
- **Phase 4: Ecosystem**: Developer tools, Flatpak/AppImage layers, Tailscale mesh networking.
- **Phase 5: Polish & Release**: ISO releases, stable/testing tracks, performance tuning, and documentation.

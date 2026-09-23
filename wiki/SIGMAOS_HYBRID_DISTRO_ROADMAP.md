# 🎯 SigmaOS Hybrid Linux & BSD Architecture Roadmap

This document outlines the strategic hybrid Linux & BSD roadmap for **SigmaOS**, combining the hardware compatibility and modern package ecosystem of Linux with the simplicity, clean defaults, security discipline, and service predictability of BSD.

---

## 🏛️ Target Hybrid Formula

- **Kernel**: Linux LTS Kernel for broad hardware, GPU/KMS, and eBPF/io_uring support.
- **Service Model**: `systemd` & `rc.conf`-style declarative configuration semantics.
- **Package Ecosystem**: Curated `sigpkg` repos + Flatpak + AppImage compatibility.
- **Security Architecture**: AppArmor/SELinux + OpenBSD `pledge()`/`unveil()` sandboxing + FreeBSD Capsicum capability rights.
- **Resilient Filesystem**: Btrfs and ZFS Copy-on-Write snapshots with sub-millisecond rollback capabilities.
- **Desktop Environment**: Wayland-first Zenith / Omarchy desktop with Quickshell plugins and keyboard tiling.
- **Administration**: GUI Control Center + 1:1 CLI command parity.
- **Update Model**: Signed atomic updates with pre-flight Btrfs/ZFS snapshots.

---

## 🧭 11 Core Roadmap Pillars

### 1. Base Philosophy
- Stable by default, modern when needed.
- Minimal core OS surface with zero unverified dependencies.
- Declarative configuration (`shell.json`, `bindings.lua`, `login.conf`).
- Clear separation between base OS (`/usr/share/omarchy/`), app packages, and user configs (`~/.config/`).

### 2. Core System Layers
- **Package Management**: Signed `sigpkg` manifests, repository metadata verification, dependency graph resolution, rollback snapshots, and multi-channel support (`stable`, `testing`, `unstable`).
- **Init/Service Subsystem**: Clean service dependencies, health checks, automatic restart policies, and `/etc/rc.conf` parity.
- **Filesystem Hierarchy**: Standard FHS directory layout with Btrfs subvolume / ZFS dataset snapshots.

### 3. Security-as-a-Core Feature
- Default `pf`-style firewall rules and network isolation.
- Read-only system partition enforcement.
- Mandatory user permission model and secure file umask defaults (`0077`).
- Automated security updates and audit log tracing.

### 4. Modern Desktop Environment
- Wayland-first custom compositor (`Zenith`) with Hyprland tiling/dwindle/scrolling modes.
- XDG portal compliance and display topology management.
- Integrated OSD, lock screen, and notification center.

### 5. Package Ecosystem
- Official curated packages with post-quantum Kyber/Dilithium signature verification.
- Flatpak runtime container support.
- LTS and rolling-edge release tracks.

### 6. System Administration & Ergonomics
- Unified GUI Control Center and CLI parity (`omarchy <group> <action>`).
- Privileged action management via `sudo` or `pkexec`.

### 7. Resilient Storage & Backup
- Automated pre-flight update snapshots (`btrfs subvolume snapshot`).
- Disk SMART health monitoring and time-travel rollbacks.

### 8. Predictable Networking
- Zero-copy socket routing and VPN auto-join (Tailscale, WireGuard).
- Reverse proxy support for local dev containers.

### 9. Development Workflows
- Built-in polyglot dev stack (Rust, Go, Python, Node, C/C++).
- Language version managers (`mise` / `rtx`).
- Integrated container runtimes (Docker, Podman).

### 10. Release Discipline
- Predictable 3-track release cadence (`stable`, `testing`, `edge`).
- Versioned ISO installer and automated `cidata` cloud-init installations.

### 11. Product Identity
- Polished Calamares-inspired GUI installer.
- Guided first-run onboarding wizard.
- Privacy controls and system health dashboard.

---

## 🏁 5-Phase Implementation Roadmap

1. **Phase 1: Foundation**: Package manager, repo signing, installable base system, service management, filesystem rollbacks, and recovery environment.
2. **Phase 2: Desktop**: Compositor, app launcher, settings app, file manager, gestures, and accessibility.
3. **Phase 3: Security**: Firewall, sandboxing, package verification, update signing, and system hardening.
4. **Phase 4: Ecosystem**: App store expansion, dev tools, virtual machines, containers, and onboarding.
5. **Phase 5: Polish & Release**: Release channels, stable/testing/edge tracks, bug triage, support docs, and performance tuning.

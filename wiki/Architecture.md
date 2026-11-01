# SigmaOS Architecture

> This document describes the full system architecture of SigmaOS, including layer responsibilities, component interaction, ABI contracts, IPC model, and security boundaries.

---

## Table of Contents

1. [Design Philosophy](#design-philosophy)
2. [Layer Model](#layer-model)
3. [Kernel Layer](#kernel-layer)
4. [System Layer](#system-layer)
5. [Desktop Layer](#desktop-layer)
6. [App Layer](#app-layer)
7. [Component Interaction Diagram](#component-interaction-diagram)
8. [ABI Contracts](#abi-contracts)
9. [IPC Model](#ipc-model)
10. [Security Layers](#security-layers)
11. [Boot Sequence](#boot-sequence)
12. [AI Integration Points](#ai-integration-points)

---

## Design Philosophy

SigmaOS architecture follows a strict **layered design** where each layer:

- Depends only on layers below it, never above (no circular dependencies)
- Exposes a stable, versioned API to the layer above
- Can be replaced independently without breaking other layers
- Enforces least-privilege — a component gets only the capabilities it needs

The architecture is inspired by:
- **Linux Mint**: Proven desktop-layer composability on top of a stable base
- **Omarchy**: Philosophy-first design, opinionated defaults, user sovereignty
- **Plan 9**: Everything is a file / uniform resource access
- **seL4**: Formally verified separation between mechanism and policy

---

## Layer Model

```
╔═══════════════════════════════════════════════════════════════╗
║                        APP LAYER (Ring 3)                     ║
║  sigma-browser  sigma-files  sigma-terminal  sigma-ai         ║
║  sigma-settings  sigma-store  sigma-media  sigma-code-editor  ║
╠═══════════════════════════════════════════════════════════════╣
║                     DESKTOP LAYER (Ring 3)                    ║
║  sigma-shell  sigma-wm  sigma-panel  sigma-dm  sigma-theme    ║
║  sigma-notifications  sigma-search  sigma-session  sigma-polkit║
╠═══════════════════════════════════════════════════════════════╣
║                     SYSTEM LAYER (Ring 0/3)                   ║
║  sigma-init  sigma-pkg  sigma-update  sigma-net  sigma-audio  ║
║  sigma-input  sigma-display  sigma-power  sigma-udev           ║
╠═══════════════════════════════════════════════════════════════╣
║                     KERNEL LAYER (Ring 0)                     ║
║  sigma-kernel  sigma-mm  sigma-sched  sigma-ipc  sigma-sec    ║
║  sigma-drivers  sigma-fs  sigma-net-core  sigma-crypto        ║
╠═══════════════════════════════════════════════════════════════╣
║                   FIRMWARE / BOOT LAYER                       ║
║  sigma-bootloader  sigma-uefi  sigma-initrd  sigma-acpi       ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## Kernel Layer

The kernel layer is the foundation. SigmaOS uses the **Linux kernel** (6.x LTS) as its base, extended with:

- **Rust kernel modules** for new drivers and security-critical subsystems
- A curated Kconfig that strips unnecessary features (minimal kernel philosophy)
- Custom scheduler tweaks for desktop responsiveness (EEVDF-based)
- Mandatory Access Control via Landlock + seccomp + namespaces

### Kernel Components

| Component | Description | Language |
|-----------|-------------|----------|
| `sigma-kernel` | Kernel config, patches, and build system | C, Rust |
| `sigma-mm` | Memory management policies and overrides | C, Rust |
| `sigma-sched` | Scheduler configuration and RT tuning | C |
| `sigma-ipc-core` | Core IPC primitives (pipes, sockets, shared mem) | C |
| `sigma-sec` | LSM hooks, seccomp profiles, Landlock policies | C, Rust |
| `sigma-drivers` | In-tree driver selection and OOT Rust drivers | C, Rust |
| `sigma-fs` | Filesystem layer (btrfs primary, ext4 secondary) | C |
| `sigma-net-core` | Network stack config and eBPF programs | C |
| `sigma-crypto` | Crypto API selection, kernel keyring | C |

### Kernel Design Principles

- **Minimal Kernel Philosophy**: Only what is needed is compiled in
- **Memory Safety**: All new subsystems written in Rust where kernel Rust API allows
- **Modular Kernel Design**: Modules are loadable/unloadable at runtime
- **Separation of Mechanism and Policy**: Kernel provides mechanisms; userspace decides policy

---

## System Layer

The system layer runs in userspace but provides OS-level services. It bridges kernel primitives and the desktop.

### System Components

| Component | Description | Language |
|-----------|-------------|----------|
| `sigma-init` | PID 1 init system (systemd-based with custom units) | Rust |
| `sigma-pkg` | Package manager (APT/pacman backend with Rust frontend) | Rust |
| `sigma-update` | Secure, atomic, signed update daemon | Rust |
| `sigma-net` | NetworkManager integration + DNS-over-HTTPS | Rust |
| `sigma-audio` | PipeWire session manager | Rust |
| `sigma-input` | libinput config daemon | Rust |
| `sigma-display` | Display server abstraction (Wayland/X11) | Rust |
| `sigma-power` | Power management, suspend/resume hooks | Rust |
| `sigma-udev` | udev rules and device event routing | Shell, Rust |
| `sigma-dbus` | D-Bus policy and session management | Rust |
| `sigma-keyring` | Secret service / credential store | Rust |
| `sigma-journal` | Structured logging aggregator | Rust |

### System Layer Contracts

The system layer exposes services via:

1. **D-Bus** (session and system bus) — for desktop and app integration
2. **Unix domain sockets** — for low-latency daemon communication
3. **Environment variables** — for process configuration at startup
4. **`/etc/sigma/`** — machine-level configuration directory

---

## Desktop Layer

The desktop layer is responsible for the user-visible graphical environment. SigmaOS targets **Wayland-first** with X11 compatibility via XWayland.

### Desktop Components

| Component | Description | Language |
|-----------|-------------|----------|
| `sigma-shell` | Wayland compositor shell (wlroots-based) | Rust |
| `sigma-wm` | Window management policies and tiling logic | Rust |
| `sigma-panel` | Top/bottom panel with applets | Rust |
| `sigma-dm` | Display manager / greeter (greetd-based) | Rust |
| `sigma-theme` | GTK4 + Qt6 theme, icon pack, cursor | Rust, CSS |
| `sigma-notifications` | Notification daemon (freedesktop spec) | Rust |
| `sigma-search` | Global search / app launcher | Rust |
| `sigma-session` | Session manager, autostart, XDG compliance | Rust |
| `sigma-polkit` | Privilege escalation agent | Rust |
| `sigma-portal` | XDG desktop portals (sandboxed app access) | Rust |
| `sigma-clipboard` | Clipboard manager with history | Rust |
| `sigma-screen` | Screenshot / screen recording | Rust |
| `sigma-wallpaper` | Wallpaper engine with dynamic/AI backgrounds | Rust |

### Desktop Layer Protocol

The desktop layer communicates via:
- **Wayland protocols** (wl_compositor, xdg_shell, wlr_* extensions)
- **D-Bus session bus** for applet and notification communication
- **XDG Base Directory Specification** for config/data/cache paths

---

## App Layer

Applications are first-class citizens in SigmaOS. They run sandboxed via:
- **Flatpak** for GUI apps (XDG portals as the permission boundary)
- **Bubblewrap** for system tools
- **Landlock** for filesystem access restriction

### Core App Components

| Component | Description | Language |
|-----------|-------------|----------|
| `sigma-browser` | Privacy-first web browser (Firefox-based) | Rust, C++ |
| `sigma-files` | File manager (Nautilus-inspired) | Rust |
| `sigma-terminal` | GPU-accelerated terminal emulator | Rust |
| `sigma-ai-assistant` | AI assistant with local and cloud backends | Rust, Python |
| `sigma-settings` | System settings UI | Rust |
| `sigma-store` | App store (Flatpak + native packages) | Rust |
| `sigma-media` | Media player | Rust |
| `sigma-text-editor` | Text editor | Rust |
| `sigma-code-editor` | Developer IDE | Rust |
| `sigma-image-viewer` | Image viewer | Rust |
| `sigma-archive-manager` | Archive tool | Rust |
| `sigma-calendar` | Calendar app | Rust |
| `sigma-contacts` | Contacts manager | Rust |

---

## Component Interaction Diagram

```
                    USER
                      │
          ┌───────────▼───────────┐
          │       APP LAYER        │
          │  sigma-browser         │
          │  sigma-files           │
          │  sigma-ai-assistant    │
          └──────────┬────────────┘
                     │ XDG Portals / D-Bus
          ┌──────────▼────────────┐
          │     DESKTOP LAYER      │
          │  sigma-shell (Wayland) │◄──── sigma-dm (greeter)
          │  sigma-wm              │
          │  sigma-panel           │◄──── sigma-notifications
          │  sigma-search          │
          └──────────┬────────────┘
                     │ D-Bus / Unix sockets
          ┌──────────▼────────────┐
          │     SYSTEM LAYER       │
          │  sigma-init (PID 1)   │◄──── sigma-journal
          │  sigma-pkg            │◄──── sigma-update
          │  sigma-net            │◄──── sigma-keyring
          │  sigma-audio          │◄──── sigma-power
          │  sigma-display        │
          └──────────┬────────────┘
                     │ syscalls / ioctl
          ┌──────────▼────────────┐
          │     KERNEL LAYER       │
          │  Linux 6.x + Rust      │
          │  sigma-mm              │
          │  sigma-sched           │
          │  sigma-sec (LSM)       │
          │  sigma-drivers         │
          └──────────┬────────────┘
                     │
          ┌──────────▼────────────┐
          │   FIRMWARE / BOOT      │
          │  sigma-bootloader      │
          │  sigma-uefi            │
          │  sigma-initrd          │
          └───────────────────────┘
```

---

## ABI Contracts

SigmaOS defines strict ABI contracts at each layer boundary to ensure independent upgradability.

### Kernel ↔ System Layer ABI

- **Syscall stability**: Only stable Linux syscalls used (no private kernel APIs)
- **Netlink contracts**: All kernel netlink interfaces versioned
- **ioctl versioning**: Custom ioctl commands carry version fields
- **Module ABI**: Kernel modules declare their minimum kernel version

### System ↔ Desktop Layer ABI

- **D-Bus interfaces**: All D-Bus interfaces carry a `Version` property; breaking changes increment major version
- **Unix socket protocols**: Length-prefixed msgpack frames; schema versioned
- **Environment contracts**: `WAYLAND_DISPLAY`, `XDG_*`, `SIGMA_*` variables are stable

### Desktop ↔ App Layer ABI

- **Wayland protocol stability**: Only stable/xdg protocol extensions used in production
- **XDG Portal API versioning**: Portal interfaces specify minimum portal version required
- **Flatpak metadata**: Manifests pin runtime versions for reproducibility

### ABI Breakage Policy

- **Major version bump** required for any breaking change at any layer boundary
- **6-month deprecation window** for removed APIs
- **`sigma-compat`** shim layer for temporary backward compatibility during transitions

---

## IPC Model

SigmaOS uses a **multi-mechanism IPC model** — the right tool for the right job:

| Mechanism | Use Case | Latency | Complexity |
|-----------|----------|---------|------------|
| **D-Bus** | Service discovery, desktop integration | ~100µs | Low |
| **Unix domain sockets** | Daemon-to-daemon, high-frequency | ~10µs | Medium |
| **Shared memory + futex** | Multimedia, zero-copy data transfer | ~1µs | High |
| **Netlink** | Kernel ↔ userspace events | ~50µs | Medium |
| **Pipes / FIFOs** | Shell pipelines, simple data streams | ~5µs | Low |
| **io_uring** | Async I/O for server-class components | <1µs | High |

### IPC Principles

- **No polling**: All IPC mechanisms are event-driven
- **Zero-copy paths**: Multimedia and display buffers use DMA-BUF / shared memory
- **Capability passing**: File descriptors are passed via SCM_RIGHTS for capability transfer
- **Rate limiting**: All D-Bus interfaces enforce rate limits to prevent DoS

---

## Security Layers

SigmaOS implements defense-in-depth with multiple independent security layers:

```
Layer 5: APPLICATION SANDBOX
  └─ Flatpak portals, Bubblewrap namespaces, Landlock filesystem rules

Layer 4: PRIVILEGE SEPARATION
  └─ No SUID binaries (polkit for privilege escalation)
  └─ Capabilities model (drop all, add-back only needed caps)
  └─ Separate system/session D-Bus with access policies

Layer 3: PROCESS ISOLATION
  └─ PID namespaces per sandboxed app
  └─ Network namespaces for untrusted processes
  └─ Mount namespaces with read-only system paths

Layer 2: KERNEL HARDENING
  └─ seccomp-bpf syscall filtering (allowlist per process class)
  └─ Landlock LSM for filesystem access control
  └─ KASLR, SMEP, SMAP, CET enabled
  └─ Kernel lockdown mode

Layer 1: BOOT INTEGRITY
  └─ UEFI Secure Boot with SigmaOS-signed shim
  └─ GRUB2 signed bootloader
  └─ initrd integrity verification (dm-verity)
  └─ TPM2-backed measured boot
```

### Threat Model Summary

| Threat | Mitigation |
|--------|-----------|
| Malicious app | Flatpak sandbox + portal gating |
| Kernel exploit | Seccomp + Landlock + KASLR |
| Supply chain attack | Signed packages + reproducible builds |
| Privilege escalation | No SUID + polkit + capability drops |
| Boot tampering | Secure Boot + dm-verity |
| Network attack | AppArmor network profiles + firewalld |

---

## Boot Sequence

```
1. UEFI firmware
   └─ Verifies shim signature
2. sigma-shim (signed)
   └─ Loads GRUB2 (signed)
3. GRUB2
   └─ Loads sigma-kernel + initrd
   └─ Passes kernel command line
4. sigma-initrd
   └─ Mounts root fs (btrfs, dm-verity verified)
   └─ Unlocks LUKS if encrypted
   └─ Pivots to real root
5. sigma-init (PID 1 / systemd)
   └─ Starts system services in dependency order
   └─ Mounts /proc, /sys, /dev, /run
6. sigma-dm (greetd)
   └─ Presents login greeter
7. User session
   └─ sigma-shell (Wayland compositor)
   └─ sigma-panel, sigma-notifications, autostart apps
```

---

## AI Integration Points

SigmaOS treats AI as a first-class citizen, integrated at multiple layers:

| Layer | Integration | Description |
|-------|-------------|-------------|
| App | `sigma-ai-assistant` | Full AI assistant with local (Ollama) and cloud backends |
| Desktop | `sigma-search` | AI-powered search with natural language queries |
| Desktop | `sigma-shell` | AI-suggested window layouts and workspace management |
| System | `sigma-pkg` | AI-assisted package recommendations and conflict resolution |
| System | `sigma-journal` | AI log analysis for anomaly detection |
| Kernel | `sigma-sched` | ML-informed scheduling hints (future) |

All AI features are **opt-in** and **local-first**. Cloud AI backends require explicit user consent and use end-to-end encrypted connections.

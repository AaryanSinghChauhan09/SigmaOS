# SigmaOS Architecture Overview

> Canonical architecture reference for the SigmaOS multi-kernel distro platform.
> This document defines the layered model, backend adapter pattern, and canonical
> repo families that govern the entire ecosystem.

---

## Design Principles

SigmaOS is built on **one common architecture** that supports **multiple kernel backends**
(Linux and BSD). The architecture enforces:

- **Shared interfaces** — every subsystem defines a trait-based contract
- **Kernel-specific adapters** — backends implement the shared traits
- **Low coupling** — subsystems depend on traits, never on concrete backends
- **Stable public contracts** — breaking changes require an ADR + deprecation cycle
- **One config model** — `sigma-config` shared across all backends
- **One package manager abstraction** — backend adapters for pacman/apt/dnf/pkg
- **One desktop model** — the SigmaOS desktop shell is kernel-agnostic

---

## The 8-Layer Model

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 8 — App and Support                                  │
│  browser · office · file-manager · terminal · media · notes │
├─────────────────────────────────────────────────────────────┤
│  Layer 7 — Security and Trust                                │
│  security · auth · policy · keyring · signing · audit · log │
├─────────────────────────────────────────────────────────────┤
│  Layer 6 — Config and Theming                                │
│  config · defaults · theme · wallpaper · fonts · icons       │
├─────────────────────────────────────────────────────────────┤
│  Layer 5 — Desktop Shell                                     │
│  desktop · panel · launcher · menu · WM · compositor · notif │
├─────────────────────────────────────────────────────────────┤
│  Layer 4 — Package Layer                                      │
│  package-manager · repo-manager · mirror · update · upgrade   │
├─────────────────────────────────────────────────────────────┤
│  Layer 3 — Base System                                        │
│  system · service-manager · session-manager · display-manager │
├─────────────────────────────────────────────────────────────┤
│  Layer 2 — Boot and Init                                      │
│  boot · boot-manager · bootloader · initramfs · init          │
├─────────────────────────────────────────────────────────────┤
│  Layer 1 — Kernel Abstraction                                │
│  ┌──────────────────┐  ┌──────────────────┐                  │
│  │ Linux Backend     │  │ BSD Backend      │                  │
│  │ (systemd, NM,     │  │ (rc.d, ifconfig, │                  │
│  │  pacman, SELinux) │  │  pkg, pledge)    │                  │
│  └──────────────────┘  └──────────────────┘                  │
├─────────────────────────────────────────────────────────────┤
│  Layer 0 — Hardware                                          │
│  drivers · gpu · audio · network · bluetooth · power · monitor│
└─────────────────────────────────────────────────────────────┘
```

---

## Backend Adapter Pattern

The core architectural decision: **shared subsystems define traits; backends implement them.**

```rust
trait KernelBackend {
    fn boot_log() -> BootLog;
    fn services() -> ServiceManager;
    fn packages() -> PackageAdapter;
    fn network() -> NetworkAdapter;
    fn security() -> SecurityAdapter;
    fn hardware() -> HardwareAdapter;
}
```

- **LinuxBackend** implements using systemd, NetworkManager, pacman/apt/dnf, SELinux/AppArmor, sysfs, udev, modprobe.
- **BsdBackend** implements using rc.d, ifconfig/wpa_supplicant, pkg/pkg_add, pledge/unveil/Capsicum, devinfo, devd, kldload.

Selected at boot via `SIGMA_KERNEL_BACKEND=linux|bsd`. A backend that fails to initialize
falls back to a minimal mode (no-op adapters) rather than panicking. New backends can be
added by implementing the trait — no shared code changes.

See: [Kernel Backend Adapter Contract](platform-contracts/kernel-backend.md)

---

## Canonical Repo Families (30 active)

| Family | Canonical Repo | Merges |
|--------|---------------|--------|
| Core | SigmaOS-core | — |
| Kernel | SigmaOS-kernel | — |
| Boot | SigmaOS-boot | boot + boot-manager + bootloader |
| Initramfs | SigmaOS-initramfs | — |
| Installer | SigmaOS-installer | installer + live-iso + first-boot + setup |
| System | SigmaOS-system | system + systemd + rc + service-manager + session-manager |
| Display | SigmaOS-display-manager | display-manager + greeter |
| Package | SigmaOS-package-manager | packages + packages-kit + packages-tools |
| Repo | SigmaOS-repo-manager | repo-manager + mirror |
| Update | SigmaOS-update | update + upgrade + upgrade-info |
| Security | SigmaOS-security | security + auth + policy + polkit |
| Config | SigmaOS-config | config + configurator + defaults |
| Desktop | SigmaOS-desktop | desktop + desktop-environment + session |
| Panel | SigmaOS-panel | panel + tray |
| Launcher | SigmaOS-launcher | launcher + menu |
| Control | SigmaOS-control-center | control-center + settings + daemon-settings |
| Theme | SigmaOS-theme | theme + theme-registry + theme-marketplace |
| File | SigmaOS-file-manager-system | file-manager + file-writer |
| Network | SigmaOS-network | network + network-manager |
| Drivers | SigmaOS-drivers | — |
| Hardware | SigmaOS-hardware | — |
| Docs | SigmaOS-docs | docs + guide-* family |
| Release | SigmaOS-release | release + state-of-components |
| Performance | SigmaOS-performance | + kernel-tuning + zram + scheduler + cpu-profile + io-scheduler + fs-tuning + swap + power-policy |
| GPU | SigmaOS-gpu | gpu + NVIDIA + amd + intel + vulkan |
| Gaming | SigmaOS-gaming-profile | gaming-profile + gaming-tools |
| Terminal | SigmaOS-terminal | terminal + term |
| Browser | SigmaOS-browser | browser + chromium |
| Office | SigmaOS-office | office + spreadsheet + presentation |
| Apps | SigmaOS-apps | app catalog + app-center |

All other repos are classified as: optional extension, compatibility adapter, plugin,
or experimental. See [REPO-TRIAGE.md](REPO-TRIAGE.md) for the full classification.

---

## Performance Layer (CachyOS-inspired)

SigmaOS absorbs CachyOS's performance-first identity through a dedicated tuning layer:

| Repo | Responsibility |
|------|---------------|
| SigmaOS-performance | Profile manager (balanced/performance/gaming/battery/workstation) |
| SigmaOS-kernel-tuning | Kernel parameters (swappiness, THP, sched latency) |
| SigmaOS-zram | Compressed memory device setup |
| SigmaOS-scheduler | Scheduler selection (EEVDF/BORE/CFS) |
| SigmaOS-cpu-profile | CPU governor + freq scaling |
| SigmaOS-io-scheduler | Block I/O scheduler (mq-deadline/kyber/bfq/none) |
| SigmaOS-fs-tuning | Mount options per filesystem type |
| SigmaOS-gaming-profile | Gaming mode (CPU perf + low-latency audio + GPU hints) |
| SigmaOS-gpu | GPU detection + PRIME switching (NVIDIA/AMD/Intel) |
| SigmaOS-vulkan | Vulkan driver + layer config |
| SigmaOS-swap | Swap/zswap policy |
| SigmaOS-power-policy | Performance vs power profile switcher |

---

## Platform Contracts

Shared, kernel-agnostic interface definitions live in `platform-contracts/`:

- [Installation Flow](platform-contracts/installation-flow.md)
- [Package Lifecycle](platform-contracts/package-lifecycle.md)
- [Service Management](platform-contracts/service-management.md)
- [Config System](platform-contracts/config-system.md)
- [Desktop Session](platform-contracts/desktop-session.md)
- [App Discovery](platform-contracts/app-discovery.md)
- [Security & Trust](platform-contracts/security-trust.md)
- [Update Flow](platform-contracts/update-flow.md)
- [Hardware Abstraction](platform-contracts/hardware-abstraction.md)
- [Kernel Backend Adapter](platform-contracts/kernel-backend.md)

---

## Build Order

The correct sequence to build SigmaOS into a real distro:

1. Repo cleanup (triage + merge + archive) ✅
2. Shared architecture (platform contracts) ✅
3. Kernel backend abstraction ✅
4. Installer + boot stack
5. Package manager + repo trust
6. Tuning profiles ✅
7. Desktop shell
8. Config / theme
9. Hardware support
10. Gaming / workstation defaults ✅
11. App ecosystem
12. Security + release validation

Items marked ✅ are scaffolded and compiling; runtime integration is Milestone 1 work.

---

## What This Is NOT

- Not "one repo per idea" — repos represent real subsystem boundaries
- Not "every distro at once" — Milestone 1 is Linux-first; BSD is Milestone 2
- Not a generic Linux remix — SigmaOS has a performance-first identity (CachyOS-inspired)
- Not judged by repo count — judged by working boot, install, packages, desktop, security, updates

The project is judged by: **working boot, working install, working package manager,
working desktop, working update and security model, working app ecosystem, clean
architecture, consistent user experience.**

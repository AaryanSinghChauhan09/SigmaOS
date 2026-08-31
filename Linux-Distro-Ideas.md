# 🐧 Linux & BSD Distro Ideas

SigmaOS draws inspiration from the best features of major Linux distributions and BSDs.

## Implementation Status

| Distribution | Feature Borrowed | Implementation | Status |
|-------------|-----------------|----------------|--------|
| **Arch Linux** | Rolling release model | sigma-pkg continuous updates | ✅ |
| **Arch Linux** | AUR (Arch User Repository) | AUR compatibility layer | 🚧 |
| **CachyOS** | BORE scheduler (burst-oriented) | BORE + EEVDF hybrid | ✅ |
| **CachyOS** | LTO/PGO optimized builds | Build system optimization | ✅ |
| **Fedora** | DNF package resolver | DNF/RPM bridge | 🚧 |
| **Fedora** | Bleeding edge tech adoption | Always-latest kernel | ✅ |
| **Debian** | APT package management | dpkg/APT bridge | 🚧 |
| **Debian** | Stable LTS releases | LTS release channels | 📋 |
| **Ubuntu** | AppArmor profiles | AppArmor integration | 🚧 |
| **Ubuntu** | Snap packages | Snap bridge | 📋 |
| **openSUSE** | Btrfs snapshotting | Auto Btrfs snapshots | ✅ |
| **openSUSE** | YaST-like config | sigma-control-center | 📋 |
| **NixOS** | Declarative config | Nix expression support | 🔬 |
| **NixOS** | Atomic upgrades | Content-addressed store | ✅ |
| **Gentoo** | Source-based compilation | Portage bridge | 🔬 |
| **Gentoo** | USE flags optimization | Feature flag system | 📋 |
| **Void Linux** | runit init system | runit bridge | ✅ |
| **Void Linux** | XBPS package manager | XBPS compatibility | 📋 |
| **Alpine Linux** | Security-first minimal base | Minimal kernel mode | ✅ |
| **Alpine Linux** | musl libc | musl compatibility | 📋 |
| **Pop!_OS** | Auto-tiling window manager | Tiling WM | ✅ |
| **Pop!_OS** | System76 firmware tools | Firmware manager | 📋 |
| **Garuda Linux** | Dr460nized theme | Dark/blur theme | ✅ |
| **Garuda Linux** | Gaming optimizations | Gaming mode | 📋 |
| **antiX/MX Linux** | Lightweight tools | Busybox-style tools | ✅ |
| **antiX** | systemd-free option | runit/OpenRC bridge | ✅ |
| **Zorin OS** | Windows-like layout | Layout switcher | ✅ |
| **Zorin OS** | Windows app support | Wine/Proton bridge | 🔬 |
| **EndeavourOS** | Calamares installer | Guided installer | 📋 |
| **Kali Linux** | Penetration testing tools | PenTest engine | 🔬 |
| **Parrot OS** | AnonSurf anonymization | AnonSurf module | 🔬 |
| **FreeBSD** | pf firewall | eBPF + pf rules | ✅ |
| **OpenBSD** | pledge/unveil syscall | pledge/unveil impl | ✅ |
| **OpenBSD** | Secure by default | Hardened defaults | ✅ |
| **DragonFly BSD** | HAMMER2 filesystem | HAMMER2 driver | 🔬 |

## Detailed Feature Implementations

### Arch Linux Parity
- **AUR client**: Resolve, download, build, and install AUR packages natively
- **Pacman compatibility**: pacman-style CLI wrapper around sigma-pkg
- **Rolling release**: Continuous package updates without version pinning

### CachyOS Parity
- **BORE scheduler**: Burst-Oriented Response Enhancer reduces latency for interactive workloads
- **EEVDF scheduler**: Earliest Eligible Virtual Deadline First for fairness
- **LTO builds**: Link-time optimization for kernel and userspace
- **zstd compression**: Fast compression for initramfs and packages

### NixOS Parity
- **Declarative system**: Define entire system state in Nix expressions
- **Atomic rollback**: Every update is atomic; rollback to any generation
- **Content-addressed store**: Packages stored by hash for reproducibility
- **Dependency isolation**: No DLL hell — packages carry their own dependencies

### OpenBSD Parity
- **pledge()**: Processes declare capability requirements upfront
- **unveil()**: Filesystem access restricted to declared paths
- **Secure levels**: Kernel hardening levels (securelevel 0-3)
- **W^X enforcement**: Writable XOR executable memory pages

### FreeBSD Parity
- **pf firewall**: Packet Filter with stateful inspection
- **Jails**: Lightweight OS-level virtualization (via namespaces)
- **ZFS**: Advanced storage with checksums and snapshots
- **bhyve-inspired VMM**: Type-2 hypervisor model

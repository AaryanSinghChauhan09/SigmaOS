# Changelog

All notable changes to SigmaOS are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### Added
- `ARCHITECTURE.md` — comprehensive system architecture documentation
- `ROADMAP.md` — phased development roadmap (v0.1→v1.0)
- `SECURITY.md` — security policy and vulnerability reporting process
- `INSTALL.md` — installation and build guide
- `docs/KERNEL.md` — kernel internals documentation
- `docs/PACKAGE_MANAGER.md` — sigpkg package manager guide
- `docs/DISTRO_COMPAT.md` — Linux/BSD compatibility documentation
- `src/distro/arch_inspirations.rs` — Arch Linux rolling release, AUR, PKGBUILD, pacman parity
- `src/distro/nixos_inspirations.rs` — NixOS declarative config, atomic upgrades, content-addressed store
- `src/distro/gentoo_inspirations.rs` — Gentoo USE flags, Portage resolver, ebuilds
- `src/klib/sigma_string_utils.rs` — zero-alloc byte-slice string utilities

### Merged
- `bolt/vec-string-bulk-copy-opt` — Vec `extend_from_slice` bulk copy + SigmaString trim optimisation
- `feat/universal-package-oop-extensions` — Fedora bugzilla2fedmsg bridge, universal OOP package extensions
- `fix/linux-bsd-distro-improvements` — FedoraPlanetAggregationEngine, FedoraTahrirEngine
- `fix/open-source-parity-and-tests` — Fedora Cockpit, PipeWire desktop, FreeIPA Kerberos auth
- `impl/wiki-md-features-sync` — Fedora 'The New Hotness' upstream version monitor
- `jules-1368290922701548926` — Fedora status.fpo infrastructure monitoring
- `jules-666776792259392766` — FedoraAnityaReleaseMonitoringEngine, CachyOS parity improvements
- `master-1423622165343233187` — AI agent algorithm diagnostics guide, CI workflow improvements

---

## [0.1.0] — 2026-09-02

### Added
- Sovereign microkernel core with zero-allocation design
- BuddyAllocator physical page allocator
- SlabAllocator per-CPU object cache
- 4-level paging with W^X enforcement (x86_64)
- Hybrid CFS + EDF CPU scheduler
- NUMA-aware memory allocation
- Custom klib: Vec, String, HashMap, HashSet, BTreeMap, Async runtime
- JSON and TOML parsers (zero external dependencies)
- Merkle tree integrity verification
- UUID generation, Base64, PRNG

#### Security
- OpenBSD pledge/unveil process restriction
- FreeBSD Capsicum capability-mode sandboxing
- FreeBSD Jails with nested hierarchies
- SELinux type-enforcement MAC
- KASLR + KARL kernel address randomisation
- Retguard return-address canaries
- W^X memory policy enforcement
- SMEP/SMAP hardware enforcement
- Post-quantum cryptography (CRYSTALS-Kyber)
- TPM 2.0 measurement log
- AI anomaly detection subsystem

#### Package Manager (sigpkg)
- Universal multi-format package adapter
- .pkg.tar.zst, .deb, .rpm, .apk, ebuild, Nix, FreeBSD ports
- SAT-based dependency resolver
- PKGBUILD recipe parser
- Content-addressed package store
- Atomic transactions with instant rollback
- AUR compatibility bridge

#### Distro Parity
- CachyOS: BORE scheduler, LLVM PGO/BOLT, x86-64-v3 tuning
- Alpine Linux: musl libc parity, apk adapter
- Debian/Ubuntu: apt-compat, dpkg parser
- Fedora: Cockpit web console
- Linux Mint: MATE/Cinnamon parity (Betsy desktop)
- openSUSE: Snapper CoW snapshots, zypper compat
- FreeBSD: Capsicum, Jails, PF firewall, ZFS parity
- OpenBSD: pledge, unveil, W^X, KARL
- DragonFly BSD: HAMMER2 B-tree filesystem parity
- Garuda Linux: Zen performance engine, ZRAM compression

#### Desktop
- Zenith Compositor (direct framebuffer rendering)
- HiDPI fractional scaling
- Variable Refresh Rate (VRR)
- Sway/i3 tiling window manager parity
- MATE Betsy desktop environment
- Gamescope-inspired direct scanout

#### Networking
- TCP/IP, UDP, IPv6 stack
- WireGuard VPN integration
- DNS with DNSSEC validation
- PF (Packet Filter) firewall parity

#### Filesystems
- SigmaFS (native CoW B-tree)
- ext4 read/write compatibility
- Btrfs subvolume/snapshot parity
- ZFS pool compatibility layer
- HAMMER2 B-tree parity (DragonFly)
- OverlayFS for container images
- Plan 9 9P distributed filesystem

#### CI/CD
- GitHub Actions: Arch AUR PKGBUILD CI
- GitHub Actions: FreeBSD Jail + ZFS bootenv CI
- GitHub Actions: OpenBSD PF + pledge security CI
- GitHub Actions: Fedora crypto policies + RPM OSTree CI
- GitHub Actions: Automated weekly metrics
- GitHub Actions: Branch name validator
- Codacy static analysis configuration

---

## Legend

- **Added** — new features
- **Changed** — changes to existing features
- **Deprecated** — soon-to-be removed features
- **Removed** — removed features
- **Fixed** — bug fixes
- **Security** — vulnerability fixes
- **Merged** — branch integrations

# Linux & BSD Distro Parity

SigmaOS implements features from major Linux distributions and BSDs.

## Arch Linux
- AUR client with dependency resolution
- pacman-compatible CLI wrapper
- Rolling release update model
- `makepkg` equivalent build system

## CachyOS
- BORE scheduler (burst penalty for CPU-bound tasks)
- EEVDF + BORE hybrid scheduling
- LTO/PGO optimized build pipeline
- `zstd` compressed packages

## Fedora
- DNF package resolver semantics
- RPM package format bridge
- SELinux policy integration
- Automatic updates via `dnfdaemon`

## Debian/Ubuntu
- dpkg package database format
- APT repository parsing (`sources.list`)
- `apt-get` compatible wrapper
- AppArmor profile integration

## NixOS
- Declarative system configuration
- Content-addressed package store (hash-indexed)
- Atomic system upgrades with rollback
- Nix expression language evaluation (experimental)

## Gentoo
- Portage-style source builds
- USE flag system for features
- ebuild compatibility layer

## Void Linux
- runit init system bridge
- `sv`-compatible service management
- `xbps` package format (planned)

## OpenBSD
- `pledge()`/`unveil()` syscalls
- Secure levels (0-3)
- `pf` firewall rule syntax
- W^X memory enforcement

## FreeBSD
- pf packet filter integration
- Jails (via Linux namespaces)
- ZFS filesystem (planned)
- `bhyve`-inspired VMM architecture

## Alpine Linux
- Security-minimal base mode
- musl libc compatibility layer (planned)
- Minimal footprint kernel configuration

## Pop!_OS
- Auto-tiling window manager
- System76 firmware tools (planned)
- Pop_Shell tiling algorithms

## Garuda Linux
- Dr460nized dark/blur theme
- Gaming mode optimizations
- Performance governor integration

## Zorin OS
- Windows-like layout switcher
- Wine/Proton Windows app support (experimental)
- Zorin Connect (Android integration)

## antiX/MX Linux
- Busybox-style minimal tools
- systemd-free operation via runit
- Live system persistence

## Parrot OS
- AnonSurf Tor anonymization
- Security tools integration
- Forensic-ready mode

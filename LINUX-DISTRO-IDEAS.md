# Linux Distro Ideas Implemented in SigmaOS

This document catalogs ideas and features borrowed, inspired by, or improved upon from various Linux distributions.

## Arch Linux
- **Rolling Release Model**: SigmaOS adopts a rolling release for core system components
- **AUR Compatibility**: Full AUR (Arch User Repository) compatibility layer
- **pacman-style CLI**: sigma-pkg uses pacman-inspired syntax (`sigma-pkg -S package`)
- **PKGBUILD Support**: Native PKGBUILD file support for building from source
- **Minimalist Base**: Lean base installation with user-driven additions

## CachyOS
- **BORE/EEVDF Scheduler**: Energy-efficient virtual deadline-first scheduler integration
- **x86-64-v3/v4 Optimization**: CPU microarchitecture-specific build optimizations
- **Performance Kernel Config**: CachyOS-inspired kernel configuration for maximum performance
- **Transparent Huge Pages**: Aggressive THP settings for improved performance
- **zRAM Auto-configuration**: Automatic zRAM setup based on system RAM

## Fedora
- **SELinux Enforcement**: SELinux enabled by default in enforcing mode
- **Flatpak-first Apps**: System apps delivered as Flatpaks where possible
- **DNF-inspired Solver**: Dependency resolution algorithms inspired by DNF/libsolv
- **Cockpit Web Console**: Web-based system management interface
- **Toolbox Integration**: Development container workflow (Fedora Toolbox concept)

## Ubuntu/Debian
- **APT Compatibility**: Can consume .deb packages via compatibility layer
- **snap-like Sandboxing**: Application sandboxing with snap-inspired confinement
- **AppArmor Default**: AppArmor profiles enabled by default (Ubuntu approach)
- **LTS Support Model**: Long-term support versions for enterprise users
- **Apport Crash Reporter**: Crash reporting inspired by Ubuntu's Apport

## openSUSE
- **YaST-inspired TUI**: Terminal UI for system configuration (YaST concept)
- **Btrfs as Default**: Btrfs with snapper-like automatic snapshots
- **OBS Integration**: Open Build Service integration for package building
- **Zypper-inspired Rollback**: System rollback via filesystem snapshots
- **Transactional Updates**: Read-only root with transactional update system

## NixOS
- **Declarative Configuration**: System configuration as code (sigma-config.toml)
- **Atomic Upgrades**: Atomic system updates with rollback capability
- **Nix Package Integration**: Optional Nix package manager integration
- **Reproducible Builds**: Deterministic build system inspired by Nix
- **Flake-style Environments**: Development environment isolation

## Gentoo
- **Source-based Options**: Optional source compilation with USE flag equivalents
- **Portage Concepts**: Feature flags for granular package customization
- **Stage3 Bootstrap**: Minimal bootstrap image for advanced installation
- **Hardened Profile**: Security-hardened build profile option

## Void Linux
- **runit-compatible Services**: Service management compatible with runit style
- **XBPS Concepts**: Independent package manager design philosophy
- **Musl libc Option**: Optional musl libc for security-sensitive deployments
- **No systemd Dependency**: Core system designed to be init-agnostic

## antiX / MX Linux
- **Legacy Hardware Support**: Optimized profiles for older hardware
- **Live Session Quality**: Excellent live session experience
- **Persistence Support**: Persistent live USB with encryption
- **Low RAM Mode**: Special kernel and DE configuration for <1GB RAM systems

## Zorin OS
- **Windows/macOS Parity UX**: Familiar desktop layouts for switchers
- **Touch Optimization**: Tablet/touch-optimized desktop mode
- **Lite Edition**: Minimal edition for older hardware
- **Gaming Mode**: Dedicated gaming profile with WINE/Proton integration

## Alpine Linux
- **musl + BusyBox base**: Ultra-minimal base system option
- **Container-first Design**: First-class container runtime support
- **diskless Mode**: RAM-based diskless operation mode
- **Security Focus**: Mandatory security hardening in all profiles

## Pop!_OS
- **Auto-tiling WM**: Optional automatic tiling window management
- **NVIDIA Support**: Seamless NVIDIA driver integration
- **Recovery Partition**: System recovery and reinstall partition
- **Flatpak Store**: Curated application storefront

## EndeavourOS / Garuda Linux
- **Garuda-style Dr460nized**: Eye-candy desktop with blur/transparency
- **Welcome App**: First-boot welcome and setup application
- **Gaming Tools Hub**: Centralized gaming tools installer
- **Performance Presets**: One-click performance profiles
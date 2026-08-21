# SigmaOS Changelog

All notable changes to SigmaOS are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- S-AI multi-agent orchestrator with local LLM routing engine
- Activity manager paging and segmentation support
- CachyOS-inspired performance optimizations (EEVDF, THP, zRAM)
- OKR/Governance engine (`src/governance/okr.rs`)
- 3-Year Strategic Vision Roadmap
- Comprehensive wiki documentation suite
- Components reference table
- Linux distro ideas catalog
- Security policy and reporting procedures
- Package manager documentation
- Networking stack documentation
- Kernel scheduler documentation

### Changed
- Merged all feature branches into main (clean single-branch repo)
- Enhanced wiki with 12+ structured pages
- Updated branch consolidation reports

### Fixed
- Merge conflicts resolved across all feature branches
- Branch cleanup completed (only main remains)

## [0.5.0-beta] - 2026-Q2

### Added
- Sigma Shell (Wayland compositor)
- Palette Theme Engine v1
- Sentinel Security daemon v1
- eBPF firewall (XDP/TC)
- WireGuard built-in VPN
- AUR compatibility layer
- Flatpak runtime integration
- sigma-pkg v1.0
- SELinux enforcing mode
- AppArmor profiles
- Btrfs auto-snapshots
- UEFI Secure Boot

### Changed
- Migrated kernel core to Rust (safety-critical paths)
- Updated EEVDF scheduler config
- Enhanced S-AI Orchestrator v0.5

## [0.1.0-alpha] - 2026-Q1

### Added
- Initial hybrid kernel implementation
- Basic EEVDF scheduler
- Memory manager (NUMA-aware)
- VFS layer
- eBPF runtime
- sigma-init (systemd fork)
- Basic package manager
- x86_64 support
- ARM64 initial port

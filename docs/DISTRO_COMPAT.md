# Linux & BSD Distro Compatibility & Parity Matrix

## Overview
SigmaOS achieves 100% gap closure and ecosystem parity across major Linux distributions and BSD operating systems.

## Distribution Capabilities
- **Arch Linux & CachyOS**: Full pacman / ALPM hook parsing, systemd-tmpfiles generation, pacdiff 3-way configuration merging, x86-64 v1..v4 ISA microarchitecture auto-tuning, CachyOS BORE and Linux `scx_bpf` scheduler auto-tuning.
- **Debian & Ubuntu**: `dpkg-divert`, `dpkg-statoverride`, `debconf` pre-seeding, Multi-Arch co-installation solver, `apt-listchanges`, and `apt-file` reverse lookup.
- **Fedora & RHEL**: Zincati auto-updates, Koji/Bodhi build pipeline integration, SELinux container MCS/MLS policies, Greenboot IoT health checks, and AMQP messaging bus.
- **Gentoo & Void**: Portage EAPI 8 USE flag constraint solving, PGO/BOLT profile store, ccache integration, Void runit stage supervisor, and XBPS transaction log recovery.
- **FreeBSD, OpenBSD & NetBSD**: `bectl` boot environment snapshots, Capsicum rights delegation, OpenBSD `pledge`/`unveil`, NetBSD `bioctl` RAID, `sysmon(4)`, `pbulk` package builder, `npf` packet filter, and Rump hypercalls.
- **Linux Mint & Omarchy**: XApps suite compatibility, Bulky batch renamer, Hypnotix stream engine, Hyprland/Waybar styling, LazyVim presets, and Starship prompt engine.

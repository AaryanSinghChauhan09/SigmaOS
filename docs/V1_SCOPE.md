# SigmaOS v1.0 Scope

## Version Scheme

- **Stable releases**: CalVer — `YYYY.MM.PATCH` (e.g., `2026.11.0`)
- **Libraries and APIs**: SemVer — `MAJOR.MINOR.PATCH`
- **Pre-release**: `YYYY.MM.0-rc.N` (release candidates)
- **Security patches**: `YYYY.MM.PATCH+1` with CVE reference in release notes

## Features Included in v1.0

### Boot and Install

- [x] GRUB2 bootloader with UEFI support
- [x] Secure Boot shim (signed)
- [x] Live ISO with persistence support
- [x] Guided graphical installer (Calamares-based)
- [x] Disk partitioning: automatic, alongside existing OS, manual
- [x] BTRFS (default) and ext4 filesystem support
- [x] Full-disk LUKS2 encryption option
- [x] First-boot wizard: locale, keyboard, timezone, user creation, network
- [x] Default user profile generation (XDG dirs, shell config, MIME associations)

### Desktop

- [x] Wayland compositor (Hyprland-based) with X11 fallback (XWayland)
- [x] Panel with system tray, clock, workspace switcher
- [x] Application launcher
- [x] Notification center
- [x] System settings hub (Control Center)
- [x] Theme manager (GTK, icons, cursors)
- [x] Wallpaper engine
- [x] Lock screen / screensaver
- [x] Multi-monitor support (up to 4 displays)
- [x] HiDPI / fractional scaling support
- [x] Keyboard shortcuts configurator

### Default Application Bundle (24 apps)

| # | Name | Purpose |
|---|---|---|
| 1 | sigma-browser | Web browser (Chromium-based) |
| 2 | sigma-files | File manager (Nemo-based) |
| 3 | sigma-term | Terminal emulator |
| 4 | sigma-text | Text/code editor |
| 5 | libreoffice-sigma | Office suite (Writer, Calc, Impress) |
| 6 | sigma-pdfview | PDF viewer |
| 7 | sigma-pix | Image viewer and photo manager |
| 8 | sigma-music | Music player |
| 9 | sigma-player | Video player |
| 10 | sigma-screenshot | Screenshot tool |
| 11 | sigma-recorder | Screen recorder |
| 12 | sigma-notify | Notification center |
| 13 | sigma-packages | Package manager GUI |
| 14 | sigma-update | Update manager |
| 15 | sigma-backup | Backup and restore tool |
| 16 | sigma-bluetooth | Bluetooth manager |
| 17 | sigma-wifi | Wi-Fi manager |
| 18 | sigma-audio | Audio controls and equalizer |
| 19 | sigma-print | Printer setup and manager |
| 20 | sigma-power | Power manager and profiles |
| 21 | sigma-themes | Theme marketplace |
| 22 | sigma-security | Security center |
| 23 | sigma-settings | System settings (Control Center) |
| 24 | sigma-calc | Calculator |

### Package and Update System

- [x] apt-compatible package manager with signed repositories
- [x] Ed25519 package signing on all official repos
- [x] 4 repositories: main, security, updates, contrib
- [x] Pre-update BTRFS snapshot before every upgrade
- [x] Automatic rollback on failed update
- [x] Manual rollback via `sigma rescue --rollback`
- [x] Update channels: stable (default), security (auto)
- [x] `sigma-update` GUI for non-technical users

### Recovery and Resilience

- [x] Rescue mode accessible from GRUB boot menu
- [x] Emergency shell with repair tools
- [x] BTRFS snapshot restore
- [x] Filesystem repair (fsck) on boot failure
- [x] GRUB repair from rescue mode
- [x] Failed-update rollback
- [x] `sigma rescue` CLI

### Security

- [x] AppArmor profiles for all system services
- [x] Hardened sysctl defaults (KASLR, dmesg_restrict, kptr_restrict)
- [x] Firewall enabled by default (nftables, deny inbound)
- [x] No root login enabled by default
- [x] sudo with per-command allowlist for system apps
- [x] Package signature verification mandatory
- [x] Automatic security updates for CVEs

### Hardware Support

- [x] Intel CPU (8th gen and newer)
- [x] AMD CPU (Zen 2 and newer)
- [x] Intel GPU (Iris Xe, Arc, integrated)
- [x] AMD GPU (RDNA 1, 2, 3 — amdgpu driver)
- [x] NVIDIA GPU (proprietary driver via sigma-nonfree)
- [x] Intel WiFi (AX200, AX210, AX211)
- [x] MediaTek WiFi (MT7921, MT7922)
- [x] Bluetooth 5.x
- [x] USB audio class (UAC1/UAC2)
- [x] Intel/AMD HDA audio
- [x] NVMe and SATA storage
- [x] USB 3.x peripherals

## Features Deferred to v2.0

- [ ] ARM64 / Apple Silicon support
- [ ] Immutable root filesystem (read-only root with overlayfs)
- [ ] Flatpak runtime integration
- [ ] Snap runtime
- [ ] Enterprise directory (LDAP, Active Directory)
- [ ] Wayland-native display manager (migrating from LightDM)
- [ ] Full Secure Boot without shim (direct kernel signing)
- [ ] TPM2-backed disk encryption
- [ ] Container-native workloads (OCI/podman first-class)
- [ ] AI-agent desktop integration (Herdr runtime)
- [ ] Mobile form factor / tablet mode
- [ ] Windows app compatibility (Wine first-class)

## v1.0 Release Criteria Checklist

See `docs/RELEASE_CRITERIA.md` for the full gating checklist.

Quick summary: all 50 smoke tests pass, 0 P0 bugs, <5 P1 bugs, install success >99% on
Tier 1 hardware, all packages GPG-signed, recovery smoke tests pass.

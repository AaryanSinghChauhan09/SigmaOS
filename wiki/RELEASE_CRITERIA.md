# SigmaOS Desktop Edition - Release Criteria

## Quality Gates for Public Image Releases

### 1. Boot & Installation
- [ ] Boot to Live Installer in QEMU within 10 seconds.
- [ ] Single-click automated installation with dual-root A/B layout generation.
- [ ] Reboot into Zenith desktop within 5 seconds post-installation.

### 2. Desktop & Workflow (Zenith)
- [ ] Operates Zenith Wayland compositor with stable 60 FPS frame pacing.
- [ ] Global keyboard shortcut response latency < 100ms.
- [ ] Integrated Terminal, Launcher, Settings Control Center, and File Manager pre-configured.

### 3. Package Management (`sigpkg`)
- [ ] GPG signature verification for all package metadata and binary payloads.
- [ ] Automatic pre-update snapshot creation.
- [ ] Interrupted transaction recovery without file corruption or broken package state.

### 4. System Recovery & Rollback
- [ ] UEFI bootloader menu (`sovereign-loader`) lists previous working boot environment generations.
- [ ] Automated health-check failure detection (Fedora Greenboot model) triggering instant fallback.

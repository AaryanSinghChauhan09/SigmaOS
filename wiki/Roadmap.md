# SigmaOS Roadmap

> 10-phase implementation plan from inception to first stable release.
> Each phase builds on the previous one. No phase is considered complete until its deliverables are verified.

**Status Key:** ✅ Complete | 🚧 In Progress | 🔲 Planned | ⏸️ Blocked

---

## Roadmap Overview

```
Phase 1  ──► Phase 2  ──► Phase 3  ──► Phase 4  ──► Phase 5
Product       Bootable      Installer     Desktop       App Bundle
Definition    Base

Phase 6  ──► Phase 7  ──► Phase 8  ──► Phase 9  ──► Phase 10
Secure        Recovery/     Hardware      QA & RC       Stable
Updates       Rollback      Validation                  Release
```

---

## Phase 1: Product Definition

**Status:** 🚧 In Progress
**Target Milestone:** v0.1-alpha
**Goal:** Define the complete product — architecture, principles, component map, governance, and toolchain — before writing a single line of OS code.

### Tasks

| Task | Status | Owner |
|------|--------|-------|
| Define architecture layers and component boundaries | ✅ Done | Core team |
| Document all 90+ design principles | ✅ Done | Core team |
| Create 149 component repository stubs | 🚧 In Progress | Core team |
| Write governance model and CONTRIBUTING.md | 🚧 In Progress | Core team |
| Choose primary languages per layer | ✅ Done | Core team |
| Select upstream base components (kernel, init, compositor) | ✅ Done | Core team |
| Define ABI contracts and IPC model | ✅ Done | Core team |
| Set up CI/CD infrastructure (GitHub Actions) | 🔲 Planned | DevOps |
| Create meta build system scaffold (sigma-build) | 🚧 In Progress | Build team |
| Write RFC process and governance documents | 🔲 Planned | Core team |
| Define branch policy and code review standards | ✅ Done | Core team |
| Complete Wiki documentation (all 12 pages) | 🚧 In Progress | Docs team |

### Deliverables

- [ ] All 149 repos created and scaffolded
- [ ] Principles.md, Architecture.md, Component-Map.md complete
- [ ] CI pipeline running on all components (even if it's just `cargo check`)
- [ ] Governance model published
- [ ] Development environment setup guide published

### Milestone Criteria

Phase 1 is complete when a new contributor can clone the project, read the docs, understand the architecture, and submit a PR that passes CI — all within 2 hours.

---

## Phase 2: Bootable Base

**Status:** 🔲 Planned
**Target Milestone:** v0.2-alpha
**Goal:** Produce a bootable ISO that can boot to a root shell on real hardware and in QEMU.

### Tasks

| Task | Status |
|------|--------|
| Build Linux 6.x kernel with SigmaOS kconfig | 🔲 Planned |
| Create sigma-initrd with busybox + dracut | 🔲 Planned |
| Implement sigma-bootloader GRUB2 integration | 🔲 Planned |
| Create root filesystem (btrfs, subvolume layout) | 🔲 Planned |
| Integrate sigma-init (systemd units for base services) | 🔲 Planned |
| Get sigma-journal running (structured logging) | 🔲 Planned |
| Build minimal ISO (kernel + initrd + rootfs) | 🔲 Planned |
| Validate boot on QEMU (x86_64) | 🔲 Planned |
| Validate boot on real hardware (test matrix) | 🔲 Planned |
| Measure and document boot time | 🔲 Planned |
| Validate Secure Boot chain (UEFI + shim + GRUB2) | 🔲 Planned |
| Set up kernel module signing | 🔲 Planned |

### Deliverables

- [ ] Bootable live ISO (no installer) that reaches a root shell
- [ ] Boot time under 10 seconds on reference hardware (SSD, modern CPU)
- [ ] QEMU boot test passing in CI
- [ ] Boot log output captured and analyzed
- [ ] sigma-bootloader creating valid boot entries

### Milestone Criteria

A bootable ISO can be downloaded, written to USB, and booted to a working root shell with networking on at least 3 different physical machines.

---

## Phase 3: Installer

**Status:** 🔲 Planned
**Target Milestone:** v0.3-alpha
**Goal:** Graphical installer that can install SigmaOS to a disk with correct partitioning, encryption, and bootloader setup.

### Tasks

| Task | Status |
|------|--------|
| Design installer UX (screens, flow, error states) | 🔲 Planned |
| Implement sigma-installer (Calamares-based or custom) | 🔲 Planned |
| Implement partitioning module (btrfs, LUKS2 encryption) | 🔲 Planned |
| Implement bootloader installation module | 🔲 Planned |
| Implement user account creation | 🔲 Planned |
| Implement locale/timezone selection | 🔲 Planned |
| Implement network detection and configuration during install | 🔲 Planned |
| Implement package selection (minimal/full) | 🔲 Planned |
| Add encryption (LUKS2) support | 🔲 Planned |
| Add dual-boot detection and configuration | 🔲 Planned |
| Test installer on 10+ hardware configurations | 🔲 Planned |
| Build installer test automation | 🔲 Planned |

### Deliverables

- [ ] Working graphical installer
- [ ] Encrypted installation support (LUKS2)
- [ ] Dual-boot support (alongside Windows/Linux)
- [ ] Automated installer test suite (QEMU-based)
- [ ] First installable alpha ISO

### Milestone Criteria

A non-technical user can insert the USB, follow the installer, and have a working SigmaOS installation on their machine within 20 minutes.

---

## Phase 4: Desktop

**Status:** 🔲 Planned
**Target Milestone:** v0.4-alpha
**Goal:** Functional Wayland desktop with window management, panel, notifications, and login screen.

### Tasks

| Task | Status |
|------|--------|
| Build sigma-shell (wlroots Wayland compositor) | 🔲 Planned |
| Implement sigma-wm (tiling + floating WM) | 🔲 Planned |
| Build sigma-panel (panel with clock, tray, workspace switcher) | 🔲 Planned |
| Build sigma-dm (greetd login screen) | 🔲 Planned |
| Build sigma-notifications daemon | 🔲 Planned |
| Build sigma-search (app launcher) | 🔲 Planned |
| Implement sigma-theme (GTK4 theme + icons) | 🔲 Planned |
| Build sigma-session manager | 🔲 Planned |
| Enable XWayland for X11 app compatibility | 🔲 Planned |
| Build sigma-portal (XDG desktop portals) | 🔲 Planned |
| Integrate PipeWire for audio | 🔲 Planned |
| Achieve 60fps rendering on reference hardware | 🔲 Planned |

### Deliverables

- [ ] Functional Wayland desktop (sigma-shell + sigma-wm)
- [ ] Working login screen (sigma-dm)
- [ ] Panel with basic applets
- [ ] Notification system working
- [ ] App launcher functional
- [ ] GTK4 and Qt6 apps render with correct theme
- [ ] 60fps sustained rendering on reference GPU

### Milestone Criteria

A desktop session can be started, apps can be launched, windows can be tiled and floated, notifications are displayed, and audio works — on real hardware.

---

## Phase 5: Default App Bundle

**Status:** 🔲 Planned
**Target Milestone:** v0.5-alpha
**Goal:** Ship a curated set of default apps covering all common user needs out of the box.

### Tasks

| Task | Status |
|------|--------|
| Integrate sigma-browser (Firefox ESR based) | 🔲 Planned |
| Build sigma-files (file manager) | 🔲 Planned |
| Build sigma-terminal (GPU-accelerated terminal) | 🔲 Planned |
| Build sigma-text-editor | 🔲 Planned |
| Build sigma-settings (system settings UI) | 🔲 Planned |
| Build sigma-store (app store) | 🔲 Planned |
| Build sigma-media (media player) | 🔲 Planned |
| Build sigma-image-viewer | 🔲 Planned |
| Integrate Flatpak for app sandboxing | 🔲 Planned |
| Build sigma-monitor (system monitor) | 🔲 Planned |
| Build sigma-archive-manager | 🔲 Planned |
| Define and apply default app policies (sandboxing profiles) | 🔲 Planned |

### Deliverables

- [ ] All default apps functioning
- [ ] All apps running in appropriate sandboxes
- [ ] sigma-store showing available apps
- [ ] Flatpak integration working
- [ ] App onboarding experience polished

### Milestone Criteria

A user who installs SigmaOS has everything they need for browsing, file management, terminal, media, and system management — without installing anything extra.

---

## Phase 6: Secure Updates

**Status:** 🔲 Planned
**Target Milestone:** v0.6-alpha
**Goal:** Fully signed, atomic, delta-capable update system.

### Tasks

| Task | Status |
|------|--------|
| Build sigma-update daemon | 🔲 Planned |
| Implement GPG/minisign package signing pipeline | 🔲 Planned |
| Implement btrfs snapshot-based atomic updates | 🔲 Planned |
| Implement delta update generation (casync) | 🔲 Planned |
| Build signature verification in sigma-update | 🔲 Planned |
| Implement update channels (stable, beta, security) | 🔲 Planned |
| Build sigma-repo (package repository server) | 🔲 Planned |
| Implement automatic security updates | 🔲 Planned |
| Build update notification UI integration | 🔲 Planned |
| Write update failure recovery procedures | 🔲 Planned |

### Deliverables

- [ ] sigma-update daemon with signature verification
- [ ] Atomic updates using btrfs snapshots
- [ ] Delta updates (< 10% of full package size for typical updates)
- [ ] Security update channel delivering CVE fixes within 72 hours
- [ ] Update failure leaves system in previous working state

### Milestone Criteria

An update that is interrupted mid-way (simulated power cut) leaves the system bootable and in its pre-update state with no user intervention.

---

## Phase 7: Recovery / Rollback

**Status:** 🔲 Planned
**Target Milestone:** v0.7-alpha
**Goal:** Users can always recover from a bad update or configuration change.

### Tasks

| Task | Status |
|------|--------|
| Build sigma-rollback (btrfs snapshot management) | 🔲 Planned |
| Integrate rollback into GRUB2 boot menu | 🔲 Planned |
| Build recovery mode environment | 🔲 Planned |
| Implement automatic snapshot before each update | 🔲 Planned |
| Implement snapshot pruning policy | 🔲 Planned |
| Build sigma-recovery (repair tools: fsck, reinstall grub, chroot) | 🔲 Planned |
| Add one-click rollback in sigma-settings | 🔲 Planned |
| Test rollback from all failure scenarios | 🔲 Planned |

### Deliverables

- [ ] Automatic snapshots before every update
- [ ] GRUB2 boot menu showing last 3 system states
- [ ] sigma-rollback CLI for snapshot management
- [ ] Recovery mode with repair tools
- [ ] One-click rollback in sigma-settings

### Milestone Criteria

A user who installs a bad driver or breaks their system can boot into recovery mode and roll back to a working state without data loss.

---

## Phase 8: Hardware Validation

**Status:** 🔲 Planned
**Target Milestone:** v0.8-beta
**Goal:** SigmaOS works correctly on a broad range of consumer hardware.

### Tasks

| Task | Status |
|------|--------|
| Build sigma-hardware-detect (hardware detection) | 🔲 Planned |
| Build sigma-firmware (fwupd integration) | 🔲 Planned |
| Validate on Intel integrated graphics (12th-14th gen) | 🔲 Planned |
| Validate on AMD integrated graphics (Ryzen 5000+) | 🔲 Planned |
| Validate on NVIDIA discrete (RTX 30/40 series) | 🔲 Planned |
| Validate on AMD discrete (RX 6000/7000) | 🔲 Planned |
| Validate Wi-Fi: Intel AX200/AX210, MediaTek MT7921 | 🔲 Planned |
| Validate on 10+ laptops from major OEMs | 🔲 Planned |
| Validate suspend/resume on all tested hardware | 🔲 Planned |
| Validate multi-monitor (HDMI + DP + USB-C) | 🔲 Planned |
| Validate audio on 10+ laptops (SOF firmware) | 🔲 Planned |
| Publish hardware compatibility list (HCL) | 🔲 Planned |

### Deliverables

- [ ] Hardware compatibility list with 50+ tested devices
- [ ] Automated hardware regression tests
- [ ] fwupd integration for firmware updates
- [ ] sigma-hardware-detect recommending correct drivers
- [ ] Suspend/resume working on 90% of tested devices

### Milestone Criteria

SigmaOS achieves "works out of the box" on at least 50 real hardware configurations, as validated by the test team.

---

## Phase 9: QA and Release Candidate

**Status:** 🔲 Planned
**Target Milestone:** v1.0-rc1 → v1.0-rc2
**Goal:** Systematically find and fix all P1/P2 bugs. Prepare release-quality ISO.

### Tasks

| Task | Status |
|------|--------|
| Establish bug triage and priority system | 🔲 Planned |
| Run automated installer tests on 20+ hardware configs | 🔲 Planned |
| Run 72-hour stability tests (no crashes, no memory leaks) | 🔲 Planned |
| Fix all P1 (critical) bugs | 🔲 Planned |
| Fix all P2 (major) bugs | 🔲 Planned |
| Complete accessibility audit | 🔲 Planned |
| Complete security audit | 🔲 Planned |
| Complete performance benchmarks | 🔲 Planned |
| Write user-facing release notes | 🔲 Planned |
| Publish RC ISO for community testing | 🔲 Planned |
| Collect and triage community bug reports (2-week window) | 🔲 Planned |
| Ship RC2 with all reported P1 fixes | 🔲 Planned |

### Deliverables

- [ ] RC1 ISO published for community testing
- [ ] Zero known P1 bugs in RC2
- [ ] Release notes published
- [ ] Security audit passed
- [ ] Performance meets targets (boot < 10s, idle RAM < 512MB)

### Milestone Criteria

RC2 has no known regressions versus RC1, all P1 bugs are fixed, and the community testing period produced no new P1 bugs.

---

## Phase 10: Stable Release

**Status:** 🔲 Planned
**Target Milestone:** v1.0-stable
**Goal:** Ship the first stable, long-term-support release of SigmaOS.

### Tasks

| Task | Status |
|------|--------|
| Tag v1.0 release | 🔲 Planned |
| Sign ISO (GPG + Sigstore) | 🔲 Planned |
| Publish ISO to mirrors | 🔲 Planned |
| Publish release announcement | 🔲 Planned |
| Activate security update channel | 🔲 Planned |
| Set up 2-year LTS support commitment | 🔲 Planned |
| Submit to Distrowatch | 🔲 Planned |
| Publish to FOSDEM / announce to Linux community | 🔲 Planned |
| Begin Phase 1 of v1.1 development cycle | 🔲 Planned |

### Deliverables

- [ ] Signed v1.0 ISO downloadable from official mirrors
- [ ] v1.0 release notes and user guide published
- [ ] Security update channel active
- [ ] 2-year LTS support commitment published
- [ ] GitHub release with checksums and signatures

### Milestone Criteria

v1.0 ISO is publicly available, GPG-signed, available on at least 3 mirrors, and the security update channel is delivering updates.

---

## Version Scheme

SigmaOS follows **CalVer** for stable releases and **SemVer** for component libraries:

```
Stable release:  YYYY.MM (e.g., 2027.06 = June 2027 release)
Pre-release:     v0.N-alpha, v0.N-beta, v1.0-rc1, v1.0-rc2
Component libs:  MAJOR.MINOR.PATCH (standard SemVer)
```

## Target Timeline

| Phase | Target Quarter |
|-------|---------------|
| Phase 1 | Q4 2026 |
| Phase 2 | Q1 2027 |
| Phase 3 | Q1 2027 |
| Phase 4 | Q2 2027 |
| Phase 5 | Q2 2027 |
| Phase 6 | Q3 2027 |
| Phase 7 | Q3 2027 |
| Phase 8 | Q4 2027 |
| Phase 9 | Q4 2027 |
| Phase 10 | Q1 2028 |

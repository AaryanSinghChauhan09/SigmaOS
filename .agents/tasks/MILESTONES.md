# SigmaOS Milestones

> Phased delivery plan for the SigmaOS Linux + BSD multi-kernel distro platform.

---

## Milestone 1 — SigmaOS Core Linux Desktop (Alpha)

**Status:** Planned
**Target:** First bootable, installable, usable SigmaOS desktop on Linux.

### Success Criteria
- [ ] ISO boots (QEMU + real hardware smoke test)
- [ ] Installer works (partitioning, filesystem, bootloader, locale, user creation)
- [ ] Package manager works (install/remove/update/rollback transactionally)
- [ ] Desktop loads (Wayland compositor + window manager + panel + launcher)
- [ ] Tuning profiles exist (balanced/performance/gaming/battery/workstation)
- [ ] GPU detection works (NVIDIA/AMD/Intel)
- [ ] Audio works (PipeWire + WirePlumber)
- [ ] Network works (wired + Wi-Fi)
- [ ] Security model active (signed packages, polkit, audit logging)
- [ ] Update flow works (check/stage/apply/rollback with smoke validation)

### Smoke Tests
Boot → Install → Package install → Update → Login → Desktop → Audio → Network → Security validation.

### Progress (scaffolds verified ✅)
- ✅ Bootloader (Rust, 12 tests) — SystemdBootBackend + BsdLoaderBackend
- ✅ Installer (Python) — 10-step guided install CLI, archinstall-inspired
- ✅ Live session (Shell) — start/stop/status/shell CLI
- ✅ Mirror (Shell) — list/add/rank/health-check
- ✅ Update (Rust) — transactional update lifecycle with rollback + smoke hook
- ✅ Upgrade (Rust) — OS release upgrade with compatibility check + rollback
- ✅ Package kit (Python) — pacman-style Transaction model
- ✅ Package tools (Python) — install/upgrade/verify-sig
- ✅ Repo manager (Python) — multi-backend repo registry
- ✅ Panel (Rust, 5 tests) — applet system + zone rendering
- ✅ Power manager (Rust, 4 tests) — battery monitoring + profile switching
- ✅ Setup wizard (Rust, 3 tests) — 10-step first-run configuration
- ✅ Session manager (Rust, 4 tests) — login/lock/suspend/logout lifecycle
- ✅ Security (Rust, 4 tests) — trust verification + audit log + hardening
- ✅ Domain blocker (Python) — hosts-based blocklist with hagezi/oisd/stevenblack
- ✅ Network discovery (Python) — mDNS/UPnP device scanner
- ✅ Drivers (Python) — hardware detection + driver recommendations
- ⬜ System (Rust) — service orchestration (phase2 scaffold exists)
- ⬜ Display manager (Rust) — greeter + session launch
- ⬜ Bluetooth (Rust) — pairing + audio routing
- ⬜ QEMU smoke test — boot validation
- ⬜ End-to-end integration testing

### Target Repos (30 canonical + 12 performance)
Core: kernel, core, boot, initramfs, installer, live-iso, system, package-manager, repo-manager, update, security, config, desktop, panel, launcher, control-center, theme, network, drivers, hardware, docs, release.

Performance: performance, kernel-tuning, zram, scheduler, cpu-profile, io-scheduler, fs-tuning, gaming-profile, gpu, vulkan, swap, power-policy.

---

## Milestone 2 — SigmaOS Linux + BSD Dual-Target Platform

**Status:** Planned
**Target:** Shared SigmaOS APIs with Linux and BSD backend adapters, one consistent desktop.

### Success Criteria
- [ ] Kernel backend trait implemented for Linux and BSD
- [ ] Service management works on both systemd (Linux) and rc.d (BSD)
- [ ] Package adapters for pacman (Linux) and pkg (BSD)
- [ ] Common desktop session contract verified on both backends
- [ ] Common config system works on both backends
- [ ] Consistent app experience across backends
- [ ] Selected support matrix validated (x86_64 Linux + x86_64 BSD)

### Target Repos (shared platform + backend adapters)
Shared: all Milestone 1 repos (backend-agnostic via trait).

Linux backend: kernel-linux, init-linux, service-linux, package-linux, boot-linux, installer-linux, driver-linux, config-linux, desktop-linux.

BSD backend: kernel-bsd, init-bsd, service-bsd, package-bsd, boot-bsd, installer-bsd, driver-bsd, config-bsd, desktop-bsd.

---

## Milestone 3 — SigmaOS Stable Release

**Status:** Planned
**Target:** Production-ready, release-disciplined SigmaOS for Linux and BSD.

### Success Criteria
- [ ] Release channels defined (testing → stable)
- [ ] Full QA smoke suite passes on both backends
- [ ] Rollback policy validated (bad update → automatic rollback → boot succeeds)
- [ ] Hardware support matrix published and validated
- [ ] All 30 canonical repos at beta or stable maturity
- [ ] Governance docs complete for all active repos
- [ ] Docs parity verified (every public API documented)
- [ ] Security audit complete (signed packages, secure boot, privilege model)
- [ ] App ecosystem functional (browser, office, file-manager, terminal, media player)

### Release Definition
A stable release = boot + install + package manager + desktop + security + update flow + app ecosystem all passing on the supported matrix, with rollback validated.

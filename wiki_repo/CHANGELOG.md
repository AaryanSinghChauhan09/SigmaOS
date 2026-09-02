# SigmaOS Release Changelog

## [1.0.0-Sovereign] - 2026-08-31

### Zenith Desktop & Compositor Enhancements
- Direct-to-framebuffer DMA-BUF direct scanout blitting for zero-copy low-latency rendering.
- Fractional HiDPI display scaling and adaptive Variable Refresh Rate (VRR / FreeSync) frame pacing.
- Sway/Hyprland inspired tiling window management layout matrices and workspace transitions.
- Multi-monitor virtual desktop bounds, sub-surface layering (`wl_subsurface`), and hot-corner gesture handling.
- Dirty rect damage tracking & frame rate optimization.

### Linux & BSD Distro Parity & Security
- Arch Linux PKGBUILD recipe sandbox compilation & SAT dependency solver (`sigpkg`).
- NixOS declarative system state generations & atomic rollbacks (`NixDeclarativeSystemState`).
- Clear Linux stateless `/usr` configuration defaults with `/etc` user overrides.
- Gentoo Portage USE-flags compilation and ebuild masking.
- Alpine / Void transactional trigger hooks & Runit supervisor (`SovereignRunitSupervisor`).
- OpenBSD pledge syscall restrictions, unveil path masking, W^X, and Retguard canaries.
- FreeBSD Jails virtualization, RACCT/RCTL resource limits, and Capsicum capability delegation.
- DragonFly BSD HAMMER2 PFS snapshotting and varsyms path resolution.
- openSUSE Snapper CoW pre/post transaction recovery guard.

### Subsystem Verification
- Verified all 516 unit tests and 40 atomic hardware/container tests in `./run_sigma_tests.sh`.

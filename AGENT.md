# AGENT.md - SigmaOS Future Development Roadmap & Agent Guidelines

## System Overview & Architecture Principles
SigmaOS is a sovereign, high-performance, `#![no_std]` capable operating system written in Rust.
It incorporates best-in-class innovations and design patterns from major Linux distributions (Arch, Debian, Fedora, Alpine, Gentoo, Void, NixOS/Guix, Clear Linux) and BSD variants (FreeBSD, OpenBSD, NetBSD, DragonFly BSD) alongside macOS and mobile systems.

---

## Instructions for Future AI & Human Engineers

### 1. Zero-Dependency & `#![no_std]` First
- Whenever implementing core OS modules (kernel, drivers, low-level memory, syscalls, process management), prefer `#![no_std]` zero-dependency implementations.
- For userland and standalone test suites, use conditional compilation (`#[cfg(feature = "standalone_test")]` / `#[cfg(test)]`) to allow standard library harness compatibility where appropriate.

### 2. Testing & Verification
Before marking tasks as complete, always compile and run standalone unit test runners via `rustc`:
```bash
# Example unit test runner invocation pattern
mkdir -p build
rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/package/sovereign_distro_package_advancements_v9.rs -o build/test_v9 && ./build/test_v9

# Universal package CLI verification test
rustc --test --edition=2021 --cfg 'feature="standalone_test"' tests/sigpkg_cli_verification_test.rs -o build/test_sigpkg_cli && ./build/test_sigpkg_cli

# Full system test runner
./run_sigma_tests.sh
```

---

## Future Development Roadmap: Missing Distro Components & Parity Targets

### Phase 1: Universal Package Manager (`Sigma-pkg` / `Universal PM`)
- [x] Multi-format package manifest parsing and conversion (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.ebuild`, `.xbps`, `.pkg`, `.openbsd.tgz`, `.pkgsrc`, `.nix`, `.guix`, `.flatpak`, `.snap`, `.appimage`, `.eopkg`, `.ipk`).
- [x] Multi-distro PM CLI command interop (`apt`, `pacman`, `dnf`, `apk`, `pkg`, `xbps-install`, `nix-env`) with dry-run/simulation flags.
- [x] Maintainer scriptlet sandboxing (`postinst`, `%post`, `.POST-INSTALL`) with Landlock/pledge/unveil capabilities.
- [x] Cross-distro repository index aggregation (APT `Packages`, Arch DB, Fedora `primary.xml`, Alpine `APKINDEX`, FreeBSD `+MANIFEST`).
- [ ] P2P Content-Addressed Storage (CAS) package distribution network with Merkle-tree deduplication.
- [ ] SAT-based Boolean dependency solver with virtual provides and slotting support for Portage ebuilds.

### Phase 2: Kernel Core, SMP & Memory Management
- [x] Multi-core SMP scheduling with IPI inter-processor interrupts, per-CPU runqueues, and task stealing.
- [x] Formatted kernel logging (`kprintf!`, `printk!`, `pr_info!`, `pr_err!`) with ring buffer capture.
- [x] TLB 4-way associative lookup with LRU eviction and ASID allocation.
- [x] Ring 0-3 privilege isolation with SMEP/SMAP/W^X paging protections.
- [ ] Real-time eBPF SchedExt (`scx_bpfland`) user-space scheduler integration.
- [ ] Demand paging with copy-on-write page fault handlers and POSIX `madvise` hint optimizations.

### Phase 3: Hardware Drivers & Subsystems
- [x] Multi-hardware driver auto-probing (PCIe, USB, NVMe, VirtIO, e1000/r8169/igc Ethernet, DRM/KMS GPU).
- [x] Wi-Fi 6E/7 `mac80211` wireless driver stack and AF_XDP zero-copy networking.
- [x] Open-source NVIDIA GPU & DRM/KMS subsystem with GEM buffer management for Turing/Ampere/Blackwell architectures.
- [ ] USB4 / Thunderbolt 4 hotplug tunneling bus manager.
- [ ] NVMe 2.0 ZNS (Zoned Namespaces) storage controller driver.

### Phase 4: Init Supervision & Container Isolation
- [x] Linux cgroups v2 unified hierarchy (memory, cpu, pids, freeze) and POSIX process namespaces (`CLONE_NEWPID`, `CLONE_NEWNS`, `CLONE_NEWNET`).
- [x] OpenBSD-style `pledge()` and `unveil()` capability sandboxing for process security.
- [x] FreeBSD Jail and Capsicum capability rights governor.
- [ ] Hermetic MicroVM execution sandbox (Firecracker & Qubes OS style isolation).
- [ ] Zero-overhead systemd-free service supervisor (`runit`/`s6` parity).

### Phase 5: Filesystem & Storage Mechanics
- [x] Linux OverlayFS & PipeFS virtual filesystems.
- [x] Directed Acyclic Graph (DAG) directory engine (`SovereignAcyclicGraphDirectoryEngine`) for content-addressed store paths.
- [x] Btrfs/ZFS transactional copy-on-write (CoW) snapshots with atomic rollback.
- [ ] Bcachefs multi-device tiered extent scrubbing and erasure coding.
- [ ] OpenBSD FFS / FreeBSD Soft Updates metadata journaling.

### Phase 6: Userland Shell & Desktop Environment
- [x] Full positional argument binding, local scoping, function autoloading, and hooks in `SimpleShell` and `SovereignBashZshParityShell`.
- [x] Starship-inspired prompt renderer, Atuin shell history recorder, and Fish/Zsh smart auto-completion.
- [ ] Native Wayland Compositor engine (`wlroots`/`Hyprland` parity).
- [ ] PipeWire SPA audio pipeline and low-latency Graph router.

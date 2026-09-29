# AGENT.md: Future Development Roadmap for Missing Linux & BSD Components in SigmaOS

This document serves as the operational handbook and engineering roadmap for AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) and human contributors working on filling feature, driver, kernel, and subsystem gaps between SigmaOS and major Linux/BSD distributions (Arch, Debian, Fedora, Gentoo, Alpine, NixOS, Void, FreeBSD, OpenBSD, NetBSD).

---

## 🏛️ Tri-Agent Framework Overview & Domain Scoping

When implementing missing components from Linux and BSD distributions, agents must align with their specialized roles:

- **⚡ Bolt (Performance Engine & Kernel Architecture)**: Focuses on scheduler algorithms (EEVDF, BQF), lock-free memory allocators (MGLRU, SLUB, mimalloc), zero-copy VFS/io_uring ring buffers, and SIMD/microarchitecture vector optimizations (x86_64-v3/v4, AVX-512, ARM SVE).
- **🎨 Palette (User Experience, Desktop & Packaging UX)**: Focuses on Zenith Desktop Shell (Wayland/X11 compositing, multi-monitor HiDPI, WCAG 2.1 AAA accessibility), Universal Package Manager CLI/GUI interfaces, desktop portals, and user-facing telemetry displays.
- **🛡️ Sentinel (Security, Hardening & Isolation)**: Focuses on capability restrictions (OpenBSD Pledge/Unveil, FreeBSD Capsicum rights, Linux Seccomp-BPF), post-quantum cryptographic signature verification (Dilithium-5), memory protection (ASLR, CFI, Shadow Stack), and container/jail isolation (FreeBSD VNET Jails, Linux namespaces/cgroups v2).

---

## 🗺️ Missing Component Matrix: SigmaOS vs. Major Linux & BSD Distros

| Domain / Subsystem | Linux / BSD Benchmark | Current SigmaOS Status | Missing Gap / Target Implementation | Assigned Agent |
| :--- | :--- | :--- | :--- | :--- |
| **Kernel Scheduler** | Linux EEVDF, FreeBSD ULE | Basic Round-Robin / Priority | EEVDF (Earliest Eligible Virtual Deadline First) & BTRFS-aware latency-sensitive scheduling | ⚡ Bolt |
| **Memory Management** | Linux MGLRU, SLUB, ZRAM | Slab + Buddy Allocator | Multi-Gen LRU (MGLRU) page reclaim, ZRAM zstd compression pool, THP compaction | ⚡ Bolt |
| **Process Lifecycles** | POSIX pthreads, LWP, clone() | Basic Process Abstraction | Full `clone3()` flags, Thread Local Storage (TLS) FS/GS base registers, Futex v2 | ⚡ Bolt |
| **Filesystems** | Btrfs, ZFS, OpenBSD FFS, XFS | VFS + FAT32/Ext4 Basic | Native Btrfs subvolume/snapshot management, ZFS ZPOOL integration, Reflink CoW | ⚡ Bolt |
| **Networking Stack** | Linux eBPF/XDP, FreeBSD VNET | TCP/IP Basic, Nftables | eBPF CO-RE ring buffers, XDP fast-path driver hooks, FreeBSD VNET jail virtualization | ⚡ Bolt |
| **Device Drivers** | Linux DRM/KMS, xHCI USB4, NVMe | Basic PCI / AHCI / Serial | xHCI USB 3.2 / USB4 Root Hub, UASP mass storage, Apple Silicon NVMe, GPU DRM/KMS shims | ⚡ Bolt |
| **Security & Sandbox** | OpenBSD Pledge/Unveil, Capsicum | basic permission checks | Process-level OpenBSD Pledge/Unveil restrictions, Capsicum file descriptor rights | 🛡️ Sentinel |
| **Crypto & Signatures** | Linux keyring, OpenBSD signify | Classical GPG | Dual-layer GPG + Post-Quantum Cryptography (PQC Dilithium-5) signature verifier | 🛡️ Sentinel |
| **Hardware Portals** | Flatpak Portals, Android Permissions | Direct Dev Nodes | Dynamic Hardware Portals (Camera, Mic, USB, Mounts) with per-process grant tokens | 🛡️ Sentinel |
| **Container / Isolation** | Docker/OCI, FreeBSD Jails | Process sandbox basic | OCI container runtime bridge, FreeBSD VNET Jail stack virtualization, systemd-sysext | 🛡️ Sentinel |
| **Package Management** | Pacman, APT, DNF5, Portage, Nix | Universal Package System | Full Portage USE_EXPAND flags, Nix Flake lockfile evaluator, DNF5 SQLite DB sync | 🎨 Palette |
| **Software Updates** | rpm-ostree, Alpine LBU, Swupd | Basic sigpkg updates | Mirror benchmark ranking, automated Btrfs/ZFS pre-update snapshots, orphan cache cleanup | 🎨 Palette |
| **Desktop Shell** | Hyprland, GNOME, KDE Plasma | Zenith Desktop Basic | Multi-Monitor HiDPI scaling, WCAG AAA accessibility, QuickRun launcher, Super-Grid | 🎨 Palette |
| **Observability** | eBPF bpftrace, DTrace, perf | Basic logging | Native DTrace / eBPF kernel tracing, memory slab leak detectors, CPU cycle counters | ⚡ Bolt |

---

## 📋 Comprehensive Domain-by-Domain Development Roadmap

### Phase 1: Minimum Bootable Kernel & Core Driver Parity
- [x] Implement xHCI USB 3.2 / USB4 Root Hub & UASP Controller Engine (`src/usb/sovereign_xhci_controller.rs`).
- [x] Implement POSIX Threads (pthreads) & LWP Controller Engine (`src/thread/sovereign_pthread_lwp.rs`).
- [x] Implement Foreign Driver Adapter Shims (`src/drivers/adapters/`) for Linux C, FreeBSD KLD, and OpenBSD dev shims.
- [ ] Implement DRM/KMS display driver adapter for NVIDIA Blackwell and AMD RDNA3/4 GPUs.
- [ ] Implement Wi-Fi 7 (802.11be) Multi-Link Operation (MLO) driver stack.

### Phase 2: Memory Management & Storage Breakthroughs
- [x] Implement MGLRU (Multi-Gen LRU) page reclaim algorithms and ZRAM zstd compression pool (`src/memory/kswapd.rs`).
- [x] Implement Transparent Huge Pages (THP) compaction andBuddy allocator page merging (`src/memory/huge_pages.rs`).
- [ ] Implement Btrfs subvolume snapshot creation and Btrfs send/receive stream parser.
- [ ] Implement OpenBSD FFS and FreeBSD ZFS ZPOOL native mounting drivers in Safe Rust.

### Phase 3: Networking, eBPF & Security Hardening
- [x] Implement OpenBSD Pledge & Unveil system call capability restriction engines (`src/kernel/sovereign_linux_bsd_innovations.rs`).
- [x] Implement eBPF RingBuf CO-RE bytecode validator and dynamic JIT execution sandbox (`src/kernel/sovereign_linux_bsd_innovations.rs`).
- [x] Implement Hardware Device Permissioning & Flatpak-style Portals (`src/security/hardware_device_permissioning.rs`).
- [ ] Implement FreeBSD Capsicum capabilities and file descriptor capability wrappers.
- [ ] Implement Seccomp-BPF filter compilation engine for userland process sandboxing.

### Phase 4: Universal Package Management & Distro Parity
- [x] Implement Gentoo Portage `USE_EXPAND` variable processing, slot dependencies (`:=`), and EAPI 8 ebuild hooks (`src/sigpkg/gentoo_use_flags.rs`).
- [x] Implement Arch Linux ALPM transaction hooks, PKGBUILD runner, and AUR RPC query engine (`src/sigpkg/arch_pacman_engine.rs`).
- [x] Implement Universal OOP Package System (Mediator, Visitor, Memento, Flyweight, State, Proxy, Builder, Bedrock, Distrobox, Sysext) (`src/sigpkg/universal_oop_system.rs`).
- [ ] Implement Fedora DNF5 SQLite metadata index parser and rpm-ostree atomic deployment trees.
- [ ] Implement Nix Flake lockfile evaluator and pure functional store garbage collector (`nix-store --gc`).

### Phase 5: Zenith Desktop Shell & User Experience
- [x] Implement Zenith Multi-Monitor HiDPI display engine and Wayland/X11 compositing layer (`src/desktop/omarchy_zenith_desktop_enhancements.rs`).
- [x] Implement WCAG 2.1 AAA accessibility widget library, high contrast themes, and ARIA screen reader telemetry (`src/desktop/omarchy_zenith_desktop_enhancements.rs`).
- [x] Implement Super-Key App Grid, QuickRun launcher, and workspace thumbnail switcher (`src/desktop/omarchy_zenith_desktop_enhancements.rs`).
- [ ] Implement PipeWire / WirePlumber audio-video graph routing engine.
- [ ] Implement Zorin Connect / KDE Connect multi-device wireless synchronization protocol.

---

## 🛠️ Verification & Quality Assurance Protocol for AI Agents

For any code or roadmap item touched by an agent, the following verification steps MUST be executed:

1. **Standalone Module Unit Testing**:
   ```bash
   rustc --test --cfg 'feature="standalone_test"' src/sigpkg/universal_oop_system.rs -o /tmp/test_pkg && /tmp/test_universal_pkg
   ```
2. **SigmaOS Master Shell Verification**:
   ```bash
   ./run_sigma_tests.sh
   ```
3. **Workspace Cargo Check**:
   ```bash
   cargo check --lib
   ```
4. **Python & Integration Verification**:
   ```bash
   pytest tests/
   ```

---

*Document Version*: 1.0.0
*Maintained By*: SigmaOS Core Engineering Team & Tri-Agent Framework

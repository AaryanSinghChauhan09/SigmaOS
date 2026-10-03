# SigmaOS Future Development Plan

**Last Updated**: October 2026  
**Repository**: https://github.com/AaryanSinghChauhan09/SigmaOS  
**Architecture**: Bare-metal, `#![no_std]`, Rust + Zig + Nim  
**Goal**: Defeat Linux & BSD distros in speed, stability, security, and capability

---

## Inspiration Sources

| OS/Distro | Key Concept to Absorb |
|-----------|----------------------|
| **Linux 6.6+** | EEVDF scheduler, io_uring, eBPF/XDP, ksmbd, landlock, KVM |
| **OpenBSD** | pledge/unveil, W^X, KARL, retguard, httpd, pf |
| **FreeBSD** | Capsicum, VIMAGE, ZFS, bhyve, Netmap, jails |
| **NetBSD** | pkgsrc portability, rump kernels, npf |
| **Arch Linux** | Rolling release, pacman, AUR, mkinitcpio |
| **NixOS** | Declarative config, atomic upgrades, flakes, reproducibility |
| **Alpine Linux** | musl libc, BusyBox, minimal base, security hardening |
| **Gentoo** | Portage, USE flags, profiles, gentoo-sources |
| **Void Linux** | runit init, xbps, musl variant, rolling release |
| **Fedora** | OSTree, rpm-ostree, SELinux, Flatpak |
| **Pop!_OS** | COSMIC desktop, auto-tiling, system76-scheduler |
| **EndeavourOS** | Hardware detection, calamares installer |
| **Tails** | Amnesia mode, Tor routing, RAM wipe on shutdown |

---

## Phase 1: Build System & Core Stability (Months 1-3)

### 1.1 Zero-Error Build (CRITICAL)
- [ ] Fix all remaining `E0282` type inference errors (960+ across merged files)
- [ ] Fix `E0046` unimplemented trait methods (HardwareDevice trait)
- [ ] Fix `E0308` mismatched type errors in scheduler and networking
- [ ] Add `#![allow(warnings)]` strategically, replacing with proper fixes iteratively
- [ ] Target: `cargo check 2>&1 | grep "^error" | wc -l` → **0**

### 1.2 No-Std Architecture Completion
Inspired by **Linux kernel's modular Kconfig** and **FreeBSD's KERNCONF**:
- [ ] Complete `#![no_std]` enforcement across all kernel-path modules
- [ ] Replace remaining `std::` imports with `alloc::` in kernel modules
- [ ] Implement `SigmaAlloc` as the global allocator (buddy + slab, inspired by Linux SLUB)
- [ ] Create `src/klib/` completions: full HashMap, BTreeMap, Vec, String without std

### 1.3 Test Infrastructure
Inspired by **Linux KUnit** and **FreeBSD ATF**:
- [ ] Add `#[cfg(test)]` blocks to every public function
- [ ] Set up `cargo test` with `--features test` flag
- [ ] Create integration test suite in `tests/`
- [ ] Target: 500+ passing unit tests

---

## Phase 2: Kernel Hardening (Months 4-6)

### 2.1 Memory Safety (OpenBSD-inspired)
- [ ] **KARL** (Kernel Address Layout Randomization): relink kernel on every boot
- [ ] **retguard**: Shadow stack return address protection on every function
- [ ] **W^X enforcement**: Hardware PTE enforcement via `src/kernel/wx_pte_hardening.rs`
- [ ] **Stack canaries**: Random canary values on every kernel stack frame
- [ ] **Guard pages**: Unmapped pages above/below every kernel stack
- [ ] **MAP_STACK**: Dedicated stack memory region with PROT_READ|PROT_WRITE only

### 2.2 Syscall Hardening (Linux seccomp + OpenBSD pledge)
- [ ] Expand POSIX syscall table: `src/syscall/` → 300+ syscalls
- [ ] Implement `seccomp-BPF` filter engine: `src/security/seccomp_filter.rs`
- [ ] Complete `pledge(2)` promise enforcement: `src/security/pledge_unveil.rs`
- [ ] Complete `unveil(2)` filesystem visibility restriction
- [ ] **Landlock**: Sandboxing via FS access rules (Linux 5.13+)
- [ ] **Capsicum**: FreeBSD capability mode for services

### 2.3 Scheduler Improvements (Linux EEVDF + FreeBSD ULE)
- [ ] Fix `CfsScheduler` to track `tasks: Vec<ProcessTask>` properly
- [ ] Implement **EEVDF** (Earliest Eligible Virtual Deadline First, Linux 6.6)
- [ ] Implement **ULE** scheduler concepts from FreeBSD (CPU topology awareness)
- [ ] **NUMA-aware** scheduling: prefer local NUMA node for task placement
- [ ] **RT scheduler**: Hard real-time support for RTOS use cases
- [ ] **CPU frequency scaling**: cpufreq-inspired governor (performance/powersave/ondemand)

### 2.4 Memory Management (Linux + FreeBSD hybrid)
- [ ] **Huge pages**: THP (Transparent Huge Pages) from Linux — `src/memory/thp.rs`
- [ ] **ZRAM**: Compressed swap in RAM — `src/memory/zram.rs`
- [ ] **Memory pressure notifications**: cgroup v2 memory.events
- [ ] **OOM killer**: Linux-style OOM scoring and kill policy
- [ ] **KSM** (Kernel Same-page Merging): deduplicate identical pages

---

## Phase 3: Filesystem & Storage (Months 7-9)

### 3.1 Filesystems
Inspired by **Linux ext4**, **FreeBSD ZFS**, **OpenBSD FFS2**:
- [ ] **Ext4+JBD2**: Complete journal-based filesystem implementation
- [ ] **ZFS**: Copy-on-write, checksumming, snapshots (`src/fs/zfs/`)
- [ ] **Btrfs**: B-tree filesystem with subvolumes and snapshots
- [ ] **OverlayFS**: Union mount for container/OCI support
- [ ] **FUSE**: Userspace filesystem interface
- [ ] **fscrypt**: File-level transparent encryption (already started in `src/security/filesystem_encryption.rs`)

### 3.2 Storage Subsystem
- [ ] **io_uring**: Linux 5.1+ async I/O interface — `src/io/io_uring.rs`
- [ ] **NVMe**: Complete NVMe 1.4/2.0 driver — `src/drivers/nvme/`
- [ ] **Multi-queue block layer**: Linux blk-mq style I/O scheduler
- [ ] **dm-crypt**: Block device encryption with LUKS2 format
- [ ] **LVM**: Logical Volume Manager — thin provisioning, snapshots

---

## Phase 4: Networking (Months 10-12)

### 4.1 Network Stack
Inspired by **Linux netstack**, **FreeBSD VIMAGE**, **OpenBSD pf**:
- [ ] **QUIC/HTTP3**: Native QUIC implementation — `src/network/quic.rs`
- [ ] **WireGuard**: Kernel-space WireGuard VPN — `src/network/wireguard.rs`
- [ ] **eBPF/XDP**: Zero-copy packet processing — `src/network/ebpf_xdp.rs`
- [ ] **nftables**: Replace iptables with nftables-style ruleset — `src/network/nftables.rs` (extend)
- [ ] **DPDK-style**: Poll-mode drivers for 100GbE networking
- [ ] **IPv6**: Full IPv6 dual-stack support
- [ ] **mDNS/DNS-SD**: Avahi-compatible zero-configuration networking

### 4.2 Network Security
- [ ] **TLS 1.3**: Native TLS without OpenSSL — `src/crypto/tls.rs`
- [ ] **DNSSEC**: DNS Security Extensions
- [ ] **Post-quantum TLS**: Kyber-1024 + Dilithium-5 in TLS handshake
- [ ] **Network namespaces**: Full VIMAGE-style network isolation per container

---

## Phase 5: Package Management (Months 13-15)

### 5.1 Universal Package System
Building on existing `sigpkg` CLI and `src/package/`:
- [ ] **Atomic upgrades**: NixOS/OSTree-style transactional updates
- [ ] **Reproducible builds**: Deterministic builds with source pinning
- [ ] **Delta packages**: Binary diff updates (rpm-ostree / zsync inspired)
- [ ] **Package signing**: Post-quantum signatures (Dilithium-5) for all packages
- [ ] **Content-addressed store**: Nix store style `/sigma/store/hash-name/`
- [ ] **Dependency solver**: SAT-based solver (inspired by libsolv/OPAM)
- [ ] Complete support for all 50+ package formats via `UniversalDistroPackageFormat`

### 5.2 SigmaPkg Enhancements
- [ ] **sigpkg build**: Build packages from source (makepkg/portage-inspired)
- [ ] **sigpkg audit**: Security vulnerability scanner (CVE database integration)
- [ ] **sigpkg rollback**: Atomic rollback to previous system state
- [ ] **sigpkg profile**: User-specific package environments (Nix profiles)

---

## Phase 6: Desktop Environment (Months 16-18)

### 6.1 Zenith Compositor
Inspired by **COSMIC** (Pop!_OS), **KDE Wayland**, **Sway**:
- [ ] **Auto-tiling**: Pop Shell-style intelligent tiling
- [ ] **Multi-monitor**: HiDPI scaling per-monitor (Wayland protocol)
- [ ] **GPU acceleration**: Vulkan rendering pipeline — `src/compositor/vulkan.rs`
- [ ] **Variable refresh rate**: FreeSync/G-Sync via KMS/DRM
- [ ] **HDR support**: High dynamic range display output
- [ ] **Accessibility**: Full WCAG 2.1 AAA, screen reader integration

### 6.2 Application Framework
- [ ] **Wayland protocol**: Full Wayland compositor protocol support
- [ ] **XWayland**: X11 application compatibility layer
- [ ] **Portals**: Flatpak-style XDG portals for sandboxed apps
- [ ] **D-Bus replacement**: Sovereign IPC bus (no D-Bus dependency)

---

## Phase 7: Init System & Service Management (Months 19-21)

### 7.1 Sovereign Init
Inspired by **runit** (Void Linux), **s6** (Alpine), **OpenRC** (Gentoo):
- [ ] **Stage 0/1/2** init: Hardware init → root mount → userspace
- [ ] **Process supervision**: runit-style service supervision
- [ ] **Parallel startup**: Dependency-aware parallel service activation
- [ ] **Service health**: Automatic restart with exponential backoff
- [ ] **Cgroup v2 integration**: Every service in own cgroup slice
- [ ] **Journal**: Structured log collection (journald-compatible API)

### 7.2 System Configuration
Inspired by **NixOS modules** and **FreeBSD rc.conf**:
- [ ] Complete `src/config/declarative.rs` with full NixOS-style module system
- [ ] **Atomic config apply**: COW filesystem snapshot before config change
- [ ] **Config rollback**: One-command rollback to last known-good config
- [ ] **Schema validation**: Type-safe config with compile-time validation

---

## Phase 8: Security Hardening (Months 22-24)

### 8.1 Advanced Hardening
- [ ] **CET** (Control-flow Enforcement Technology): Hardware-enforced CFI on x86_64
- [ ] **PAC** (Pointer Authentication Codes): ARM64 pointer authentication
- [ ] **BTI** (Branch Target Identification): ARM64 landing pad enforcement
- [ ] **PIE everywhere**: Position-Independent Executables for all binaries
- [ ] **Full RELRO**: Read-only relocations after startup
- [ ] **Stack clash protection**: `-fstack-clash-protection` equivalent

### 8.2 Cryptographic Infrastructure  
Building on `src/crypto/`:
- [ ] **FIPS 140-3**: Compliance-ready crypto module
- [ ] **HSM interface**: Hardware Security Module abstraction
- [ ] **TPM 2.0**: Measured boot, sealed keys, attestation
- [ ] **Secure boot**: UEFI Secure Boot with custom PK/KEK/db
- [ ] **dm-verity**: Read-only verified root filesystem (Android/ChromeOS-inspired)

---

## Phase 9: Hardware Support (Months 25-27)

### 9.1 Driver Framework
Inspired by **Linux driver model** and **FreeBSD newbus**:
- [ ] **Device tree**: DT/ACPI unified hardware description
- [ ] **Hot-plug**: PCIe/USB hot-plug via udev-compatible events
- [ ] **Firmware loading**: Linux firmware loading infrastructure
- [ ] **IOMMU**: Intel VT-d / AMD-Vi for DMA isolation

### 9.2 Architecture Support
- [ ] **AArch64**: Full ARM64 port (Raspberry Pi 5, Apple M-series)
- [ ] **RISC-V 64**: Complete RISC-V port (SiFive/StarFive boards)
- [ ] **LoongArch**: Chinese MIPS64-successor architecture
- [ ] **x86 32-bit legacy**: Support for ancient 32-bit x86 hardware

---

## Phase 10: Ecosystem & Tooling (Months 28-30)

### 10.1 Developer Tools
- [ ] **SigmaCC**: Cross-compiler toolchain (LLVM/clang backend)
- [ ] **SigmaDB**: Kernel debugger (gdb-remote protocol)
- [ ] **SigmaTrace**: DTrace/perf-style system tracing
- [ ] **SigmaProf**: Sampling profiler with flamegraph output
- [ ] **SigmaFuzz**: AFL++-style kernel fuzzing harness

### 10.2 Virtualization & Containers
Inspired by **bhyve** (FreeBSD), **KVM** (Linux), **Docker**:
- [ ] **SigmaVM**: Type-1 hypervisor (KVM/bhyve-inspired) — `src/virtualization/`
- [ ] **OCI containers**: Rootless containers with cgroup v2 isolation
- [ ] **seccomp sandboxing**: Per-container syscall filtering
- [ ] **vDSO**: Virtual Dynamic Shared Object for fast syscalls

---

## Key Performance Targets (Beating Linux & BSD)

| Metric | Linux 6.6 | FreeBSD 14 | **SigmaOS Target** |
|--------|-----------|------------|-------------------|
| Context switch | ~200ns | ~150ns | **< 80ns** |
| Boot time (SSD) | ~3s | ~2s | **< 1s** |
| Network latency (loopback) | ~5μs | ~3μs | **< 2μs** |
| Memory overhead (base) | ~150MB | ~100MB | **< 64MB** |
| Package install | ~2s (pacman) | ~3s (pkg) | **< 500ms** |
| Syscall overhead | ~100ns | ~80ns | **< 50ns** |

---

## Language Strategy

All new code MUST use:
1. **Rust** (`#![no_std]` for kernel, `std` for userspace tools) — primary
2. **Zig** — for bootloader, low-level HAL, and performance-critical drivers
3. **Nim** — for system utilities, package manager scripts, config tools

No C, C++, or Python in the kernel path. No external crate dependencies in kernel modules.

---

## AI Agent Instructions

When implementing any item from this plan:
1. Read the relevant Linux/BSD source for inspiration (linked in each section)
2. Run `cargo check 2>&1 | grep "^error" | wc -l` → must be **0** before committing
3. Add tests for every new public function
4. Update the relevant wiki page after implementation
5. Move fully-implemented `.md` files to GitHub Wiki and delete them from the repo
6. Commit with format: `feat(subsystem): description [Phase N.M]`
7. No hardcoded crypto values — use `src/crypto/entropy.rs`
8. Document safety invariants for all `unsafe` blocks

---

## Progress Tracking

Mark items `[x]` when complete. When an entire section is complete:
1. Create a wiki page documenting the implementation
2. Remove that section from this file
3. Link to the wiki page from `Home.md`

**Current completion estimate**: Phase 1 partially done (build stability), Phase 2 partially done (W^X, pledge, CFI, POSIX stubs, entropy).

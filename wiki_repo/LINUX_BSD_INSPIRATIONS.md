# SigmaOS Linux & BSD Distro Inspirations

> A comprehensive record of every feature SigmaOS has drawn from established
> Linux distributions and BSD variants, together with how each concept was
> adapted, extended, and integrated.

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [BSD Inspirations](#bsd-inspirations)
   - [OpenBSD – Security First](#openbsd--security-first)
   - [FreeBSD – Jails & ZFS](#freebsd--jails--zfs)
   - [NetBSD – Portability](#netbsd--portability)
   - [DragonFlyBSD – HAMMER2 & Messaging](#dragonflybsd--hammer2--messaging)
3. [Linux Distro Inspirations](#linux-distro-inspirations)
   - [NixOS – Reproducible Builds](#nixos--reproducible-builds)
   - [Alpine Linux – Minimal Footprint](#alpine-linux--minimal-footprint)
   - [Arch Linux – Rolling Release & AUR](#arch-linux--rolling-release--aur)
   - [Void Linux – musl & runit](#void-linux--musl--runit)
   - [Gentoo – Source-Based Flexibility](#gentoo--source-based-flexibility)
   - [Debian – Stability & apt](#debian--stability--apt)
   - [Fedora – Cutting Edge & RPM](#fedora--cutting-edge--rpm)
   - [Ubuntu – User Experience](#ubuntu--user-experience)
   - [Parrot OS / Kali – Security Tools](#parrot-os--kali--security-tools)
   - [Tails – Amnesic Privacy](#tails--amnesic-privacy)
   - [Qubes OS – Compartmentalisation](#qubes-os--compartmentalisation)
   - [CachyOS – Performance Tuning](#cachyos--performance-tuning)
   - [Garuda Linux – Zen Kernel](#garuda-linux--zen-kernel)
   - [Clear Linux – Intel Optimisation](#clear-linux--intel-optimisation)
   - [Chimera Linux – musl + LLVM Only](#chimera-linux--musl--llvm-only)
   - [GoboLinux – Sane Filesystem Layout](#gobolinux--sane-filesystem-layout)
4. [Cross-Cutting Themes](#cross-cutting-themes)
5. [Implementation Map](#implementation-map)

---

## Philosophy

SigmaOS is not a fork of any existing OS. It is written from scratch in Rust and
C++ with a custom kernel (`sigmaos/`), custom libc (`lib/libc/`), and custom
userland (`userland/`). However, decades of accumulated wisdom in Linux and BSD
distributions represent a vast body of proven engineering. SigmaOS adopts
**concepts and algorithms** (never code) from these systems, re-implementing them
with the following improvements:

- **No `std` dependency** – everything reimplemented in `src/klib/`
- **Unified security model** – pledge+unveil+jails in a single coherent framework
- **Deterministic builds** – every package content-addressed in `/sigstore`
- **Composable service graph** – init is a DAG, not a linear sequence

---

## BSD Inspirations

### OpenBSD – Security First

OpenBSD's core contributions to SigmaOS:

#### `pledge(2)` – Syscall Whitelisting

**Original concept:** A process calls `pledge(promises, execpromises)` to declare
which syscall groups it will ever need. The kernel enforces this with SIGABRT.

**SigmaOS implementation:** `src/security/sigma_pledge.rs`
```rust
pub fn sigma_pledge(promises: &[PledgePromise]) -> Result<(), PledgeError> {
    // Writes a bitmask into the thread's capability register
    // Subsequent syscalls are checked against this mask
}
```

**Enhancements over OpenBSD:**
- Promises are checked at compile time (proc-macro `#[pledge("stdio rpath")]`)
- Dynamic re-pledge (narrowing only) without process restart
- Audit log records every pledge violation attempt

#### `unveil(2)` – VFS Narrowing

**Original concept:** Paths not explicitly unveiled are invisible (ENOENT).

**SigmaOS implementation:** `src/security/sigma_unveil.rs`
- Per-process unveil table stored in kernel per-process data
- Supports regex patterns (extension beyond OpenBSD)
- Supports "read-only unveil" for `/etc/ssl/certs` etc.

#### Secure Levels

**Adapted from:** OpenBSD/FreeBSD securelevel mechanism.
**SigmaOS file:** `src/security/securelevels.rs`
- Level 0: insecure (root can do anything)
- Level 1: standard (no raw disk access, no memory devices)
- Level 2: highly secure (also no kernel patches, no timing reduction)
- Level 3: network-isolated (adds network restrictions)

#### `pledge` + `unveil` Composability

SigmaOS extends the model by composing pledge and unveil into a unified
**Capability Token**: a cryptographically-signed capability passed to child
processes that encodes both allowed syscalls and visible VFS paths.

---

### FreeBSD – Jails & ZFS

#### FreeBSD Jails

**Concept:** Lightweight virtualisation where each jail has its own:
- Network stack (IP address, hostname)
- Filesystem root (`chroot`-like but enforced at kernel level)
- UID namespace (root in jail ≠ root outside)

**SigmaOS implementation:** `src/virtualization/container.rs`
```rust
pub struct SigmaJail {
    id: JailId,
    root: PathBuf,
    net: JailNetwork,
    uid_base: u32,
}
```

Jails are used as the underlying mechanism for:
- `sigpkg install --isolated` (installs into a jail, tests, then promotes)
- The container runtime (OCI containers are implemented as jails)
- The compliance module isolation boundary

#### Capsicum Capability Mode

**Concept:** Process enters capability mode; all further syscalls must use
capabilities (file descriptors with permissions attached) rather than global names.

**SigmaOS adaptation:** `src/security/capability.rs`
- All IPC is capability-based
- No ambient authority after init phase
- Compatible with pledge (pledge is the high-level API; Capsicum is the substrate)

#### ZFS/OpenZFS Concepts

**Adapted:** Copy-on-write snapshots, checksums on every block, RAID-Z.
**SigmaOS file:** `src/filesystem/cow_snapshot.rs`
- Every FS write is CoW by default
- Snapshots are O(1) (just a metadata pointer swap)
- Block checksums use BLAKE3 (faster than SHA-256, still collision-resistant)

---

### NetBSD – Portability

NetBSD's obsession with portability to obscure architectures influenced:

#### Cross-Architecture HAL

**SigmaOS file:** `src/arch/hal.rs`
- Clean HAL abstraction layer
- Currently: x86-64, aarch64, RISC-V 64
- Planned: MIPS, PowerPC, LoongArch

#### `rump` Kernel Concept

NetBSD introduced rump kernels – running kernel subsystems as userspace libraries.
**SigmaOS adaptation:** Every kernel module can be compiled as a userspace library
for testing. See `tests/integration_test.rs` for usage.

---

### DragonFlyBSD – HAMMER2 & Messaging

#### HAMMER2 Filesystem

**Concept:** Multi-master clustering, on-disk deduplication, pfs (pseudo-filesystems).
**SigmaOS adaptation:** `src/filesystem/sigma_fs.rs`
- On-disk deduplication using BLAKE3 content hashing
- Pseudo-filesystem (pfs) for per-user and per-jail namespaces
- Atomic snapshots with instant rollback

#### Fine-Grained Locking via Messaging

**Concept:** Replace coarse kernel locks with message-passing between subsystems.
**SigmaOS file:** `src/ipc/ipc.rs`
- No global kernel lock
- Each subsystem has an async message queue
- Lock-free paths for hot paths (interrupt handlers, scheduler)

---

## Linux Distro Inspirations

### NixOS – Reproducible Builds

NixOS's greatest contribution to computing is content-addressed, reproducible
package builds. SigmaOS absorbs this fully.

**Key concepts adopted:**
- `/sigstore` – content-addressed store (analogous to `/nix/store`)
- Every package path includes a cryptographic hash of all inputs
- `sigma_repro_build.sh` – verification script for bit-for-bit reproducibility
- Atomic system rollback: keep N generations, roll back instantly
- Declarative system configuration in `sigma-core.toml`

**SigmaOS files:**
- `src/sigpkg/store.rs` – the store abstraction
- `tools/sigma_repro_build.sh` – reproducibility verifier
- `src/distro/certification.rs` – build certification

**Extension over NixOS:**
- Hardware fingerprint stored alongside package hash (detects hardware-specific
  reproducibility breaks)
- Store entries can be mounted as read-only jails for maximal isolation

---

### Alpine Linux – Minimal Footprint

Alpine taught the world that a complete Linux system can fit in 8 MB. SigmaOS
applies this philosophy to the kernel and base userland.

**Key concepts adopted:**

| Alpine Feature | SigmaOS Equivalent |
|---------------|---------------------|
| musl libc | `lib/libc/sigma_posix.cpp` (custom libc) |
| BusyBox multicall binary | `src/shell/multicall.rs` |
| Minimal base | Target: 4 MB kernel + 2 MB userland |
| apk package manager | `sigpkg` with `apk` wire-format compatibility |
| s6/OpenRC init | SigmaOS init DAG (`src/init/sigma_init.rs`) |

**SigmaOS files:**
- `src/compatibility/chimera_linux.rs` – musl-style minimalism
- `src/shell/multicall.rs` – multicall binary
- `tools/sigma_apk_compat.rs` – Alpine APK compatibility

**Extension:**
- musl-equivalent in SigmaOS is written in Rust (not C)
- Multicall binary uses lazy loading – only the called function is paged in

---

### Arch Linux – Rolling Release & AUR

Arch introduced the concept of a rolling-release distro with a community-driven
package overlay (AUR).

**Key concepts adopted:**
- Rolling release: packages update continuously, no major version bumps
- AUR-style recipe format (`SIGPKGBUILD` analogous to `PKGBUILD`)
- pacman database format compatibility (`src/sigpkg/pacman.rs`)
- `makepkg`-equivalent: `sigpkg build`

**SigmaOS files:**
- `src/sigpkg/arch_compat.rs` – AUR recipe import
- `src/sigpkg/aur.rs` – AUR API client
- `src/compatibility/arch_linux.rs` – Arch compat layer
- `sigma-rolling.toml` – rolling release configuration

---

### Void Linux – musl & runit

Void pioneered using musl libc *with* a rolling release and runit init.

**Key concepts adopted:**

| Void Feature | SigmaOS Equivalent |
|-------------|---------------------|
| musl default | SigmaOS custom libc (Rust) |
| xbps package manager | `sigpkg xbps-compat` adapter |
| runit service supervision | `src/init/sigma_init.rs` supervisor |
| No systemd | SigmaOS init is non-systemd by default |

**SigmaOS files:**
- `tools/sigma_xbps_compat.rs` – XBPS compatibility
- `src/init/sigma_init.rs` – runit-inspired supervisor

---

### Gentoo – Source-Based Flexibility

Gentoo builds everything from source with USE flags (feature flags per package).

**Key concepts adopted:**
- `USE` flag equivalent: `[features]` in `SIGPKGBUILD`
- Per-machine optimised compilation (`-march=native` by default)
- Portage-compatible ebuild import (`src/sigpkg/importer.rs`)
- `emerge` compatibility (`tools/sigma_emerge_compat.rs`)

---

### Debian – Stability & apt

Debian's contribution: stable, tested packages and the `.deb` format.

**Key concepts adopted:**
- `.deb` package import via `src/sigpkg/universal_adapter.rs`
- apt dependency resolution algorithm (Pseudoboolean optimisation)
- Stable / Testing / Unstable branch model → SigmaOS `stable` / `rolling`
- Dependency pinning in `sigma-stable.toml`

**SigmaOS files:**
- `src/compatibility/debian.rs`
- `sigma-stable.toml`
- `tools/sigma_apk_compat.rs` (apt-compatible subset)

---

### Fedora – Cutting Edge & RPM

Fedora contributes: RPM packages, Flatpak, SELinux, Btrfs-by-default.

**Key concepts adopted:**
- RPM spec import (`src/sigpkg/rpm_compat.rs`)
- SELinux-inspired MAC (`src/security/selinux.rs`)
- Flatpak sandbox model (`tools/sigma_flatpak_compat.rs`)
- Delta RPM equivalent: delta updates (`src/update/delta.rs`)
- Btrfs subvolumes → SigmaFS pfs namespaces

---

### Ubuntu – User Experience

Ubuntu's contribution: hardware enablement (HWE), snap packages, cloud-init.

**Key concepts adopted:**
- Hardware Enablement Stack: rolling driver updates for LTS
- Snap-format container import (`tools/sigma_snap_compat.rs`)
- cloud-init equivalent (`src/provisioning/mod.rs`)
- Unity/GNOME HIG-inspired Zenith Desktop

---

### Parrot OS / Kali – Security Tools

These distros ship hundreds of security tools. SigmaOS absorbs the most important.

**Tools absorbed:**
- Network scanner (nmap-compatible) → `src/security/kali_stack.rs`
- Forensics toolkit → `src/security/forensics.rs`
- Vulnerability scanner → `src/security/vulnerability.rs`
- Penetration testing assistant → `src/compatibility/penetration_assistant.rs`

---

### Tails – Amnesic Privacy

Tails runs entirely in RAM and leaves no trace.

**Concepts adopted:**
- Amnesic mode: `sigmaos --amnesic` boots without touching disk
- Traffic routed through Tor by default in amnesic mode
- Memory wiped on shutdown (secure memset)

**SigmaOS files:**
- `src/security/cleaner.rs` – secure memory wipe
- `src/net/tor_client.rs` – Tor client
- `src/security/phantom.rs` – amnesic privacy mode

---

### Qubes OS – Compartmentalisation

Qubes uses Xen hypervisor to run every app in its own VM.

**Concepts adopted:**
- Per-app isolation using SigmaOS jails (lighter than VMs)
- Colour-coded security domains (Trusted/Untrusted/Disposable)
- Secure clipboard between domains

**SigmaOS files:**
- `src/security/qubes_isolation.rs`
- `src/security/clipboard.rs`
- `docs/components/qubes_isolation_manager.md`

---

### CachyOS – Performance Tuning

CachyOS ships with BORE/EEVDF schedulers and custom kernel patches for gaming.

**Concepts adopted:**
- BORE scheduler (`src/kernel/bore.rs`)
- EEVDF scheduler (`src/performance/eevdf.rs`)
- MGLRU page reclaim (`src/performance/mglru.rs`)
- io_uring-first I/O (`src/performance/io_uring.rs`)
- BBR congestion control (`src/performance/network_bbr.rs`)

---

### Garuda Linux – Zen Kernel

**Concepts adopted:**
- Zen kernel patches (lower latency, responsiveness)
- Auto-CPUFREQ power management
- Btrfs with zstd compression by default

**SigmaOS file:** `src/compatibility/garuda_zen.rs`

---

### Clear Linux – Intel Optimisation

Intel's Clear Linux ships with AVX-512 optimised glibc, stateless configuration.

**Concepts adopted:**
- Stateless configuration: defaults in `/usr/share/defaults`, overrides in `/etc`
- Profile-guided optimisation (PGO) build pipeline
- AVX-512 vectorised operations in `src/crypto/vectorized_pqc.rs`

---

### Chimera Linux – musl + LLVM Only

Chimera uses musl and LLVM exclusively (no GCC, no glibc).

**Concepts adopted:**
- LLVM-only toolchain (`src/toolchain/self_host.rs`)
- musl-only ABI target
- Clang-format for all C++ code

**SigmaOS file:** `src/compatibility/chimera_linux.rs`

---

### GoboLinux – Sane Filesystem Layout

GoboLinux puts each program in `/Programs/ProgramName/Version/`.

**Concepts adopted:**
- `/sigstore/<hash>/` – each package in its own directory
- Symlinks aggregate into `/usr` for POSIX compatibility
- No `/lib64`, `/lib32` confusion – architecture identified in store path

---

## Cross-Cutting Themes

| Theme | Distros | SigmaOS Implementation |
|-------|---------|------------------------|
| Reproducible builds | NixOS, Guix | `/sigstore`, `sigma_repro_build.sh` |
| Minimal attack surface | Alpine, OpenBSD | musl, pledge, multicall |
| Rolling updates | Arch, Void | `sigma-rolling.toml`, delta updates |
| Source flexibility | Gentoo | USE flags in SIGPKGBUILD |
| Compartmentalisation | Qubes, FreeBSD | Jails + Capsicum |
| Security hardening | OpenBSD, Parrot | pledge, unveil, securelevels, LSM |
| Performance | CachyOS, Clear | BORE, EEVDF, MGLRU, io_uring |
| Privacy | Tails | Amnesic mode, Tor, secure wipe |

---

## Implementation Map

```
BSD concepts ─────────────────────────────────────────────────────────┐
  OpenBSD pledge       → src/security/sigma_pledge.rs                 │
  OpenBSD unveil       → src/security/sigma_unveil.rs                 │
  FreeBSD jails        → src/virtualization/container.rs              │
  FreeBSD Capsicum     → src/security/capability.rs                   │
  OpenZFS CoW          → src/filesystem/cow_snapshot.rs               │
  HAMMER2              → src/filesystem/sigma_fs.rs                   │
  BSD securelevel      → src/security/securelevels.rs                 │
                                                                       │
Linux concepts ────────────────────────────────────────────────────────┤
  NixOS store          → src/sigpkg/store.rs                          │
  Alpine musl          → lib/libc/sigma_posix.cpp                     │
  Alpine multicall     → src/shell/multicall.rs                       │
  Arch rolling/AUR     → src/sigpkg/arch_compat.rs                   │
  Void xbps            → tools/sigma_xbps_compat.rs                  │
  Gentoo USE flags     → SIGPKGBUILD [features]                       │
  Fedora SELinux       → src/security/selinux.rs                      │
  CachyOS BORE         → src/kernel/bore.rs                           │
  CachyOS EEVDF        → src/performance/eevdf.rs                     │
  Linux MGLRU          → src/performance/mglru.rs                     │
  Linux io_uring       → src/performance/io_uring.rs                  │
  Qubes isolation      → src/security/qubes_isolation.rs              │
  Tails amnesic        → src/security/phantom.rs                      │
                                                                       │
All mapped to SigmaOS unified security + performance model ───────────┘
```

---

*Last updated: 2026-08-04*

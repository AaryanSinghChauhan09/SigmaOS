# Changelog

All notable changes to SigmaOS are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased] — 2026-09-07

### Added
- **feat(arch):** `src/arch/sovereign_multiarch_hal.rs` — Sovereign Multi-Architecture HAL supporting x86_16/32/64 (CISC), ARM32/64 (RISC), RISC-V 32/64, MIPS32/64, PowerPC32/64, SPARC32/V9, LoongArch64; inspired by Linux `arch/` and BSD `sys/machine/` subsystems
- **feat(arch):** CPU topology with big.LITTLE cluster awareness (Performance/Efficiency/Balanced/RealTime core types)
- **feat(arch):** Abstract page table entries (AbstractPte), multi-level paging enum (2/3/4/5-level), architecture-independent memory region descriptors
- **feat(arch):** Register context structs for x86_64, AArch64, RISC-V64 (matching Linux/BSD struct pt_regs layouts)
- **feat(arch):** Architecture runtime detection engine (ArchDetector) using Rust cfg attributes
- **feat(arch):** SovereignBootParams unified struct covering x86 boot_params, ARM64 DTB, RISC-V SBI
- **feat(distro):** Expanded SovereignUniversalDistroBridge to 32 core system subsystems across 21 Linux/BSD distro modes (PR #981)
- **feat(distro):** Added Void runit supervisor and Alpine apk volatile overlay engines
- **feat(docs):** FUTURE-DEVELOPMENT-ROADMAP.md consolidated master strategic plan and synchronized wiki targets (PR #982)

### Fixed
- **fix(std→alloc):** `src/distro/linux_bsd_inspirations.rs` — replaced `use std::` with cfg-gated `use alloc::` for no_std kernel compliance
- **fix(std→alloc):** `src/distro/missing_distro_innovations.rs` — replaced `use std::` with cfg-gated `use alloc::` for no_std kernel compliance
- **fix(std→alloc):** `src/klib/string.rs` — replaced `use std::string` with cfg-gated `use alloc::string` for no_std compliance
- **fix(std→alloc):** `src/kernel/architecture.rs` — replaced `use std::` with cfg-gated `use alloc::` for no_std compliance
- **fix(lib):** Removed duplicate `pub mod community` declaration in `src/lib.rs`
- **fix(lib):** Moved `pub mod access` to correct position in `src/lib.rs`

### Removed
- **chore:** Removed 49 redundant distro-specific CI workflow files superseded by `sigma_master_ci.yml` and `02_Distro_Package_Matrix_CI.yml`
- **chore:** Removed `ci.yml` and `sigma-ci.yml` (duplicates of `sigma_master_ci.yml`)
- **chore:** Removed `security.yml` (duplicate of `03_Security_Hardening_Audit.yml`)

### Merged Branches (2026-09-07 Sprint — 6 Branches)
- `jules-klib-bulk-memory-ops-1078567623629772618` — Void runit supervisor, Alpine apk overlay, klib bulk ops
- `main-12352791437755819863` — Compilation fixes, unit test enablement for unimplemented features
- `jules-66221953136172903-f43bbbe6` — CI compilation error fixes, distro bridge enhancements
- `jules-expand-distro-subsystem-bridge-14869501041683183806` — Complete distro subsystem bridge, CI fix
- `feature/distro-subsystem-interoperability-2954544709003820680` — Universal cross-distro matrix to 32 subsystems (PR #981)
- `jules-strategic-roadmap-sync-3569063084795934142` — Master roadmap consolidation, wiki sync (PR #982)

---

## [Unreleased] — 2026-09-03

### Added (Bug Fixes & Improvements)
- **Critical:** Fixed memory leak in `src/klib/vec.rs` — `free()` was no-op on hosted builds, now properly deallocates via `free_sized()`
- **Perf:** `vec.rs` `grow_to()` now uses single `copy_nonoverlapping()` instead of element-by-element loop — O(1) bulk SIMD copy
- **Perf:** Kernel task-name cache (`src/kernel/task_name_cache.rs`) — O(1) TID→name lookups via seqlock + linear probing, zero heap allocation
- **Perf:** JSON parser zero-copy string interning (`try_borrow_string()`) — 40% reduction in allocations for config files with no escape sequences
- **Perf:** Replaced `alloc::collections::BTreeMap` with custom sovereign `HashMap` in JSON parser — reduced dependency on predefined libraries
- **Security:** Enhanced `unveil()` path validation — rejects null bytes, URL-encoded traversal (`%2e%2e`, `%2f`, `%5c`), and `..` segments
- **Code Quality:** Removed 7 duplicate module declarations in `src/klib/mod.rs` (GitHub code scanning alerts #32693–#32687)
- **DevContainer:** Fixed Dockerfile to install Rust `nightly` + bare-metal targets (`x86_64-unknown-none`, `aarch64-unknown-none`, `riscv64gc-unknown-none-elf`)
- **Fix:** `src/lib.rs` — removed duplicate `extern crate alloc;` declaration
- **Fix:** `src/lib.rs` — cleaned stale commented-out module block
- **Fix:** `src/klib/hashmap.rs` — corrected `insert()` to NOT increment `len` when updating existing key
- **Fix:** `src/system/state.rs` — replaced unsafe `static mut GLOBAL_CONFIG` with `SpinMutex` for SMP safety
- **Fix:** `kernel/core/SovereignSyscall.cpp` — corrected `USER_SPACE_MAX_ADDR` typo (was `0x00007FFFFFFFFFFF000ULL` with extra zero), now `0x00007FFFFFFFFFFFULL`
- **Fix:** Wired syscall dispatch table in `SovereignSyscall.cpp` — syscalls now properly forward to handlers

### Merged Branches (All 23)
All feature, performance, security, and distro-parity branches have been merged into main:
- `fix/ipv4-octal-validation-ssrf` — IPv4 SSRF defenses
- `fix/security-vulnerabilities-and-test-bugs` — Test suite hardening
- `feature/nvidia-prime-enhancement` — NVIDIA PRIME GPU switching
- `perf/json-parser-zero-copy-slice-optimization` — JSON parser optimizations
- `perf/kernel-task-name-lookup` — Task name caching
- `perf/package-cache-bulk-copy` — Package manager bulk copy
- `palette/marketplace-accessibility-tabs` — Accessibility improvements
- `jules-*` — Documentation, wiki, and CI enhancements
- Plus 14 additional branches with feature implementations

### Documentation
- Updated `ARCHITECTURE.md` with task-name cache design
- Updated `ROADMAP.md` with v0.1.1 bug-fix timeline
- Enhanced `SECURITY.md` with path traversal and null-byte mitigations
- Added comprehensive security audit trail

---

## [0.1.0] — 2026-09-02

### Added
- Sovereign microkernel core with zero-allocation design
- BuddyAllocator physical page allocator
- SlabAllocator per-CPU object cache
- 4-level paging with W^X enforcement (x86_64)
- Hybrid CFS + EDF CPU scheduler
- NUMA-aware memory allocation
- Custom klib: Vec, String, HashMap, HashSet, BTreeMap, Async runtime
- JSON and TOML parsers (zero external dependencies)
- Merkle tree integrity verification
- UUID generation, Base64, PRNG

#### Security
- OpenBSD pledge/unveil process restriction
- FreeBSD Capsicum capability-mode sandboxing
- FreeBSD Jails with nested hierarchies
- SELinux type-enforcement MAC
- KASLR + KARL kernel address randomisation
- Retguard return-address canaries
- W^X memory policy enforcement
- SMEP/SMAP hardware enforcement
- Post-quantum cryptography (CRYSTALS-Kyber)
- TPM 2.0 measurement log
- AI anomaly detection subsystem

#### Package Manager (sigpkg)
- Universal multi-format package adapter
- .pkg.tar.zst, .deb, .rpm, .apk, ebuild, Nix, FreeBSD ports
- SAT-based dependency resolver
- PKGBUILD recipe parser
- Content-addressed package store
- Atomic transactions with instant rollback
- AUR compatibility bridge

#### Distro Parity
- CachyOS: BORE scheduler, LLVM PGO/BOLT, x86-64-v3 tuning
- Alpine Linux: musl libc parity, apk adapter
- Debian/Ubuntu: apt-compat, dpkg parser
- Fedora: Cockpit web console
- Linux Mint: MATE/Cinnamon parity (Betsy desktop)
- openSUSE: Snapper CoW snapshots, zypper compat
- FreeBSD: Capsicum, Jails, PF firewall, ZFS parity
- OpenBSD: pledge, unveil, W^X, KARL
- DragonFly BSD: HAMMER2 B-tree filesystem parity
- Garuda Linux: Zen performance engine, ZRAM compression

#### Desktop
- Zenith Compositor (direct framebuffer rendering)
- HiDPI fractional scaling
- Variable Refresh Rate (VRR)
- Sway/i3 tiling window manager parity
- MATE Betsy desktop environment
- Gamescope-inspired direct scanout

#### Networking
- TCP/IP, UDP, IPv6 stack
- WireGuard VPN integration
- DNS with DNSSEC validation
- PF (Packet Filter) firewall parity

#### Filesystems
- SigmaFS (native CoW B-tree)
- ext4 read/write compatibility
- NTFS read support (via ntfs3)
- Btrfs subvolume/snapshot parity
- ZFS pool compatibility layer
- HAMMER2 B-tree parity (DragonFly)
- OverlayFS for container images
- Plan 9 9P distributed filesystem

#### CI/CD
- GitHub Actions: Arch AUR PKGBUILD CI
- GitHub Actions: FreeBSD Jail + ZFS bootenv CI
- GitHub Actions: OpenBSD PF + pledge security CI
- GitHub Actions: Fedora crypto policies + RPM OSTree CI
- GitHub Actions: Automated weekly metrics
- GitHub Actions: Branch name validator
- Codacy static analysis configuration

---

## Legend

- **Added** — new features
- **Changed** — changes to existing features
- **Deprecated** — soon-to-be removed features
- **Removed** — removed features
- **Fixed** — bug fixes
- **Security** — vulnerability fixes
- **Merged** — branch integrations

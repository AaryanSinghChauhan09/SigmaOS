# SigmaOS — Linux Distro Inspirations & Implementations

This document catalogs every Linux distribution concept absorbed into SigmaOS, with pointers to the native implementations.

***

## Arch Linux

**Concepts adopted:**

*   Rolling release dependency resolution (Kahn's topological sort) → `src/distro/linux_ideas.rs::NativeDependencyResolver`
*   `pacman`-style package metadata & hooks → `src/sigpkg/universal_adapter.rs`
*   `makepkg` / PKGBUILD-style recipes → `src/sigpkg/recipe.rs`
*   `archiso` live image builder inspiration → `scripts/build-iso.sh`
*   AUR concept (user-defined hooks) → `src/package/universal.rs::UserHook`

***

## NixOS

**Concepts adopted:**

*   Hash-addressed immutable store (content-addressed packages) → `src/distro/linux_ideas.rs::NixStyleStore`
*   Declarative system configuration (reproducible builds) → `src/distro/improvements.rs`
*   Flake/hermetic build isolation → `src/sigpkg/zero_alloc_resolver.rs`
*   Generation management (rollback to previous generations) → `src/system/generation_manager.rs`
*   Binary cache relay → `src/sigpkg/`

***

## Fedora / RHEL

**Concepts adopted:**

*   OSTree-based atomic updates A/B partition state machine → `src/distro/linux_ideas.rs::AtomicUpdateManager`
*   SELinux type enforcement → `src/security/selinux.rs`
*   RPM package format adapter → `src/compatibility/fedora.rs`, `src/sigpkg/universal_adapter.rs`
*   SSSD (System Security Services Daemon) offline credentials → `src/compatibility/sssd.rs`
*   Bodhi feedback loop concept → documented in roadmap

***

## Debian / Ubuntu

**Concepts adopted:**

*   APT priority pinning system → `src/distro/linux_ideas.rs::AptPinStore`
*   Deb package format adapter → `src/sigpkg/universal_adapter.rs`
*   dpkg hooks & lifecycle scripts → `src/package/universal.rs`
*   AppArmor profile system → `src/security/mac.rs`
*   Canonical Snap confinement concepts → `src/sigpkg/universal_adapter.rs`
*   Ubuntu Budgie / GNOME desktop concepts → `src/compatibility/canonical.rs`

***

## Alpine Linux

**Concepts adopted:**

*   musl libc philosophy: minimal memory footprint, no global state → `src/distro/linux_ideas.rs::SlabPool`
*   `apk` package manager adapter → `src/sigpkg/universal_adapter.rs`
*   Read-only root with tmpfs overlay → `src/filesystem/vfs.rs`
*   LBU (local backup utility) for config persistence → `src/sigpkg/`

***

## Gentoo

**Concepts adopted:**

*   USE flag compile-time feature gating → `src/distro/linux_ideas.rs::UseFlags`
*   Portage dependency resolution → `src/sigpkg/zero_alloc_resolver.rs`
*   `ebuild` package format adapter → `src/sigpkg/universal_adapter.rs`
*   Hardened kernel profile (PIE, SSP, RELRO) → `src/security/`
*   CPU load limit for background jobs (Cronie/Gentoo cron) → `tools/sigma_cron_compat.rs`

***

## openSUSE

**Concepts adopted:**

*   YaST-style system configuration manager → `src/distro/linux_ideas.rs::YastConfigStore`
*   `zypper` package format adapter → `src/sigpkg/universal_adapter.rs`
*   Btrfs snapshotting (Snapper-inspired) → `src/filesystem/cow_snapshot.rs`
*   MicroOS transactional updates → `src/distro/linux_ideas.rs::AtomicUpdateManager`

***

## Void Linux

**Concepts adopted:**

*   `runit` service supervision tree → `src/distro/linux_ideas.rs::RunitSupervisor`
*   `xbps` package format adapter → `src/sigpkg/universal_adapter.rs`
*   musl + runit minimal base philosophy

***

## Intel Clear Linux

**Concepts adopted:**

*   CPU-topology-aware thread affinity (NUMA-aware scheduling) → `src/distro/linux_ideas.rs::CpuTopology`
*   Auto-vectorization friendly data structures → `src/kernel/performance.rs`
*   stateless `/usr` merge with immutable base

***

## SteamOS (Valve)

**Concepts adopted:**

*   Dual A/B rootfs atomic updates → `src/distro/linux_ideas.rs::AtomicUpdateManager`
*   Gamescope compositor concepts → `shards/third_party/326_gamescope_perf_compositor.js`
*   GPU thermal management → `src/drivers/`, `src/driver/`
*   Self-healing GPU recovery → `src/driver/device.rs`

***

## Qubes OS

**Concepts adopted:**

*   VM-based domain isolation (AppVM / TemplateVM model) → `src/security/qubes_isolation.rs`
*   Xen hypervisor concepts for isolation → `src/virtualization/`
*   Policy-based inter-VM communication

***

## Parrot OS / Kali Linux

**Concepts adopted:**

*   AnonSurf anonymization layer → `src/security/parrot_parity.rs`
*   Penetration testing toolkit integration → `src/compatibility/penetration_assistant.rs`
*   Forensics tools integration → `src/security/forensics.rs`

***

## Mint Linux

**Concepts adopted:**

*   Update Manager stability tiers (Mint Update Level 1-5) → `src/compatibility/mint_linux.rs`
*   Cinnamon desktop concepts → `src/compatibility/`

***

## Other Distros

| Distro | Concept | Implementation |
|---|---|---|
| CachyOS | Kernel scheduler tuning (BORE/EEVDF) | `src/compatibility/cachy_os.rs` |
| Garuda Linux | Zen kernel + Dr460nized theming | `src/compatibility/garuda_zen.rs` |
| EndeavourOS | Welcome wizard / community layer | `src/compatibility/endeavour.rs` |
| Bodhi Linux | Moksha/Enlightenment desktop canvas | `src/compatibility/bodhi_moksha.rs` |
| ReactOS | Win32 API compatibility layer | `src/compatibility/reactos.rs` |
| Chakra Linux | Half-rolling model | `src/compatibility/chakra.rs` |
| Chimera Linux | LLVM/musl clean-room base | `src/compatibility/chimera_linux.rs` |
| FreeDOS | DOS compatibility layer | `src/compatibility/freedos.rs` |
| TempleOS | Bare-metal simplicity | `src/compatibility/templeos.rs` |

***

## Native Implementations (Reduced Library Dependency)

All implementations follow the SigmaOS zero-dependency philosophy:

| Standard Library Feature | Native SigmaOS Replacement |
|---|---|
| `std::collections::HashMap` | `src/klib/hashmap.rs` |
| `std::collections::BTreeMap` | `src/klib/btreemap.rs` |
| `std::collections::HashSet` | `src/klib/hashset.rs` |
| `std::collections::VecDeque` | `src/klib/vecdeque.rs` |
| `std::vec::Vec` | `src/klib/vec.rs` |
| `std::string::String` | `src/klib/` |
| `std::alloc` (buddy allocator) | `src/klib/buddy_allocator.rs` |
| `std::alloc` (slab allocator) | `src/kernel/slab_allocator.rs` |
| `openssl` / TLS | Native TLS in `src/net/` |
| `libc` memory functions | `src/klib/`, `src/kernel/` |
| `regex` crate | Native pattern matching in shell |
| `serde` serialization | Native serializers per format |
| `rand` crate | Entropy mixing in `src/kernel/crypto/` |

# Changelog

See also: [CHANGELOG.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CHANGELOG.md) in the repository.

---

## v0.1.0 — Sovereign Foundation (2026-09-02)

### 🆕 New: Kernel Core
- BuddyAllocator physical page allocator (binary buddy system)
- SlabAllocator per-CPU object cache
- 4-level paging with W^X enforcement (x86_64)
- Hybrid CFS + EDF CPU scheduler
- NUMA-aware memory allocation
- Optimised Vec with bulk-copy (`extend_from_slice` improvement)
- Optimised SigmaString with trim allocation improvement

### 🆕 New: Kernel Library (klib)
- Custom HashMap (FNV-based, no std)
- Custom HashSet
- Custom BTreeMap
- Custom Vec (bulk-copy optimised)
- Custom String (trim-optimised)
- `sigma_string_utils` — zero-alloc byte-slice string utils
- Async runtime (no_std executor)
- Merkle tree, JSON parser, TOML parser
- UUID, Base64, PRNG

### 🆕 New: Security
- OpenBSD pledge/unveil process restriction
- FreeBSD Capsicum capability-mode sandboxing
- FreeBSD Jails with nested hierarchies
- SELinux type-enforcement MAC
- KASLR + KARL kernel address randomisation
- W^X memory policy enforcement
- SMEP/SMAP hardware enforcement
- Post-quantum cryptography (CRYSTALS-Kyber)
- Retguard return-address canaries
- AI anomaly detection

### 🆕 New: Package Manager (sigpkg)
- Universal multi-format package adapter (arch, deb, rpm, apk, ebuild, nix, bsd)
- SAT-based dependency solver
- Content-addressed package store
- Atomic transactions with instant rollback
- AUR compatibility bridge
- PKGBUILD recipe parser

### 🆕 New: Distro Parity
- `arch_inspirations.rs` — rolling release, AUR, PKGBUILD
- `nixos_inspirations.rs` — declarative config, generations, store
- `gentoo_inspirations.rs` — USE flags, Portage, ebuilds
- Fedora: Cockpit, PipeWire, FreeIPA, Anitya, FedoraTahrir
- CachyOS: BORE scheduler, LLVM PGO/BOLT, x86-64-v3
- Alpine, Debian, Linux Mint, Garuda, openSUSE, Void Linux parity

### 🆕 New: Desktop
- Zenith Compositor (direct framebuffer, no Wayland/X11)
- HiDPI fractional scaling
- VRR (Variable Refresh Rate)
- Sway/i3 tiling window manager
- MATE Betsy desktop

### 🆕 New: CI/CD
- GitHub Actions: Arch, FreeBSD, OpenBSD, Fedora workflows
- Automated weekly metrics
- Branch name validator
- Codacy static analysis

### 🆕 New: Documentation
- ARCHITECTURE.md
- ROADMAP.md
- SECURITY.md
- CHANGELOG.md
- INSTALL.md
- docs/KERNEL.md
- docs/PACKAGE_MANAGER.md
- docs/DISTRO_COMPAT.md

### 🔀 Merged Branches
- `bolt/vec-string-bulk-copy-opt` — Vec + String performance
- `feat/universal-package-oop-extensions` — Fedora messaging bridge
- `fix/linux-bsd-distro-improvements` — Fedora Planet, Tahrir
- `fix/open-source-parity-and-tests` — Cockpit, PipeWire, FreeIPA
- `impl/wiki-md-features-sync` — Fedora New Hotness monitor
- `jules-1368290922701548926` — Fedora status monitoring
- `jules-666776792259392766` — Anitya release monitor, CachyOS
- `master-1423622165343233187` — CI workflows, diagnostics guide

---

## Earlier History

For complete commit history, see:
```
git log --oneline
```

Or browse [GitHub commits](https://github.com/AaryanSinghChauhan09/SigmaOS/commits/main).

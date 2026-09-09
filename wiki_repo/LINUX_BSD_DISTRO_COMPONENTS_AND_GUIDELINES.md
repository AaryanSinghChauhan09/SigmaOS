# Linux & BSD Distribution Inspirations: System Components & Engineering Guidelines

## Overview
This wiki guide details Linux and BSD distribution-inspired components, architecture maps, and engineering guidelines for AI coding agents and developers working on SigmaOS. It covers cross-subsystem interoperability, foreign package adapters, container zones, security capability models, and zero-dependency `klib` guidelines.

## Distro Interoperability Matrix
- **Arch Linux**: Pacman PKGBUILD parsing, makepkg clean chroot builds (`ApkChrootBuildSandboxEngine`).
- **Debian / Fedora / Gentoo / Alpine / Void**: Control/SPEC/ebuild/APKINDEX/XBPS manifest parsing and dependency canonicalization (`UniversalDependencyMapper`).
- **FreeBSD / OpenBSD / NetBSD**: UCL manifests, Capsicum descriptor rights, Soft Updates (`BsdSoftUpdatesEngine`), pledge/unveil sandboxing, rump kernels.
- **Solaris / Illumos**: Zones container isolation (`SovereignZonesManager`) with proportional CPU share math (`calculate_cpu_percentage`) and VNIC networking.
- **Haiku OS**: `.hpkg` packagefs manifest parsing (`HaikuHpkgManifest`).

## Cross-Subsystem Bridge (`src/distro/linux_bsd_inspirations.rs`)
```rust
let mut bridge = SovereignUniversalDistroBridge::new();
bridge.dispatch_cross_subsystem_operation(
    LinuxBsdDistroMode::FreeBsd,
    "security",
    "capsicum_rights_limit",
)?;
```

## Related Documents
- `docs/LINUX_BSD_DISTRO_COMPONENTS_AND_GUIDELINES.md`
- `docs/MASTER_LINUX_BSD_GAP_CLOSURE_STRATEGIC_PLAN.md`
- `wiki/Home.md`

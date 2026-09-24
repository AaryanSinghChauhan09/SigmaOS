# 📦 SigmaOS Universal Package Manager (`sigpkg`) Strategic Development Plan

## Executive Summary & Design Vision

The **SigmaOS Universal Package Manager** (`sigpkg` / `UniversalPackageManager`) is engineered as a zero-dependency, Safe-Rust package management infrastructure capable of seamlessly ingesting, converting, resolving, and installing software packages across all major Linux and BSD package formats (`.deb`, `.rpm`, `.apk`, `.xbps`, `.ebuild`, `.pkg.tar.zst`, `.pkg`, `.tgz`, `.flatpak`, `.snap`, AppImage, `.nixpkg`).

Rather than forcing users to manage separate package tools, `sigpkg` provides a single, unified, content-addressed, atomic, and rollback-safe user experience drawing direct architectural inspiration from the best open-source package management systems.

---

## 1. Multi-Distro Architectural Inspirations

### 1.1 Debian/Ubuntu (APT / Dpkg)
- **Inspirations**: Control metadata (`Priority`, `Depends`, `Provides`, `Conflicts`, `Recommends`), file diversion rules (`dpkg-divert`), statoverride permissions, debconf pre-seeding, and `apt-mark` hold/auto/manual state tracking.
- **SigmaOS Integration**: `AptDebManifest` parser, `DpkgDivertEngine`, `DebianDebconfStatoverrideEngine`, and `DebianAptMarkPackageStateGovernor`.

### 1.2 Arch Linux (Pacman / ALPM / AUR)
- **Inspirations**: ALPM sync databases, PKGBUILD recipe execution, AUR RPC client metadata, pacman-key Web of Trust keyring, split packages, path-triggered hooks, and `pacdiff` config merge governor.
- **SigmaOS Integration**: `ArchPacmanDatabaseSyncEngine`, `ArchAurRpcClient`, `ArchPacmanKeyringManager`, `ArchSplitPackageHookRunnerEngine`, and `PacdiffConfigMergeGovernorEngine`.

### 1.3 Fedora / RHEL (DNF5 / RPM / OSTree)
- **Inspirations**: RPM CPIO archive extraction, DNF5 advisory security filtering, binary Delta RPM reconstitution, comps group install solver, GPG key rotation, and `rpm-ostree` layered deployment trees.
- **SigmaOS Integration**: `FedoraDnf5PackageEngine`, `DeltaRpmReconstitutionEngine`, `RpmOstreeLayeredImageGovernorEngine`, and `FedoraBootupdBootloaderEngine`.

### 1.4 Alpine Linux (APK v3 / LBU)
- **Inspirations**: APK v3 index signature verification, declarative `/etc/apk/world` rules, ephemeral virtual build dependency groups (`.build-deps`), LBU RAM overlay commits (`apkovl`), and LAN P2P cache discovery.
- **SigmaOS Integration**: `ApkV3SignatureEngine`, `AlpineApkWorldAndVirtualPkgEngine`, `AlpineLbuRamOverlayCommitEngine`, and `AlpineApkCachePeerSyncEngine`.

### 1.5 Void Linux (XBPS / xbps-src)
- **Inspirations**: Fast C-like binary metadata, shared library ELF SONAME dependency tracking, orphan package resolution, local downgrade repositories, package holds, and `xbps-src` template build sandboxes.
- **SigmaOS Integration**: `XbpsSonameAndOrphanEngine`, `XbpsDowngradeRepoEngine`, `XbpsRestrictedNonFreeLicenseEngine`, `XbpsSrcTemplateSandboxEngine`, and `XbpsDebianAlternativesGovernorEngine`.

### 1.6 Gentoo Linux (Portage / ebuild / EAPI)
- **Inspirations**: Conditional `USE` flags, USE_EXPAND variables, EAPI feature level enforcement (7/8/9), subslot ABI rebuild solver (`:=`), `package.env` per-package compiler flags, and `ACCEPT_LICENSE` governors.
- **SigmaOS Integration**: `GentooUseFlagsManager`, `GentooPortageSubslotAndUseExpandEngine`, `GentooPortageEapiSlotOperatorEngine`, `PortagePackageEnvEngine`, and `PortagePackageLicenseGovernorEngine`.

### 1.7 NixOS & GNU Guix (CAS / Flakes / Store)
- **Inspirations**: Content-Addressed Storage (CAS) store paths (`/nix/store/hash-name`), hermetic store closure size calculator, Flakes lockfile reproducibility, store hardlink deduplication, profile generations, and GC policy scheduler.
- **SigmaOS Integration**: `NixStyleStore`, `HermeticStoreClosureEngine`, `NixFlakesDevshellResolverEngine`, `NixGuixStoreDeduplicatorEngine`, and `NixGuixGcPolicySchedulerEngine`.

### 1.8 FreeBSD (Ports / pkg / Poudriere)
- **Inspirations**: Ports Flavours (`@py311`, `@default`), VuXML package vulnerability database auditing, `pkg-audit` CVSS risk score blocking, `pkg-message` post-install notifications, and Poudriere clean-jail parallel build matrix.
- **SigmaOS Integration**: `FreeBsdPortsFlavoursAndVuxmlEngine`, `FreeBsdPkgAuditEngine`, `FreeBsdPkgMessageNotifierEngine`, and `FreeBsdPoudriereMatrixEngine`.

### 1.9 OpenBSD (pkg_add / signify / pledge)
- **Inspirations**: Signify cryptographic key verification, `PKG_PATH` mirror failover, and OpenBSD `pledge`/`unveil` scriptlet sandboxing rules (`stdio`, `rpath`, `wpath`).
- **SigmaOS Integration**: `OpenBsdPkgAddSignifyEngine`, `OpenBsdSignifyBinaryIntegrityEngine`, and `OpenBsdPledgeUnveilSandboxScriptletEngine`.

---

## 2. Core Architectural Subsystems of `sigpkg`

```text
┌────────────────────────────────────────────────----------------───────────┐
│                        SigmaOS CLI & Zenith UI                            │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                PackageFormat & UniversalPackageAdapter                    │
│      (Detects & ingests .deb, .rpm, .apk, .xbps, .ebuild, .pkg.tar.zst)     │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 TUF & PQC Signature Verification Engine                   │
│         (Threshold signing, key rotation, Dilithium5/Signify keys)        │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                    SAT Dependency Solver & Resolution                     │
│          (Kahn's topo-sort, Gentoo USE flags, FreeBSD Flavours)          │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Content-Addressed Storage (CAS) Store & Hardlinks           │
│        (SHA-256 deduplication, hermetic store closures, GC scheduler)     │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                Sandboxed Scriptlet & Trigger Execution                     │
│              (OpenBSD pledge/unveil & ALPM path hook runner)              │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             Transactional Journal & Pre-Flight Snapshot Rollback          │
│            (Btrfs CoW, ZFS BootEnv, A/B root deployment layers)          │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Phased Package Manager Development Roadmap

### Phase 1: Native `.sigpkg` Format & Foreign Ingestion (Q4 2026)
- Stabilize native signed `.sigpkg` archive format (`src/sigpkg/`).
- Complete `PackageFormat::from_filename` detection and ingestion adapters for `.deb`, `.rpm`, `.apk`, `.xbps`, `.ebuild`, and `.pkg.tar.zst`.
- Implement Kahn's topological sort dependency solver (`ArchDependencyResolver`).

### Phase 2: TUF Security & Sandboxed Maintainer Scriptlets (Q1 2027)
- Implement TUF (The Update Framework) delegated repository roles, root key rotation, and rollback attack prevention.
- Enforce OpenBSD `pledge`/`unveil` sandboxing for maintainer install scriptlets.
- Integrate FreeBSD VuXML & CVE vulnerability database auditing (`FreeBsdPkgAuditEngine`).

### Phase 3: CAS Deduplication & Transactional Rollback (Q2 2027)
- Deploy Content-Addressed Storage (CAS) store deduplication and GC profile generations.
- Integrate pre-flight Btrfs CoW and ZFS BootEnv snapshots before transaction commits.
- Add `sigma-pkg snapshot create` and `sigma-pkg rollback <snapshot>` CLI commands.

### Phase 4: P2P LAN Mirror & Developer Recipe SDK (Q3 2027+)
- Implement local LAN P2P cache discovery and artifact sharing (`AlpineApkCachePeerSyncEngine`).
- Publish developer recipe SDK (`sigpkg-build`) with clean chroot build support (`ArchVoidCleanChrootBuildEngine`).
- Deploy reproducible build SLSA Level 4 attestation generator (`SovereignPackageBuildProvenanceEngine`).

---

## 4. Verification & Testing Standards

All package manager components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/sigpkg/universal_engine.rs`
- `src/sigpkg/universal_adapter.rs`
- `src/package/universal_package_innovations_suite.rs`
- `src/package/bsd_linux_package_innovations.rs`

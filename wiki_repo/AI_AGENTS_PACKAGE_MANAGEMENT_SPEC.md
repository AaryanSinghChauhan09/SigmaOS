# AI Agents Package Management Specification for SigmaOS

## Abstract
SigmaOS provides a zero-dependency, universal package management architecture designed for autonomous and developer-assisted AI coding agents. This specification defines how AI agents query, build, translate, sandbox, install, and rollback packages across foreign formats (DEB, RPM, Pacman, APK, XBPS, Ebuild, Nix, Flatpak, Snap, AppImage) into native `SigmaPkg` format.

---

## 1. Core Architecture & Interoperability

AI agents interact with the package manager via `UniversalPackageManager`, `UniversalPackageTranslator`, and `SovereignPackageSnapshotRollbackEngine`:

```
[ AI Agent Task / CLI Request ]
              │
              ▼
[ UniversalPackageManager::convert_to_sigpkg ]
              │
  ┌───────────┴───────────┐
  ▼                       ▼
[ UniversalDependencyMapper ] [ UniversalScriptletConverter ]
  │                       │
  └───────────┬───────────┘
              ▼
[ Isolated Store Path: /sovereign/store/<pkg>-<ver>-<hash> ]
              │
              ▼ (Pre-flight Btrfs/ZFS Snapshot Trigger)
[ PackageSnapshotRollbackEngine::create_snapshot ]
```

---

## 2. Agent Operational Workflows

### 2.1 Package Format Auto-Detection & Translation
1. **Magic Header Detection**: `PackageFormatDetector` identifies incoming binary packages or source recipes (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.ebuild`, `.nixpkg`, `.recipe`).
2. **Canonical Dependency Mapping**: `UniversalDependencyMapper` translates distro-specific library names (e.g. `libssl-dev` in Debian, `openssl-devel` in Fedora, `openssl` in Arch) to canonical SigmaOS identifiers (`sovereign-openssl`).
3. **Scriptlet Translation**: Foreign install scriptlets (`preinst`, `postinst`, `prerm`, `postrm`) are safely translated into zero-dependency Sigma lifecycle hooks (`PreInstall`, `PostInstall`, `PreRemove`, `PostRemove`).

### 2.2 Dry-Run Simulation & Pre-Flight Verification
- AI agents MUST execute a dry-run transaction using `UniversalDryRunSimulator` prior to system-wide package changes.
- Simulations verify dependency DAG completeness, detect file conflict overlaps, calculate storage deltas, and ensure no circular dependency loops exist.

### 2.3 Pre-Flight Snapshots & Rollback Protection
- Every transaction automatically triggers `SovereignPackageSnapshotRollbackEngine::create_snapshot`.
- If an agent-driven build or installation fails validation tests, `SovereignPackageSnapshotRollbackEngine::rollback_generation` executes an atomic sub-millisecond state restoration.

---

## 3. Sandboxing & Build Isolation

1. **Clean-Room Build Sandbox**:
   - Package compilations (`PkgBuildChrootRunner` / `ArchRecipeSandboxCompiler`) execute inside isolated unprivileged container shards (`SigmaContainer`) with read-only rootfs mounts.
2. **Syscall Restrictions**:
   - Compilers operate under `pledge("stdio rpath wpath cpath inet")` and Landlock LSM path restrictions to prevent host pollution.
3. **Reproducible Build Manifests**:
   - `ReproduciblePackageBuilder` enforces deterministic builds by locking `SOURCE_DATE_EPOCH`, sanitizing build environments (`LC_ALL=C`, `TZ=UTC`), and recording SHA-256 build provenance.

---

## 4. Security Auditing & CVE Tracking

- **Advisory Tracking**: AI agents query `SecurityAdvisoryTracker` to check package versions against vulnerability databases before installation.
- **Dilithium-5 / Ed25519 Signatures**: Binary packages require Dilithium-5 post-quantum signatures or GPG trust signatures validated by `ArchLinuxPgpKeyringEngine` / `DkmsAbiRebuildEngine`.

---

## 5. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_PACKAGE_MANAGEMENT_SPEC.md`
- `wiki/AI_AGENTS_PACKAGE_MANAGEMENT_SPEC.md`
- `wiki_repo/AI_AGENTS_PACKAGE_MANAGEMENT_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Package Management Architecture*

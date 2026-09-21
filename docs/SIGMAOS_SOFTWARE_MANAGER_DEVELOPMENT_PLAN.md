# SigmaOS Software Manager & Universal Package Center - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-Software` (`sigpkg-gui`) is the unified graphical software center, universal package manager, and repository management suite for **SigmaOS**. Combining the multi-format adaptation of APT, Pacman, DNF, APK, XBPS, and FreeBSD Pkg, the AppStream visual richness of GNOME Software and KDE Discover, the DPLL SAT dependency solver of Fedora DNF5 and Gentoo Portage, and the cryptographic verification of OpenBSD `signify` and SLSA-Level 4 build provenance attestations, `Sigma-Software` provides an effortless One-Click installation experience for end-users and a robust CLI for power sysadmins.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Package Management Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Arch Linux ALPM & AUR** | Parallel mirror ranking, AUR v5 RPC search, `makepkg` sandboxed PKGBUILD compilation, `namcap` package linter. | `src/sigpkg/` & `src/package/` |
| **Debian APT & `dpkg`** | APT control manifest parsing, `debconf` preseed automated configuration, `dpkg-divert` file diversion, `apt-mark` hold/manual/auto state governor. | `src/package/debian_apt.rs` |
| **Fedora DNF5 & RPM** | DNF5 transaction history & journal rollback, DeltaRPM patch reconstitution, `comps` package group solver, COPR build gateway. | `src/compatibility/fedora.rs` |
| **Alpine APK v3 & Void XBPS** | Alpine declarative `/etc/apk/world` file, Void XBPS SONAME library dependency tracking, orphan package resolution, `xbps-src` restricted non-free licensing. | `src/package/bsd_linux_package_innovations.rs` |
| **FreeBSD Ports & OpenBSD `pkg_add`** | FreeBSD VuXML & `pkg-audit` vulnerability scanner, OpenBSD `signify` binary signature verification, `PKG_PATH` mirror resolver. | `src/package/bsd_linux_package_innovations.rs` |
| **GNOME Software & KDE Discover** | AppStream XML metadata parsing, application screenshots, user reviews, category taxonomy, Flatpak/Snap/AppImage container integration. | `src/desktop/appstore.rs` |

---

## 3. 5-Layer Software Manager Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Sovereign Security, SLSA Provenance & OpenBSD Signify Verifier│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: DPLL SAT Dependency Solver, Subslot ABI & Rollback Journal    │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Graphical Application Store UI (GNOME Software/Discover Style)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Parallel Repository Sync, Mirror Ranker & DeltaRPM Engine     │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Universal Package Format Adapter Engine (Deb, RPM, Pacman...) │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Universal Package Format Adapter Engine
- **Multi-Format Format Detection:** Automatic filename and header magic detection for `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.pkg`, `.hpkg`, `.eopkg`, `.flatpak`, `.snap`, `.AppImage`, and native `.sigpkg`.
- **Metadata Translation:** Normalizing package manifests (name, version, dependencies, architecture, maintainer, license, description) into `UnifiedPackage` objects.

### Layer 2: Parallel Repository Sync & DeltaRPM Engine
- **Parallel Mirror Sync:** Multi-stream HTTP/HTTPS parallel mirror ranker (`netselect-apt` / `reflector` inspired).
- **DeltaRPM & xdelta Reconstitution:** Downloading bandwidth-efficient binary patch deltas and reconstructing full packages locally.

### Layer 3: Graphical Application Store UI
- **AppStream Rich Metadata:** Displays app icons, high-resolution screenshots, release notes, categories (*Development*, *Games*, *Graphics*, *Productivity*, *System*), and user ratings.
- **One-Click Transactions:** Asynchronous background package downloading, progress bar, and desktop notifications upon installation completion.

### Layer 4: DPLL SAT Dependency Solver & Rollback Journal
- **SAT Solver:** DPLL-based dependency resolution supporting boolean OR dependencies, version constraints (`>=`, `<=`, `=`), and conflict resolution.
- **Portage Subslot ABI Evaluation:** Detecting library SONAME changes and automatically queuing dependent packages for ABI rebuilds.
- **DNF-Style Transaction History:** Journaling every install/upgrade/remove operation with point-in-time single-command rollbacks.

### Layer 5: Sovereign Security, SLSA Provenance & Signify Verifier
- **Cryptographic Signature Verification:** Enforcing GPG and OpenBSD `signify` dual signatures on repository indexes and binary archives.
- **SLSA-Level 4 Attestation:** Verifying build provenance manifests (git commit hash, build flags, source date epoch).
- **Scriptlet Sandboxing:** Executing maintainer installation scriptlets (`preinst`, `postinst`) inside Landlock and OpenBSD `pledge`/`unveil` sandboxes.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Universal Adapter | Implement metadata adapters for `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.pkg`, and `.sigpkg`. | Implemented |
| **Milestone 2** | Repo Sync & Deltas | Implement parallel mirror sync, ALPM/APT index parser, and DeltaRPM reconstitution engine. | Implemented |
| **Milestone 3** | Graphical App Store | Build AppStream XML parser, category catalog, screenshot viewer, and One-Click install UI in `src/desktop/`. | Implemented |
| **Milestone 4** | Dependency Solver | Implement DPLL SAT solver, Portage subslot ABI rebuild evaluator, and DNF transaction rollback journal. | Implemented |
| **Milestone 5** | Security & Sandboxing | Enforce GPG/Signify signature verification, SLSA-Level 4 provenance check, and scriptlet sandboxing. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/package/universal.rs`, `src/sigpkg/universal_adapter.rs`, and `src/package/bsd_linux_package_innovations.rs`.
2. **Multi-Format Installation Tests:** Verifying `.deb`, `.rpm`, `.pkg.tar.zst`, and `.apk` packages install and translate cleanly.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.

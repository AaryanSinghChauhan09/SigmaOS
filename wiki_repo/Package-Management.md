# 📦 SigPkg: Sovereign Package Management Subsystem

**SigPkg** is SigmaOS's zero-dependency, zero-allocation-ready, safe Rust package management engine supporting native `.sigpkg` archives as well as universal cross-distro package absorption.

---

## 1. Universal Cross-Distro Adapter (`src/sigpkg/universal_adapter.rs`)

SigPkg automatically ingests, translates, and sandboxes foreign Linux and BSD package formats:

| Package Format | Target Origin | Extracted Metadata & Sandbox Mapping |
|---|---|---|
| **Apt (`.deb`)** | Debian, Ubuntu, Parrot, Mint | Control header parsing, dependency mapping, system essential priority handling |
| **Yum/Dnf (`.rpm` / `.spec`)** | Fedora, RHEL, openSUSE | Spec file parsing, `%pre` / `%post` scriptlet hooks, libdnf5 solver matrix |
| **Pacman (`PKGBUILD`)** | Arch Linux, Manjaro, EndeavourOS | PKGBUILD script compilation, `provides` / `conflicts` resolution |
| **Apk (`.apk`)** | Alpine Linux | APKINDEX binary parsing, musl C library dependency tracking |
| **Xbps (`.xbps`)** | Void Linux | XBPS CAS metadata extraction, xbps-src template triggers |
| **Snap (`snapcraft.yaml`)** | Ubuntu / Canonical | Plugs & slots translation into SigmaOS Capability Permissions |
| **Flatpak (`.json`)** | Fedora / GNOME | Finish-args sandboxing (`--share=network`, `--share=ipc`) |

---

## 2. ALPM Transaction & Hook Engine (`src/sigpkg/arch_compat.rs`)

* **Transaction Lifecycle:** Atomically executes transactions via `AlpmTransactionEngine` (`Init` -> `Prepared` -> `Committed` -> `RolledBack`).
* **ALPM Hooks (`AlpmHookManager`):** Parses `/etc/pacman.d/hooks/*.hook` files, matching pre-transaction and post-transaction triggers (e.g. `update-desktop-database`, `mkinitcpio`).
* **Sync Database Parser (`AlpmDatabaseSync`):** Parses `.db.tar.gz` sync databases and ranks mirror download latency (`rankmirrors`).
* **Conflict Solver (`AlpmConflictSolver`):** SAT-solver conflict detection for package overlaps (`conflicts=()`) and virtual provisions (`provides=()`).

---

## 3. Package Snapshot & Point-in-Time Rollback (`src/sigpkg/package_snapshot_rollback.rs`)

* **Transaction Journaling:** Every install, update, or removal records a `PackageTransactionJournal` entry with pre-state and post-state checksums.
* **Point-in-Time Rollback:** `SovereignPackageSnapshotRollbackEngine` restores previous filesystem package states automatically upon transaction failure or user command.

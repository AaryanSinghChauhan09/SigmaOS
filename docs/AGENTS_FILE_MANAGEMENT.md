# AI Agent File Management Specification for SigmaOS

This document provides a detailed specification for AI agents managing files, Virtual File System (VFS) paths, and package manifests across the **SigmaOS** operating system.

---

## 1. Virtual File System (VFS) Path Translation Matrix

SigmaOS implements a unified Virtual File System router capable of translating generic Linux/BSD system paths to distro-specific locations depending on active subsystem modes:

| Distro Mode | Generic Path | Translated Path | Description |
| :--- | :--- | :--- | :--- |
| `LinuxNix` | `/etc` | `/etc/nixos` | NixOS declarative configuration directory |
| `LinuxGuix` | `/etc` | `/etc/config.scm` | GNU Guix Scheme configuration entry |
| `LinuxNix` / `LinuxGuix` | `/var/lib/pkg` | `/nix/store` | Content-Addressed Store |
| `LinuxArch` | `/var/lib/pkg` | `/var/lib/pacman` | Pacman package database |
| `LinuxDebian` / `LinuxPopOs` | `/var/lib/pkg` | `/var/lib/dpkg` | Dpkg package database |
| `LinuxAlpine` | `/var/lib/pkg` | `/lib/apk/db` | Alpine APK database |
| `LinuxVoid` | `/var/lib/pkg` | `/var/db/xbps` | Void XBPS database |
| `FreeBsd` / `OpenBsd` / `NetBsd` | `/var/lib/pkg` | `/var/db/pkg` | BSD package database |
| BSDs (`FreeBsd`, `OpenBsd`, `NetBsd`) | `/etc` | `/usr/local/etc` | Local userland configuration path |
| `LinuxClear` | `/etc` | `/usr/etc` | Clear Linux stateless factory defaults |

AI agents modifying path resolution logic must update `translate_vfs_path` in `src/distro/linux_bsd_inspirations.rs`.

---

## 2. File Modification & Safety Rules for AI Agents

1. **Transactional File Modifications**:
   - For transactional or stateful updates (e.g. conffiles, system generations), AI agents must ensure 3-way merge or snapshot backup before overwriting.
   - Use `ConffileMergeEngine` in `src/sigpkg/universal_oop_system.rs` for 3-way conffile reconciliation.

2. **Atomic File Writes**:
   - Write new content to a temporary location or buffer, verify integrity, and atomically swap/rename to destination path.

3. **Sandboxed & Isolated Paths**:
   - Honor OpenBSD `unveil` restrictions and Linux Landlock LSM path rules when handling file system access in userland tools.
   - Restrict path escalation beyond registered jail/chroot boundaries (`FreeBSDJail`, `ApkChrootBuildSandboxEngine`).

---

## 3. Package Manifest File Formats

When adding support for new package file types in `src/sigpkg/universal_adapter.rs` or `src/sigpkg/universal_oop_system.rs`:

1. Update `PackageFormat` enum.
2. Update extension auto-detection in `PackageFormat::from_filename` and `UniversalPackageAdapter::detect_format_by_extension`.
3. Add magic byte header detection in `UniversalPackageAdapter::detect_format_by_header`.
4. Register concrete adapter implementations in `PackageParserFactory::new()`.

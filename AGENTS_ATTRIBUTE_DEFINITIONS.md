# SigmaOS AI Agent Attribute Definition Table Management Directive (`AGENTS_ATTRIBUTE_DEFINITIONS.md`)

This document defines management guidelines and operational rules for AI autonomous engineering agents managing attribute definition tables across SigmaOS.

---

## 1. Overview of Attribute Definition Systems in SigmaOS

Attribute definition tables in SigmaOS provide structured metadata representation across kernel, filesystem, and package management subsystems:

1. **Linux Sysfs Dynamic Hardware Attributes (`SysfsAttribute` in `src/process/linux_sysfs.rs`):**
   - Represents dynamic kernel and hardware state (battery capacity, CPU core online status, power states).
   - Enforces strict read-only vs. read-write permission gating (`writable` boolean flag).

2. **POSIX PAX Tar Extended Attributes (`PaxTarHeader` in `src/tools/archive.rs`):**
   - Extended POSIX tar header attributes capturing nanosecond `mtime`, UID/GID owner strings, and extended xattr key-value pairs for Arch Linux `.pkg.tar.zst` and Debian `.deb` archives.

3. **SVN & VCS Extended Xattrs (`SvnXattrProperties` in `src/sigpkg/svntogit_repro.rs`):**
   - Extended xattr property mapping for SVN-to-Git migration (`svn:mime-type`, `svn:ignore`, `svn:keywords`).

4. **Landlock v5 Access Attributes (`SovereignLandlockV5Guard` in `src/distro/sovereign_nextgen_distro_leap.rs`):**
   - Access control attribute vectors defining file path read/write/exec permissions and TCP socket bind/connect restrictions.

---

## 2. AI Agent Attribute Management Rules

1. **Permission Immutability Gating:**
   - Read-only attributes (`writable = false`) must never be modified directly without explicit permission elevation.
   - Attempted writes to read-only attributes must return `Err("Permission denied: sysfs attribute is read-only")`.

2. **Zero External Dependencies:**
   - Attribute definition tables must be stored using native `#![no_std]` compliant structures (`klib` / `alloc::collections::BTreeMap` / `alloc::collections::HashMap`).

3. **Verification Checklist:**
   - Run `./run_sigma_tests.sh` to ensure all sysfs, archive, and security attribute tests pass.

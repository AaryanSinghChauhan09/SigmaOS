# AI Agent Boot Configuration & System Databases Management Guidelines

## 1. Overview & Architecture
This document specifies AI agent guidelines for managing POSIX `/etc` system databases (`/etc/passwd`, `/etc/group`, `/etc/fstab`, `/etc/hosts`), declarative configuration stores, and administrative system databases in SigmaOS (`src/system/config.rs`).

---

## 2. Operational Directives for AI Agents

### 2.1 POSIX System Databases
- **`/etc/passwd` & `/etc/group` Handling**: AI agents extending user management must parse standard colon-separated `/etc/passwd` (username, password placeholder, UID, GID, GECOS, home, shell) and `/etc/group` entries safely.
- **`/etc/fstab` Filesystem Table**: Support parsing mount options (`defaults`, `ro`, `rw`, `noatime`, `subvol`), device UUIDs, and mount points for automated VFS mount processing.

### 2.2 Declarative Configuration Synchronization
- **Atomic File Updates**: Modifications to system configuration databases must be written to temporary files and atomically renamed (`renameat2`) to prevent partial file corruption on power failure.
- **Validation Before Commit**: All generated configuration files must pass strict syntax validation parsers before committing changes to `/etc`.

---

## 3. Related Files
- `src/system/config.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`

# 🔗 SigmaOS Symbolic Links (`symlink`) Strategic Development Plan

## Executive Summary & Design Vision

Symbolic links (symlinks) are fundamental POSIX file system abstractions allowing files and directories to reference other paths across filesystems. In modern operating systems, symlinks are not merely file aliases—they serve as critical infrastructure for atomic software updates, profile switching, device node creation (`/dev`), dynamic binary alternative management, security boundary isolation, and system statelessness.

This strategic plan establishes the architectural design, security guards, multi-distro inspirations, core subsystems, phased roadmap, and verification standards for **SigmaOS Symbolic Link Infrastructure**.

---

## 1. Multi-Distro & Multi-OS Symbolic Link Inspirations

### 1.1 Linux VFS & Filesystem Fast Symlinks (ext4, Btrfs, EROFS, tmpfs)
- **Inspirations**: Fast vs. slow symlinks. Targets shorter than inode data payload capacity (e.g., <= 60 bytes in ext4/tmpfs) are stored directly inside the inode (`i_block` payload), avoiding disk block allocation and I/O latency.
- **SigmaOS Integration**: Fast inline symlink storage in `SigmaTmpfs`, `ErofsEngine`, and kernel VFS inode structures.

### 1.2 NixOS & GNU Guix (Atomic Profile & Generation Swapping)
- **Inspirations**: Atomic user profile generations (`/nix/var/nix/profiles/default -> default-42-link -> /nix/store/...`). System rollbacks and profile switches are implemented as single atomic `renameat2` symlink swaps, providing zero-downtime transactional system updates.
- **SigmaOS Integration**: `NixStyleStore`, `AtomicUpdateManager`, and `NixProfileStore` profile generation symlink pointer management in `src/sigpkg/`.

### 1.3 Arch Linux & Fedora (System Alternatives & Diverts)
- **Inspirations**: Debian `update-alternatives`, Arch `pacdiff` / binary overrides, and Fedora `bootupd` symlink chains (`/usr/bin/editor -> /etc/alternatives/editor -> /usr/bin/nvim`).
- **SigmaOS Integration**: `DebianAlternativesGovernorEngine` and `XbpsDebianAlternativesGovernorEngine` symlink alternative tracking in `src/sigpkg/sovereign_package_innovations.rs`.

### 1.4 Linux `devtmpfs` & `udev` / `systemd-udevd`
- **Inspirations**: Dynamic device node symlinking in `/dev` (`/dev/disk/by-uuid/`, `/dev/disk/by-label/`, `/dev/stdout -> /proc/self/fd/1`).
- **SigmaOS Integration**: `DevTmpFs` rule engine supporting `create_symlink` aliases in `src/kernel/fs/devtmpfs.rs`.

### 1.5 OpenBSD & FreeBSD Security Sandboxing (`unveil`, `pledge`, `O_NOFOLLOW`)
- **Inspirations**: Mitigation of Symlink Traversal Attacks (TOCTOU, race conditions, symlink races in `/tmp`). OpenBSD `unveil(2)` path prefix verification resolves canonical symlink targets before checking permissions. POSIX `O_NOFOLLOW` prevents symlink dereferencing when opening system files.
- **SigmaOS Integration**: Safe path resolution in `src/security/input_validation.rs`, `PledgeManager::validate_unveil_access` in `src/security/pledge.rs`, and `O_NOFOLLOW` enforcement in kernel VFS lookup.

### 1.6 macOS & Apple Darwin (Firmlinks & APFS Symlinks)
- **Inspirations**: APFS firmlinks bridging read-only OS volumes (`/System/Volumes/Data`) with writable user data transparently at the file system level.
- **SigmaOS Integration**: `StatelessSystemEngine` overlay symlinks bridging `/etc` and `/var` defaults to `/usr/share/factory/`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Userland System & Shell / CLI                         │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                    POSIX System Calls (`symlink`, `readlink`)             │
│            (`symlinkat`, `readlinkat`, `unlinkat`, `renameat2`)           │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                  VFS Path Resolution & Security Engine                    │
│      - Cycle Detection (max depth = 40 levels, ELOOP error)               │
│      - OpenBSD `unveil` & Landlock path canonicalization                   │
│      - Safe Traversal (`O_NOFOLLOW`, NUL byte check, `..` guard)           │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Kernel Inode Storage (Fast vs. Slow Symlinks)               │
│      - Fast Symlink (target <= 60 bytes, stored in Inode payload)          │
│      - Slow Symlink (target > 60 bytes, stored in dedicated data block)   │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│            Package Manager & System Subsystem Integrations               │
│      - `sigpkg` Atomic Profile Generation Swapping (`/var/sigma/profiles`) │
│      - `DevTmpFs` Dynamic Node Symlink Aliasing (`/dev/disk/by-uuid/`)     │
│      - System Alternatives Governor (`/etc/alternatives/`)                │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Cycle Detection & Depth Guard
- Maximum symlink resolution depth: **40 levels** (matching Linux VFS standards).
- Exceeding depth 40 returns POSIX `ELOOP` ("Too many levels of symbolic links").
- Self-referential loop detection (`a -> b -> a`) aborts immediately.

### 2.2 Security & Path Traversal Guards
- Symlink traversal during `unveil` or Landlock sandbox evaluation converts symlinks to absolute, canonicalized paths prior to permission checks.
- Rejection of NUL-byte injection (`\0`), URL-encoded traversal (`%2e%2e`), and sticky bit `/tmp` symlink exploitation.

### 2.3 Atomic Generation Swapping
- Package managers and system configuration daemons switch active generations using POSIX `renameat2(..., RENAME_EXCHANGE)` or atomic symlink overwrite, guaranteeing that reader threads never see a broken or partially updated symlink.

---

## 3. Phased Development Roadmap

### Phase 1: VFS Kernel POSIX Symlink Engine (Q4 2026)
- Standardize `symlink`, `symlinkat`, `readlink`, and `readlinkat` syscall implementations across all supported architectures.
- Implement Fast Inline Symlinks (<= 60 bytes) in `SigmaTmpfs` and `ErofsEngine`.
- Enforce 40-level recursion limits with `ELOOP` handling in path lookup.

### Phase 2: Security Boundary Integration & Unveil Resolution (Q1 2027)
- Integrate symlink canonical path resolution into OpenBSD `unveil` and Linux Landlock LSM policies (`src/security/pledge.rs`).
- Implement `O_NOFOLLOW` flag handling in file open paths to prevent SUID/privileged symlink target replacement attacks.
- Add `/tmp` and `/var/tmp` symlink restrictions (`sysctl fs.protected_symlinks = 1`).

### Phase 3: Package Manager Atomic Profile Swapping (Q2 2027)
- Enable atomic symlink pointer swaps in `sigpkg` (`UniversalPackageManager`).
- Implement system software alternatives governor (`/etc/alternatives/`).
- Deploy `devtmpfs` dynamic device node symlink aliases (`/dev/disk/by-uuid/`, `/dev/disk/by-label/`).

### Phase 4: Userland Tooling & Benchmarks (Q3 2027+)
- Expose symlink creation and inspection in shell builtins (`ln -s`, `readlink -f`, `realpath`).
- Implement automated benchmark tests measuring symlink traversal overhead in `scripts/tech_media_benchmark_suite.sh`.
- Conduct security audit against known POSIX symlink race condition CVEs.

---

## 4. Verification & Testing Standards

All symbolic link components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/fs/sigma_tmpfs.rs` (symlink creation, readlink, and follow depth recursion)
- `src/fs/sigma_sysfs.rs` (sysfs kobject symlink targets)
- `src/kernel/fs/devtmpfs.rs` (udev rule symlink aliases)
- `src/security/pledge.rs` & `src/security/input_validation.rs` (unveil path traversal & canonicalization)
- `src/sigpkg/universal_oop_system.rs` (atomic profile generation symlink swaps)

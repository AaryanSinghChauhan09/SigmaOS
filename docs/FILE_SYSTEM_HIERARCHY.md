# SigmaOS File Hierarchy System Specification (`Sovereign FHS`)

## 1. Executive Summary

The SigmaOS File Hierarchy System (`Sovereign FHS`) synthesizes proven directory paradigms from Linux FHS 3.0, Fedora/Arch UsrMerge, FreeBSD base isolation (`/usr/local`), NixOS/Guix content-addressed stores (`/sigma/store`), and macOS/GoboLinux human-readable declarative alias links into a unified, secure file system layout.

## 2. Directory Taxonomy & Architecture

```
/ (Root Virtual File System / Sovereign VFS)
├── bin -> usr/bin                      # UsrMerge compat symlink
├── lib -> usr/lib                      # UsrMerge compat symlink
├── lib64 -> usr/lib                    # UsrMerge 64-bit compat symlink
├── sbin -> usr/bin                     # UsrMerge sys-admin compat symlink
│
├── boot/                               # Static bootloader & kernel images
│   ├── EFI/                            # UEFI ESP boot binaries
│   ├── initramfs.img                   # Microkernel initramfs
│   └── sigma-kernel.elf                # Microkernel executable
│
├── dev/                                # Dynamic device nodes (devfs)
│   ├── null, zero, urandom, tty
│   ├── fd -> /proc/self/fd             # Standard file descriptor links
│   └── nvme0n1p1                       # Block storage partitions
│
├── etc/                                # Host-specific configuration files
│   ├── locale.conf                     # System locale settings
│   ├── fstab                           # File system mount matrix
│   └── zenith/                         # Zenith desktop config files
│
├── home/                               # User home directories (@home)
│   └── jules/                          # User profile workspace
│       ├── .config/                    # XDG user configuration
│       └── .local/share/               # XDG user data
│
├── proc/                               # Process & system information (procfs)
├── sys/                                # Kernel object hierarchy (sysfs)
├── run/                                # Volatile runtime state (tmpfs)
│
├── sigma/                              # Sovereign Store & Native Namespaces
│   ├── store/                          # Immutable content-addressable packages
│   │   └── 3f8a9b...-zenith-1.2.0/     # Merkle pinned package store
│   └── system/                         # Generation snapshots (@system)
│
├── usr/                                # Immutable System Software Hierarchy
│   ├── bin/                            # System binary executables
│   ├── lib/                            # Shared libraries & objects
│   ├── share/                          # Architecture-independent data
│   └── local/                          # FreeBSD-style third-party software
│       ├── bin/                        # Non-base user binaries
│       └── etc/                        # Non-base configurations
│
└── var/                                # Variable State Data
    ├── log/                            # System binary journals (journald)
    ├── cache/                          # Package manager download cache
    └── empty/                          # OpenBSD-style secure privilege drop chroot
```

## 3. Integration Paradigms from Linux & BSD

### 3.1 UsrMerge (Fedora / Arch Linux Inspiration)
To simplify library loading and eliminate duplicate binary paths, `/bin`, `/sbin`, `/lib`, and `/lib64` are symbolic links pointing into `/usr`:
- `/bin` -> `/usr/bin`
- `/sbin` -> `/usr/bin`
- `/lib` -> `/usr/lib`
- `/lib64` -> `/usr/lib`

### 3.2 Content-Addressed Store (NixOS / Guix Inspiration)
Packages installed via declarative generation pinning reside in `/sigma/store/<hash>-<name>-<version>/`. Environment profiles dynamically map binary paths into `/usr/bin` or user environments using atomic symlinks.

### 3.3 Base / Local Software Separation (FreeBSD Inspiration)
Core system binaries belong to the base system (`/usr/bin`), whereas user-compiled binaries, ports, or standalone packages reside in `/usr/local/bin` and `/usr/local/etc`, preventing third-party packages from modifying base system files.

### 3.4 Privilege Drop Chroots (OpenBSD Inspiration)
Unprivileged daemons drop permissions into immutable empty directories (`/var/empty` or `/var/chroot/<service>`), isolating daemons from the wider file system.

## 4. Sovereign Namespace Mapping Matrix

For native SigmaOS microkernel IPC and capability isolation, traditional FHS paths map to sovereign `@` namespaces:

| Standard POSIX Path | Sovereign Namespace Path | Description |
|---|---|---|
| `/` | `@root` | VFS Root Mount |
| `/usr` | `@system` | Immutable Core System State |
| `/sigma/store` | `@store` | Merkle Package Store |
| `/home` | `@user` | User Data Partition |
| `/var/log` | `@log` | Centralized Journal Storage |
| `/tmp` & `/run` | `@volatile` | Ephemeral RAM Disks |

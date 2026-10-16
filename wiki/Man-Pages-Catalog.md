# Consolidated Man Pages Catalog

This catalog consolidates Unix/Linux/BSD manual pages across all 9 standard POSIX/BSD sections for SigmaOS.

## Registered Manual Pages

| Title | Section | Origin / Inspiration | Synopsis |
| :--- | :--- | :--- | :--- |
| **`bectl`** | Section 8 | FreeBSD bectl(8) & Illumos beadm(1) | `bectl [subcommand] [args]` |
| **`bpf`** | Section 4 | FreeBSD & OpenBSD bpf(4) | `/dev/bpf` |
| **`buddy_allocator`** | Section 9 | Linux mm buddy allocator & FreeBSD vm_page | `void *buddy_alloc(size_t order);` |
| **`cap_rights_limit`** | Section 3 | FreeBSD Capsicum cap_rights_limit(3) | `int cap_rights_limit(int fd, const cap_rights_t *rights);` |
| **`cgroups`** | Section 7 | Linux kernel cgroups(7) | `cgroup v2 hierarchy` |
| **`fstab`** | Section 5 | POSIX & NetBSD fstab(5) | `/etc/fstab` |
| **`gettext`** | Section 3 | GNU gettext(3) & FreeBSD catgets(3) | `char *gettext(const char *msgid);` |
| **`pacman.conf`** | Section 5 | Arch Linux pacman.conf(5) | `/etc/pacman.conf` |
| **`pledge`** | Section 2 | OpenBSD pledge(2) | `int pledge(const char *promises, const char *execpromises);` |
| **`pqc_signatures`** | Section 7 | OpenBSD signify(1) & NIST PQC Standard | `Post-Quantum Cryptographic Attestation Specification` |
| **`seccomp`** | Section 2 | Linux seccomp(2) | `int seccomp(unsigned int operation, unsigned int flags, void *args);` |
| **`sigma-ctl`** | Section 1 | systemd systemctl & OpenBSD rcctl(8) | `sigma-ctl [COMMAND] [TARGET]` |
| **`sigma-fetch`** | Section 1 | Omarchy Fastfetch & Arch Linux custom tools | `sigma-fetch [--json]` |
| **`sigma-pkg`** | Section 8 | FreeBSD pkg(8) & Arch Linux pacman(8) hybrid | `sigma-pkg [OPTIONS] COMMAND [ARGS]...` |
| **`sigma-quest`** | Section 6 | BSD Games tradition (rogue/hack) | `sigma-quest` |
| **`sigma-sh`** | Section 1 | OpenBSD ksh(1) & GNU bash(1) | `sigma-sh [-c COMMAND] [FILE]` |
| **`sigma-top`** | Section 1 | Linux htop(1) & FreeBSD top(1) | `sigma-top [-d DELAY] [-p PID]` |
| **`unveil`** | Section 2 | OpenBSD unveil(2) | `int unveil(const char *path, const char *permissions);` |
| **`vfs`** | Section 9 | FreeBSD & DragonFly BSD vfs(9) | `int vfs_mount(struct mount *mp, ...);` |
| **`zfs`** | Section 4 | OpenZFS / FreeBSD zfs(4) | `/dev/zfs` |

---

## Detailed Manual Page Specifications

# bectl(8)

**Section:** 8 - System Administration and Daemons
**Origin/Inspiration:** FreeBSD bectl(8) & Illumos beadm(1)

## SYNOPSIS
```text
bectl [subcommand] [args]
```

## DESCRIPTION
ZFS boot environment management utility for creating, mounting, and rolling back boot snapshots.

## OPTIONS
* `activate`: Set specified boot environment as default for next boot
* `create`: Create a new boot environment snapshot

## SEE ALSO
zfs(8), snapper(8)

---

# bpf(4)

**Section:** 4 - Special Files and Drivers
**Origin/Inspiration:** FreeBSD & OpenBSD bpf(4)

## SYNOPSIS
```text
/dev/bpf
```

## DESCRIPTION
Berkeley Packet Filter device node for raw link-layer packet capturing.

## SEE ALSO
pcap(3), netintro(4)

---

# buddy_allocator(9)

**Section:** 9 - Kernel Subsystems and BSD Internals
**Origin/Inspiration:** Linux mm buddy allocator & FreeBSD vm_page

## SYNOPSIS
```text
void *buddy_alloc(size_t order);
```

## DESCRIPTION
Physical buddy memory allocator with anti-fragmentation page compaction passes.

## SEE ALSO
vfs(9)

---

# cap_rights_limit(3)

**Section:** 3 - C/POSIX Library Functions
**Origin/Inspiration:** FreeBSD Capsicum cap_rights_limit(3)

## SYNOPSIS
```text
int cap_rights_limit(int fd, const cap_rights_t *rights);
```

## DESCRIPTION
FreeBSD Capsicum capability rights limitation for file descriptors.

## SEE ALSO
pledge(2), rights(4)

---

# cgroups(7)

**Section:** 7 - Miscellaneous and Conventions
**Origin/Inspiration:** Linux kernel cgroups(7)

## SYNOPSIS
```text
cgroup v2 hierarchy
```

## DESCRIPTION
Linux control groups v2 resource management and container isolation.

## SEE ALSO
systemd(1), landlock(7)

---

# fstab(5)

**Section:** 5 - File Formats and Configurations
**Origin/Inspiration:** POSIX & NetBSD fstab(5)

## SYNOPSIS
```text
/etc/fstab
```

## DESCRIPTION
Static filesystem information table specifying mount points and options.

## SEE ALSO
mount(8), swapon(8)

---

# gettext(3)

**Section:** 3 - C/POSIX Library Functions
**Origin/Inspiration:** GNU gettext(3) & FreeBSD catgets(3)

## SYNOPSIS
```text
char *gettext(const char *msgid);
```

## DESCRIPTION
Looks up localized message strings in standard message catalogs.

## SEE ALSO
catgets(3), ngettext(3)

---

# pacman.conf(5)

**Section:** 5 - File Formats and Configurations
**Origin/Inspiration:** Arch Linux pacman.conf(5)

## SYNOPSIS
```text
/etc/pacman.conf
```

## DESCRIPTION
Package manager configuration file specifying repositories, mirrors, and signature verification options.

## SEE ALSO
sigma-pkg(8), pkg.conf(5)

---

# pledge(2)

**Section:** 2 - Kernel System Calls
**Origin/Inspiration:** OpenBSD pledge(2)

## SYNOPSIS
```text
int pledge(const char *promises, const char *execpromises);
```

## DESCRIPTION
OpenBSD-inspired system call restricting process privileges and kernel capability access.

## SEE ALSO
unveil(2), seccomp(2), landlock(7)

---

# pqc_signatures(7)

**Section:** 7 - Miscellaneous and Conventions
**Origin/Inspiration:** OpenBSD signify(1) & NIST PQC Standard

## SYNOPSIS
```text
Post-Quantum Cryptographic Attestation Specification
```

## DESCRIPTION
Dilithium5 and Signify post-quantum digital signature standards used for package and kernel module attestation.

## SEE ALSO
sigma-pkg(8), signify(1)

---

# seccomp(2)

**Section:** 2 - Kernel System Calls
**Origin/Inspiration:** Linux seccomp(2)

## SYNOPSIS
```text
int seccomp(unsigned int operation, unsigned int flags, void *args);
```

## DESCRIPTION
Linux Seccomp-BPF system call filtering mechanism for sandboxing isolated processes.

## SEE ALSO
pledge(2), bpf(4)

---

# sigma-ctl(1)

**Section:** 1 - General User Commands
**Origin/Inspiration:** systemd systemctl & OpenBSD rcctl(8)

## SYNOPSIS
```text
sigma-ctl [COMMAND] [TARGET]
```

## DESCRIPTION
System control utility for service supervision, target state changes, and unit inspection.

## OPTIONS
* `restart`: Restart specified system daemon
* `status`: Query runtime status of service unit

## SEE ALSO
systemctl(1), rcctl(8)

---

# sigma-fetch(1)

**Section:** 1 - General User Commands
**Origin/Inspiration:** Omarchy Fastfetch & Arch Linux custom tools

## SYNOPSIS
```text
sigma-fetch [--json]
```

## DESCRIPTION
Fast system information generator detailing kernel version, uptime, microarchitecture, and memory usage.

## OPTIONS
* `--json`: Output system telemetry in JSON format

## SEE ALSO
fastfetch(1), neofetch(1)

---

# sigma-pkg(8)

**Section:** 8 - System Administration and Daemons
**Origin/Inspiration:** FreeBSD pkg(8) & Arch Linux pacman(8) hybrid

## SYNOPSIS
```text
sigma-pkg [OPTIONS] COMMAND [ARGS]...
```

## DESCRIPTION
Universal package manager for SigmaOS combining FreeBSD pkg, Arch pacman, Debian apt, and Nix CAS store.

## OPTIONS
* `-S, --sync`: Synchronize repository databases
* `-i, --install`: Install package with PQC attestation
* `-u, --update`: Perform transactional rootfs update

## SEE ALSO
pacman(8), pkg(8), apt(8)

---

# sigma-quest(6)

**Section:** 6 - Games and Demos
**Origin/Inspiration:** BSD Games tradition (rogue/hack)

## SYNOPSIS
```text
sigma-quest
```

## DESCRIPTION
Ncurses terminal adventure game demonstrating kernel syscall and audio subsystem features.

## SEE ALSO
ncurses(3)

---

# sigma-sh(1)

**Section:** 1 - General User Commands
**Origin/Inspiration:** OpenBSD ksh(1) & GNU bash(1)

## SYNOPSIS
```text
sigma-sh [-c COMMAND] [FILE]
```

## DESCRIPTION
Sovereign interactive POSIX-compliant command interpreter with eBPF history tracking.

## OPTIONS
* `-c`: Execute string command
* `-l`: Start as login shell

## SEE ALSO
bash(1), zsh(1), tcsh(1)

---

# sigma-top(1)

**Section:** 1 - General User Commands
**Origin/Inspiration:** Linux htop(1) & FreeBSD top(1)

## SYNOPSIS
```text
sigma-top [-d DELAY] [-p PID]
```

## DESCRIPTION
Interactive real-time process monitor with Cgroup v2 PSI metrics and per-CPU breakdown.

## OPTIONS
* `-d`: Update delay interval in seconds
* `-p`: Filter inspection to specific process ID

## SEE ALSO
htop(1), top(1)

---

# unveil(2)

**Section:** 2 - Kernel System Calls
**Origin/Inspiration:** OpenBSD unveil(2)

## SYNOPSIS
```text
int unveil(const char *path, const char *permissions);
```

## DESCRIPTION
Restricts filesystem visibility for the calling process to explicit path subsets.

## SEE ALSO
pledge(2), landlock(7)

---

# vfs(9)

**Section:** 9 - Kernel Subsystems and BSD Internals
**Origin/Inspiration:** FreeBSD & DragonFly BSD vfs(9)

## SYNOPSIS
```text
int vfs_mount(struct mount *mp, ...);
```

## DESCRIPTION
Virtual Filesystem interface and inode/vnode dispatch layer.

## SEE ALSO
vnode(9), mount(8)

---

# zfs(4)

**Section:** 4 - Special Files and Drivers
**Origin/Inspiration:** OpenZFS / FreeBSD zfs(4)

## SYNOPSIS
```text
/dev/zfs
```

## DESCRIPTION
OpenZFS control device providing pool management, datasets, and snapshot transactions.

## SEE ALSO
zpool(8), bectl(8)

---

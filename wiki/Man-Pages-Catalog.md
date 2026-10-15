# Consolidated Man Pages Catalog (Sections 1..9)

## Summary
In SigmaOS, traditional Unix/BSD/Linux manual pages (Sections 1..9) are consolidated into single-source GitHub Wiki Markdown specifications. This eliminates fragmented `/usr/share/man` directories while maintaining troff/mdoc structured formatting (SYNOPSIS, DESCRIPTION, OPTIONS, SEE ALSO).

---

## Section 1: User Commands
Executable programs, shell tools, and userland applications.

### `sigma-sh(1)`
- **Synopsis:** `sigma-sh [-c COMMAND] [FILE]`
- **Description:** Sovereign interactive POSIX-compliant command interpreter with eBPF history tracking and zero-copy string buffer pipelines.
- **Options:**
  - `-c`: Execute string command.
  - `-l`: Start as login shell.
- **See Also:** `bash(1)`, `zsh(1)`, `tcsh(1)`
- **Origin/Inspiration:** OpenBSD `ksh(1)` & GNU `bash(1)`

---

## Section 2: System Calls
Kernel entry points and system call interfaces.

### `pledge(2)`
- **Synopsis:** `int pledge(const char *promises, const char *execpromises);`
- **Description:** Restricts process privilege levels and kernel capability access for process sandboxing.
- **See Also:** `unveil(2)`, `seccomp(2)`, `landlock(7)`
- **Origin/Inspiration:** OpenBSD `pledge(2)`

### `unveil(2)`
- **Synopsis:** `int unveil(const char *path, const char *permissions);`
- **Description:** Restricts process filesystem view to designated paths and permission masks (`r`, `w`, `x`, `c`).
- **See Also:** `pledge(2)`, `landlock(7)`
- **Origin/Inspiration:** OpenBSD `unveil(2)`

---

## Section 3: Library Functions
C/POSIX system library and runtime functions.

### `cap_rights_limit(3)`
- **Synopsis:** `int cap_rights_limit(int fd, const cap_rights_t *rights);`
- **Description:** Limits rights on file descriptors using capability-based security.
- **See Also:** `pledge(2)`, `rights(4)`
- **Origin/Inspiration:** FreeBSD Capsicum `cap_rights_limit(3)`

---

## Section 4: Special Files & Drivers
Kernel device nodes and virtual drivers.

### `bpf(4)`
- **Synopsis:** `/dev/bpf`
- **Description:** Berkeley Packet Filter device node for raw link-layer packet capturing and traffic inspection.
- **See Also:** `pcap(3)`, `netintro(4)`
- **Origin/Inspiration:** FreeBSD & OpenBSD `bpf(4)`

---

## Section 5: File Formats
Static configuration file formats and layouts.

### `fstab(5)`
- **Synopsis:** `/etc/fstab`
- **Description:** Static filesystem information table specifying mount points, options, and pass numbers.
- **See Also:** `mount(8)`, `swapon(8)`
- **Origin/Inspiration:** POSIX & NetBSD `fstab(5)`

---

## Section 6: Games & Demos
Terminal utilities, screensavers, and interactive entertainment.

### `tetris(6)`
- **Synopsis:** `tetris [-s] [level]`
- **Description:** Classic BSD terminal tetris implementation using ncurses viewport rendering.
- **See Also:** `ncurses(3)`
- **Origin/Inspiration:** OpenBSD `tetris(6)`

---

## Section 7: Miscellaneous
Conventions, macro packages, and kernel subsystems.

### `cgroups(7)`
- **Synopsis:** `cgroup v2 hierarchy`
- **Description:** Linux control groups v2 resource management for CPU, memory, I/O, and PID limits.
- **See Also:** `systemd(1)`, `landlock(7)`
- **Origin/Inspiration:** Linux kernel `cgroups(7)`

---

## Section 8: System Administration
Daemons, system utilities, and package managers.

### `sigma-pkg(8)`
- **Synopsis:** `sigma-pkg [OPTIONS] COMMAND [ARGS]...`
- **Description:** Universal package manager combining FreeBSD `pkg`, Arch `pacman`, Debian `apt`, and Nix CAS store deduplication with PQC Dilithium-5 attestation.
- **Options:**
  - `-i, --install`: Install package with PQC attestation verification.
  - `-S, --sync`: Synchronize repository databases.
  - `-u, --update`: Perform transactional rootfs update with snapshot fallback.
- **See Also:** `pacman(8)`, `pkg(8)`, `apt(8)`
- **Origin/Inspiration:** FreeBSD `pkg(8)` & Arch Linux `pacman(8)` hybrid

---

## Section 9: Kernel Subsystems & BSD Internals
VFS routines, kernel locking, and driver interfaces.

### `vfs(9)`
- **Synopsis:** `int vfs_mount(struct mount *mp, ...);`
- **Description:** Virtual Filesystem interface and vnode dispatch layer supporting zero-copy page caching and transactional CoW operations.
- **See Also:** `vnode(9)`, `mount(8)`
- **Origin/Inspiration:** FreeBSD & DragonFly BSD `vfs(9)`

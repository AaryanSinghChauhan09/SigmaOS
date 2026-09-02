# Distro Inspirations

Features SigmaOS has adopted from the best Linux and BSD distributions.

---

## Philosophy

SigmaOS doesn't reinvent the wheel — it combines the best wheels. Every major design decision in SigmaOS traces back to a proven innovation from the open-source ecosystem:

| Feature | Inspired By | SigmaOS Implementation |
|---------|------------|------------------------|
| Rolling release | Arch Linux | `src/distro/arch_inspirations.rs` |
| Content-addressed packages | NixOS | `/sigma/store/` |
| Atomic upgrades | NixOS/OSTree | Two-phase commit |
| USE flags | Gentoo | `src/distro/gentoo_inspirations.rs` |
| pledge/unveil | OpenBSD | `src/security/pledge.rs` |
| Capsicum | FreeBSD | `src/security/capsicum.rs` |
| BORE scheduler | CachyOS | `src/kernel/sched/` |
| HAMMER2 FS | DragonFly BSD | `src/filesystem/` |
| Snapper snapshots | openSUSE | `src/distro/` |
| KARL | OpenBSD | `src/security/openbsd_karl.rs` |
| W^X | OpenBSD | Kernel paging |
| Jails | FreeBSD | `src/security/jails.rs` |
| SELinux | Fedora/NSA | `src/security/selinux.rs` |
| PGO kernel | CachyOS | Build system |
| Cockpit | Fedora | `src/remote/` |

---

## Arch Linux Innovations

**Rolling Release Model**
- Packages released as soon as they're ready
- Three channels: Edge (daily), Stable (weekly), LTS (security only)
- No "big bang" upgrade cycles

**AUR (Arch User Repository)**
- User-contributed PKGBUILD recipes
- `sigpkg aur install <anything>` gives access to thousands of packages
- Sandboxed builds for safety

**pacman Speed**
- O(1) package lookup via signed package database
- Parallel download of multiple packages
- Delta updates (only download changed bytes)

---

## NixOS Innovations

**Declarative System State**
- Entire system described in one configuration file
- `sigma-apply config.sigma` converges system to desired state
- No "configuration drift" — system always matches spec

**Content-Addressed Store**
- Package stored at `/sigma/store/<sha256-hash>/`
- Identical content = identical path = automatic deduplication
- Multiple versions of any package coexist

**Atomic Generations**
- Every upgrade creates a new generation
- Switching generations is a single atomic operation
- Boot menu lists all generations (you can boot any)

---

## Gentoo Innovations

**USE Flags**
- Compile packages with only the features you need
- `ssl`, `http2`, `perl`, `gui` — enable/disable per-package
- Reduces attack surface by excluding unused code

**Source-Based Building**
- Every package compiled from source with your exact flags
- Maximum CPU-specific optimisation (`-march=native`)
- Deep understanding of what's in your system

---

## OpenBSD Innovations

**pledge()**
- At startup, process declares exact syscalls it needs
- Any syscall outside the list kills the process
- Can be called multiple times to progressively restrict

**unveil()**
- Show process only the filesystem paths it legitimately needs
- `/bin/sh` only sees `/bin`, `/lib`, `/tmp` — nothing else
- Effective against path traversal attacks

**W^X (Write XOR Execute)**
- No memory page is both writable and executable
- Eliminates JIT-spray attacks
- Enforced in hardware (NX bit)

**KARL (Kernel Address Randomised Link)**
- Kernel binary is relinked at every boot
- Different ROP gadget locations each boot
- Makes exploitation dramatically harder

---

## FreeBSD Innovations

**Capsicum**
- Fine-grained capability model for file descriptors
- Process enters "capability mode" — no ambient authority
- Can only use explicitly-granted fds with explicitly-granted rights

**Jails**
- Lightweight OS virtualisation
- Each jail has own process tree, network, filesystem
- Lower overhead than full VMs

**ZFS**
- Copy-on-Write filesystem
- Snapshots are near-instant, space-efficient
- RAIDZ for software RAID

---

## CachyOS Innovations

**BORE Scheduler**
- Burst-Oriented Response Enhancer
- Tracks CPU-burst patterns per task
- Interactive tasks get priority boost → lower desktop latency

**LLVM PGO (Profile-Guided Optimisation)**
- Kernel compiled with CPU-usage profiles
- Hot code paths get better instruction layout
- Up to 15% performance improvement reported

**x86-64-v3 Microarchitecture**
- Uses AVX2, BMI2, FMA instructions not in base x86-64
- Packages built specifically for modern CPUs
- Compatible hardware required (Intel Haswell+, AMD Zen+)

---

## DragonFly BSD Innovations

**HAMMER2 Filesystem**
- Multi-master clustering
- PFS (pseudo-filesystems) — multiple FS volumes per partition
- On-the-fly compression and deduplication
- Crash-safe B-tree

---

## Fedora Innovations

**Cockpit Web Console**
- Browser-based system administration
- Manage services, network, storage from any browser
- Zero-install admin interface

**FreeIPA / Kerberos**
- Enterprise identity management
- Single sign-on across the entire infrastructure
- LDAP + Kerberos + DNS + Certificate management integrated

**Anitya Release Monitoring**
- Automatically tracks upstream project versions
- Alerts when installed packages fall behind upstream
- Integrates with release automation

---

## openSUSE Innovations

**Snapper**
- Automatic pre/post snapshots around package upgrades
- "What changed when I installed nginx?" — diff between snapshots
- Rollback to any snapshot from boot menu

**Zypper**
- Powerful repository management
- Vendor locking prevents unintended upgrades
- Rich search and dependency query

---

## Summary: The Best of All Worlds

SigmaOS takes these innovations and unifies them:

```
                    SigmaOS
                       │
    ┌──────────────────┼──────────────────┐
    │                  │                  │
  Safety           Performance        Flexibility
    │                  │                  │
OpenBSD            CachyOS            Arch/NixOS
pledge/unveil      BORE sched         Rolling + Decl.
W^X/KARL           PGO/BOLT           AUR + Store
Capsicum           x86-64-v3          Generations
```

# 🆚 SigmaOS vs Linux Distros

> This page documents the core philosophy differences between **SigmaOS Zenith** and popular Linux distributions. SigmaOS absorbs the *ideas* from Linux — but **reimplements every component from scratch**, never importing glibc, POSIX APIs, or any external library.

---

## Comparison Matrix

| Feature | Linux (e.g., Arch, Ubuntu) | SigmaOS Zenith |
|:--|:--|:--|
| **C Library** | glibc / musl | ❌ None — `sovereign_*` functions only |
| **Kernel ABI** | POSIX syscall table | 256-slot Sigma Syscall Dispatcher |
| **Memory Allocator** | SLUB + glibc `malloc` | `sigma_slab_allocator.cpp` — zero-dependency SLUB clone |
| **Paging** | x86_64 4-level PT via mm/ | `sigma_paging.cpp` — direct CR3 manipulation |
| **Filesystem** | ext4, btrfs via VFS layer | `sigma_fat32.cpp` — FAT32 clone |
| **Shell** | bash, dash, zsh | `sigma_sh.cpp` — BusyBox-inspired sovereign shell |
| **NIC Driver** | `drivers/net/e1000/` | `sigma_e1000.cpp` — MMIO, TX/RX ring buffers |
| **Display** | DRM / KMS / fbdev | `sigma_vga.cpp` — direct 0xB8000 VGA write |
| **Scheduler** | CFS + SCHED_FIFO/RR | `sigma_rt_scheduler.cpp` — EDF + priority inheritance |
| **Init System** | systemd (2M LOC) | `S01_Genesis` — sovereign init shard |
| **Package Manager** | apt / pacman / dnf | Sovereign Package System (roadmap) |
| **Cryptography** | OpenSSL / libgcrypt | `sigma_app_signer.cpp` — Dilithium-5 stub |
| **printf** | glibc `printf()` | `sigma_vga_printf()` — built from scratch |

---

## Why No Predefined Libraries?

Linux distributions depend on a massive chain of trust:

```
App → glibc → syscall → kernel → hardware
```

SigmaOS eliminates every link in that chain except one:

```
Shard → Sigma Syscall Dispatcher → hardware
```

This means:
- **Zero attack surface** from third-party library vulnerabilities.
- **No undefined behavior** from standard library version mismatches.
- **Full auditability** — every function that runs is one we wrote.

---

## Absorbed Concepts (Reimplemented)

| Linux Concept | Source Inspiration | SigmaOS Implementation |
|:--|:--|:--|
| SLUB allocator | Linux `mm/slub.c` | `kernel/memory/sigma_slab_allocator.cpp` |
| 4-level paging | Linux `arch/x86/mm/` | `kernel/memory/sigma_paging.cpp` |
| FAT32 support | Linux `fs/fat/` | `kernel/fs/sigma_fat32.cpp` |
| e1000 NIC driver | Linux `drivers/net/ethernet/intel/` | `kernel/drivers/sigma_e1000.cpp` |
| BusyBox shell | BusyBox `shell/ash.c` | `usr/sigma_sh.cpp` |
| EDF scheduling | Linux `kernel/sched/deadline.c` | `kernel/scheduler/sigma_rt_scheduler.cpp` |
| VGA framebuffer | Linux `drivers/video/fbdev/` | `kernel/drivers/sigma_vga.cpp` |

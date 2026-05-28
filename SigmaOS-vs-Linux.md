# 🆚 SigmaOS vs Linux Distros

> This page documents the core philosophy differences between **SigmaOS Zenith** and popular Linux distributions. SigmaOS absorbs the *ideas* from Linux — but **reimplements every component from scratch**, never importing glibc, POSIX APIs, or any external library.

---

## Current Technical Limitations (The Reality Check)

While SigmaOS is an ambitious, ground-up research operating system focusing on sovereignty, sandboxing, and performance, it is essential for users migrating from mature ecosystems like Linux or Windows to understand our current boundaries.

### 🔧 Core OS Functionality
- **Limited Driver Support**: SigmaOS currently implements only essential drivers (PS/2 keyboard, VGA framebuffer, ATA/SATA/VirtIO storage, and Intel e1000 networking). In contrast, Linux distros support thousands of devices across GPUs, Wi-Fi chipsets, printers, Bluetooth, etc.
- **File Systems**: SigmaOS supports FAT32 and Ext2 natively, with a custom Sovereign ZFS pool in development. Linux supports a wide range of advanced filesystems (Ext4, Btrfs, XFS, etc.) with mature journaling.
- **Syscall ABI**: SigmaOS uses a non-POSIX ABI to enforce its custom security model. This means existing Linux applications cannot run natively without porting.

### 🖥️ Userland & Applications
- **Minimal Tools**: SigmaOS provides sovereign, clean-room replacements for basic utilities (`ls`, `cat`, `awk`, `sed`, `tar`) and a text-mode HTML browser.
- **Package Ecosystem**: There is currently no equivalent of `apt`, `dnf`, or `pacman`. Software distribution is handled via source compilation, though the OmniPackage Manager is on our roadmap.
- **UI/UX**: The current environment is primarily text-mode based. A full GUI (Zenith Window Manager) is planned but currently lacks the maturity of GNOME or KDE.

### 🔒 Security & Stability
- **Kernel Maturity**: The Linux kernel has decades of rigorous testing, bug fixes, and performance optimization. The SigmaOS kernel is early-stage, with foundational scheduling and memory allocation.
- **Networking Stack**: SigmaOS currently provides a basic e1000 driver. A fully fledged, hardened TCP/IP stack is under active development.

> [!WARNING]
> **Summary**: SigmaOS is a fascinating research OS and a glimpse into a sovereign computing future. However, it is more of a proof-of-concept than a daily-driver right now.

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

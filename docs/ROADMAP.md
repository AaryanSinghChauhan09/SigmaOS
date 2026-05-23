# 🗺️ SigmaOS Zenith Roadmap

> "Absorb the world's best ideas, build them from scratch with zero dependencies."

## 🚀 Phase 1: The Sovereign Foundation (Short-Term)
**Goal:** Establish a robust, self-hosting microkernel that can manage its own memory, read files, and handle basic input/output.

- [x] **Memory Management:** SLUB-inspired allocator (`sigma_slab_allocator`) and 4-level x86_64 paging.
- [x] **Basic Drivers:** VGA Framebuffer (`sigma_vga`), PS/2 Keyboard (`sigma_keyboard`), PS/2 Mouse (`sigma_mouse`).
- [x] **Block Storage:** NVMe driver (`sigma_nvme`), ATA disk driver.
- [x] **File Systems:** FAT32 read-only implementation (`sigma_fat32`), basic ext2 superblock parser (`sigma_ext2`).
- [x] **Tools (Phase 1):** BusyBox-inspired shell (`sigma_sh`), `ls`, and `echo` fully implemented via raw syscalls.

## 🌐 Phase 2: The Connectivity Leap (Mid-Term)
**Goal:** Bring the system online with a sovereign network stack.

- [x] **Network Interface Cards (NIC):** Intel e1000 (`sigma_e1000`), Realtek RTL8139 (`sigma_rtl8139`).
- [ ] **Sovereign TCP/IP Stack:** ARP, IPv4, ICMP, UDP, and finally TCP—all written from scratch with zero sockets from libc.
- [ ] **File System Expansion:** Full read/write support for ext2, moving towards journaled ext4 clone.
- [ ] **Bootloader:** Custom Stage 1/Stage 2 bootloader replacing GRUB, loading the SigmaOS ELF kernel directly.

## 📦 Phase 3: The Userland Singularity (Long-Term)
**Goal:** Create a modern, graphical, secure OS environment.

- [ ] **OmniPackage Manager:** Transactional, containerized package manager bypassing dependency hell (no `/usr/lib` conflicts).
- [ ] **Zenith GUI:** A compositor and window manager using the VGA framebuffer, implementing a glassmorphic Sovereign UI.
- [ ] **Bootstrap Compiler:** A minimal C++ compiler capable of compiling SigmaOS within SigmaOS (Self-Hosting).
- [ ] **Post-Quantum Security:** Dilithium-5 digital signatures enforcing that only signed shards execute.
- [ ] **S-ZFS:** Full copy-on-write storage pool with RAID-Z-like striping.

---
**Guiding Principle throughout all phases:** Zero `libc`, zero `std::vector`, zero `#include <stdio.h>`. Silicon direct.

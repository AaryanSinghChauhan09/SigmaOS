# 🏗️ SigmaOS Architecture Overview

> **Σ SigmaOS Zenith** is a sovereign, zero-dependency microkernel operating system built on a 600-shard C++ singleton lattice — targeting x86_64, ARM64, and RISC-V with absolute hardware independence.

---

## Memory Layout

```
0x0000_0000 — 0x0007_FFFF  ▪ BIOS & Legacy regions (reserved)
0x0008_0000 — 0x0009_FFFF  ▪ VGA BIOS ROM
0x000B_8000 — 0x000B_FFFF  ▪ VGA Text Mode Framebuffer (0xB8000)
0x0010_0000 — 0x001F_FFFF  ▪ Kernel Image (.text, .data, .bss)
0x0020_0000 — 0x002F_FFFF  ▪ Page Table Pool (sigma_paging)
0x0030_0000 — 0x003F_FFFF  ▪ Kernel Stack
0x0040_0000 — 0x013F_FFFF  ▪ Slab Allocator Arena (sigma_slab)
0x8000_0000 — 0xFFFF_FFFF  ▪ MMIO / Device Registers (e.g., e1000 NIC)
FFFF_8000 — 0xFFFF_FFFF_FFFF  ▪ Higher-Half Kernel Virtual (future)
```

---

## Bootloader → Kernel Handoff

1. **Stage 1**: BIOS loads the 512-byte MBR boot sector from LBA 0.
2. **Stage 2**: Enters 32-bit Protected Mode, loads the kernel ELF.
3. **Kernel Init** (`S01_Genesis/init.cpp`):
   - Calls `sigma_vga_init()` — screen is live.
   - Calls `sigma_slab_init()` — memory allocator ready.
   - Calls `sigma_paging_init()` — 4-level page tables activated.
   - Calls `sigma_fat32_mount()` — filesystem mounted.
   - Calls `sigma_e1000_init()` — network link up.
   - Calls `sigma_sh_run()` — drops into the Sovereign Shell.

---

## Subsystem Map

| Subsystem | File | Technique |
|:--|:--|:--|
| Memory Allocator | `kernel/memory/sigma_slab_allocator.cpp` | SLUB power-of-2 buckets, intrusive free lists |
| Virtual Memory | `kernel/memory/sigma_paging.cpp` | x86_64 4-level paging, TLB flush via `invlpg` |
| File System | `kernel/fs/sigma_fat32.cpp` | FAT32 BPB parsing, cluster chain traversal |
| ATA Disk | `kernel/drivers/sigma_ata_driver.cpp` | Port I/O via inline assembly (`inb`, `outb`) |
| NIC Driver | `kernel/drivers/sigma_e1000.cpp` | Intel e1000 MMIO, TX/RX ring buffers |
| VGA Display | `kernel/drivers/sigma_vga.cpp` | Direct 0xB8000 mapping, hardware cursor |
| Touch Input | `kernel/drivers/sigma_touch_driver.cpp` | I2C HID parsing, SPSC ring buffer |
| Real-Time Sched | `kernel/scheduler/sigma_rt_scheduler.cpp` | EDF algorithm, priority inheritance |
| Adaptive Sched | `kernel/core/SovereignAdaptiveScheduler.cpp` | EWMA slice predictor, class-aware priorities |
| Self-Healing | `kernel/core/SovereignSelfHealingKernel.cpp` | Subsystem watches, runtime live patching |
| Config Rollback | `tools/cli/SovereignConfigRollbackCLI.cpp` | NixOS-style generation management |
| Registry | `kernel/core/sigma_registry_manager.cpp` | Key-value persistence store |
| Shell | `usr/sigma_sh.cpp` | BusyBox-inspired, PS/2 keyboard polling |
| App Signer | `tools/sigma_app_signer.cpp` | Dilithium-5 PQC attestation |
| Forensics | `tools/sigma_forensics.cpp` | CR0-CR4 register dumps |
| GST Calc | `tools/gst_court_calculator.cpp` | Fixed-point integer math |

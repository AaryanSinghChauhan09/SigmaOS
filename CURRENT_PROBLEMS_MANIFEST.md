# Σ# SigmaOS: Active Bug Manifest

*This document tracks known issues that require immediate contributor attention.*

## High Priority (Kernel/Core)

- **[#1132] Ext4 Journal Checkpointing:** The JBD2 journal (`fs/ext4_journal.c`) implementation is currently a skeleton. It needs proper commit transaction flushing to disk.
- **[#1133] VFS / Block Dev Integration:** The `fs/ext4.c` uses simulated block reads. This needs to be hooked up to a real NVMe or SATA AHCI driver.
- **[#1134] Memory Fragmentation:** The page allocator experiences external fragmentation after intensive VMM shard creation/destruction cycles. Buddy allocator rewrite required.

## Medium Priority (Drivers/HAL)

- **[#844] UEFI Framebuffer:** The fallback framebuffer (`drivers/gpu/vesa.c`) works, but we lack native KMS drivers for AMD/Intel graphics, preventing hardware-accelerated composition.
- **[#850] PCIe MSI-X:** Interrupt routing occasionally drops vectors on multi-socket NUMA boards during high network I/O.

## Low Priority (Userland/UI)

- **[#512] Zenith Compositor:** Native C++ Wayland compositor is planned, but current Zenith UI is just a JavaScript prototype.
- **[#520] Shell Globbing:** `sigma-sh` lacks wildcard expansion for path matching.

---

*Found a bug? Open a shard-ticket in the lattice tracking system.*

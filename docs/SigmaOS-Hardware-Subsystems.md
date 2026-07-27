# SigmaOS Zenith: Hardware & I/O Subsystems (v15.2)

To complete the abstraction between raw silicon and the SigmaOS Zenith sovereign logic layer, we have synthesized six critical hardware and I/O pipelines. These subsystems draw directly from the architectures of the Linux and FreeBSD device trees, allowing SigmaOS to manage physical devices autonomously.

All code is written in strict, standalone C11.

---

## 17. Block I/O Scheduler
**Inspirations:** Linux `block/blk-core.c`, FreeBSD `subr_disk.c`
**Implementation:** `kernel/core/storage/sigma_block.c`

A non-blocking request queue for block storage devices (NVMe, SATA, SD). Implements basic I/O merging and elevator logic (NOOP simulation) to queue `REQ_OP_READ` and `REQ_OP_WRITE` payloads for DMA engines.

## 18. Page Cache / Buffer Cache
**Inspirations:** Linux `mm/filemap.c`, FreeBSD `vm_page.c`
**Implementation:** `kernel/core/mem/sigma_pagecache.c`

A RAM-backed caching layer bridging the VFS and the Block I/O scheduler. Uses a strict LRU (Least Recently Used) eviction algorithm to maintain hot filesystem blocks in physical memory, drastically reducing disk latency.

## 19. VirtIO Split Ring Subsystem
**Inspirations:** Linux `drivers/virtio/virtio_ring.c`, OASIS VIRTIO Spec
**Implementation:** `kernel/core/virt/sigma_virtio.c`

Paravirtualization support via standard split virtqueues (Available Ring, Used Ring, Descriptor Table). This enables SigmaOS to run natively inside hypervisors (QEMU/KVM) using hardware-accelerated VirtIO storage and network adapters.

## 20. Kobject & Sysfs Architecture
**Inspirations:** Linux `lib/kobject.c`, `fs/sysfs/`
**Implementation:** `kernel/core/system/sigma_kobject.c`

A hierarchical, reference-counted object tree. This provides the structural foundation for dynamically plugging devices, managing lifetimes, and eventually projecting a `/sys` virtual filesystem for runtime hardware introspection.

## 21. Framebuffer Device (fbdev)
**Inspirations:** Linux `drivers/video/fbdev/core/fbmem.c`
**Implementation:** `kernel/core/graphics/sigma_fbdev.c`

A generic Linear Framebuffer (LFB) wrapper. Standardizes 16bpp and 32bpp pixel plotting over VESA or UEFI GOP interfaces, providing the foundational plotting API for the Zenith UI Composer without requiring a full GPU driver.

## 22. Evdev Input Subsystem
**Inspirations:** Linux `drivers/input/input.c`, FreeBSD `evdev.c`
**Implementation:** `kernel/core/hardware/sigma_evdev.c`

A centralized event ring-buffer routing hardware interrupts from keyboards, mice, and touchscreens. Formats incoming scans into standard `EV_KEY`, `EV_REL`, and `EV_ABS` packets complete with microsecond timestamps, ready for user-space polling.

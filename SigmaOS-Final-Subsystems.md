# SigmaOS Zenith: Final Kernel Components (v15.2)

To completely finalize the Zenith microkernel architecture and achieve absolute parity with monolithic operating systems, we have implemented the final tier of resource control, memory allocation, and performance monitoring logic.

All components adhere strictly to ISO C11.

---

## 35. SLAB Allocator

**Inspirations:** Linux `mm/slab.c` / `mm/slub.c`, FreeBSD `subr_uma.c`
**Implementation:** `kernel/core/mem/sigma_slab.c`

A high-performance object-caching memory allocator. Instead of fragmenting generic heap memory, this subsystem provisions contiguous physical pages specifically chunked for fixed-size kernel objects (like inodes or task structs), vastly improving cache locality and allocation speed.

## 36. TCP State Machine

**Inspirations:** Linux `net/ipv4/tcp.c`, RFC 793
**Implementation:** `net/tcp.c`

A complete connection-oriented transport protocol implementation. Fully handles the TCP lifecycle (LISTEN, SYN_SENT, SYN_RECV, ESTABLISHED, FIN_WAIT, CLOSE_WAIT, TIME_WAIT) to ensure reliable byte-stream telemetry delivery without needing user-space TCP daemons.

## 37. Filesystem in Userspace (FUSE)

**Inspirations:** Linux `fs/fuse/dev.c`
**Implementation:** `fs/fuse.c`

A bridge between the internal Virtual Filesystem (VFS) and external user-space daemons. Exposes a queue mechanism where the kernel can dispatch standard `OP_LOOKUP`, `OP_READ`, and `OP_WRITE` payloads to isolated shards, receiving serialized responses securely.

## 38. Control Groups (Cgroups)

**Inspirations:** Linux `kernel/cgroup/cgroup.c`
**Implementation:** `kernel/core/process/sigma_cgroup.c`

Provides hierarchical resource tracking and hard-limiting for process isolation. Enables the kernel to group tasks and strictly enforce physical memory ceilings (`memory_limit_bytes`) and proportional CPU scheduling weights across different execution tiers.

## 39. Performance Events (Perf)

**Inspirations:** Linux `kernel/events/core.c`, FreeBSD `hwpmc_mod.c`
**Implementation:** `kernel/core/diag/sigma_perf.c`

An abstraction over hardware Performance Monitoring Units (PMUs). Allows the kernel to program MSRs and cleanly read exact hardware metrics, such as raw CPU cycles, retired instructions, and cache misses, for real-time workload profiling.

## 40. IOMMU (Input-Output Memory Management Unit)

**Inspirations:** Linux `drivers/iommu/iommu.c`, FreeBSD `intel_drv.c`
**Implementation:** `kernel/core/hardware/sigma_iommu.c`

Provides absolute DMA remapping and isolation. Maps physical RAM into isolated I/O Virtual Addresses (IOVA) specific to individual hardware domains. This prevents rogue or compromised PCI devices from corrupting kernel memory via unauthenticated DMA requests.

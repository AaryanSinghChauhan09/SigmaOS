# Linux & BSD Innovations Implemented in SigmaOS

This page catalogs concrete features and design patterns borrowed and adapted
from Linux, FreeBSD, OpenBSD, NetBSD, and other Unix-like operating systems.

***

## Kernel Scheduler

### EEVDF (Earliest Eligible Virtual Deadline First)

*   **Source**: Linux 6.6+ (replacing CFS)
*   **Location**: `src/kernel/scheduler.rs`
*   **Features**:
    *   Virtual deadline tracking per process
    *   Nice-value weighted scheduling
    *   Interactive process detection and priority boost
    *   Thermal-aware time slice adjustment
    *   Multi-core affinity masks

### EDF (Earliest Deadline First) — Real-Time

*   **Source**: POSIX real-time scheduling, Linux `SCHED_DEADLINE`
*   **Features**: Hard-deadline tasks with EDF deadline field

***

## Memory Management

### Buddy Allocator

*   **Source**: Linux `mm/page_alloc.c` buddy system
*   **Location**: `src/klib/buddy_allocator.rs`
*   **Features**:
    *   Power-of-2 block splitting/coalescing
    *   Lazy page reclaim (inspired by Linux `kswapd`)
    *   Fragmentation ratio tracking
    *   Memory leak detection (Valgrind/LeakSanitizer-inspired)

### 4-Level Page Tables (PML4)

*   **Source**: Linux/x86\_64 page table design
*   **Location**: `src/klib/paging.rs`
*   **Features**:
    *   PML4 → PDPT → PD → PT hierarchy
    *   Copy-on-Write (CoW) page support
    *   Transparent Huge Pages (THP) flag
    *   Multi-core TLB shootdown protocol
    *   User/kernel isolation via privilege levels

### Copy-on-Write Memory

*   **Source**: Linux fork + CoW
*   **Features**: Marked via `cow` bit in page table entry, lazy copy on write fault

***

## Security

### Capability-Based Security

*   **Source**: Linux capabilities, FreeBSD Capsicum
*   **Location**: `src/security/capability.rs`
*   **Features**: Fine-grained capability tokens per process

### Pledge/Unveil Analogues

*   **Source**: OpenBSD `pledge(2)` and `unveil(2)` syscalls
*   **Location**: `src/security/` (planned `src/security/pledge.rs`)
*   **Description**: Restrict process syscall surface at runtime

### Namespace Isolation

*   **Source**: Linux namespaces (`clone()` flags)
*   **Features**: Process, network, filesystem namespace isolation

***

## GPU Driver

### Timeout Detection & Recovery (TDR)

*   **Source**: Windows WDDM TDR + Linux DRM GPU reset
*   **Location**: `src/drivers/gpu.rs`
*   **Features**: Simulated GPU hang detection, pipeline reconstruction, error recovery

### Vulkan-Inspired Command Buffer

*   **Source**: Vulkan API design
*   **Features**: BindPipeline, DrawIndexed, DrawRect, Present commands

***

## Filesystem

### POSIX VFS Layer

*   **Source**: Linux VFS (Virtual Filesystem Switch)
*   **Location**: `src/filesystem/`
*   **Features**: Unified filesystem interface, multiple backend support

***

## Networking

### Packet Processing

*   **Source**: BSD socket layer, Linux netfilter
*   **Location**: `src/net/` (in progress)

***

## Driver Framework

### Unified OOP Driver Model

*   **Source**: Linux device model (`struct device`), FreeBSD devclass
*   **Location**: `src/drivers/peripheral.rs`
*   **Features**:
    *   `PeripheralDevice` trait with standardized interface
    *   Device generations (Legacy, Modern, NextGen)
    *   Power state management (`On`, `Off`, `Standby`)
    *   Unified read/write/initialize/shutdown interface

***

## Power Management

### CPU Frequency Governor

*   **Source**: Linux CPUfreq governors (ondemand, schedutil)
*   **Location**: `src/power/governor.rs`
*   **Features**: Adaptive frequency scaling based on load

***

## Package Management

### Universal Package System

*   **Source**: Inspired by Arch pacman, Gentoo portage, BSD ports
*   **Location**: `src/sigpkg/`
*   **Features**: Source builds, binary packages, dependency resolution

***

## Related Reading

*   [Linux Kernel Documentation](https://www.kernel.org/doc/html/latest/)
*   [FreeBSD Handbook](https://docs.freebsd.org/en/books/handbook/)
*   [OpenBSD pledge(2)](https://man.openbsd.org/pledge.2)
*   [Capsicum — FreeBSD capabilities](https://man.freebsd.org/cgi/man.cgi?capsicum\(4\))

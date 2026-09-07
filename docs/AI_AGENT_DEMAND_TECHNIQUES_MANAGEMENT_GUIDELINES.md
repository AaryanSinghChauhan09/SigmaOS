# SigmaOS AI Agent Various Demand Techniques Operation Management Guidelines

## 1. Overview
SigmaOS implements adaptive, on-demand kernel and userland resource management frameworks operated autonomously by AI system agents (such as `DemandTechniquesGovernor`, `DemandPagingEngine`, `DemandExecutableLoader`, and `OnDemandDriverHotplugger`). These guidelines define demand paging (lazy zero-fill-on-demand), Copy-on-Write (COW) memory sharing, demand-paged ELF/APE executable loading, dynamic library Procedure Linkage Table (PLT/GOT) lazy symbol binding, and on-demand device driver loading for AI agents in SigmaOS.

## 2. Core Demand Techniques Management Principles

### 2.1 Demand Paging & Lazy Zero-Fill Page Allocation
- **Lazy Page Allocation**: Physical page frame allocations for virtual memory mappings (`mmap` / `brk`) are deferred until the process accesses an unmapped virtual address.
- **Page Fault Handling**: Upon a demand page fault (`#PF`), `DemandPagingEngine` allocates an order-0 physical page frame ($4\text{KB}$) from the PMM bitmap, maps the PML4 entry, zeroes the page contents, and resumes execution seamlessly.

### 2.2 Copy-on-Write (COW) Memory Sharing
- **Process Forking Optimization**: During process creation (`fork`), parent and child page table entries are marked read-only with a COW flag (`PTE_COW`).
- **Demand Copying on Write Fault**: When either process writes to a shared page, a write page fault triggers `DemandPagingEngine` to allocate a duplicate page frame, copy the payload, and update permissions to read-write (`PTE_RW`).

### 2.3 Demand-Paged ELF Executable Loading
- **Memory-Mapped Executables**: ELF and Cosmopolitan Actually Portable Executable (APE) binary segments (`.text`, `.rodata`, `.data`) are mapped into memory via file-backed demand `mmap`.
- **Fault-Driven Segment Loading**: Executable pages are read from storage into page cache on demand when instruction pointers access un-cached code segments.

### 2.4 Dynamic Symbol Binding (PLT / GOT)
- **Lazy Symbol Relocation**: Dynamic library symbol resolution uses Procedure Linkage Table (PLT) and Global Offset Table (GOT) trampolines (`LinuxBsdAbiBridge`). Symbols are resolved on first call rather than at startup, accelerating application launch times.

### 2.5 On-Demand Driver & Service Loading
- **Module Auto-Loading**: Hardware hotplug events (`BsdDevdHardwareEventDispatcher`) automatically load necessary device drivers (`modprobe` aliases) on demand upon device insertion.
- **On-Demand Service Activation**: Socket-activated daemons (`RunsvSupervisor` / `OpenRC`) spawn background services on demand upon receiving incoming connection requests.

---
*Maintained by the SigmaOS Kernel, Memory & Systems Steering Committee.*

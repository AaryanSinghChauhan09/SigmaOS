# SigmaOS AI Agent Physical Memory & Container Zones Management Guidelines

## 1. Overview
SigmaOS implements physical memory hardware zones and virtualized container zones managed autonomously by AI system agents (such as `PhysicalMemoryZoneGovernor`, `SmartOsZoneEngine`, and `FreeBsdZoneQueueOptimizer`). These guidelines define physical memory hardware boundaries (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`), FreeBSD VM zone queues, and SmartOS ZFS-backed ephemeral Zone containers for AI agents in SigmaOS.

## 2. Core Zones Management Principles

### 2.1 Physical Memory Hardware Zones
- **`ZONE_DMA` (< 16MB)**: Reserved for legacy ISA 24-bit direct memory access peripherals.
- **`ZONE_DMA32` (< 4GB)**: Reserved for 32-bit PCI device DMA transfers.
- **`ZONE_NORMAL` (> 4GB)**: Used for general 64-bit kernel page frame allocations, process virtual memory, and high-performance DMA buffers.

### 2.2 SmartOS-Style Virtualized Zones
- **Ephemeral ZFS-Backed Zones**: AI agents manage SmartOS-inspired Zone containers (`SmartOsZoneEngine` in `src/distro/missing_distro_innovations.rs`) using ZFS datasets, `vmadm` VM administration, and `imgadm` image management.
- **Zone Resource Bounds**: Zones enforce strict CPU thread pinning (`NumaAffinityMap`), memory caps, and ZFS ARC cache limits.

---
*Maintained by the SigmaOS Memory & Virtualization Steering Committee.*

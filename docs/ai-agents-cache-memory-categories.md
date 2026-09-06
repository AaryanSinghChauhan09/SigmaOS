# 🇸🇴 AI Agents Cache Memory Categories & Taxonomy in SigmaOS

## Executive Overview

SigmaOS classifies all cache memory structures across the operating system into **five distinct Cache Memory Categories**, managed autonomously by specialized microkernel and userland AI Cache Agents. Operating inside SigmaOS's zero-dependency `#![no_std]` Rust architecture, these agents continuously monitor cache hit/miss telemetry, pressure stall information (PSI), and access frequency heatmaps (DAMON) to optimize memory allocation, eliminate cache thrashing, and maintain ultra-low context switch latency (<0.12 µs).

Drawing inspiration from Linux memory access frameworks and BSD virtual memory subsystems, the SigmaOS Cache Taxonomy spans hardware registers down to AI LLM context token caches.

---

## 📊 Taxonomy Matrix of Cache Memory Categories

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                             SigmaOS Cache Memory Taxonomy                                │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ Category 1:     ││ Category 2:     ││ Category 3:     ││ Category 4/5:   │
│ Hardware Caches ││ Kernel Objects  ││ Storage & Swap  ││ Network & AI    │
│ (L1/L2/L3/TLB)  ││ (SLAB/VFS Page) ││ (ZFS ARC/zswap) ││ (eBPF/LLM/GTK)  │
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

| Category | Cache Tier | Scope | Primary Linux/BSD Paradigm | Governing AI Agent |
|---|---|---|---|---|
| **1. Hardware** | L1I/L1D, L2, L3 (LLC), TLB | Processor / MMU | Intel CAT / AMD L3 QoS, Linux PMU counters | `CpuCacheAllocatorAgent` |
| **2. Kernel Objects** | SLAB/SLUB, VFS Page Cache, Dentry/Inode | Microkernel / VFS | Linux DAMON, PSI, `kmem_cache_shrink` | `SlabCacheGovernorAgent` & `PageCacheGovernorAgent` |
| **3. Storage & Swap** | ZFS ARC/L2ARC, HAMMER2 Extents, zswap, zram | Block / Storage | FreeBSD ZFS ARC, Linux zswap/zram | `ZfsArcAdaptiveAgent` & `CompressedCacheAgent` |
| **4. Subsystem** | eBPF Sockmap/XDP, mDNS/SSDP Discovery, Sysctl MIB | Networking & IPC | Linux eBPF TC/XDP, ZeroConf mDNS | `NetworkQosAgent` |
| **5. Userland & AI** | GTK Render Trees, Tensor Memory, ACP/MCP Token Cache | Userland & LLMs | Zenith GTK4/Adw, LocalLlmDaemon, QwenPaw | `ContextCacheAgent` |

---

## 🗂️ Detailed Breakdown by Category

### Category 1: Hardware Cache Memory (L1, L2, L3 & TLB)
- **Structures:** L1 Instruction/Data Cache (32KB/core), L2 Unified Cache (512KB-1MB/core), L3 Shared Last-Level Cache (LLC, 16-64MB/socket), Translation Lookaside Buffer (TLB).
- **Paradigms Absorbed:**
  - **Linux Hardware PMU Telemetry:** Uses Performance Monitoring Unit (PMU) counters (`cache-misses`, `LLC-load-misses`, `TLB-misses`).
  - **Intel CAT / AMD L3 QoS Way Partitioning:** Allocates reserved L3 cache ways for latency-sensitive Zenith Desktop threads.
- **Autonomous Agent Action:**
  - Dynamically adjusts L3 cache allocation bitmasks to prevent background compilation tasks from evicting interactive UI render data.
  - Triggers inter-core thread migrations to prevent cross-socket NUMA cache line invalidation stalls.

### Category 2: Kernel Structure & Object Caches
- **Structures:**
  - **Kernel SLAB/SLUB Object Caches:** Pre-allocated pools for `kmalloc-128`, `inode_cache`, `dentry_cache`, `socket_cache`, `task_struct`.
  - **VFS Page & Buffer Cache:** Memory pages caching file I/O blocks.
- **Paradigms Absorbed:**
  - **Linux DAMON (Data Access Monitor) & PSI:** Evaluates memory pressure stalls (`PSI_SOME`, `PSI_FULL`) and access heatmaps.
  - **OpenBSD Lazy Zeroing:** Zeroes freed cache pages asynchronously in background idle loops.
- **Autonomous Agent Action:**
  - Triggers automated SLAB cache shrinking (`kmem_cache_shrink`) when idle object ratios exceed 40%.
  - Proactively reclaims cold VFS page caches during heavy background processing.

### Category 3: Storage, Swap & Compressed Memory Caches
- **Structures:**
  - **FreeBSD ZFS ARC (Adaptive Replacement Cache):** Dual MRU (Most Recently Used) and MFU (Most Frequently Used) eviction queues.
  - **ZFS L2ARC:** NVMe/SSD read cache extension.
  - **zswap & zram:** Compressed write-through swap buffers and RAM block swap devices.
- **Paradigms Absorbed:**
  - **FreeBSD ZFS ARC Rebalancing:** Dynamically sizes `vfs.zfs.arc_max` based on physical RAM availability.
  - **Linux zswap/zram Compression:** Uses LZ4/ZSTD algorithms for in-memory compression.
- **Autonomous Agent Action:**
  - Dynamically expands zswap pool allocation when RAM pressure builds, avoiding slow disk swapping.
  - Promotes frequent metadata extents into ZFS L2ARC NVMe devices to accelerate file system search operations.

### Category 4: Subsystem & Protocol Caches
- **Structures:**
  - **eBPF Sockmap & XDP Routing Cache:** Fast-path socket redirection tables.
  - **Sovereign Network Discovery Cache:** mDNS/DNS-SD, UPnP/SSDP, LLMNR host resolution cache.
  - **POSIX/BSD Sysctl MIB Cache:** Cached MIB tree entries for microsecond kernel parameter lookup.
- **Paradigms Absorbed:**
  - **Linux eBPF Socket Redirection:** Zero-copy socket-to-socket forwarding bypassing the TCP/IP stack.
  - **FreeBSD VNET Network Stack Caching:** Isolated routing tables and ARP/NDP caches per network jail container.
- **Autonomous Agent Action:**
  - Flushes stale mDNS/SSDP discovery entries upon network interface state changes.
  - Pre-populates eBPF TC/XDP routing maps to accelerate high-bandwidth streaming channels.

### Category 5: Userland, UI & AI Context Caches
- **Structures:**
  - **Zenith GTK / Libadwaita Render Tree Cache:** Cached UI scene graphs, glyph textures, and CSS providers.
  - **Local LLM Prompt & Context Token Cache (ACP/MCP):** Key-Value (KV) attention token cache for local AI models (`LocalLlmDaemon`, `QwenPaw`, `KimiCodeAgent`).
  - **Tensor Memory Cache:** Pre-allocated GPU/NPU tensor buffers.
- **Paradigms Absorbed:**
  - **Model Context Protocol (MCP) Context Caching:** Reuses KV attention matrices across consecutive agent prompt turns.
- **Autonomous Agent Action:**
  - Compresses or offloads idle LLM KV context caches to zswap when user switches from AI workspace to Zenith Desktop games.
  - Purges GTK render texture caches when display monitors enter sleep mode.

---

## 🔒 Safety, Attestation & ACP/MCP Protocol Integration

1. **Agent Client Protocol (ACP) Control:** Users and developer tools query or override cache policies across all 5 categories via `sigma-sh` or Zenith Desktop Control Center.
2. **Model Context Protocol (MCP) Isolation:** Context caches expose telemetry to local AI models while enforcing OpenBSD `unveil` file boundaries.
3. **Post-Quantum Attestation:** Agent cache tuning policies are signed using Dilithium-5 post-quantum signatures to prevent unauthorized policy tampering.

---

## 🛠️ System Inspection Commands

Command-line inspection via `sigma-sh`:

```bash
# View cache memory usage breakdown across all 5 categories
sigma-sh> ai-agent inspect cache-categories

# Inspect hardware L1/L2/L3 cache miss telemetry
sigma-sh> ai-agent inspect hardware-cache

# Query ZFS ARC / L2ARC hit/miss distribution (Category 3)
sigma-sh> ai-agent inspect zfs-arc-cache

# Purge userland UI render and LLM KV token caches (Category 5)
sigma-sh> ai-agent purge-userland-caches
```

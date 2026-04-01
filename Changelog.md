# SigmaOS: Version By Version Changelog

This is the definitive historical changelog tracking the brutal evolution of the SigmaOS ecosystem. Our trajectory has rarely been about "adding features"; rather, it is a history of stripping away middle-men, deleting dependencies, and enforcing absolute bare-metal sovereignty.

---

## 🟢 v1.0.0 "Absolute Sovereignty" (Current Release)
*The milestone achieving complete independence from legacy software paradigms.*
- **[Architectural Pivot] Shard-On-Demand (SOD):** Banned all Dynamic Link Libraries (`.so` / `.dll`). Every executable is now dynamically mapped into Ring-0 memory as a raw `.c` payload upon invocation. 
- **[Feature] Domain Dominance Matrix:** Integrated the `indian_law.c` and `ncert_core.c` native shards. Mathematically rendering online subscription portals obsolete.
- **[Feature] Legal Execution Module:** Embedded the new Bharatiya Nyaya Sanhita (BNS), BNSS, and BSA procedural checks deep into the kernel. 
- **[Core] Zero-Copy DMA Networking:** `SovereignNetMesh.c` officially bypassed all standard BSD socket layers to hook network queues directly to the NIC hardware ring.
- **[UX] Custom Keybindings:** Stabilized the `keyboard_master.c` to hook kernel-level shortcuts (Alt+S, Alt+E) instantaneously.

---

## 🟡 v0.9.0 "The Persona Paradigm"
*Structuring contextual boundaries.*
- **[Core] Contextual Hypervisors:** Introduced the core 5 Personas (Developer, Gamer, Forensic Analyst, Researcher, Student). CPU governors (TSC loops) now alter dynamically when a persona shifts.
- **[Removal] Cloud-Sync Severed:** Actively eliminated all cloud-backup daemons. Replaced with `backup_manager.c` utilizing local-only air-gapped, `.tar.gz` and SHA-256 encrypted snapshots.
- **[Optimization] Aether-Shard Bootstrapping:** Dropped boot latencies beneath 0.1s by rewriting the early `SigmaCore.asm` 64-bit Long Mode handover.

---

## 🟠 v0.8.0 "The Great Purge"
*The most violent refactoring in OS history.*
- **[Removal] `glibc` Extinction:** Completely purged `#include <stdio.h>` and all standard C library headers. 
- **[Core] SovereignLibC:** Built `SovereignLibC.h` from scratch utilizing strict bare-metal `SYS_WRITE` and `SYS_READ` bounds. 
- **[Memory] The Death of `malloc`:** Garbage collection and standard heap allocators were annihilated. Introduced the native Physical Memory Manager (PMM) and Slab Allocator (`sigma_slab_alloc`).

---

## 🔴 v0.7.0 "The Silicon Lock"
*Targeting jitter and establishing deterministic latency constraints.*
- **[Removal] Background Daemons Exiled:** Deleted the system indexer. Finding files natively no longer utilizes background telemetry or indexing spikes.
- **[Core] O(1) Sovereign VFS:** Deployed algorithmic hash-mapping within the Virtual File System natively.
- **[Feature] AI Kernel Zenith:** Connected the GPU tensor processors directly to the `omni_shell`, establishing the base intelligence layer.

---

## 🟣 v0.4.0 "The GUI Annihilation"
*Escaping web-wrapper bloat.*
- **[Removal] Electron Shell Destroyed:** We realized idle DOM rendering via Electron wrappers consumed 1.2GB of RAM. The entire UI stack was burned to the ground.
- **[Feature] Fractional JS Orchestrator:** Engineered the initial "Zenith-Gold" webview using purely native lightweight Javascript mapping against underlying C arrays, cutting UI overhead to under 40MB.

---

## ⚪ v0.1.0 "Prototype Aether"
*Proof of Concept.*
- **[Core] Initial C11/Assembly Hybrid Kernel**
- **[Feature] Basic POSIX Translation Layer** (later removed for proprietary locking mechanisms).
- **[UX] The first iteration of the Omni Shell.**

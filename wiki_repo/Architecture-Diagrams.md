# 🏗️ Architecture Diagrams: SigmaOS Core Layouts

This document details the architectural layouts, pipelines, and structural models governing SigmaOS's subsystems.

---

## 1. Microkernel Shards Architecture

SigmaOS decomposes standard OS capabilities into isolated, hot-swappable microkernel shards. This eliminates monolithic single-point-of-failure vulnerabilities.

```mermaid
graph TD
    %% Userland Interface
    Userland[Userland Applications] -->|Syscall Capability Gate| S-SEC[S-SEC: Security Shard]

    %% Security Layer Verification
    subgraph Security Gate
        S-SEC -->|Pledge/Unveil Verification| S-SEC-Verifier{Tokens Valid?}
    end

    %% Sovereign IPC Transaction Bus
    S-SEC-Verifier -->|Yes: Send Message| Bus[Sovereign IPC Bus]
    S-SEC-Verifier -->|No: Abort/Panic| Terminate[Access Denied]

    %% Microkernel Core Shards
    subgraph Kernel Shards (Hot-Swappable)
        Bus <--> S-SCHED[S-SCHED: EEVDF & Round-Robin Scheduler]
        Bus <--> S-MM[S-MM: Buddy Allocator & Paging]
        Bus <--> S-FS[S-FS: Distributed Filesystem VFS]
        Bus <--> S-NET[S-NET: Zero-Trust Stack]
        Bus <--> S-AI[S-AI: LLM Task Planner]
    end
```

---

## 2. SigmaFS Transactional Log-Structured Layout

SigmaFS organizes physical block storage as a transactional log of Merkle-tree states, enabling instant snapshots and sub-millisecond rollbacks.

```
+---------------------------------------------------------------------------------+
|                                 SigmaFS Merkle Tree                             |
+---------------------------------------------------------------------------------+
|                               [Root Node (Hash)]                                |
|                               /                \                                |
|                   [Dir Inode Hash]          [File Inode Hash]                   |
|                     /          \               /          \                     |
|              [Data Block]  [Data Block]  [Data Block]  [Data Block]             |
+---------------------------------------------------------------------------------+
                                     |
                                     v (Transaction Commit / Log Roll)
+---------------------------------------------------------------------------------+
|                               Log-Structured Blocks                             |
|  [Block N (Old Root)] -> [Block N+1 (Modified Data)] -> [Block N+2 (New Root)]   |
+---------------------------------------------------------------------------------+
```

---

## 3. Zenith Display Compositor & Low-Latency Audio Pipeline

Zenith Desktop employs a unified Vulkan-native rendering pipeline coupled with low-latency audio HRTF mixing page-shared via S-MM.

```
+------------------------+      +------------------------+
|      Vulkan GUI        |      |      HRTF Audio        |
|  Zenith Window Manager |      |  Spatial Audio Mixer   |
+------------------------+      +------------------------+
            \                                /
             \                              /
              v                            v
+--------------------------------------------------------+
|                      S-MM Shard                        |
|  [Zero-Copy Shared Video / Audio Buffer Memory Pages]  |
+--------------------------------------------------------+
                            |
                            v (DMA Transfer)
+--------------------------------------------------------+
|                   Hardware Drivers                     |
|  [VGA/VESA Framebuffer GPU]  [SoundBlaster/HDA Audio]  |
+--------------------------------------------------------+
```

---

## 4. `sigmapkg` Content-Addressed Transaction Flow

`sigmapkg` ensures package installation integrity using content-addressed storage (CAS) and DPLL dependency resolution.

```
+-------------------------+
|      Package Recipe     | -> [Verification of cryptographic Dilithium-5 key]
+-------------------------+
            |
            v
+-------------------------+
|     DPLL SAT Solver     | -> [Calculates non-conflicting dependency graph]
+-------------------------+
            |
            v
+-------------------------+
| Content-Addressed Store | -> [Hashes files, downloads payloads to CAS, links paths]
+-------------------------+
            |
            v
+-------------------------+
|     Transaction Bus     | -> [Atomic commit of links. On failure: instant rollback]
+-------------------------+
```

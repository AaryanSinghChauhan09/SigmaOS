# SigmaOS Wiki: Architecture Overview

Welcome to the official ground-truth technical wiki for SigmaOS. This document provides an exhaustive description of the sovereign 10-layer lattice architecture, its core key architectural features, scheduling engines, and sovereign design principles.

---

## 🏛️ 1. The Sovereign 10-Layer Lattice Architecture

SigmaOS is structured as a hierarchical lattice, ensuring that each layer has a strictly defined responsibility and zero-dependency upward.

```
+-----------------------------------------------------------------------------+
| Layer 10: Sovereign Nexus - Enterprise Suite (ERP/CRM, Productivity)        |
+-----------------------------------------------------------------------------+
| Layer 9: Ecosystem Abstraction (S99 - POSIX, Linux binary compatibility)    |
+-----------------------------------------------------------------------------+
| Layer 8: Sovereign Claw AI Automation (Agent Gateway, Live Canvas)          |
+-----------------------------------------------------------------------------+
| Layer 7: Sovereign AI & Orchestration (Intent-to-Shard Dispatch)            |
+-----------------------------------------------------------------------------+
| Layer 6: Zenith UI & Morphic Shell (Wayland-Native, Vulkan Shader UI)       |
+-----------------------------------------------------------------------------+
| Layer 5: Sovereign Package Ecosystem (sigma-pkg, Repositories, CAS Store)   |
+-----------------------------------------------------------------------------+
| Layer 4: Capability-Gated Security (PQC, TPM 2.0, Pledge/Unveil, MAC)       |
+-----------------------------------------------------------------------------+
| Layer 3: Sovereign Virtual Filesystem (VFS, Descriptor Handles, Locks)       |
+-----------------------------------------------------------------------------+
| Layer 2: Genesis Kernel & Scheduling (IRQ/IDT, S-MM Slab, SHS Scheduler)    |
+-----------------------------------------------------------------------------+
| Layer 1: Universal Hardware Abstraction (HAL, Direct Silicon, Interrupts)  |
+-----------------------------------------------------------------------------+
```

### Layer Descriptions

*   **Layer 1: Universal Hardware Abstraction (HAL)**
    *   Direct silicon interfaces (NVMe, USB, VGA / VESA / KMS framebuffers).
    *   Platform-specific driver initialization (e.g., PCI-e, cellular modems, SoC clocks).
    *   Hardware capability detection, capability-checked device registers, and reporting.
    *   Low-level interrupt routing and physical CPU core IRQ management.
*   **Layer 2: Genesis Kernel & Scheduling**
    *   IRQ/IDT handling, exception vectors, and dynamic interrupt dispatch.
    *   Memory management (S-MM lock-free slab allocator, Buddy page frame allocator).
    *   SHS (Sovereign Hybrid Scheduler) CFS + EEVDF real-time execution engine.
    *   Process and lightweight userland Shard lifecycle supervision.
    *   Privileged system call interface.
*   **Layer 3: Sovereign Virtual Filesystem**
    *   Capability-backed filesystem treating all resources (files, sockets, hardware) as unified handles.
    *   Unified VFS abstraction for multiple filesystem types (SigmaFS, NTFS, Ext4, FAT32, APFS).
    *   Descriptor handle allocation and sandboxed process namespace isolation.
    *   Path resolution and target namespace management (unveil).
    *   Asynchronous file locking and safe synchronization channels.
*   **Layer 4: Capability-Gated Security**
    *   NIST FIPS 203/204 Post-Quantum Cryptographic operations (Kyber-1024, Dilithium-5 KEM/signatures).
    *   Hardware-enforced TPM 2.0 remote attestation and local key escrow management.
    *   Capability-based access control and system-call vocabulary limiting (pledge).
    *   Mandatory Access Control (MAC) enforcement and multi-category security labels.
    *   Secure boot chain verification starting directly from UEFI.
*   **Layer 5: Sovereign Package Ecosystem**
    *   Dependency Directed Acyclic Graph (DAG) management via `sigma-pkg` and `SatSolver`.
    *   Content-Addressed Storage (CAS) package store and signature verification.
    *   Strictly hermetic, reproducible, standard-library-free compilation pipelines.
    *   Dynamic package repository mirroring and decentralized S-CDN.
    *   Delta update support for bandwidth-constrained nodes.
*   **Layer 6: Zenith UI & Morphic Shell**
    *   Zero-Wayland/X11 direct-to-hardware framebuffer compositing.
    *   Vulkan-accelerated Morphic shader-based rendering and window transitions.
    *   Low-latency input device handling, pressure curves, and mouse pointer translation.
    *   Dynamic tiling matrices and screen-space layout composition.
    *   Direct display driver and KMS monitor output integration.
*   **Layer 7: Sovereign AI & Orchestration**
    *   High-level intent-to-shard dynamic task dispatch system.
    *   Local LLM model integration for system telemetry and filesystem indexing.
    *   Predictive, non-reactive P-state and hardware resource scaling.
    *   Workload classification, pipeline trace analysis, and CPU core pinning.
    *   AI-assisted system monitoring and administrative decision making.
*   **Layer 8: Sovereign Claw AI Automation**
    *   Autonomous AI agent gateway and multi-agent plan scheduler.
    *   Multi-step task goal execution, execution tracing, and self-correction.
    *   Intent decomposition, subtask planning, and tool integration.
    *   Interactive capability validation and safe VM sandboxing for agent tasks.
    *   Live canvas conversational terminal interface.
*   **Layer 9: Ecosystem Abstraction (S99)**
    *   POSIX-compatible translation layer and API mapping inside isolated containers.
    *   Linux ELF binary compatibility (loading and executing unmodified Linux files).
    *   On-the-fly system call translation.
    *   Dynamic library compatibility shims and glibc emulation boundaries.
    *   Legacy enterprise application support.
*   **Layer 10: Sovereign Nexus - Enterprise Suite**
    *   Integrated, high-density Enterprise Resource Planning (ERP) and CRM suites.
    *   Standard-library-free, zero-dependency productivity (Office) suites.
    *   Professional development tools and compilers.
    *   Business process automation with smart-contract verified ledgers.
    *   Enterprise data cataloging, lineage tracking, and Merkle database audits.

---

## ⚙️ 2. Key Architectural Features

### ⏱️ SHS (Sovereign Hybrid Scheduler)
Merges the stability of Fedora's CFS with the priority-based preemptive scheduling of Windows:
*   **Real-Time Prioritization:** Processes are classified into hard real-time (EDF), interactive (CFS), and batch queues.
*   **Fair Share Allocation:** Uses virtual runtime weights to ensure fair CPU time slice distributions.
*   **AI-Enhanced Workload Prediction:** Local models anticipate impending compile or rendering tasks and scale up scheduling quantum parameters beforehand.
*   **Adaptive Quantum Management:** Quantum budgets are dynamically allocated and strictly enforced via high-precision hardware TSC timers (`rdtsc`).

### 🔄 Snapshot & Restore
Combines openSUSE Snapper-style CoW snapshots with Windows-style System Restore checkpoints:
*   **Absolute State Recovery:** Allows capturing and restoring system images at any lattice checkpoint.
*   **Instant Rollback Capability:** System states can be rolled back atomically in under 1ms.
*   **Space-Efficient Storage:** Snapshots are managed as Copy-on-Write (CoW) nodes over physical disk Merkle trees, sharing identical immutable blocks.
*   **Boot-Time Verification:** Every snapshot is cryptographically verified against signature registries before launching.

### 🛡️ Zero-Trust Inter-Shard Communication
All inter-shard communication in v15.0 is strictly Zero-Trust:
*   **Capability Verified:** Every IPC transaction must carry a valid, un-expired `CapabilityToken`.
*   **PQC-Encrypted by Default:** Data frames are encrypted using Kyber-1024/Dilithium-5 keys.
*   **Origin Authentication Required:** Senders must authenticate their physical shard identity.
*   **Audit Logging:** Every transition is logged directly into the append-only cryptographic ledger.

### ⚡ Fast Startup Mechanism
SigmaOS implements a Fast Startup mechanism inspired by Windows:
*   **Silicon-Direct State Serialization:** Microkernel state is serialized directly to storage at shutdown.
*   **Critical Driver Preservation:** Driver shard memory states are kept intact inside pre-allocated NVMe sectors.
*   **Traditional Bypass:** Bypasses legacy BIOS/UEFI hardware re-initialization during boot.
*   **0.8s Restore:** System boots and is fully responsive in under 0.8 seconds.

### 🧠 Neural Memory Management
The memory manager uses a Neural Network (S09) to predict shard access:
*   **Predictive Pre-Loading:** Analyzes user commands to predict which shards will be required next.
*   **NVMe to DRAM Pre-Fetching:** Pre-loads predicted shards in the background before they are explicitly requested.
*   **Near-Zero Latency:** Dramatically reduces virtual memory page-fault latencies.
*   **Adaptive Patterns:** Learns and adjusts to user behavior over time.

### 🎮 GPU-Accelerated UI
The Zenith compositor utilizes Vulkan compute integration:
*   **Direct-to-GPU Splicing:** Offloads visual window transforms and filters directly to GPU pipelines.
*   **Fluid 120Hz Output:** Maintains a smooth 120Hz interface even under heavy parallel compile loads.
*   **SIMD Shaders:** Employs hardware-level SIMD shaders for adaptive contrast and high-speed magnification.
*   **Optimized Memory Bandwidth:** Zero intermediate context-switching and buffer copy actions.

---

## 📑 3. The Core Scheduling Algorithm

1.  **Timer Interrupt (IRQ0):** Triggers every 1ms (configurable).
2.  **Context Save:** The active task's registers are serialized and saved via inline Assembly instructions.
3.  **Selection:** SHS selects the next task based on virtual runtime and AI-predicted priorities.
4.  **Quantum Enforcement:** Budget enforcement is monitored and verified using the `rdtsc` instruction.
5.  **Context Restore:** Resumes execution of the newly selected process.

---

## 🤖 4. Embedded AI Integrations

*   **AI Watchdog (S09):** Monitors scheduling traces and memory footprints, predicting resource contention and preemptively triggering rollbacks before a panic can occur.
*   **Predictive Scheduler:** Anticipates upcoming workload patterns, optimizing task placements across CPU core sockets.
*   **Memory Predictor:** Forecasts virtual page access paths, executing pre-fetching strategies.
*   **Anomaly Detection:** Flags unusual inter-shard communication sequences and mitigates threat behaviors.

---

## 📊 5. Logging & Telemetry

All sovereign events are logged in structured JSON/CSV formats:
*   **Sovereign Data Science Shard (S17):** Powers the high-density `sigma-top` dashboard.
*   **Predictive Analytics:** Enables proactive, telemetry-driven system tuning.
*   **Structured Logging:** Outputs machine-readable event streams.
*   **Real-Time Performance Metrics:** Tracks allocator fragmentation, network queue latency, and core temperatures.

---

## 🛠️ 6. Hardware Abstraction Layer

The HAL (S04) implements plug-and-play detection:
*   **Atomic Driver Shards:** Drivers are isolated as individual, restartable Ring 3 threads.
*   **Fallback Drivers:** Ensures generic VGA/VESA and keyboard input availability during bootstrap.
*   **Capability Device Access:** Restricts raw port or memory access exclusively to verified driver shards.
*   **Hot-Plug Support:** Automatically initializes newly discovered PCIe or USB hardware on the fly.

---

## 🗄️ 7. Sovereign Registry

Inspired by Windows Registry but reimagined for sovereignty:
*   **Centralized Hierarchical Configuration Lattice:** Maps all device configurations and user preferences under a unified tree.
*   **Capability-Gated Access:** Access to registry branches is restricted via capability tokens.
*   **Atomic Transactions:** Configuration updates are applied atomically or rolled back cleanly.
*   **Version-Controlled History:** Maintains a transaction history, allowing instant system-state rollbacks.
*   **PQC-Encrypted Sensitive Data:** Passwords, API tokens, and private keys are encrypted using Kyber-1024.

---

## 🎯 8. Core Design Principles

1.  **Usability First:** Fluid Zenith compositor coupled with `sigma-pkg` for an intuitive user experience.
2.  **Security Next:** Hardware TPM 2.0 attestation paired with post-quantum encryption for maximum cryptographic resilience.
3.  **Resilience:** Self-healing FS scrubbers, atomic snapshots, and the AI Watchdog to guarantee absolute system uptime.
4.  **Differentiation:** Adaptive visual layers and native, edge AI automation assistants to deliver unique capabilities.

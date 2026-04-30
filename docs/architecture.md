# 🏛️ SigmaOS Architecture

This document outlines the directory structure and technical principles of the SigmaOS Sovereign Lattice.

## 📁 Directory Map

| Directory | Purpose |
| :--- | :--- |
| `include/` | Sovereign headers (`sigma_*.h`) and LibC definitions. |
| `kernel/core/` | Core sovereign shards (Scheduler, MMU, Allocator, Init). |
| `kernel/hal/` | Hardware Abstraction Layer (HAL) for silicon discovery. |
| `kernel/shards/` | Optional and specialized modular shards (AI, Networking, Storage). |
| `drivers/` | Silicon-native hardware drivers. |
| `userland/` | Zenith Desktop components and user applications. |
| `docs/` | Technical documentation and architecture specifications. |
| `scripts/` | Build tools, linting scripts, and lattice metrics. |

## 🧩 The Sharding Model (600-Shard Modular Lattice)

SigmaOS is built on the principle of **Atomic Sharding**. Instead of a monolithic kernel or a traditional microkernel, SigmaOS treats every system service as an independent "Shard".

### Shard Characteristics
1. **Isolated**: Each shard runs in its own memory boundary.
2. **Stateless**: Shards are designed for "Amnesic Execution" (see below).
3. **Reactive**: Shards communicate via the Wait-Free Atomic Exchange (WFAE) IPC.

## 🧩 The SovereignEngine Pattern (Modular Singleton)

To ensure **Modular Atomicity** and **Zero-Dependency** integrity, SigmaOS employs the `SovereignEngine` pattern. Each major kernel subsystem (MMU, SMP, Kube, AISched, IPC, Process, Orchestrator, Boot, Bluetooth, Dash, Syscall, Recover, Monitor, Entropy, Audit, etc.) is encapsulated within a C++ Singleton class.

### Pattern Benefits:
- **State Encapsulation**: Internal tracking variables (like counts, flags, registries) are kept private.
- **Thread-Safe Initialization**: The `getInstance()` method guarantees safe instantiation.
- **ABI Stability**: We expose standard C-linkage (`extern "C"`) wrapper functions to allow legacy Assembly or C components to interact with the Engine without needing C++ name mangling.


## ⚡ Technical Core

### 🛡️ Hardware Abstraction Layer (HAL)
SigmaOS performs a **Silicon Audit** during boot. The HAL scans the PCI bus and ACPI tables to identify hardware shards. Unlike Linux, which loads generic modules, SigmaOS synthesizes shard-logic specifically for the detected silicon.

### 🧠 Amnesic Memory Management
"Amnesic Memory" refers to our stateless execution strategy:
- **Stateless Shards**: Critical paths are designed to be re-entrant and side-effect free.
- **QBMP Allocator**: The Quantum-Bucket Memory Pool provides O(1) allocation without fragmentation.
- **Phantom RAM**: Memory is scrubbed using the S80 protocol immediately after a shard lifecycle ends.

### 🌐 Networking Shards
Networking is implemented as a distributed lattice:
- **Layer 0**: Silicon-native NIC orchestration.
- **Layer 1**: Sovereign Protocol Stack (Zero-Trust).
- **Lattice-Sync**: Automated state synchronization across network nodes.

---

*Σ SIGMAOS: Sovereign Architecture. Absolute Integrity.*

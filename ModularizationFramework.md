
# SigmaOS Modularization Framework


SigmaOS is built on a foundation of extreme modularity, where every system component is an isolated **Sovereign Shard**.

```mermaid
graph TD
    subgraph "Modular Core Shards (Ring 0)"
        Genesis[S01 Genesis]
        Silicon[S02 Silicon HAL]
        Orch[S03 Orchestrator]
    end
    
    subgraph "Service Layer (Sovereign Lattice)"
        Mem[S05 Memory Shard]
        Store[S06 Storage Shard]
        Net[S07 Network Shard]
    end
    
    subgraph "Plugin Layer (User-Space)"
        UI[Zenith Dashboard]
        Ext[Hot-Swap Plugins]
        WASM[S21 SafeCode WASM]
    end
    
    Genesis --> Orch
    Silicon --> Orch
    Orch --> Mem
    Orch --> Store
    UI --> Orch
    Ext --> UI
    WASM --> Orch
```


## 1. Absolute Shard Architecture

SigmaOS has achieved **Absolute Modularity**. Every system component, including the bootstrap process, the hardware abstraction layer, and the memory manager, is an isolated **Sovereign Shard**. Shards communicate exclusively via the **S03 Orchestrator** using secure handle-based IPC.


## 2. Shard Isolation & Sandboxing

To ensure absolute reliability, SigmaOS treats each shard as a containerized unit.

```mermaid
graph LR
    subgraph "Sovereign Sandbox"
        Shard[Active Shard]
        Cap[Capability Node]
        Mem[Isolated Memory]
    end
    
    Orch[S03 Orchestrator]
    HAL[HAL Shard]
    
    Shard -- IPC --> Orch
    Orch -- Validate --> Cap
    Cap -- Grant --> HAL
    Mem -- Protect --> Shard
```


### Container-Like Isolation (Docker Inspired)

Each shard operates within its own **Lattice Jail** (`suites/S13_Virtualization/shard_jails.c`). This provides:
- **Namespace Isolation**: Shards only see their own resources via S-9P.
- **Resource Limits**: Deterministic memory and CPU allocation per shard.
- **Independent Lifecycle**: Shards can be updated or restarted without affecting the global lattice state.


## 3. Fault-Tolerant Supervision (Erlang Inspired)

We implement **Supervision Trees** (`suites/S03_Orchestrator/shard_supervision.c`) to monitor shard health. If a shard crashes, its supervisor can automatically restart it using predefined strategies (One-for-One, One-for-All).


## 4. Dynamic Service Loading

Shards can be hot-swapped or loaded on-demand via the **SigmaPKG** manager and the **Sovereign UI Toolkit**. This allows for a system that evolves without reboots.


## 5. Capability-Based Isolation

Using the **S-Cap** system, shards are restricted to only the resources they need, preventing lateral movement in the event of a breach.


## Comparison Table


| Paradigm | SigmaOS Approach | Inspiration |
|----------|------------------|-------------|
| Kernel | Freestanding Lattice Shards | MINIX / Genode |
| Drivers | User-space Modular Drivers | L4 / Redox |
| Orchestration | Supervision Trees | Erlang / OTP |
| Config | Declarative JSON | NixOS / Systemd |


## 6. Service Orchestration (Systemd Inspired)

SigmaOS utilizes **S-Systemd** style unit files (suites/S03_Orchestrator/shard_units.c) to define shard dependencies and lifecycle events. Shards can specify After= and Requires= relationships, ensuring a deterministic and reliable boot sequence across the 500-shard lattice.


## 7. Personalization Widgets (Conky Inspired)

Through the **Zenith Widget System** (web_ui/widgets/), users can deploy real-time monitoring tools like the conky_widget.js to track lattice performance and system health directly on their dashboard.


## 8. Capability-Based Security (Genode Inspired)

SigmaOS implements the **S-Cap Registry** (suites/S03_Orchestrator/shard_cap_registry.c) to enforce strict hardware isolation. Shards must explicitly request and be granted capabilities (e.g., CAP_DISK_READ) via the Sovereign Orchestrator. This prevents unauthorized shards from accessing sensitive hardware resources.


## 9. Unified Virtual Filesystem (Plan 9 Inspired)

Through the **S-VFS** (suites/S06_Storage/shard_vfs.c), SigmaOS treats everything as a shard-stream. Hardware drivers (like S04_HAL_Disk), network protocols, and even the system orchestrator are mounted into a unified hierarchical namespace, allowing for seamless resource access via standard file operations.

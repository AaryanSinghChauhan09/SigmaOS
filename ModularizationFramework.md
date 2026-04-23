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

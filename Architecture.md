# Σ SIGMAOS ZENITH: SOVEREIGN ARCHITECTURE 🏗️

The architecture of SigmaOS is a **Sharded Monolith** with zero external dependencies, optimized for high-performance silicon execution.

## Component Dependency Graph (Mermaid) 📊

```mermaid
graph TD
    subgraph "Hardware Layer (Silicon)"
        H1[x86_64 CPU] --- H2[RAM/PMM]
        H2 --- H3[I/O Devices]
    end

    subgraph "Kernel Layer (Sovereign Core)"
        K1[boot.asm] --> K2[kmain.c]
        K2 --> K3[SigmaCore]

        subgraph "Memory Management"
            M1[slab.c] --- M2[vmm.c]
            M2 --- M3[pmm.c]
        end

        subgraph "Process Management"
            P1[process.c] --- P2[scheduler.c]
            P2 --- P3[signal.c]
        end

        subgraph "Filesystem"
            V1[vfs.c] --- V2[ramfs]
            V2 --- V3[procfs]
        end
    end

    subgraph "User-Space Shards"
        S1[OmniAgent] --- S2[SovereignShell]
        S2 --- S3[UserFramework]
    end

    subgraph "Web Interface (Zenith Dashboard)"
        W1[index.html] --- W2[SigmaSystem.js]
        W2 --- W3[SigmaWM.js]
        W3 --- W4[DeltaTerminal]
    end

    K3 --> V1
    K3 --> M1
    K3 --> P1
    V1 --> S1
    S1 --> W1
```

## System Standards 🛡️

- **C11**: Zero-dependency C for industrial reliability.
- **Assembly**: Performance-tight hand-optimized silicon calls.
- **Glassmorphism**: Modern dashboard aesthetics for observability.
- **Atomic Locking (B4/B7)**: System-wide thread safety.

## Memory Model (B2/B6) 🧠

- **Stack-Bottom Canary**: 0xDEADC0DE (B6 Protection).
- **Slab Allocation**: 4MB Pool, thread-safe (B2).
- **VMM Isolation**: PML4/PDP/PD/PT tables for process separation.

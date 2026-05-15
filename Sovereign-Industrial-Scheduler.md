# Sovereign Industrial Scheduler (S-SCHED)

The **Sovereign Industrial Scheduler (S-SCHED)** is the multi-tasking orchestrator for **SigmaOS v15.0 "Horizon"**. It provides deterministic, priority-based execution for the 600-shard microkernel lattice.

## Architecture

S-SCHED utilizes an **Industrial Priority-based Round Robin (IP-RR)** algorithm, ensuring that critical system shards (Watchdog, Networking) receive prioritized CPU cycles while maintaining fairness for background AI processes.

### Key Features

- **Deterministic Latency**: Sub-microsecond context switches via optimized TCB (Thread Control Block) management.
- **Priority Classes**: Dynamic priority adjustment for foreground "Zenith" UI shards.
- **Lattice Affinity**: Shards are pinned to silicon nodes to maximize cache locality and minimize IPI (Inter-Processor Interrupt) overhead.

## Implementation

The scheduler is implemented in `kernel/core/system/SovereignScheduler.cpp`.

### Core Structures

- `ThreadControlBlock (TCB)`: Stores the machine state (stack pointer, registers, priority, time-slice).
- `ReadyQueue`: A multi-level feedback queue (planned) currently operating as a prioritized linked-list for Ring-0 stability.

### API Bridge

- `sched_init()`: Cold-boot ignition of the scheduling engine.
- `sched_spawn(id, priority)`: Spawns a new industrial thread into the lattice.
- `sched_yield()`: Voluntarily relinquishes the CPU to the next optimal shard.

## Integration

S-SCHED is ignited during **Stage 5** of the **Asynchronous Shard Ignition (ASI)** plan. Core system threads spawned at boot include:
1.  **Sovereign Shell (0x1001)**: The professional CLI interface.
2.  **Sovereign Watchdog (0x2001)**: The lattice integrity guardian.
3.  **Sovereign AI Shard (0x3001)**: Background neural compute.

---
*Stay Sovereign.*

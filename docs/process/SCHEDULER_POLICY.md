# SigmaOS Scheduler Policy Specification

## Objective

Maintain the SigmaOS scheduler as a safe, observable, testable scheduling system that creates no leaks, races, starvation, or undefined behavior while connecting process models to a validated boot-to-shell path.

## Authoritative Process Lifecycle Contract

SigmaOS process states move through seven explicit lifecycle states:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### State Behavior Guarantees

| State | Required Behavior |
|---|---|
| **NEW** | PID allocated, address space and credentials initialized |
| **READY** | Placed in scheduler run queue |
| **RUNNING** | Executes on CPU with preemption defined |
| **SLEEPING** | Blocked on a wait queue with wakeup path and lost-wakeup protection |
| **STOPPED** | Suspended by signal or debugger |
| **ZOMBIE** | Exit status retained until reaped; orphan children reparented to PID 1 |
| **REAPED** | PID, kernel resources, and accounting released |

A process must never be scheduled after it exits, and a zombie must eventually be reaped.

## Scheduling Class and Algorithm

SigmaOS implements a hybrid multi-level priority queue scheduler with virtual runtime (`vruntime`) tracking inspired by CFS/EEVDF and 64 priority levels (0..63):

1. **Priority Levels**:
   - 0..15: Idle / background tasks
   - 16..31: Normal userland tasks
   - 32..47: High priority interactive tasks
   - 48..63: Real-time system services
2. **Time-Slice Quantum**: Default 10 timer ticks (100ms at 100Hz).
3. **Run-Queue Capacity**: Bounded per CPU core (default 1024 tasks). Enqueue fails closed with `CapacityExceeded` when full.
4. **Preemption Points**:
   - Timer tick expiry (`remaining_quantum_ticks == 0`)
   - Higher priority task enqueue
   - Voluntary yield or sleep
   - Preemption disabled during critical spinlock sections or interrupt handlers.
5. **Zero Runnable Tasks**:
   - When no runnable tasks exist on a core, the CPU enters the idle state (`hlt`/`cpuidle`).

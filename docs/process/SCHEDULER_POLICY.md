# SigmaOS Scheduler Policy Specification

## Objective

Maintain the SigmaOS scheduler as a safe, observable, testable scheduling system that creates no memory leaks, races, starvation, or undefined behavior while connecting process models to a validated boot-to-interactive-shell runtime path.

---

## Authoritative Lifecycle Contract

The process scheduler enforces one canonical lifecycle state machine:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### State Behavior Requirements

| State | Required Behavior |
|---|---|
| **NEW** | PID allocated, address space, and credentials initialized. Task cannot execute until admitted to READY. |
| **READY** | Placed in bounded scheduler run queue according to priority and CPU affinity mask. |
| **RUNNING** | Executes on designated CPU core with explicit preemption and time-slice quantum tracking. |
| **SLEEPING** | Blocked on wait queue with designated wakeup event path. Protected against lost wakeups via `wake_event_pending` tracking. |
| **STOPPED** | Suspended by POSIX signal or debugger intervention. Removed from run queues. |
| **ZOMBIE** | Execution terminated, exit status retained in process control block until parent reaps process. |
| **REAPED** | PID returned to allocator, process control block removed, kernel resources released completely. |

A process must **never** be scheduled after it enters ZOMBIE or REAPED state, and a ZOMBIE process must eventually be reaped or reparented to init (PID 1).

---

## Ten Core Scheduler Dimensions

### 1. Scheduling Class
SigmaOS uses a **hybrid priority / CFS-like scheduling class**. Interactive and real-time tasks utilize high multi-level priority levels, while general tasks use virtual runtime (`vruntime`) tracking to achieve red-black/fair slice execution.

### 2. Run-Queue Structure
Each CPU core maintains an independent `BoundedRunQueue` with a bounded capacity (`MAX_RUNQUEUE_CAPACITY = 1024`) across 64 discrete priority levels with 64-bit bitmap selection.

### 3. Time-Slice Policy
Time-slice quanta are allocated dynamically based on process priority and nice value (default 10 ticks per quantum for normal priority tasks). Timer ticks decrement remaining quantum; upon expiration, preemption occurs.

### 4. Priority and Nice Semantics
Nice levels range from -20 (highest priority) to +19 (lowest priority), mapped into 64 priority levels. Starvation boosting dynamically elevates processes waiting beyond the starvation threshold (`DEFAULT_STARVATION_THRESHOLD_TICKS = 50`).

### 5. Preemption Points
Preemption occurs at explicit preemption points:
- Timer interrupt quantum expiration
- Higher priority task arrival in run queue
- Voluntary yield (`sys_yield`)
- Transition to SLEEPING / STOPPED
- Preemption guards (`PreemptGuard`) released

### 6. Fairness Metric
Fairness is evaluated using virtual execution time (`vruntime`) normalized by task priority. Runnable-vs-waiting ratio is monitored continuously to ensure latency targets are met.

### 7. Starvation Prevention Strategy
Per-CPU run queues scan for runnable tasks whose wait time exceeds 50 ticks. Starved tasks receive temporary priority boosting to level 15 to ensure bounded latency.

### 8. Priority-Inversion Handling
Priority inheritance and priority boost mechanisms prevent unbounded priority inversion when lower priority tasks hold synchronization locks required by higher priority tasks.

### 9. CPU-Affinity Behavior
Each task carries a `CpuAffinityMask` (64-bit bitmask). Load balancing distributes ready tasks across permitted CPU cores based on run-queue depth, avoiding migration when CPU affinity is restricted.

### 10. Zero Runnable Tasks Handling
When a CPU run queue is empty, the core transitions to an idle task state (`CState::C1/C2`) to save power until woken by an interrupt or inter-processor interrupt (IPI).

---

## Periodic Subsystem Maintenance Calendar

| Cadence | Task | Output | Required PR |
|---|---|---|---|
| **Every PR** | Build, lint, unit, and integration tests | CI pass result | `feat`, `fix`, `test`, `docs`, or `chore` |
| **Daily** | CI/nightly triage | Root cause + fix | `fix(scheduler)` or `fix(ci)` |
| **Weekly** | Subsystem review | Gap list + status update | `docs(scheduler)` |
| **Weekly** | Linux/BSD scheduler study | Study notes | `docs(studies)` |
| **Biweekly** | Fairness & preemption audit | Fairness findings | `fix(scheduler)` / `test(scheduler)` |
| **Biweekly** | Priority & affinity audit | Semantics findings | `fix(scheduler)` |
| **Monthly** | Boot-to-shell validation | Boot log + smoke report | `test(scheduler)` |
| **Monthly** | Scheduler stress/fuzz triage | Crash reports + fixes | `fix(scheduler)` |
| **Quarterly** | Subsystem architecture review | ADR + roadmap update | `docs(adr)` |
| **Quarterly** | Security review | Threat-model update | `security(scheduler)` |
| **Quarterly** | Release readiness review | Release checklist | `docs(release)` |

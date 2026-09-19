# AI Agent Assignment Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       Master AI Task Assignment Agent                           |
|            (TaskAssignmentDispatcher, Intent Parser, Goal Planner)              |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       Subagent Orchestration Pipeline                           |
|          (Task Decomposition, DAG Dependency Graph, SLA Scheduler)             |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
|  Worker Subagent A    |   |  Worker Subagent B    |   |  Worker Subagent C    |
| (NUMA Node 0, P-Cores)|   | (NUMA Node 1, E-Cores)|   | (WASM Hostcall Fast)  |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                    Resource Governor & Telemetry Monitor                        |
|        (ContainerResourceGovernor, eBPF Task Tracker, Micro-Checkpoints)        |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Task Decomposition & DAG Dependency Resolver**:
   - Decomposes multi-step operations into a Directed Acyclic Graph (DAG) of subtasks.
   - Evaluates subtask dependencies and dispatches ready tasks to available worker subagents.

2. **NUMA Affinity & Hardware Allocation**:
   - `SmpTopologyManager` queries CPU topology (P/E cores, NUMA nodes, L3 cache sharing).
   - `NumaAffinityMap` pins subagent threads to optimal memory and compute domains to minimize cross-socket interconnect latency.

3. **Resource Quotas & Execution Monitoring**:
   - `ContainerResourceGovernor` enforces cgroups v2 memory limits, CPU percentages, and block I/O bandwidth.
   - Stalled subagents are detected via eBPF heartbeats and re-scheduled or terminated cleanly.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.

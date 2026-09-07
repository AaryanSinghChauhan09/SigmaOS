# SigmaOS AI Agent Assignment Management Guidelines

## 1. Overview
SigmaOS implements an autonomous task assignment and subagent orchestration framework governing AI agents (such as `TaskAssignmentDispatcher`, `SubagentOrchestrator`, `NumaAffinityScheduler`, and `ContainerResourceGovernor`). These guidelines define task allocation rules, workload balancing, NUMA core pinning, execution tracking, and SLA priority handling for AI agents in SigmaOS.

## 2. Core Assignment Management Principles

### 2.1 Task Decomposition & Subagent Delegation
- **Hierarchical Assignment**: Primary AI agents decompose complex user or system requests into discrete, parallelizable subtasks.
- **Subagent Lifecycle Control**: Subagent processes operate with explicit execution bounds, parent-child IPC channels (`SovereignPipe`), and automated cancellation tokens (`CancellationToken`).

### 2.2 NUMA Core Pinning & Processor Affinity
- **NUMA & SMP Topology Matching**: Agents assign compute-heavy AI tasks (e.g. LLM inference, tensor operations, code compilation) to specific CPU core clusters using `SmpTopologyManager` and `NumaAffinityMap`.
- **Performance vs. Efficiency Cores**: High-priority real-time subtasks are pinned to Performance cores, while background maintenance subtasks run on Efficiency cores.

### 2.3 SLA Priority Classes & Resource Quotas
- **Priority Queue Allocation**: Tasks are categorized into SLA priority tiers (Real-Time, Interactive, Normal, Batch) mapped to EEVDF, BORE, and SCHED_ULE scheduling queues.
- **Resource Limit Enforcement**: Subagents operate under strict cgroups v2 resource quotas (`ContainerResourceGovernor`) limiting max RAM, CPU percentage, and disk I/O bandwidth.

### 2.4 Workload Monitoring & State Recovery
- **Execution Telemetry**: Subagent execution progress is monitored continuously. Stalled or unresponsive subagents are automatically terminated and re-assigned by `SubagentOrchestrator`.
- **Task Micro-Checkpoints**: Task state is periodically saved to local VFS storage (`/var/run/sigma_agent_tasks/`) to enable seamless resumption across system reboots or failures.

---
*Maintained by the SigmaOS AI & Process Management Steering Committee.*

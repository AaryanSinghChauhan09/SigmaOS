# SigmaOS AI Agents Circular Wait & Deadlock Avoidance Guide

Welcome to the **SigmaOS AI Agents Circular Wait & Deadlock Avoidance Guide**. This document details Coffman's 4 deadlock conditions, Resource Allocation Graphs (RAG), Banker's Deadlock Avoidance, lock ordering hierarchies, and automated deadlock remediation for autonomous AI agents and kernel developers in SigmaOS.

---

## 1. Coffman's Deadlock Conditions & Circular Wait

Deadlocks in multiprocessor kernel subsystems occur when all 4 **Coffman Conditions** hold simultaneously:

1. **Mutual Exclusion**: Non-shareable resource allocated to at most one process.
2. **Hold and Wait**: Process holding resources requests additional resources currently held by others.
3. **No Preemption**: Allocated resources cannot be forcibly confiscated until released by holder.
4. **Circular Wait**: A closed chain of processes ($P_0, P_1, \dots, P_n$) exists where $P_0$ waits for a resource held by $P_1$, $P_1$ waits for $P_2$, and $P_n$ waits for $P_0$.

> **Core Directive for AI Agents**: Eliminate condition #4 (**Circular Wait**) by enforcing strict global lock ordering and Banker's safety state evaluation.

---

## 2. Resource Allocation Graph (RAG) & Cycle Detection

SigmaOS kernel resource managers maintain a **Resource Allocation Graph (RAG)** (`src/process/blocked_state.rs`, `src/system/state.rs`):

- **Request Edges ($P_i \to R_j$)**: Directed edge indicating process $P_i$ is waiting for resource $R_j$.
- **Assignment Edges ($R_j \to P_i$)**: Directed edge indicating resource $R_j$ is allocated to process $P_i$.
- **Cycle Detection Algorithm**: Depth-First Search (DFS) traversal over RAG edges detecting closed directed cycles in $O(V + E)$ time.

---

## 3. Lock Hierarchy & Banker's Deadlock Avoidance

AI agents acquiring multiple system locks MUST acquire them in strictly increasing order of lock rank:

```
Rank 1: Memory Paging / Spinlock (Highest Rank)
Rank 2: VFS Inode Lock
Rank 3: Process PCB Lock
Rank 4: IPC Channel Ring Lock (Lowest Rank)
```

### Banker's Safety State Check (`src/resilience/automated_fixer.rs`)
Before granting a multi-resource request, evaluate if the resulting state remains safe:

$$\text{Need}[i][j] = \text{Max}[i][j] - \text{Allocation}[i][j] \le \text{Available}[j]$$

If no safe execution sequence exists, reject or block the requesting process.

---

## 4. Automated Deadlock Remediation (`src/resilience/automated_fixer.rs`)

When a circular wait cycle is detected by background health monitors:

1. **Identify Target Victim**: Select process with lowest priority or fewest acquired locks in the cycle.
2. **Targeted Process Termination**: Terminate or restart victim process (`deadlock_pid`).
3. **Resource Reclaim**: Release all held locks and re-evaluate RAG dependencies.

---

## 5. Checklist for AI Agents Managing Lock & Synchronization Logic

- [ ] Ensured all multi-lock acquisitions follow global lock rank hierarchy.
- [ ] Added `try_lock_timeout` fallback on spinlocks to avoid indefinite blocking.
- [ ] Confirmed RAG DFS cycle detector runs during deadlock health checks.
- [ ] Verified priority inheritance is applied when high-priority tasks wait on lower-priority lock holders.
- [ ] Executed `./run_sigma_tests.sh` to confirm kernel resilience and lock inspection tests pass cleanly.

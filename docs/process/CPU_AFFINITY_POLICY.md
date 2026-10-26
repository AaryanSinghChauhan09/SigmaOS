# SigmaOS CPU Affinity Policy Specification

## Purpose
This document defines CPU affinity mask representation, routing, load balancing, and migration semantics.

---

## 1. Affinity Mask Representation
- CPU affinity is stored as a 64-bit bitmask (`CpuAffinityMask`).
- Bit $N$ set to `1` indicates CPU core $N$ is permitted to schedule the task.

## 2. Mask Validation & Rejection
- Setting an affinity mask targeting non-existent CPU cores (e.g., bit 4 on a 2-CPU system) returns `Err(SchedulerError::InvalidCpuAffinity)`.
- An affinity mask of `0` (no CPUs permitted) is invalid and rejected.

## 3. Core Routing and Load Balancing
- Tasks are routed to permitted CPUs with the lowest run-queue length.
- If multiple permitted CPUs have equal load, routing prefers the current core or lowest CPU index to preserve cache locality.

## 4. NUMA and Hotplug Safety
- Affinity updates check topology constraints to minimize inter-socket NUMA bus transfers.
- Core removal (hotplug) redistributes tasks whose affinity mask permitted other online CPUs; if affinity is exhausted, task affinity is updated to default online mask.

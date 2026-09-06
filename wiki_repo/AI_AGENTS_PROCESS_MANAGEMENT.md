# 🤖 AI Agents Process Management Specification (`docs/AI_AGENTS_PROCESS_MANAGEMENT.md`)

This document defines the process management specifications, lifecycle hooks, IPC mechanisms, and security capabilities for autonomous AI agents operating on SigmaOS.

## Table of Contents
1. [Process Architecture](#process-architecture)
2. [Process Supervision & Isolation](#process-supervision--isolation)
3. [Inter-Process Communication & Signaling](#inter-process-communication--signaling)
4. [Fault Tolerance & Self-Healing](#fault-tolerance--self-healing)
5. [Security & Capability Bounds](#security--capability-bounds)

---

## 1. Process Architecture

Autonomous AI agents in SigmaOS run as restricted userspace or kernel helper processes:
- **Bolt (`bolt_agent`)**: Real-time performance monitoring and cgroups resource quota adjustment.
- **Palette (`palette_agent`)**: Desktop compositor layout optimization and WCAG accessibility verification.
- **Sentinel (`sentinel_agent`)**: Mandatory Access Control (MAC) auditing and privilege escalation detection.

---

## 2. Process Supervision & Isolation

Process isolation is strictly enforced via multi-OS primitives:
- **Linux**: `cgroups v2` controllers (`cpu`, `memory`, `io`) under `/sys/fs/cgroup/sigma_agents/`.
- **FreeBSD**: `rctl` resource limits and `jails` execution barriers.
- **OpenBSD**: System call restriction via `pledge("stdio rpath wpath cpath proc", NULL)` and `unveil()`.

---

## 3. Inter-Process Communication & Signaling

- Communication uses zero-copy shared-memory ring buffers (`src/klib/ring_buffer.rs`).
- Every IPC message requires a valid Kyber-1024 / Dilithium-5 signed `CapabilityToken`.

---

## 4. Fault Tolerance & Self-Healing

- The `ProcessSupervisor` performs watchdog heartbeat checks every 500 ms.
- Crash dumps are transmitted via `crashdump_netconsole` to remote audit collectors before process restart.

---

## 5. Security & Capability Bounds

- Agent processes cannot acquire `CAP_SYS_ADMIN` unless explicitly granted by the security policy.
- All mock credentials in agent test cases must contain `mock` or `test` in variable names to comply with secret scanning policies.

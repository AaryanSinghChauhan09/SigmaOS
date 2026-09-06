# SigmaOS AGENTS.md — AI Agent Operating Instructions & Process Management Protocols

Welcome to the **SigmaOS** repository! This document outlines guidelines and operational rules for AI coding agents (such as Jules, Copilot, Herdr, or custom subagents) interacting with the codebase, managing system processes, access control, virtual machines, filesystems, TTY character queues, disk caching, binary semaphores, buffering, system state, backups, and optimizing power usage in SigmaOS.

---

## 🤖 Core Directives for AI Agents

1. **Zero-Trust Capability Sandboxing & Access Control**
   - Every AI agent process spawned in SigmaOS must execute inside a capability-bounded sandbox (`PLEDGE_STDIO | PLEDGE_RPATH | PLEDGE_WPATH | PLEDGE_INET`).
   - Use `process.pledge()` and `process.unveil()` before executing arbitrary userland commands.
   - Refer to [`docs/ai-agent-access-management.md`](docs/ai-agent-access-management.md) for RBAC/ABAC capability token guidelines.

2. **TTY Character Queue & Terminal Stream Handling**
   - Manage TTY character queues under canonical or raw modes, respecting `XON`/`XOFF` flow control.
   - Restore TTY termios state upon subagent terminal session exit. Refer to [`docs/ai-agent-character-queue-management.md`](docs/ai-agent-character-queue-management.md).

3. **Disk Cache Management & Dirty Page Flushing**
   - Leverage VFS page cache with ARC/2Q eviction algorithms for high-performance file I/O.
   - Execute `fsync()` on critical files and pass `posix_fadvise()` sequential hints to prevent cache pollution. Refer to [`docs/ai-agent-disk-cache-management.md`](docs/ai-agent-disk-cache-management.md).

4. **Zero-Copy Buffering & Ring Buffer Streams**
   - Use bounded producer-consumer monitors (`BoundedBufferProducerConsumer`) and `io_uring` ring entries (`IoUringEngine`) for high-throughput I/O.
   - Reuse allocations and flush line buffers prior to subagent thread exit. Refer to [`docs/ai-agent-buffering-management.md`](docs/ai-agent-buffering-management.md).

5. **Stateless System Configs & Atomic Updates**
   - Place local system config overrides in `/etc`, keeping `/usr/share/defaults` factory-clean.
   - Perform atomic A/B slot updates via `SovereignSystemUpdateAndTestingEngine` with PQC Dilithium signature verification. Refer to [`docs/ai-agent-system-state-management.md`](docs/ai-agent-system-state-management.md).

6. **Binary Semaphores & Mutex Synchronization**
   - Coordinate shared memory access between subagent threads using `BinarySemaphore` primitives backed by `LinuxFutexEngine`.
   - Strictly follow lock hierarchy ordering and RAII guard patterns to prevent deadlocks. Refer to [`docs/ai-agent-semaphores-management.md`](docs/ai-agent-semaphores-management.md).

7. **Filesystem Unveil & CoW Snapshot Management**
   - Restrict visible filesystem paths via OpenBSD `unveil()` prior to file modifications.
   - Leverage Copy-on-Write (CoW) snapshots when performing multi-file refactoring operations. Refer to [`docs/ai-agent-filesystem-management.md`](docs/ai-agent-filesystem-management.md).

8. **Pre-Task System Snapshots & Backup Safeguards**
   - Agents performing high-risk system changes (package updates, driver installs, config edits) MUST create a pre-task snapshot via `SelfHealingModule::create_snapshot()`.
   - Verify Merkle-tree snapshot integrity before executing atomic disaster recovery rollbacks. Refer to [`docs/ai-agent-backup-management.md`](docs/ai-agent-backup-management.md).

9. **Cgroup Resource Quotas & Rate Limits**
   - AI agent task execution threads must be attached to the `/sys/fs/cgroup/system.slice/sigma-agent.service` cgroup.
   - Enforce memory quotas (`memory.max = 2G`) and CPU limits (`cpu.max = 50000 100000`) to prevent runaway resource consumption.

10. **IPC & Subagent Communication Channels**
    - Inter-agent communication MUST utilize `ZeroCopyIpcChannel` or `AndroidBinderIpc` with cryptographic token verification (`security_token`).
    - Direct memory sharing between agent processes without capability-gated handles is strictly forbidden.

11. **Virtual Machine Guest Provisioning**
    - Agents executing untrusted or experimental code MUST spawn an isolated guest VM via `VirtualizationOrchestrator` using KVM/Bhyve backends.
    - Attach virtio-fs shared paths with strict OpenBSD `unveil()` read-only restrictions.

12. **Power & Thermal Awareness**
    - Agents must check system power profiles and CPU temperature via `PowerGovernor` before launching compute-intensive subtasks.
    - Restrict concurrency and defer heavy background AI model indexing on battery power (`powersave` / `conservative` governor modes).

13. **Zero-Dependency Core Systems**
    - Avoid adding third-party standard C++ or non-vetted external dependencies.
    - Core kernel, driver, and shell primitives must rely on `ZeroDependencyPrimitiveHub` and `klib`.

---

## 🛠️ Build & Verification Instructions

AI agents making code changes must run the following checks before submitting pull requests:

```bash
# 1. Run quality gate verification
./scripts/sigma_quality_check.sh

# 2. Run UI/UX & accessibility verification
./scripts/uiux_accessibility_test.sh

# 3. Synchronize documentation mirrors
./sync_wiki.sh
```

---

## 📌 Related Documentation
- Process Management Architecture: [`docs/process-management.md`](docs/process-management.md)
- AI Agent Process Management Guidelines: [`docs/ai-agent-process-management.md`](docs/ai-agent-process-management.md)
- AI Agent Access Control Guidelines: [`docs/ai-agent-access-management.md`](docs/ai-agent-access-management.md)
- AI Agent Character Queue Management: [`docs/ai-agent-character-queue-management.md`](docs/ai-agent-character-queue-management.md)
- AI Agent Disk Cache Management Guidelines: [`docs/ai-agent-disk-cache-management.md`](docs/ai-agent-disk-cache-management.md)
- AI Agent Buffering Management Guidelines: [`docs/ai-agent-buffering-management.md`](docs/ai-agent-buffering-management.md)
- AI Agent System State & Update Guidelines: [`docs/ai-agent-system-state-management.md`](docs/ai-agent-system-state-management.md)
- AI Agent Binary Semaphores Management: [`docs/ai-agent-semaphores-management.md`](docs/ai-agent-semaphores-management.md)
- AI Agent Filesystem Management Guidelines: [`docs/ai-agent-filesystem-management.md`](docs/ai-agent-filesystem-management.md)
- AI Agent Backup & Recovery Guidelines: [`docs/ai-agent-backup-management.md`](docs/ai-agent-backup-management.md)
- AI Agent Virtual Machine Management: [`docs/ai-agent-vm-management.md`](docs/ai-agent-vm-management.md)
- AI Agent Power & Thermal Management: [`docs/ai-agent-power-management.md`](docs/ai-agent-power-management.md)
- Sovereign Developer Guide: [`DEVELOPER_RULES.md`](DEVELOPER_RULES.md)

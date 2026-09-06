# SigmaOS AGENTS.md — AI Agent Operating Instructions & Process Management Protocols

Welcome to the **SigmaOS** repository! This document outlines guidelines and operational rules for AI coding agents (such as Jules, Copilot, Herdr, or custom subagents) interacting with the codebase, managing system processes, access control, security policies, instruction execution, cluster operations, virtual machines, filesystems, TTY character queues, disk caching, binary semaphores, buffering, system state, backups, and optimizing power usage in SigmaOS.

---

## 🤖 Core Directives for AI Agents

1. **Zero-Dependency & Self-Containment (`no_std`):**
   * The kernel core and primary subsystems are designed to target bare-metal targets (`#![no_std]`).
   * Avoid adding runtime dependencies on standard `std` libraries inside microkernel shard components unless conditionally gated under test environments (`#[cfg(not(target_os = "none"))]`).
2. **Capability-Based Security Model:**
   * Never introduce generic root/admin ACL checks. System call access is authorized exclusively via hardware-enforced 64-bit `CapabilityToken` verification gates.
3. **Windows NT & Distro Parity Standards:**
   * Hardware drivers must follow the WDM-style `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension` abstractions.
   * Kernel memory allocations must respect tagged `Paged` (swappable) and `NonPaged` (always resident) memory pool boundaries.
4. **Bit Table & Hardware Field Standards:**
   * For bit tables, physical frame allocators, page table entry flags, and capability bitmasks, follow [docs/AGENTS_BIT_TABLE_MANAGEMENT.md](docs/AGENTS_BIT_TABLE_MANAGEMENT.md).
5. **Cache Memory Optimization & Coherency:**
   * For L1/L2/L3 cache alignment, false sharing prevention, non-temporal stores, and page/buffer cache management, follow [docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md](docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md).
6. **Cache Operation & Hardware Controls:**
   * For explicit CPU cache flushing (`clflushopt`/`clwb`), DMA cache coherency, JIT $I\$/D\$$ cache sync, and memory fences, follow [docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md](docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md).
7. **Cloud vs. Fog Computing Orchestration:**
   * For real-time edge processing, P2P mesh discovery, workload offloading cost function, and CRDT synchronization, follow [docs/AGENTS_CLOUD_VS_FOG_MANAGEMENT.md](docs/AGENTS_CLOUD_VS_FOG_MANAGEMENT.md).
8. **Commercial Operating System Architecture:**
   * For enterprise licensing tiers, statutory compliance governors, software certification programs, and open-core preservation rules, follow [docs/AGENTS_COMMERCIAL_OPERATION_SYSTEM.md](docs/AGENTS_COMMERCIAL_OPERATION_SYSTEM.md).

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
- AI Agent Instruction Execution Guidelines: [`docs/ai-agent-instructions-execution-management.md`](docs/ai-agent-instructions-execution-management.md)
- AI Agent Cluster Operation Guidelines: [`docs/ai-agent-cluster-operation-management.md`](docs/ai-agent-cluster-operation-management.md)
- AI Agent Security & System Policy Guidelines: [`docs/ai-agent-policy-management.md`](docs/ai-agent-policy-management.md)
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

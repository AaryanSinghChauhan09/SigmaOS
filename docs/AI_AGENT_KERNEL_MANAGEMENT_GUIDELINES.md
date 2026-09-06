# SigmaOS AI Agent Kernel Management Guidelines

## 1. Overview
SigmaOS provides specialized kernel management agents (such as `KernelSchedulerAgent`, `MemoryOptimizerAgent`, `EbpfTracerAgent`, and `SelfHealingKernelSupervisor`). These agents allow autonomous and interactive telemetry monitoring, kernel MIB sysctl parameter tuning, CPU scheduling optimization, memory page-reclaim tuning, and driver hotplug orchestration.

## 2. Core Kernel Management Principles

### 2.1 Kernel Isolation & Ring-0 Safety
- **Strict User/Kernel Boundary**: AI agents operate in Ring 3 (userland) or WASM hostcall containers. AI agents **never** execute raw unconstrained code inside Ring 0 (kernel space).
- **System Call & eBPF Interface**: Kernel management agents interact with kernel state solely through well-defined syscall interfaces, sysctl MIB nodes (`LinuxBsdSysctlEngine`), and verifier-checked eBPF bytecode programs.

### 2.2 CPU Scheduler & Task Management
- **Scheduler Telemetry**: Agents monitor task latency, runqueue depth, and cgroup CPU quotas via EEVDF, BORE, and SCHED_ULE scheduler metrics.
- **Dynamic Governor Tuning**: Agents can adjust scheduling parameters (e.g., latency target, burst bonus, NUMA thread pinning) using `SmpTopologyManager` and `NumaAffinityMap`.

### 2.3 Virtual Memory & Page Management
- **Memory Pressure Monitoring**: Agents track PMM bitmap allocations, slab object caches, and Copy-on-Write (COW) page fault rates.
- **ZRAM & Swap Optimization**: Under high memory pressure, agents trigger ZRAM zstd compression or invoke the UMA / slab object reclaim handlers.

### 2.4 eBPF & LSM Security Policies
- **Syscall Tracing**: `EbpfTracerAgent` attaches sandboxed eBPF probes to syscall entry/exit vectors for real-time anomaly detection.
- **LSM Enforcement**: Agents update AppArmor path rules (`AppArmorPathRuleEngine`) and SELinux SECMARK context labels (`SecmarkPacketLabel`) dynamically upon detecting malicious execution patterns.

### 2.5 Kernel Self-Healing & Panic Recovery
- **Driver Crash Recovery**: When a Ring 3 / RUMP userland driver fails, `Minix3ReincarnationServer` restarts the driver process automatically without panicking the kernel.
- **Micro-Checkpointing**: Kernel state descriptors and task registers (`CpuContext`) are micro-checkpointed for rapid state restoration.

---
*Maintained by the SigmaOS Kernel Steering Committee & SIG-Kernel.*

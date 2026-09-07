# 🇸🇴 AI Agents Resource Management Architecture in SigmaOS

## Executive Overview

SigmaOS introduces a **sovereign, autonomous AI Agent Resource Management Architecture** that replaces static, manual OS resource management with intelligent, real-time agentic governors. Operating directly within SigmaOS's zero-dependency `#![no_std]` Rust microkernel and userland layer, AI Agents continuously monitor telemetry, predict workload bursts, dynamically tune kernel parameters, and enforce strict safety bounds across compute, memory, storage, network, and power subsystems.

Drawing deep inspiration from advanced Linux kernel mechanisms and BSD operating system security frameworks, SigmaOS AI Agents combine autonomous decision-making with deterministic security guarantees.

---

## 🌟 Architectural Principles & Linux/BSD Inspirations

SigmaOS absorbs and unifies key resource management paradigms from leading Linux distributions and BSD operating systems:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                             SigmaOS Agentic OS Orchestrator                              │
│         (ACP / MCP Protocols, Dilithium-5 Attestation, OpenBSD Pledge Sandboxing)       │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ Memory Governor ││ CPU Scheduler   ││ Storage & ARC   ││ Network QoS     │
│ (DAMON + PSI)   ││ (BORE + EEVDF)  ││ (ZFS + HAMMER2) ││ (eBPF + VNET)   │
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

### 1. Linux Kernel Paradigms Absorbed
- **Cgroups v2 Unified Hierarchy:** AI agents dynamically adjust memory high/max watermarks, CPU quotas, and I/O weights per slice.
- **EEVDF & BORE CPU Schedulers:** The scheduler agent tunes virtual runtime deadlines (`vruntime`) and BORE (Burst-Oriented Response Enhancer) burst scores to eliminate desktop latency during heavy background compilation or AI model inference.
- **DAMON (Data Access Monitor) & PSI (Pressure Stall Information):** Autonomous memory agents track page access frequencies to proactively page out cold memory and relieve memory/I/O pressure stalls before OOM conditions trigger.
- **eBPF Programmable Telemetry & Traffic Control:** Network agents load safe eBPF bytecode to enforce per-flow QoS and socket redirection without context-switching overhead.

### 2. BSD Security & Storage Frameworks Absorbed
- **FreeBSD `racct` / `rctl` Framework:** Provides fine-grained process-level and jail-level resource accounting (CPU time, RSS, vmemory, file descriptors, PCPU usage).
- **FreeBSD ZFS Adaptive Replacement Cache (ARC):** Storage agents balance ARC MRU (Most Recently Used) and MFU (Most Frequently Used) memory allocation against system memory pressure.
- **OpenBSD `pledge(2)` & `unveil(2)` Sandboxing:** AI resource agents run inside ephemeral sandboxes restricted to explicit system calls (e.g., `pledge("stdio rpath proc")`) and restricted file paths (`unveil("/sys/cgroup", "rwc")`).
- **NetBSD Rump Kernel Isolation:** Drivers and resource agent helpers execute in isolated micro-domains, preventing kernel panics from faulty hardware or driver routines.

---

## 🤖 Core AI Resource Governors

SigmaOS deploys five specialized microkernel AI agents, each responsible for an essential system domain:

### 1. Memory & Cache Governor Agent (`MemoryGovernorAgent`)
- **Real-Time Telemetry:** Monitors Linux-inspired `PSI_SOME` and `PSI_FULL` memory stall metrics alongside DAMON access histograms.
- **Autonomous Actions:**
  - Dynamically resizes zswap/zram compressed memory pools.
  - Reclaims cold page caches during background AI inference or compilation tasks.
  - Tunes FreeBSD ZFS ARC memory caps (`vfs.zfs.arc_max`) dynamically based on real-time process demand.

### 2. Core & NUMA Scheduler Agent (`CpuSchedulerAgent`)
- **Real-Time Telemetry:** Tracks CPU core utilization, EEVDF deadline latency, and NUMA node interconnect bandwidth.
- **Autonomous Actions:**
  - Adjusts BORE scheduler burst weights for interactive Zenith Desktop applications versus background background workloads.
  - Migrates processes across NUMA sockets to optimize L3 cache locality and memory channel throughput.
  - Manages thread affinity and core parking for heterogeneous architectures (e.g., Intel Performance/Efficient cores, ARM big.LITTLE).

### 3. Storage & File System Governor Agent (`StorageGovernorAgent`)
- **Real-Time Telemetry:** Measures I/O latency, NVMe queue depths, CoW extent fragmentation, and disk wear statistics.
- **Autonomous Actions:**
  - Schedules background Btrfs/HAMMER2 extent defragmentation during idle periods.
  - Flushes dirty page buffers adaptively before heavy write bursts.
  - Manages ZFS / Btrfs CoW snapshot lifecycles and automated rollback generation.

### 4. Network & eBPF QoS Agent (`NetworkQosAgent`)
- **Real-Time Telemetry:** Monitors per-interface packet drops, RTT jitter, and socket buffer usage across VNET virtual network stacks.
- **Autonomous Actions:**
  - Dynamically updates eBPF tc (Traffic Control) classification maps to prioritize real-time audio/video streams over bulk downloads.
  - Reconfigures VNET jail interface bandwidth caps via FreeBSD `rctl`-inspired rules.

### 5. Power & Thermal Efficiency Agent (`PowerThermalAgent`)
- **Real-Time Telemetry:** Reads CPU/GPU junction temperatures, power draw (RAPL/ACPI), and battery discharge rates.
- **Autonomous Actions:**
  - Adjusts CPU energy performance preference (EPP) governors (`performance`, `balance_performance`, `power`).
  - Throttles non-essential background agents when thermal thresholds approach TJMax boundaries.

---

## 📡 Agent Communication & Control Protocols

SigmaOS agents interact with userland tools, shells (`sigma-sh`, `intelligent_terminal`), and IDEs via standardized protocols:

### Agent Client Protocol (ACP)
- **JSON-RPC Interface:** Standardized stdio and socket messaging protocol enabling editor and terminal control over AI agents.
- **Capabilities:**
  - Task planning (`plan_task`), execution approval (`request_approval`), and dry-run execution.
  - Automated diagnostic feedback when a command fails (suggesting cgroup or resource limit adjustments).

### Model Context Protocol (MCP)
- **Context Bridge:** Exposes kernel and subsystem telemetry to local LLMs (`LocalLlmDaemon`, `QwenPaw`, `KimiCodeAgent`) while enforcing OpenBSD `unveil` file boundaries and capability bounds.

---

## 🔒 Security, Attestation & Audit Governance

To ensure AI resource management decisions remain safe, deterministic, and tamper-proof, SigmaOS enforces a multi-layered security model:

1. **Post-Quantum Cryptographic Attestation:**
   - All AI agent binaries and kernel-bound policy updates are signed using Dilithium-5 post-quantum digital signatures and validated via Kyber-1024 key exchange.
2. **Capability-Bounded Ephemeral Sandboxing:**
   - Resource agents run with minimal Linux capability sets (`CAP_SYS_RESOURCE`, `CAP_SYS_NICE`) and OpenBSD-style `pledge` restrictions.
3. **Immutable Audit Logging:**
   - Every autonomous action (e.g., process termination, core pinning, memory limit adjustment) is recorded in an immutable transaction log for post-hoc inspection.
4. **Automated Rollback Safeguards:**
   - If an AI agent's policy modification leads to system degradation or high pressure stalls, the kernel automatically triggers a state rollback to the last known stable snapshot (NixOS generation or ZFS boot environment).

---

## 🛠️ Configuration & Inspection

Users and system administrators can inspect and override AI Agent policies via `sigma-sh` or the Zenith Desktop control panel:

```bash
# View active AI Resource Governors and status
sigma-sh> ai-agent status

# Query memory governor telemetry and DAMON access heatmaps
sigma-sh> ai-agent inspect memory-governor

# Set manual override bounds for CPU Scheduler Agent
sigma-sh> ai-agent set-policy --agent=cpu-scheduler --max-cgroup-quota=80%

# Verify post-quantum attestation signatures of active agents
sigma-sh> ai-agent verify-signatures
```

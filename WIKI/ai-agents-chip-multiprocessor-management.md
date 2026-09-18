# 🇸🇴 AI Agents Chip Multiprocessor (CMP) Operation Management Architecture in SigmaOS

## Executive Overview

SigmaOS implements a **sovereign, autonomous AI Agent Architecture for Chip Multiprocessor (CMP) Operation Management**, replacing static CPU scheduling and core management with real-time agentic governors. In modern heterogeneous and chiplet-based hardware architectures—featuring multi-socket NUMA nodes, Simultaneous Multithreading (SMT/HyperThreading), asymmetric Performance/Efficiency (P/E) cores, high-speed interconnects (CXL, Intel UPI, AMD Infinity Fabric), and thermal constraints—static kernel schedulers fail to optimize throughput, latency, and power consumption simultaneously.

Operating inside SigmaOS's zero-dependency `#![no_std]` Rust microkernel, dedicated **CMP AI Governor Agents** continuously process hardware telemetry, predict thread burst behavior, dynamically balance NUMA socket workloads, manage core parking, and isolate untrusted workloads to eliminate microarchitectural side-channel threats.

---

## 🌟 Architectural Principles & Linux/BSD Inspirations

SigmaOS unifies and advances CMP management paradigms from Linux kernels and BSD operating systems:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                   SigmaOS AI Agent Chip Multiprocessor (CMP) Orchestrator                 │
│         (ACP / MCP Protocols, Dilithium-5 Attestation, Zero-Alloc Microkernel Execution)   │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ CMP Topology    ││ SMT & Parking   ││ Heterogeneous   ││ Interconnect &  │
│ Governor Agent  ││ Governor Agent  ││ Core Agent      ││ Thermal Agent   │
│ (NUMA/CXL)      ││ (FreeBSD ULE)   ││ (Intel ITD/EAS) ││ (RAPL/Resctrl)  │
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

### 1. Linux Kernel Paradigms Absorbed
- **`sched_domain` & AutoNUMA Balancing:** Autonomous migration of process memory pages and threads across NUMA domains to maximize local L3 cache hits and reduce cross-socket UPI/Infinity Fabric traffic.
- **Intel Thread Director (ITD) & ARM Energy-Aware Scheduling (EAS):** Uses Hardware Feedback Interface (HFI) signals to dispatch latency-critical UI and render threads to Performance cores while routing background daemon tasks to Efficient cores.
- **`resctrl` Resource Control (CAT & MBA):** Dynamically restricts L3 cache ways and Memory Bandwidth Allocation (MBA) throttling per cgroup slice.

### 2. BSD Core Topology & Security Paradigms Absorbed
- **FreeBSD ULE Scheduler & `cpuset(1)`:** Thread affinity binding, domain-aware work stealing, and per-CPU load calculation algorithms.
- **NetBSD `cpu_topology(9)` Architecture:** Abstraction of physical packages, core clusters, die boundaries, and logical processors.
- **OpenBSD SMT Mitigation Framework (`hw.smt`):** Selective disablement or isolation of sibling SMT threads during execution of cryptographic or high-security processes to prevent Spectre/Meltdown and L1TF side-channel leaks.

---

## 🗂️ CMP Subsystem Domain Taxonomy & AI Agents

SigmaOS deploys five specialized microkernel AI Agents for complete CMP operation lifecycle management:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                           5 CMP Operation Management Domains                             │
├───────────────────┬───────────────────┬───────────────────┬───────────────────┬──────────┤
│ Domain 1:         │ Domain 2:         │ Domain 3:         │ Domain 4:         │ Domain 5:│
│ NUMA Topology     │ SMT & Core        │ Asymmetric Core   │ Interconnect &    │ Security │
│ & CXL Memory      │ Parking           │ Steering (P/E)    │ Thermal Power     │ Isolation│
└───────────────────┴───────────────────┴───────────────────┴───────────────────┴──────────┘
```

| Domain | Scope & Responsibility | Primary Linux/BSD Inspiration | Governing AI Agent |
|---|---|---|---|
| **1. NUMA & Topology** | Multi-socket NUMA node alignment, CXL memory expansion, LLC cache partitioning | Linux `sched_domain`, AutoNUMA, FreeBSD `cpuset` | `CmpTopologyAgent` |
| **2. SMT & Core Parking** | HyperThreading sibling core scheduling, dynamic core parking/unparking | FreeBSD ULE scheduler, OpenBSD `hw.smt` | `CmpSmtGovernorAgent` |
| **3. Asymmetric Steering** | Heterogeneous P-core / E-core workload dispatching, HFI telemetry | Intel Thread Director, ARM EAS, Linux EEVDF | `CmpHeterogeneousAgent` |
| **4. Interconnect & Power** | CXL/UPI interconnect bandwidth tuning, RAPL power limits, thermal throttling | Linux `resctrl`, Intel CAT/MBA, ACPI thermal | `CmpInterconnectThermalAgent` |
| **5. Security Domains** | SMT sibling isolation, speculative execution mitigation, Enclave micro-domains | OpenBSD pledge/unveil, Qubes OS isolation | `CmpSecurityDomainAgent` |

---

## 🤖 Detailed AI Agent Roles & Telemetry

### 1. CMP Topology & NUMA Agent (`CmpTopologyAgent`)
- **Telemetry:** Monitors NUMA interconnect bandwidth, cross-socket QPI/UPI/Infinity Fabric stall cycles, L3 cache load misses, and CXL memory latency.
- **Autonomous Action:**
  - Migrates memory pages and thread affinity groups to local NUMA nodes when remote memory access ratios exceed 15%.
  - Re-partitions Intel CAT L3 cache ways to guarantee zero-latency execution for Zenith Desktop UI threads.

### 2. CMP SMT & Core Parking Agent (`CmpSmtGovernorAgent`)
- **Telemetry:** Measures SMT sibling thread contention, instruction per cycle (IPC) throughput loss on shared execution units, and overall package utilization.
- **Autonomous Action:**
  - Parks idle CPU cores during low-system-load states to minimize package C-state power draw.
  - Dynamically unparks sibling SMT threads under bursty multithreaded workloads (e.g., parallel kernel compilation).

### 3. CMP Heterogeneous Core Steering Agent (`CmpHeterogeneousAgent`)
- **Telemetry:** Reads Intel HFI hardware execution class hints, instruction mix (AVX-512, AMX, scalar), and process responsiveness metrics.
- **Autonomous Action:**
  - Dispatches interactive Zenith GTK compositor threads and real-time audio pipelines directly to high-frequency P-cores.
  - Offloads background indexing, log compression, and telemetry daemons to E-cores, optimizing performance-per-watt.

### 4. CMP Interconnect & Thermal Power Agent (`CmpInterconnectThermalAgent`)
- **Telemetry:** Samples DTS (Digital Thermal Sensor) core temperatures, RAPL power consumption watts, and thermal throttling status bits.
- **Autonomous Action:**
  - Dynamically lowers CPU Energy Performance Preference (EPP) bounds when package temperature approaches $T_{\text{jmax}} - 5^\circ\text{C}$.
  - Limits memory bandwidth allocation (MBA) for heavy background processes to prevent interconnect thermal hotspots.

### 5. CMP Security Domain Isolation Agent (`CmpSecurityDomainAgent`)
- **Telemetry:** Tracks cross-SMT speculative execution attempts, cache line side-channel noise, and process security classifications.
- **Autonomous Action:**
  - Enforces OpenBSD-inspired SMT sibling isolation: when a high-security process (e.g., cryptographic vault, Dilithium-5 key manager) executes, the SMT sibling thread is scheduled with `nop` idle loops to prevent speculative side-channel leaks.

---

## 📡 Protocol Integration (ACP / MCP) & Safety Governance

1. **Agent Client Protocol (ACP):** Userland tools (`sigma-sh`, `intelligent_terminal`, Zenith Control Center) query CMP topology, inspect per-core IPC and NUMA hit rates, and issue policy adjustments via standardized JSON-RPC stdio.
2. **Model Context Protocol (MCP):** Exposes CMP topology metrics to local LLMs (`LocalLlmDaemon`, `QwenPaw`, `KimiCodeAgent`) while enforcing strict OpenBSD `unveil` file paths and capability boundaries.
3. **Post-Quantum Attestation & Zero-Alloc Microkernel Execution:**
   - All CMP governor policy updates are signed using Dilithium-5 post-quantum signatures.
   - Core CMP decision loops operate inside `#![no_std]` zero-allocation microkernel code paths, guaranteeing deterministic execution without risking heap allocations or lock inversions.

---

## 🛠️ System Inspection & Administration

Manage CMP operations via `sigma-sh`:

```bash
# View complete Chip Multiprocessor (CMP) topology and NUMA node matrix
sigma-sh> ai-agent cmp topology

# Inspect real-time P/E core steering and HFI telemetry
sigma-sh> ai-agent cmp inspect heterogeneous-agent

# Query SMT sibling contention and core parking status
sigma-sh> ai-agent cmp inspect smt-governor

# Force security domain SMT isolation for cryptographic workloads
sigma-sh> ai-agent cmp set-policy --agent=security-domain --smt-isolation=strict
```

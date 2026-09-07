# 🇸🇴 AI Agents Cores Operation Management Architecture in SigmaOS

## Executive Overview

SigmaOS introduces an **autonomous, sovereign AI Agent Cores Operation Management Architecture** designed for real-time CPU core lifecycle management, dynamic core online/offline state transitions (core hotplugging), P-core/E-core asymmetric frequency scaling, thermal throttling control, and hardware security domain isolation. Operating directly within SigmaOS's zero-dependency `#![no_std]` Rust microkernel, autonomous core governor agents (`CoreTopologyGovernorAgent`, `CorePowerThermalGovernorAgent`, `CoreIsolationGovernorAgent`) continuously monitor per-core instructions per cycle (IPC), digital thermal sensors (DTS), RAPL power consumption limits, and speculative execution side-channel risks across multi-socket and multi-core SMP systems.

By unifying Linux kernel core management paradigms (Linux CPU hotplug, `cpufreq`/`intel_pstate` governors, cgroups v2 `cpuset`, `sched_domain`) with BSD core management models (FreeBSD `cpuset(1)`, NetBSD `cpu_topology(9)`, OpenBSD `hw.smt` core isolation), SigmaOS AI Agents ensure optimal performance-per-watt and microarchitectural security under dynamically shifting workloads.

---

## 🌟 Architectural Principles & Linux/BSD Inspirations

SigmaOS integrates CPU core operation paradigms from Linux, FreeBSD, NetBSD, and OpenBSD operating systems:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                   SigmaOS AI Agent Cores Operation Management Governor                   │
│          (ACP / MCP Protocols, Dilithium-5 Attestation, Zero-Alloc Microkernel Execution)   │
└───────────────────────────┬──────────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┬──────────────────┐
         ▼                  ▼                  ▼                  ▼
┌─────────────────┐┌─────────────────┐┌─────────────────┐┌─────────────────┐
│ CPU Hotplug &   ││ P/E Core        ││ Thermal & RAPL  ││ OpenBSD SMT     │
│ Topology (Linux)││ Steering (ITD)  ││ Dynamic Power   ││ Core Isolation  │
│ (src/kernel)    ││ (src/scheduler) ││ (src/power)     ││ (src/security)  │
└─────────────────┘└─────────────────┘└─────────────────┘└─────────────────┘
```

### 1. Linux Core Lifecycle & Power Paradigms
- **CPU Core Hotplugging:** Dynamically brings CPU cores online or offline (`/sys/devices/system/cpu/cpuN/online`) to eliminate idle leakage power during low-load periods.
- **Asymmetric Core Steering (Intel Thread Director / ARM Energy-Aware Scheduling):** Routes interactive desktop tasks (Zenith GTK compositor, audio pipelines) to Performance cores (P-cores) while pinning background AI inference, compilation, and indexing tasks to Efficiency cores (E-cores).
- **Resource Partitioning (`cgroups v2 cpuset`):** Isolates sets of CPU cores for critical real-time microkernel shards and containerized workloads.

### 2. BSD Topology & Security Paradigms
- **NetBSD `cpu_topology(9)` Abstraction:** Maintains physical package, die, core cluster, and logical processor topology trees for cache-aware thread migration.
- **FreeBSD `cpuset(1)` Domain Allocation:** Binds process groups to explicit core masks and NUMA domain sets.
- **OpenBSD `hw.smt` Core Isolation:** Disables or isolates sibling SMT hyperthreads when processing cryptographic keys or executing untrusted code to prevent Spectre, Meltdown, and L1TF side-channel attacks.

---

## 🤖 Core AI Operation Governors & Domain Architecture

SigmaOS deploys three specialized microkernel AI Agents for full CPU core operation lifecycle management:

```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│                             3 Cores Operation Management Domains                         │
├───────────────────────────────┬───────────────────────────────┬──────────────────────────┤
│ Domain 1:                     │ Domain 2:                     │ Domain 3:                │
│ Core Topology & Hotplug       │ Power, Frequency & Thermal    │ Core Security Isolation  │
└───────────────────────────────┴───────────────────────────────┴──────────────────────────┘
```

| Domain | Scope & Primary Responsibility | Governing AI Agent |
|---|---|---|
| **1. Topology & Hotplug** | Core online/offline state transitions, ACPI MADT enumeration, P/E core classification | `CoreTopologyGovernorAgent` |
| **2. Power & Thermal** | Dynamic frequency scaling (P-states/C-states), DTS thermal throttling, RAPL power cap management | `CorePowerThermalGovernorAgent` |
| **3. Security & Isolation** | SMT sibling thread suppression, speculative execution mitigation, core pinning | `CoreIsolationGovernorAgent` |

---

## 📡 Agent Protocol Integration (ACP / MCP)

### Agent Client Protocol (ACP)
- **core_inspect:** Queries real-time per-core online states, temperatures, frequencies, C-state residencies, and IPC metrics.
- **core_set_state:** Dynamically brings specified CPU cores online or offline.
- **core_set_governor:** Configures core power/frequency scaling policies (e.g., `Performance`, `Powersave`, `EnergyEfficient`).

### Model Context Protocol (MCP)
- **Context Integration:** Exposes per-core thermal profiles and power consumption data to local LLMs while enforcing OpenBSD `unveil` file boundaries.

---

## 🔒 Security, Attestation & Audit Governance

1. **Post-Quantum Attestation:**
   - Core state configuration policies and frequency scaling profiles are signed using Dilithium-5 post-quantum digital signatures.
2. **Deterministic Microkernel Execution:**
   - All core state transition logic runs in `#![no_std]` zero-allocation microkernel paths, ensuring zero lock inversions or heap allocation delays during thermal emergencies.
3. **Immutable Audit Trail:**
   - Core hotplug events, thermal throttle triggers, and SMT isolation policy changes are logged in the unified audit ledger (`UnifiedLogEntry`).

---

## 🛠️ Inspection & Manual Overrides

System administrators can inspect and manage CPU core operations via `sigma-sh`:

```bash
# View status of all CPU cores, frequencies, and DTS temperatures
sigma-sh> ai-agent status cores

# Bring CPU core 4 offline (core hotplug)
sigma-sh> ai-agent set-core-state --core=4 --state=offline

# Set power governor to high-performance mode for P-cores
sigma-sh> ai-agent set-core-governor --policy=performance --core-type=p-core

# Enable strict OpenBSD-style SMT core isolation for cryptographic processes
sigma-sh> ai-agent set-core-isolation --smt=strict
```

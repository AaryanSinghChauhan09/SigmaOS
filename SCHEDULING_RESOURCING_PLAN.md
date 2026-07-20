# ⚡ SigmaOS: Asymmetric Multi-Processing Scheduler & Process Supervision Plan

This document details the strategic engineering blueprint for **SovereignSched** and **S-INIT**, the asymmetric real-time thread scheduler and s6-style process supervisor for **SigmaOS**.

---

## 🏛️ 1. ARCHITECTURAL VISION

Legacy process managers (such as systemd and monolithic Linux CFS schedulers) introduce immense security risks and latency spikes by managing scheduling and initialization under a centralized, ambient-authority daemon. SigmaOS implements a completely decentralized, lock-free, and real-time thread scheduler paired with microsecond-latency process supervision watchdogs.

```
+-----------------------------------------------------------------------------------+
|                           SOVEREIGNSCHED & S-INIT SYSTEM                          |
+-----------------------------------------------------------------------------------+
|  [S-INIT Service Watchdogs]  |  [EEVDF Priority Queue]  |  [SovereignVMM VM Sync] |
+-----------------------------------------------------------------------------------+
|                         AMP Thermal & Core-Affinity Allocator                     |
+-----------------------------------------------------------------------------------+
|                       Asynchronous Lock-Free IPC Ring Buffer                      |
+-----------------------------------------------------------------------------------+
```

---

## 🏗️ 2. CORE COMPONENT PLANS & OBJECT-ORIENTED DESIGN

The system resourcing and scheduler suite is built around high-cohesion, zero-dependency OOP structures:

### 2.1 Asymmetric Multi-Processing EEVDF Scheduler (SovereignSched)
* **Design:** Implements an Earliest Eligible Virtual Deadline First (EEVDF) scheduling class natively in the microkernel core.
* **Asymmetric Mapping:** Allocates execution priorities dynamically across heterogenous computing clusters (CPU core threads, GPU stream units, and neural NPU accelerators).
* **Lock-Free Queue Pools:** System threads are tracked inside static lock-free singly-linked lists to prevent kernel lock-contention, optimizing cache-miss ratios and context-switching metrics.

### 2.2 Decoupled Service Supervision Chains (S-INIT)
* **The systemd Flaw:** Centralizing unit files, journal logs, and login sessions inside a single PID 1 daemon creates tight architectural coupling and increases the attack surface.
* **The S-INIT Solution:** Inspired by s6 supervision trees. All system services are supervised by a hierarchy of tiny, isolated parent watchdogs. If an individual userland daemon crashes, its supervising watchdog restarts it instantly without affecting the core OS or other daemons.

### 2.3 Thermal & Predictive Core Affinity
* **Optimization:** Leverages real-time physical telemetry (CPU core temperatures, battery levels, power consumption) to dynamically scale thread allocation and avoid local thermal throttling.

---

## 📅 3. STEP-BY-STEP IMPLEMENTATION TIMELINE

* **Phase I: EEVDF Microkernel Core Schedulers (Months 1-2):**
  Integrate the static, lock-free queue pools and implement basic CFS/EDF task selection algorithms.
* **Phase II: S-INIT Process Supervision (Months 2-3):**
  Construct the hierarchical supervision watchdogs and replace systemd service files with declarative JSON unit definitions.
* **Phase III: AMP Core Affinity & Telemetry Links (Months 3-4):**
  Link the scheduler to real-time hardware performance registers, optimizing thread scheduling based on cache utilization.
* **Phase IV: Zero-Copy IPC & Signal Channels (Months 4-6):**
  Map process notification channels to lock-free ring buffers, enabling microsecond service coordination.

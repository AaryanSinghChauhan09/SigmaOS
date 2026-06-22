# SigmaOS Architecture Overview

This page describes the high-level structure of the SigmaOS Zenith microkernel.

---

## Ring Architecture

```
┌─────────────────────────────────────────────────────┐
│  Ring-3 (Userland)                                  │
│  sigma-sh | sigma-forensics | Zenith Desktop UI     │
└─────────────────────┬───────────────────────────────┘
                      │ syscall / SYSRET
┌─────────────────────▼───────────────────────────────┐
│  SyscallDispatcher  (256-slot O(1) C table)         │
└─────────────────────┬───────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────┐
│  Ring-0 (Kernel Lattice)                            │
│  Scheduler | Allocator | VFS | IPC | PQC Engine     │
└─────────────────────┬───────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────┐
│  S-HAL (Hardware Abstraction Layer)                 │
│  x86_64 APIC | ARM64 GIC | RISC-V PLIC/CLINT       │
└─────────────────────┬───────────────────────────────┘
                      │
              Physical Hardware

```

---

## Key Subsystems

| Subsystem | File | Purpose | 
| :--- | :--- | :--- | 
| **S-HAL** | `hal/SovereignHAL.cpp` | Platform-agnostic register access | 
| **Scheduler** | `kernel/scheduler/SovereignScheduler.cpp` | CFS + NUMA + SCHED_SOVEREIGN | 
| **Allocator** | `kernel/core/SovereignAllocator.cpp` | O(1) lockless slab | 
| **SPSC IPC** | `kernel/core/ipc/SovereignSPSCQueue.hpp` | Zero-copy ring buffers | 
| **Syscalls** | `kernel/core/SovereignSyscall.cpp` | Modular C dispatch table | 
| **VFS** | `kernel/core/SovereignVFS.cpp` | ZFS-inspired virtual FS | 
| **Vulkan** | `kernel/core/vulkan/sovereign_vulkan.c` | Direct SPIR-V GPU routing | 
| **UI** | `kernel/core/SovereignZenithUI.cpp` | Glassmorphic compositor | 
| **PQC** | `kernel/core/SovereignPQC.cpp` | Dilithium-5 attestation | 

---

## Boot Sequence

1. `SovereignHAL::initializeHAL()` — CPU arch detection, MMIO mapping
2. `SovereignAllocator::init()` — Slab bucket setup
3. `syscall_init()` — Dispatch table population
4. `sigma_scheduler_numa_balance()` — NUMA shard pinning
5. `SovereignVFS::mount()` — Root filesystem mount
6. `svk_init()` — GPU command queue reset
7. Drop to Ring-3, launch `sigma-sh`

---

## 🚀 Expanded Layered Architecture Diagram

The **SigmaOS Sovereign Lattice** is built on a decoupled, failure-isolated, 4-tier model integrating advanced computing foundations and Linux distribution models:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      TOP LAYER: USER-FACING INTELLIGENCE                    │
│   ┌──────────────────────────┐  ┌────────────────────────┐  ┌───────────┐   │
│   │ NLP (spaCy / HF Tokens)  │  │ Bayesian Networks      │  │ GraphQL   │   │
│   ├──────────────────────────┤  ├────────────────────────┤  ├───────────┤   │
│   │ Real-Time Forecasters    │  │ Interactive D3/Plotly  │  │ WASM GUI  │   │
│   └──────────────────────────┘  └────────────────────────┘  └───────────┘   │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
┌──────────────────────────────────────▼──────────────────────────────────────┐
│                    MIDDLE LAYER: WAREHOUSE, PIPELINES & MODEL                │
│   ┌──────────────────────────┐  ┌────────────────────────┐  ┌───────────┐   │
│   │ Galaxy Hybrid Schemas    │  │ Apache Airflow DAGs    │  │ Neo4j     │   │
│   ├──────────────────────────┤  ├────────────────────────┤  ├───────────┤   │
│   │ Columnar Parquet / ORC   │  │ Min-Max / Robust CIRT  │  │ Ontologies│   │
│   └──────────────────────────┘  └────────────────────────┘  └───────────┘   │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
┌──────────────────────────────────────▼──────────────────────────────────────┐
│                  FOUNDATION LAYER: SOVEREIGN KERNEL & COMPUTE               │
│   ┌──────────────────────────┐  ┌────────────────────────┐  ┌───────────┐   │
│   │ PQC Kernel (Dilithium-5) │  │ Ring-3 Xen Micro-VMs   │  │ Formal Coq│   │
│   ├──────────────────────────┤  ├────────────────────────┤  ├───────────┤   │
│   │ Dynamic GPU scheduling   │  │ Gentoo Auto-Optimize   │  │ SELinux   │   │
│   └──────────────────────────┘  └────────────────────────┘  └───────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
  ▲                                                                         ▲
  └────────────────────────────────────┬────────────────────────────────────┘
                  CROSS-CUTTING: COMPLIANCE, TRACING & AUDIT
     ┌─────────────────────────────────────────────────────────────────┐
     │ Coverity / SonarQube Static analysis | Strace & Perf tracing    │
     └─────────────────────────────────────────────────────────────────┘
```

---

## 📈 Unified Sovereign End-to-End Workflow

The following pipeline demonstrates how a real-time data frame moves from distributed ingestion to core operating system scheduling adjustments:

```
 ┌──────────────────────┐      Ingested SQL Transaction
 │ Relational Ingestion │ ───► Star/Snowflake Hybrid Galaxy DB (ACID + BASE)
 └──────────────────────┘
            │
            ▼
 ┌──────────────────────┐      Sovereign Airflow Pipeline
 │   Data Pipeline/ETL  │ ───► Columnar Parquet -> Robust Mean/IQR Imputer
 └──────────────────────┘
            │
            ▼
 ┌──────────────────────┐      spaCy Tokenizer & PyTorch model
 │  AI / ML Inference   │ ───► Logistical Predictions & Federated Learning
 └──────────────────────┘
            │
            ▼
 ┌──────────────────────┐      GraphQL Subscriptions API
 │  Visual Analytics    │ ───► WASM-boosted Grafana Dashboard (D3.js)
 └──────────────────────┘
            │
            ▼
 ┌──────────────────────┐      S-AI-TEL System Telemetry Shield
 │   Lattice Feedback   │ ───► Dynamic CFS Scheduler CPU Boost (SLA Enforced)
 └──────────────────────┘
```

1.  **Ingestion**: Incoming enterprise data transactions are captured by CockroachDB/YugabyteDB compatible interfaces inside `SovereignOmniMatrix.cpp`.
2.  **ETL & Preprocessing**: Automated DAG workflows transform raw tabular streams into high-density Columnar Parquet structures. Missing entries are dynamically imputed, and values are normalized via `SovereignDataPreprocess.cpp`.
3.  **AI Predictions & NLP**: Natural Language processing modules (spaCy style) tokenize query metadata, generating input feature vectors. ML models execute local PyTorch/TensorFlow predictions while federated learning keeps data private.
4.  **Web Presentation**: A high-efficiency GraphQL subscription API pushes updated analytical points to user browsers, where WebAssembly-accelerated canvases draw interactive Grafana-style dashboards.
5.  **Adaptive Scheduling Feedback**: The SLA monitor (`sigma_enterprise_sla_manager.cpp`) intercepts long-tail latencies. It signals the microkernel CFS scheduler to elevate worker threads, ensuring system uptime and meeting SLAs.

---
> **Verification Status:** BUILD-VERIFIED | 100% SILICON PURITY | PARITY ACHIEVED  
> *Last updated: 2026-05-19 | SigmaOS Zenith v15.2*

# 🧩 Separation of Policy and Mechanism Architecture Roadmap

This document outlines the architectural strategy, design specification, and implementation details for the **Separation of Policy and Mechanism, Protection, and Interrupts Layer** in **SigmaOS**, ensuring microkernel resilience.

---

## 🗺️ 1. Paradigm Vision: Policy/Mechanism Separation

Traditional operating systems fuse policy (what gets allocated) with mechanism (how it is allocated) inside the kernel core. This makes changing resource allocations, scheduling weights, or user restrictions impossible without kernel re-compiles.

**SigmaOS** introduces a **Strict Policy-Mechanism Division**:

```text
       +---------------------------------------------+
       |               Policy Manager                |  <--- Handles limits, quotas, permissions
       +---------------------------------------------+
                              | (Enforces)
                              v
       +---------------------------------------------+
       |               Resource Broker               |  <--- Mechanism: Allocates units
       +---------------------------------------------+
```

By decoupling raw allocations (Mechanism) from permission/quota validation (Policy), SigmaOS provides a dynamically adaptable runtime environment.

---

## 🏗️ 2. Key Architecture Blocks

### 2.1 Separation of Policy and Mechanism
* **ResourceBroker**: Acts as the pure mechanism. It performs unit allocations, deallocations, and manages current state counters.
* **PolicyManager**: Encapsulates decision logic. It evaluates active user quotas and capability verification tokens before authorizing any structural broker changes.

### 2.2 Protection and Isolation (`ProtectionDomain`)
* **Address Compartmentalization**: Wraps specific base/limit addresses inside highly restricted boundary classes.
* **Isolated Capabilities**: Ensures that a task running within a protection domain is strictly sandboxed.

### 2.3 Interrupt Handling & Privilege Levels (`InterruptMechanism`)
* **Vector Dispatching**: Maps classic system vectors safely to user-space handler routines.
* **Privilege State Transitions**: Drives transitions from `UserMode` to `KernelMode` safely.

### 2.4 Optimization for the Common Case (`FastPathIpc`)
* **Local Fast-Path IPC**: Bypasses heavy system-wide lookup directories for local, common-case IPC, utilizing atomic, lock-free, zero-copy message exchanges in a single instruction loop.

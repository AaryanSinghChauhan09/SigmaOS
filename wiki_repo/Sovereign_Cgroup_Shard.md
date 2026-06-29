# SigmaOS Sovereign Cgroup Shard (S-Cgroup)

The **Sovereign Cgroup Shard** is a core silicon governance and resource scheduling subsystem of SigmaOS Zenith v15.1. It provides zero-dependency, silicon-native resource accounting, automatic throttle limits, and hardware partition isolation.

By executing directly within the scheduling path, it absorbs the defining advantages of **Linux cgroups v2**, **Windows Job Objects**, and **Kubernetes ResourceQuota** without the performance overhead of external user-space agents.

---

## 🚀 Architectural Design & Parity

| Feature Domain | SigmaOS S-Cgroup | Linux cgroups v2 | Windows Job Objects | Kubernetes | 
| :--- | :--- | :--- | :--- | :--- | 
| **Purity** | Freestanding C++17 | Monolithic C / SysFS | Win32 / NT Kernel APIs | YAML Orchestrated | 
| **CPU Accounting** | Direct scheduler sweep | CFS bandwidth controller | Job Cpu Rate limits | CPU Shares / Limits | 
| **Memory Isolation** | Transactional Slab accounting | Memory controller / OOM | Memory limit / Job limits | Memory request / Limit | 
| **I/O Regulation** | Silicon weights matrix | blkio I/O throttle | I/O rate limits | Storage IOPS Quota | 
| **Execution Path** | Native microkernel scheduler | Kernel cgroup subsystem | Object Manager hooks | Kubelet / containerd | 

---

## ⚙️ Core Subsystem Architecture

The cgroup system maintains up to 12 active resource partitions in a zero-dependency static matrix. The scheduler references this matrix on every context switch to calculate thread budgets.

```mermaid
graph TD
    A[Zenith Microkernel Scheduler] --> B[Sovereign Cgroup Governor]
    B --> C{Resource Limits Exceeded?}
    C --> | YES | D[Apply Silicon Thread Throttling]
    C --> | NO | E[Proceed with O(1) Thread Dispatch]
    D --> F[Active Groups Matrix]
    E --> F

```

### Resource Partition Matrix

***`zenith_kernel`**: Reserved partition for system-critical core shards (80% CPU quota, 4GB Memory limit, High 900 IO Weight).***`citizen_apps`**: Default partition for authenticated user-space workloads (60% CPU quota, 2GB Memory limit, Medium 500 IO Weight).* **`guest_sandbox`**: Hard-isolated sandbox partition for untrusted / alien binaries (20% CPU quota, 512MB Memory limit, Low 100 IO Weight).

---

## 🛠️ Command-Line Interface (CLI)

The `sigma-cgroup` utility allows real-time, zero-reboot manipulation of partition constraints:

```bash

# Create a new resource group

sigma-cgroup create <name> <cpu_pct> <mem_mb> <io_weight>

# Manually trigger automatic governor sweep and quota check

sigma-cgroup enforce

# Print audit report of all resource groups with live CPU/MEM/IO accounting

sigma-cgroup audit

```

### Active Governor Sweeps

When the auto-governor sweeps, it analyzes live thread performance indexes. If a partition like `guest_sandbox` exceeds its CPU allotment (e.g., executing a complex loop), the governor marks it as `throttled`, forcing the microkernel scheduler to skip its slices until load stabilizes.

---

## 📂 Source Code Implementation

The S-Cgroup subsystem is built across the following zero-dependency files:

***Core Logic**: `kernel/core/SovereignCgroup.cpp`***Management CLI**: `tools/sigma_cgroup.cpp`* **Syscall Bridge**: `include/syscall_dispatcher.h`

---

> [!NOTE]
> All S-Cgroup boundaries are attested using Kyber-1024 Post-Quantum Cryptographic signatures, preventing unauthorized privilege escalation.

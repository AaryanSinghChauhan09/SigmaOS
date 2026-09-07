# SigmaOS Processor Topology, CPU Scheduling & Multi-Core Management Guide for AI Agents

This guide provides technical specifications, instruction set architecture (ISA) auto-detection, multi-core CPU scheduling algorithms, NUMA-aware work-stealing, and context switching invariants for AI agents managing processor subsystems in SigmaOS.

---

## 1. Zero-Dependency Processor Architecture

SigmaOS handles multi-core CPU architecture and execution context switching natively under `#![no_std]` Rust:

### 1.1 Instruction Set Architecture (ISA) Level Auto-Detection
* **x86-64 ISA Levels (`src/klib/isa.rs` & `src/unimplemented_features.rs`):**
  * `x86-64-v1`: Baseline 64-bit CPU features (CMPXCHG16B, CMOV, MMX, SSE, SSE2).
  * `x86-64-v2`: CMPXCHG16B, LAHF-SAHF, POPCNT, SSE3, SSSE3, SSE4.1, SSE4.2.
  * `x86-64-v3`: AVX, AVX2, BMI1, BMI2, F16C, FMA, LZCNT, MOVBE, OSXSAVE.
  * `x86-64-v4`: AVX-512 foundation (AVX512F, AVX512BW, AVX512CD, AVX512DQ, AVX512VL).
* **Vectorized `memcpy` Routing:**
  `klib::isa::vectorized_memcpy` dynamically dispatches optimized memory copy loops based on detected ISA level (AVX-512 > AVX2 > SSE2 > scalar).

---

## 2. Advanced Kernel Scheduler Algorithms (`src/scheduler/process.rs`)

SigmaOS implements hybrid CPU scheduling policies designed for low-latency interactive responsiveness and high-throughput server workloads:

### 2.1 EEVDF (Earliest Eligible Virtual Deadline First)
* Computes virtual run time $V_t$, lag time $L_i = V_t - v_i$, and virtual deadline $d_i = v_i + \frac{q}{w_i}$.
* Guarantees fair CPU time distribution proportional to task weight $w_i$.

### 2.2 BORE (Burst-Oriented Response Enhancer)
* Monitors thread execution burst history.
* Tasks with short interactive CPU bursts receive temporary priority boosts for desktop GUI responsiveness, while long compute-bound bursts are smoothly demoted to background queues.

---

## 3. SMP Multi-Core Work-Stealing & NUMA Topology

### 3.1 NUMA-Aware Work Stealing
* Each CPU core maintains a local lock-free run queue.
* When a CPU queue becomes idle, it attempts to steal tasks from sibling cores on the same NUMA node before stealing across inter-socket NUMA interconnects to minimize cache line bouncing and remote RAM access latency.

### 3.2 Context Switching Invariants
* **Task Descriptor Invariants (`SimpleProcess` / `SimpleKernelTask`):**
  Stores explicit byte lengths (`name_len: u8`) initialized during task creation to allow $O(1)$ direct slice lookups instead of $O(N)$ null-byte scans.
* **Process State Transitions:**
  State changes (`Ready`, `Running`, `Blocked`, `Zombie`) MUST update task descriptors atomically under lock-free or mutex synchronization.

---

## 4. Mathematical Performance Scaling Laws (`src/performance/scaling_laws.rs`)

AI agents tuning scheduler queue depth or core allocation MUST evaluate:

1. **Amdahl's Law (`AmdahlScalingModel`):**
   Predicts parallel speedup upper bounds given serial code fraction $s$ and CPU count $N$.
2. **Gustafson's Law (`GustafsonScalingModel`):**
   Evaluates scaled speedup when problem size grows with core count.
3. **Gunther's Universal Scalability Model (`UniversalScalabilityModel`):**
   Accounts for hardware contention $\alpha$ and inter-core coherency delay $\beta$:
   $$\text{Capacity}(N) = \frac{N}{1 + \alpha(N - 1) + \beta N (N - 1)}$$

---

## 5. Checklist for AI Agents Managing Processor Subsystems

1. **Verify Task Name Field:** Ensure `SimpleProcess` stores `name_len: u8` to maintain $O(1)$ `Process::name()` performance.
2. **Validate ISA Detection:** Run ISA level inspection unit tests:
   ```bash
   cargo test --lib -- klib::isa::tests
   ```
3. **Validate Scheduler Tests:**
   ```bash
   cargo test --lib -- scheduler::tests
   ./run_sigma_tests.sh
   ```

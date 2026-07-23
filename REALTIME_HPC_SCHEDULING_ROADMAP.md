# ⚡ SigmaOS Real-Time & HPC Scheduling (Predictive EEVDF, MPI, SigmaPower) Development Roadmap

This document establishes the strategic engineering and design roadmap for **SigmaOS's High-Performance Computing (HPC) & Real-Time variants**, taking inspiration from Linux RT (`PREEMPT_RT`) and supercomputer schedulers (`Slurm`, `Lustre`).

---

## 🏗️ 1. Technical Vision & Real-Time Performance

SigmaOS utilizes a unified **EEVDF (Earliest Eligible Virtual Deadline First) Scheduler** that can be scaled dynamically using compiler profile states (`rtos` vs. `cloud` vs. `standalone`).

```
       +-------------------------------------------------------+
       |                  EEVDF Core Scheduler                 |
       +-------------------------------------------------------+
            |                        |                       |
            v                        v                       v
   +-----------------+      +-----------------+      +-----------------+
   |   PREEMPT_RT    |      |   HPC Cluster   |      |   SigmaPower    |
   |   (Hard RT-OS)  |      |   (MPI/Slurm)   |      | (Ondemand/Saver)|
   +-----------------+      +-----------------+      +-----------------+
```

---

## ⚡ 2. PREEMPT_RT Hard Real-Time Scheduler Variant (Rust)

### 2.1 Low-Latency Scheduling Bypass Queue
- **Inspiration**: Linux `PREEMPT_RT` patch and Xenomai Co-kernel.
- **Implementation (Rust)**: The scheduler in `src/kernel/scheduler.rs` maintains an active `is_realtime_profile` flag. When enabled, all non-RT task scheduling is bypassed when critical real-time interrupts are triggered, bounding worst-case latency to sub-microseconds.

### 2.2 Threaded Interrupt Handlers
- Interrupt service routines (ISRs) are executed inside low-priority kernel threads, preventing standard driver processing from stalling critical real-time execution flows.

---

## 💻 3. HPC Cluster & Parallel Compute Orchestration (Zig)

### 3.1 Clustered Zero-Copy Communication
- **Inspiration**: Slurm workload manager, Lustre parallel filesystem, and MPI.
- **Implementation (Zig)**: HPC profiles utilize highly efficient memory mapping overlays and direct DMA transfers. Processes bypass virtual memory address translation overheads during cluster message passing, maximizing node throughput.

---

## 🔋 4. SigmaPower: Adaptive Energy Management (Rust / Nim)

### 4.1 Predictive ML Frequency Governor
- **Inspiration**: Linux `cpufreq`, `powertop`, and `TLP`.
- **Implementation (Rust)**: The energy manager in `src/power/management.rs` tracks task execution frequencies and temperatures.
- **Implementation (Nim)**: Userland governors adjust CPU P-states and cooling levels dynamically using local predictive AI routines (`src/automation/system_level.rs`) to prevent thermal throttling.

---

## 🧠 5. CPU ALU, CU, & Processor Registers Context-Saving (Rust / Assembly)

To achieve zero preemption overhead during context switching under RT profiles, SigmaOS optimizes low-level processor state handling:
*   **Arithmetic Logic Unit (ALU) & Control Unit (CU)**: Low-level interrupt controllers lock pipeline execution during critical sections to prevent instruction decoding bubble states.
*   **Vector/FPU Register Spill Optimization**: Rather than saving all 512-bit AVX/SIMD vector registers on every switch, SigmaOS utilizes a **lazy register allocation** trap. FPU and vector registers are only saved/restored if the newly scheduled thread actually executes an ALU vector instruction.

```rust
// Unified low-level CPU register context state representing a thread switch context
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CpuRegisterContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub fpu_saved: bool, // Lazy saving flag for SIMD/FPU ALU registers
}

impl CpuRegisterContext {
    pub fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0,
            rbp: 0, rsp: 0, rip: 0, rflags: 0x200, fpu_saved: false,
        }
    }
}
```

---

## 📋 6. Core Automation, Macro Recording, and Auto-Optimization Subsystems (Rust)

To match the automation power of modern environments, SigmaOS implements:
*   **Task Scheduler**: A lock-free, chronological job execution list that parses crontab syntax.
*   **Macro Recorder & Auto-Optimizer**: Records system-wide user shell inputs into `.sigma-macro` scripts, and invokes adaptive cache/performance governor tuning dynamically.

```rust
// Represents a scheduled automation task trigger
#[derive(Debug, Clone, Copy)]
pub struct AutomationTask {
    pub id: usize,
    pub cron_hour: u8,
    pub cron_minute: u8,
    pub require_ac_power: bool,
}

pub struct AutomationEngine {
    pub is_recording_macro: bool,
    pub is_idle_optimization_enabled: bool,
}

impl AutomationEngine {
    pub fn new() -> Self {
        Self {
            is_recording_macro: false,
            is_idle_optimization_enabled: true,
        }
    }
}
```

---

## 📅 7. Step-by-Step Implementation Roadmap

- [ ] **Phase 1 (Verification)**: Complete EEVDF real-time priority schedules and HPC bypass hooks in `src/kernel/scheduler.rs`.
- [ ] **Phase 2 (Zig HPC Shard)**: Develop low-latency clustering communication and direct DMA bypass drivers in Zig.
- [ ] **Phase 3 (Nim Power Daemon)**: Code the user-space adaptive TLP governor in Nim to scale CPU frequency.

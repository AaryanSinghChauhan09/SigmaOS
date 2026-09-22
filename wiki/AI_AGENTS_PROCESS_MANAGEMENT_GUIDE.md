# SigmaOS AI Agents System Process Management & Scheduler Architecture Guide

Welcome to the **SigmaOS AI Agents System Process Management Guide**. This document details task scheduling algorithms, process lifecycle states, ELF binary execution, POSIX process control, and job containment for autonomous AI agents and kernel developers in SigmaOS.

---

## 1. Process Lifecycle & Scheduler Architecture

SigmaOS manages system processes through a zero-dependency, microkernel-based process management architecture (`src/process/`, `src/scheduler/`):

### Lifecycle States
1. **`Ready`**: Task resides in the runqueue awaiting CPU time slice assignment.
2. **`Running`**: Task currently executing on a CPU core.
3. **`Blocked`**: Task waiting on I/O, IPC message arrival, or semaphore synchronization (`src/process/blocked_state.rs`).
4. **`Zombie`**: Terminated process awaiting parent `waitpid(2)` exit status harvest.

### Dual Scheduler Engines (`src/scheduler/`)
- **EEVDF Scheduler (Earliest Eligible Virtual Deadline First)**: Fair-share virtual deadline scheduling guaranteeing latency bounds for real-time and interactive GUI threads.
- **CachyOS BORE Scheduler (Burst-Oriented Response Enhancer)**: Dynamic burstiness score calculation prioritizing interactive desktop and AI inference tasks over batch compute jobs.

---

## 2. ELF Binary Loading & POSIX Control

AI agents spawning or supervising native binaries interface with `ElfLoader` and `ProcessManager` (`src/process/elf_loader.rs`, `src/process/advanced_process_control.rs`):

```rust
use sigmaos::process::elf_loader::ElfLoader;
use sigmaos::process::advanced_process_control::AdvancedProcessControl;

// Load and validate ELF x86_64 / AArch64 executable header
let elf_data = include_bytes!("../../build/runner1/libsigmaos.rlib");
let mut loader = ElfLoader::new();
assert!(loader.parse_header(elf_data).is_ok());

// Spawn supervised child process
let mut control = AdvancedProcessControl::new();
let child_pid = control.spawn_process("agent_helper", "/usr/bin/helper").expect("Failed to spawn process");
```

---

## 3. Job Objects & cgroups v2 Containment

To prevent runaway process trees from consuming excessive system resources:

- **Job Objects (`src/process/job_objects.rs`)**: Group processes into atomic containment units with hard limits on total active process count and maximum memory.
- **cgroups v2 Slices**: Hierarchical resource control (`/sys/fs/cgroup/system.slice`) limiting CPU bandwidth and RAM quotas per process group.

---

## 4. Checklist for AI Agents Managing Process Subsystems

- [ ] Confirmed process IDs (PIDs) are allocated using thread-safe atomic counters.
- [ ] Checked that parent processes harvest terminated child exit statuses to eliminate zombies.
- [ ] Verified EEVDF virtual deadlines update smoothly without integer overflow.
- [ ] Tested process spawn and job object containment under heavy workload simulation.
- [ ] Executed `./run_sigma_tests.sh` to confirm process scheduler test suites (`test_kernel_scheduler_algorithm_inspection`) pass cleanly.

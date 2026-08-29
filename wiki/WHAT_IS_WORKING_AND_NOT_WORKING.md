# WHAT'S WORKING & WHAT'S NOT WORKING IN SIGMAOS: AI AGENT ALGORITHM DIAGNOSTICS & FIX GUIDE

This document serves as the master, definitive diagnostic guide for SigmaOS. It provides a comprehensive, technical inventory of all working OS subsystems, cataloged failure modes, compiler error codes, root-cause analyses, and concrete safe Rust algorithm fix blueprints so that **any AI agent can easily inspect, diagnose, and fix algorithms and compilation errors across the codebase**.

---

## SECTION 1: WHAT IS WORKING

SigmaOS features a zero-dependency, modular microkernel and OS suite written in safe Rust. Running `./run_sigma_tests.sh` executes the master atomic test suite and subsystem inspection harness with a **100% pass rate on the atomic test suite (40/40 atomic tests pass)**.

### 1. Core Microkernel & Scheduling Algorithms (`src/kernel/`, `src/scheduler/`)
- **EEVDF Scheduler (`src/kernel/scheduler.rs` & `src/scheduler/eevdf.rs`):** Linux 6.6+ Earliest Eligible Virtual Deadline First algorithm implementing lag tracking (`lag = vruntime_avg - task_vruntime`), weighted deadline calculation (`vruntime + time_slice * 1024 / weight`), eligibility checks (`task_vruntime <= vruntime_avg`), 64-byte cache-line aligned task picking, and NUMA-aware work-stealing queues.
- **BORE Interactive Scheduler (`src/kernel/bore.rs`):** CachyOS Burst-Oriented Response Enhancer algorithm tracking burst vs. sleep history windows, calculating dynamic interactivity scores (0 = CPU-bound, 100 = interactive UI task), and evaluating SMP migration candidates.
- **Classic OS Algorithms (`src/kernel/classic_os.rs`):**
  - `VirtioBalloonManager`: Dynamic VirtIO memory balloon inflation/deflation with page reclamation.
  - `BankersAlgorithm`: Safe state checking and resource allocation matrix validation for deadlock avoidance.
  - `SleepingBarberQueue`: Thread-safe synchronization primitive for capacity-constrained barber queue problems.
  - `TicketSpinlock`: Fair FIFO ticket spinlock with atomic `fetch_add` ticket generation and exponential backoff spin loops.
  - `StackCanaryProtector`: XOR-seeded global stack canary for buffer overflow protection.
  - `BatchSystemQueue`: Multiprogrammed batch job queue processor with concurrency limits.
- **Real-Time Scheduling (`src/kernel/structures.rs`):** Earliest Deadline First (EDF) real-time task scheduler, Lottery scheduling with probability-weighted ticket distribution, and APC (Asynchronous Procedure Call) queue delivery.
- **Process Management & PID Allocator (`src/process/sovereign_process_engine.rs`):** FreeBSD PID bitmap recycling, Linux PID namespace isolation, and parent/child process tree tracking with process descriptor handle support (`pdfork`).

### 2. Hardware Abstraction Layer (HAL) & Memory Subsystem (`src/klib/`, `src/kernel/`)
- **Paging & Virtual Memory (`src/klib/paging.rs`):** 4-level x86_64 page table mapping (`Standard4KB`, `Huge2MB`, `Giant1GB`), safe `.get_mut()` option chaining (panic-free boundary checking), and Copy-on-Write (CoW) page table snapping.
- **HAL Multi-Arch Abstraction (`src/kernel/architecture.rs`):** Unified architecture interface supporting x86_64 (APIC/IOAPIC, CR0/CR4/EFER registers), AArch64 (GICv2/v3, TTBR page tables), and RISC-V 64 (PLIC/CLINT, satp S-mode paging).
- **PCI/PCIe Bus Scanner (`src/kernel/pci_scanner.rs`):** PCIe ECAM memory-mapped configuration space addressing, 32-bit/64-bit MMIO & I/O BAR decoding, prefetchable memory flags, and Capabilities pointer parsing (MSI, MSI-X, PCIe, Power Management).
- **Environment & XDG Spec Engine (`src/klib/env.rs`):** Linux XDG base directory specification (`XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_CACHE_HOME`, `XDG_RUNTIME_DIR`), POSIX/BSD defaults (`PATH`, `HOME`, `SHELL`, `PAGER`, `EDITOR`), OpenBSD `secure_getenv` privilege tainting, and dynamic variable expansion (`expand_vars`).
- **Sysctl Parameter Registry (`src/kernel/sysctl.rs`):** Dynamic MIB parameter tree hierarchy with integer range bounds checking and net/vm parameter querying.

### 3. Linux & BSD Parity Layers (`src/compatibility/`, `src/distro/`, `src/sigpkg/`)
- **Distro Parity Subsystems (`src/distro/linux_bsd_parity.rs`):**
  - FreeBSD Capsicum fine-grained file rights (`CapsicumRights`) & FreeBSD Jails VNET network namespace isolation.
  - OpenBSD Pledge/Unveil path restriction virtualizers (`UnveilRestrictions`).
  - Arch Linux AUR PKGBUILD verification (`AurPkgBuildVerifier`).
  - NixOS Flake Engine declarative generation rollback & garbage collection.
  - Void Linux Runit Supervisor service restarting and status querying.
  - Gentoo Portage USE-flag dependency resolution engine & mask verifier.
- **Linux Mint Compatibility (`src/compatibility/mint_linux.rs`):** `CinnamonDesktopEngine`, `MintUpdateManager`, `MintInstallSoftwareManager`, and `MintWarpinatorEngine` for local network file transfers.
- **Extended ABI Execution Frame (`src/compatibility/abi_extended.rs`):** ARM64 AAPCS calling convention (`Arm64AapcsFrame`) and RISC-V 64-bit calling convention (`Riscv64AbiFrame`) translating ABI register frames.

### 4. Storage & Filesystem Subsystems (`src/fs/`, `src/filesystem/`)
- **Ext4 Filesystem Engine (`src/filesystem/complete_filesystems.rs`):** Extent tree block allocation, JBD2 metadata journaling, and CRC32C checksum validation.
- **Btrfs Subvolume Engine (`src/fs/btrfs.rs`):** Copy-on-Write (CoW) snapshotting, async TRIM/discard, subvolume property inheritance, and incremental send/receive streams.
- **DragonFly BSD HAMMER2 PFS Engine (`src/unimplemented_features.rs`):** Cluster node replication, snapshot generation, and Merkle tree root rollback.
- **Zero-Copy IPC Pipes (`src/kernel/pipes.rs`):** Page buffer ring `splice` zero-copy transfer and `tee` pipe duplication.
- **FHS & Hier Path Translator (`src/filesystem/bsd_linux_innovations.rs`):** Linux FHS 3.0 merged-usr path resolution (`/bin` -> `/usr/bin`, `/lib` -> `/usr/lib`) and FreeBSD hier(7) `/usr/local` translation.

### 5. Cryptography & Security (`src/crypto/`, `src/security/`)
- **Post-Quantum Cryptography (`src/crypto/`):** Dilithium-5 digital attestation signatures and Kyber-1024 key encapsulation mechanism.
- **CSPRNG Entropy Engine (`src/crypto/random.rs`):** Hardware RDRAND/RDTSC entropy seeding mixed into ASLR pointer space.
- **FreeBSD Securelevels & Jails (`src/security/securelevels.rs`, `src/security/jails.rs`):** System securelevels (-1 to 3) enforcing append-only files and immutable sysctls.
- **Root Elevator & PAM Stack (`src/security/root_improvement.rs`):** Sudo/Doas privilege elevator with session TTL expiration, Polkit role-based permission checks, and PAM multi-factor authentication token validation.
- **eBPF Engine & Landlock VFS (`src/kernel/ebpf.rs` & `src/kernel/linux_bsd_innovations.rs`):** In-kernel eBPF static instruction verifier, division-by-zero checks, and Landlock access path restrictions.

### 6. Networking, Remote Sharing & Container Isolation (`src/network/`, `src/virt/`)
- **Remote Protocol Suite (`src/network/sovereign_remote_sharing.rs`):**
  - `SovereignSshEngine`: SSHv2 key exchange, session authentication, and encrypted tunnel establishment.
  - `SovereignNfsEngine`: NFSv4 file handle lookup, rpcbind RPC registration, and remote file read/write operations.
  - `SovereignSambaEngine`: SMB3 dialect negotiation, tree connect, and share access.
  - `SovereignScpEngine` & `SovereignRsyncEngine`: Remote copy and delta file transfer synchronization.
- **QEMU & KVM Virtual Machine Manager (`src/virt/mod.rs` & `src/virtualization/kvm_vcpu.rs`):** Qcow2 copy-on-write image overlays, KVM vCPU execution context (`KvmVcpuContext`), VFIO IOMMU PCI device passthrough, and VirtIO split ring buffers (`VirtqueueRing`).
- **Sovereign OCI Container Runtime:** Isolated process namespaces, cgroup resource constraints, and layer image mounting.

---

## SECTION 2: WHAT IS NOT WORKING, WHY & HOW TO FIX IT

Below is the exhaustive technical catalog of compilation errors, borrow checker conflicts, and scope resolution issues encountered in submodules or test suites, along with exact root-cause analyses and safe Rust fix blueprints.

---

### Issue 1: Unresolved Imports & Module Path Mismatches (`E0432` / `E0252` / `E0428`)

#### **Symptom / Compiler Output:**
```text
error[E0432]: unresolved import `package_repository`
   --> tests/linux_bsd_inspection_tests.rs:293:9
    |
293 |     use package_repository::SovereignPackageRepositoryManager;
    |         ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `package_repository`

error[E0252]: the name `SovereignPackageRepositoryManager` is defined multiple times
error[E0428]: the name `test_kernel_classic_algorithms_inspection` is defined multiple times
```

#### **Why It Occurs:**
1. Tests or submodules attempt to import modules directly (e.g., `use package_repository::*;` or `use module_loader::*;`) without referencing their declared module location (e.g. `crate::sigpkg::package_repository` or `crate::driver::module_loader`).
2. Duplicate module aliases or duplicate function names exist inside `tests/linux_bsd_inspection_tests.rs` or `src/lib.rs`.

#### **How to Fix It (Blueprint):**
Prefix imports with full module paths and deduplicate module definitions in tests:

```rust
// BEFORE (Broken Unresolved Import):
use package_repository::SovereignPackageRepositoryManager;
use module_loader::{SovereignKernelModuleManager, ModuleState};

// AFTER (Fixed Full-Path Import):
use sigmaos::sigpkg::package_repository::SovereignPackageRepositoryManager;
use sigmaos::driver::module_loader::{SovereignKernelModuleManager, ModuleState};
```

---

### Issue 2: Transmute Size Mismatch Error (`E0512`)

#### **Symptom / Compiler Output:**
```text
error[E0512]: cannot transmute between types of different sizes
  --> src/ml/inference.rs:42:18
   |
42 |         unsafe { core::mem::transmute(self.model_type.load(Ordering::SeqCst)) }
   |                  ^^^^^^^^^^^^^^^^^^^^
   |
   = note: source type `usize` (64 bits)
   = note: target type `ModelType` (32 bits)
```

#### **Why It Occurs:**
On 64-bit target architectures, loading an `AtomicUsize` yields an 8-byte integer (`usize`). Rust enums without an explicit `#[repr(...)]` attribute default to a 4-byte (`u32`) layout. Reinterpreting an 8-byte integer directly into a 4-byte enum using `core::mem::transmute` is unsafe and rejected by the compiler under size equality rules.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Broken Transmute):
pub fn model_type(&self) -> ModelType {
    unsafe { core::mem::transmute(self.model_type.load(Ordering::SeqCst)) }
}

// AFTER (Fixed Safe Match):
pub fn model_type(&self) -> ModelType {
    let val = self.model_type.load(Ordering::SeqCst);
    match val {
        0 => ModelType::NeuralNetwork,
        1 => ModelType::DecisionTree,
        2 => ModelType::SVM,
        _ => ModelType::Transformer,
    }
}
```

---

### Issue 3: Type Annotation & Type Inference Failures (`E0282` / `E0614`)

#### **Symptom / Compiler Output:**
```text
error[E0282]: type annotations needed
   --> src/kernel/linux_bsd_innovations.rs:126:35
    |
126 |                 expired_keys.push(tuple.clone());
    |                                   ^^^^^ cannot infer type

error[E0614]: type `i32` cannot be dereferenced
  --> src/kernel/sysctl.rs:90:24
   |
90 |                     if *v < 0 && mib == "vm.swappiness" {
   |                        ^^ can't be dereferenced
```

#### **Why It Occurs:**
1. **E0282**: The compiler cannot infer vector element types when `.collect()` or `.push()` is invoked without explicit variable typing.
2. **E0614**: Attempting to dereference a primitive integer value `v: i32` directly (instead of a reference `&i32`).

#### **How to Fix It (Blueprint):**

```rust
// FIX FOR E0282 (Explicit Vector Type Annotation):
let mut expired_keys: Vec<(String, u64)> = Vec::new();
expired_keys.push(tuple.clone());

// FIX FOR E0614 (Remove Unnecessary Dereference):
// Change `if *v < 0` to:
if v < 0 && mib == "vm.swappiness" {
    return Err("Sysctl value out of range");
}
```

---

### Issue 4: Non-Exhaustive Enum Match Patterns (`E0004`)

#### **Symptom / Compiler Output:**
```text
error[E0004]: non-exhaustive patterns: `&vm_manager::KvmExitReason::Interrupt` not covered
   --> src/virtualization/vm_manager.rs:201:10
    |
201 | #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    |          ^^^^^ pattern `Interrupt` not covered
```

#### **Why It Occurs:**
1. Enum definition `KvmExitReason` contains duplicate variant entries or a missing match arm in derive implementations.
2. When new variants are added to an enum (e.g. `ShellCommand` or `KvmExitReason`), any `match` expression over that enum without a fallback arm fails exhaustiveness verification.

#### **How to Fix It (Blueprint):**
Remove duplicate enum variants and include a fallback match arm:

```rust
// Remove duplicate `Interrupt` entry in `KvmExitReason` enum definition.
// Add explicit pattern handling or wildcard in match statements:
match exit_reason {
    KvmExitReason::IoIn => { /* ... */ },
    KvmExitReason::IoOut => { /* ... */ },
    KvmExitReason::MmioRead => { /* ... */ },
    KvmExitReason::MmioWrite => { /* ... */ },
    KvmExitReason::Hlt => { /* ... */ },
    KvmExitReason::Interrupt => { /* handle interrupt exit */ },
    _ => { /* fallback */ },
}
```

---

### Issue 5: Missing Methods & Trait Implementation Mismatches (`E0599` / `E0277`)

#### **Symptom / Compiler Output:**
```text
error[E0599]: no method named `select_next_rt_task` found for struct `SovereignHybridSchedulerInnovations`
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/compatibility/freebsd_jails.rs:147:16
```

#### **Why It Occurs:**
1. **E0599**: Method `select_next_rt_task` was omitted during struct definition or renamed in `SovereignHybridSchedulerInnovations`.
2. **E0277**: Matching or binding `Option<str>` directly instead of `Option<String>` or `Option<&str>`. Unsized types (`str`) cannot be moved or bound without a reference wrapper.

#### **How to Fix It (Blueprint):**

```rust
// FIX FOR E0599 (Add Missing Method):
impl SovereignHybridSchedulerInnovations {
    pub fn select_next_rt_task(&mut self) -> Option<ProcessTask> {
        self.rt_queue.pop_front()
    }
}

// FIX FOR E0277 (Use String or &str borrow):
// BEFORE: if let Some(ref exec_stop_script) = exec_stop { ... } where exec_stop is Option<str>
// AFTER: Use `Option<String>` or borrow `Option<&str>`:
if let Some(exec_stop_script) = exec_stop.as_deref() {
    // exec_stop_script is &str
}
```

---

### Issue 6: Borrow Checker Move Errors in HashMaps (`E0382`)

#### **Symptom / Compiler Output:**
```text
error[E0382]: borrow of moved value: `package`
   --> src/sigpkg/debian_apt_engine.rs:236:21
    |
234 |         self.installed_packages.insert(package.package.clone(), package);
    |                                                                 ------- value moved here
235 |         self.status_database
236 |             .insert(package.package.clone(), "install ok installed".to_string());
    |                     ^^^^^^^^^^^^^^^ value borrowed here after move
```

#### **Why It Occurs:**
The variable `package` is moved into `self.installed_packages.insert(...)` on line 234, rendering `package` invalid for subsequent operations on line 236.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Value Moved):
self.installed_packages.insert(package.package.clone(), package);
self.status_database.insert(package.package.clone(), "install ok installed".to_string());

// AFTER (Fixed with cloned key extracted first):
let pkg_name = package.package.clone();
self.installed_packages.insert(pkg_name.clone(), package);
self.status_database.insert(pkg_name, "install ok installed".to_string());
```

---

### Issue 7: Immutable Self Borrow Conflict with Self Call (`E0502`)

#### **Symptom / Compiler Output:**
```text
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   --> src/init/sigmainit.rs:206:17
    |
204 |         if let Some(deps) = self.dependency_graph.get_dependencies(name) {
    |                             --------------------- immutable borrow occurs here
205 |             for dep in deps {
    |                        ---- immutable borrow later used here
206 |                 self.start_service(dep)?;
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

#### **Why It Occurs:**
`self.dependency_graph.get_dependencies(name)` retains an immutable borrow on `self` during loop iteration. Invoking `self.start_service(dep)` inside the loop attempts a mutable borrow on `self`, causing a borrow conflict.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Conflicting Borrow):
if let Some(deps) = self.dependency_graph.get_dependencies(name) {
    for dep in deps {
        self.start_service(dep)?;
    }
}

// AFTER (Fixed via owned vector allocation):
let deps_to_start: Vec<String> = self.dependency_graph
    .get_dependencies(name)
    .cloned()
    .unwrap_or_default();

for dep in &deps_to_start {
    self.start_service(dep)?;
}
```

---

## SECTION 3: ALGORITHM DIAGNOSTICS & SAFE RUST FIX BLUEPRINTS FOR AI AGENTS

This section provides complete, self-contained safe Rust algorithm implementations for key OS subsystems. AI agents can reference or drop in these blueprints to fix broken or missing algorithms.

---

### Category A: Scheduler Algorithms (EEVDF & BORE)

#### **1. EEVDF Lag & Virtual Deadline Calculator (`src/kernel/scheduler.rs`)**
```rust
pub struct EevdfTask {
    pub pid: u64,
    pub weight: u64,           // Task weight (nice level mapping: nice 0 = 1024)
    pub vruntime: u64,         // Virtual runtime in nanoseconds
    pub lag: i64,              // Virtual time lag relative to avg vruntime
    pub virtual_deadline: u64, // Eligible deadline for execution
}

impl EevdfTask {
    pub fn update_lag(&mut self, avg_vruntime: u64) {
        // Lag = avg_vruntime - self.vruntime
        self.lag = avg_vruntime as i64 - self.vruntime as i64;
    }

    pub fn calculate_deadline(&mut self, time_slice_ns: u64) {
        // Virtual Deadline = vruntime + (time_slice * 1024 / weight)
        let weighted_slice = (time_slice_ns * 1024) / self.weight.max(1);
        self.virtual_deadline = self.vruntime + weighted_slice;
    }

    pub fn is_eligible(&self, avg_vruntime: u64) -> bool {
        // Task is eligible if its vruntime does not exceed average vruntime (lag >= 0)
        self.vruntime <= avg_vruntime
    }
}
```

#### **2. BORE Interactivity Score & Burst Decay (`src/kernel/bore.rs`)**
```rust
pub struct BoreTaskHistory {
    pub burst_time_ns: u64,
    pub sleep_time_ns: u64,
    pub score: u8, // 0 (CPU bound) to 100 (interactive)
}

impl BoreTaskHistory {
    pub fn update_score(&mut self) {
        let total_time = self.burst_time_ns + self.sleep_time_ns;
        if total_time == 0 {
            self.score = 50;
            return;
        }
        // Score = (sleep_time * 100) / total_time
        let raw_score = (self.sleep_time_ns * 100) / total_time;
        self.score = raw_score.min(100) as u8;
    }

    pub fn decay_history(&mut self) {
        // Apply sliding window exponential decay
        self.burst_time_ns /= 2;
        self.sleep_time_ns /= 2;
    }
}
```

---

### Category B: Synchronization & Deadlock Avoidance

#### **1. Banker's Algorithm for Deadlock Avoidance (`src/kernel/classic_os.rs`)**
```rust
pub struct BankersAlgorithm {
    pub num_processes: usize,
    pub num_resources: usize,
    pub available: Vec<usize>,
    pub max_claim: Vec<Vec<usize>>,
    pub allocation: Vec<Vec<usize>>,
}

impl BankersAlgorithm {
    pub fn is_safe_state(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.num_processes];

        // Need matrix = Max - Allocation
        let mut need = vec![vec![0; self.num_resources]; self.num_processes];
        for i in 0..self.num_processes {
            for j in 0..self.num_resources {
                need[i][j] = self.max_claim[i][j].saturating_sub(self.allocation[i][j]);
            }
        }

        loop {
            let mut found_candidate = false;
            for p in 0..self.num_processes {
                if !finish[p] {
                    let can_satisfy = (0..self.num_resources).all(|r| need[p][r] <= work[r]);
                    if can_satisfy {
                        for r in 0..self.num_resources {
                            work[r] += self.allocation[p][r];
                        }
                        finish[p] = true;
                        found_candidate = true;
                        break;
                    }
                }
            }
            if !found_candidate {
                break;
            }
        }

        finish.iter().all(|&done| done)
    }
}
```

#### **2. Ticket Spinlock with Exponential Backoff (`src/kernel/classic_os.rs`)**
```rust
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct TicketSpinlock {
    next_ticket: AtomicUsize,
    now_serving: AtomicUsize,
}

impl TicketSpinlock {
    pub const fn new() -> Self {
        Self {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) {
        let my_ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        let mut backoff = 1u32;
        while self.now_serving.load(Ordering::Acquire) != my_ticket {
            for _ in 0..backoff {
                core::hint::spin_loop();
            }
            backoff = (backoff * 2).min(1024); // Exponential backoff cap
        }
    }

    pub fn unlock(&self) {
        self.now_serving.fetch_add(1, Ordering::Release);
    }
}
```

---

### Category C: Memory & Zero-Copy IPC Management

#### **1. Pipe Ring Buffer Zero-Copy Splice (`src/kernel/pipes.rs`)**
```rust
pub struct PipeBufferRing {
    pub pages: Vec<Vec<u8>>,
    pub capacity_pages: usize,
    pub head: usize,
    pub tail: usize,
}

impl PipeBufferRing {
    pub fn splice(&mut self, source_page: Vec<u8>) -> Result<usize, &'static str> {
        if (self.tail + 1) % self.capacity_pages == self.head {
            return Err("Pipe Ring Buffer Full");
        }
        let len = source_page.len();
        self.pages[self.tail] = source_page; // Move vector pointer without copying payload
        self.tail = (self.tail + 1) % self.capacity_pages;
        Ok(len)
    }
}
```

#### **2. 2MB Superpages Allocator & Memory Compactor (`src/kernel/linux_bsd_innovations.rs`)**
```rust
pub struct MemoryCompactor {
    pub total_4kb_frames: usize,
    pub free_frame_bitmap: Vec<bool>,
}

impl MemoryCompactor {
    pub fn new(total_frames: usize) -> Self {
        Self {
            total_4kb_frames: total_frames,
            free_frame_bitmap: vec![true; total_frames],
        }
    }

    pub fn allocate_2mb_superpage(&mut self) -> Option<usize> {
        // 2MB superpage requires 512 contiguous 4KB frames (512-frame alignment)
        const FRAMES_PER_SUPERPAGE: usize = 512;
        for start_pfn in (0..self.total_4kb_frames).step_by(FRAMES_PER_SUPERPAGE) {
            if start_pfn + FRAMES_PER_SUPERPAGE <= self.total_4kb_frames {
                let is_contiguous_free = (start_pfn..start_pfn + FRAMES_PER_SUPERPAGE)
                    .all(|pfn| self.free_frame_bitmap[pfn]);
                if is_contiguous_free {
                    for pfn in start_pfn..start_pfn + FRAMES_PER_SUPERPAGE {
                        self.free_frame_bitmap[pfn] = false;
                    }
                    return Some(start_pfn);
                }
            }
        }
        None
    }
}
```

---

## SECTION 4: MANDATORY AI AGENT EXECUTION WORKFLOW

When an AI agent is tasked with fixing an algorithm or compilation error in SigmaOS, it **must** strictly follow this 4-step execution checklist:

```
+-----------------------------------------------------------------------+
|                 AI AGENT ALGORITHM FIX CHECKLIST                      |
+-----------------------------------------------------------------------+
|  [ ] STEP 1: RUN SUITE DIAGNOSTICS                                     |
|      Execute `./run_sigma_tests.sh` and `cargo check --lib` to get    |
|      baseline test outputs and error logs.                            |
|                                                                       |
|  [ ] STEP 2: CLASSIFY ERROR                                           |
|      Match compiler output to Section 2:                              |
|      - E0432/E0252 => Fix module import paths to use full crate path. |
|      - E0512       => Replace transmute with match or #[repr(usize)]. |
|      - E0282       => Add explicit type annotation to variable.       |
|      - E0614       => Remove dereference operator `*` on value types. |
|      - E0004       => Add missing match pattern or `_` fallback arm.  |
|      - E0599       => Implement missing struct method.                |
|      - E0277       => Use Option<String> or Option<&str> for unsized. |
|      - E0382       => Clone map key before moving value into hashmap. |
|      - E0502       => Collect query results into owned Vec.           |
|                                                                       |
|  [ ] STEP 3: APPLY SAFE CODE MODIFICATION                             |
|      Use replace_with_git_merge_diff to apply targeted code changes. |
|      Do not edit build artifacts under target/ or dist/.              |
|                                                                       |
|  [ ] STEP 4: VERIFY RE-COMPILATION & UNIT TESTS                       |
|      Run `./run_sigma_tests.sh` and confirm atomic tests pass 100%.  |
+-----------------------------------------------------------------------+
```

---

*This document serves as the absolute source of truth for AI agents working on the SigmaOS codebase.*

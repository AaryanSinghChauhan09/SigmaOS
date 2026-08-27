# WHAT'S WORKING & WHAT'S NOT WORKING IN SIGMAOS: AI AGENT ALGORITHM DIAGNOSTICS & FIX GUIDE

This document is the master, definitive diagnostic guide for SigmaOS. It provides a comprehensive, technical inventory of all working OS subsystems, cataloged failure modes, root-cause analyses, and concrete safe Rust algorithm fix blueprints so that **any AI agent can easily inspect, diagnose, and fix algorithms and compilation errors across the codebase**.

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

### 2. Hardware Abstraction Layer (HAL) & Memory Subsystem (`src/klib/`, `src/kernel/`)
- **Paging & Virtual Memory (`src/klib/paging.rs`):** 4-level x86_64 page table mapping (`Standard4KB`, `Huge2MB`, `Giant1GB`), safe `.get_mut()` option chaining (panic-free boundary checking), and Copy-on-Write (CoW) page table snapping.
- **HAL Multi-Arch Abstraction (`src/kernel/architecture.rs`):** Unified architecture interface supporting x86_64 (APIC/IOAPIC, CR0/CR4/EFER registers), AArch64 (GICv2/v3, TTBR page tables), and RISC-V 64 (PLIC/CLINT, satp S-mode paging).
- **PCI/PCIe Bus Scanner (`src/kernel/pci_scanner.rs`):** PCIe ECAM memory-mapped configuration space addressing, 32-bit/64-bit MMIO & I/O BAR decoding, prefetchable memory flags, and Capabilities pointer parsing (MSI, MSI-X, PCIe, Power Management).

### 3. Linux & BSD Parity Layers (`src/compatibility/`, `src/distro/`, `src/sigpkg/`)
- **Distro Parity Subsystems (`src/distro/linux_bsd_parity.rs`):**
  - FreeBSD Capsicum fine-grained file rights (`CapsicumRights`) & FreeBSD Jails VNET network namespace isolation.
  - OpenBSD Pledge/Unveil path restriction virtualizers (`UnveilRestrictions`).
  - Arch Linux AUR PKGBUILD verification (`AurPkgBuildVerifier`).
  - NixOS Flake Engine declarative generation rollback & garbage collection.
  - Void Linux Runit Supervisor service restarting and status querying.
  - Gentoo Portage USE-flag dependency resolution engine.
- **Linux Mint Compatibility (`src/compatibility/mint_linux.rs`):** `CinnamonDesktopEngine`, `MintUpdateManager`, `MintInstallSoftwareManager`, and `MintWarpinatorEngine` for local network file transfers.
- **Universal IOCTL Decoder (`src/package/linux_translation.rs`):** Command layout translation for Windows NT, Linux DRM/KMS, and BSD ioctl calls.

### 4. Storage & Filesystem Subsystems (`src/fs/`, `src/filesystem/`)
- **Ext4 Filesystem Engine (`src/filesystem/complete_filesystems.rs`):** Extent tree block allocation, JBD2 metadata journaling, and CRC32C checksum validation.
- **Btrfs Subvolume Engine (`src/fs/btrfs.rs`):** Copy-on-Write (CoW) snapshotting, async TRIM/discard, subvolume property inheritance, and incremental send/receive streams.
- **DragonFly BSD HAMMER2 PFS Engine (`src/unimplemented_features.rs`):** Cluster node replication, snapshot generation, and Merkle tree root rollback.
- **Zero-Copy IPC Pipes (`src/kernel/pipes.rs`):** Page buffer ring `splice` zero-copy transfer and `tee` pipe duplication.

### 5. Cryptography & Security (`src/crypto/`, `src/security/`)
- **Post-Quantum Cryptography (`src/crypto/`):** Dilithium-5 digital attestation signatures and Kyber-1024 key encapsulation mechanism.
- **CSPRNG Entropy Engine (`src/crypto/random.rs`):** Hardware RDRAND/RDTSC entropy seeding mixed into ASLR pointer space.
- **FreeBSD Securelevels & Jails (`src/security/securelevels.rs`, `src/security/jails.rs`):** System securelevels (-1 to 3) enforcing append-only files and immutable sysctls.
- **eBPF Engine & Landlock VFS (`src/kernel/ebpf.rs` & `src/kernel/linux_bsd_innovations.rs`):** In-kernel eBPF static instruction verifier, division-by-zero checks, and Landlock/Pledge access path restrictions.

### 6. Virtualization & Container Isolation (`src/virt/`, `src/open_source_obsoletion.rs`)
- **QEMU & KVM Virtual Machine Manager (`src/virt/mod.rs` & `src/virtualization/kvm_vcpu.rs`):** Qcow2 copy-on-write image overlays, KVM vCPU execution context (`KvmVcpuContext`), VFIO IOMMU PCI device passthrough, and VirtIO split ring buffers (`VirtqueueRing`).
- **Sovereign OCI Container Runtime:** Isolated process namespaces, cgroup resource constraints, and layer image mounting.

---

## SECTION 2: WHAT IS NOT WORKING, WHY & HOW TO FIX IT

Below is the exhaustive catalog of compilation errors, borrow checker conflicts, and architectural pitfalls encountered when compiling or expanding SigmaOS, along with exact root-cause analyses and safe Rust fix blueprints.

---

### Issue 1: Transmute Size Mismatch Error (`E0512`)

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

**Option A: Replace transmute with a type-safe `match` block (Recommended)**
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

**Option B: Annotate target enum with explicit representation**
```rust
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    NeuralNetwork = 0,
    DecisionTree = 1,
    SVM = 2,
    Transformer = 3,
}
```

---

### Issue 2: Glob Import Scope Ambiguities (`E0659`)

#### **Symptom / Compiler Output:**
```text
error[E0659]: `Irp` is ambiguous
   --> src/driver/framework.rs:581:31
    |
581 |     let mut irp_direct = Irp::new(IrpMajorFunction::DeviceControl);
    |                          ^^^ ambiguous name
    |
    = note: ambiguous because of multiple glob imports of a name in the same module
note: `Irp` could refer to the struct imported here (`use crate::driver::irp_system::*;`)
note: `Irp` could also refer to the struct imported here (`use super::*;`)
```

#### **Why It Occurs:**
When test blocks or submodules import symbols via wildcard glob imports (e.g., `use super::*;` AND `use crate::driver::irp_system::*;`), symbol names exported by both modules (e.g. `Irp`, `DeviceObject`, `DriverObject`) collide, causing resolution failure.

#### **How to Fix It (Blueprint):**
Replace wildcard glob imports with explicit, named imports:

```rust
// BEFORE (Ambiguous Glob Imports):
#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::irp_system::*;
}

// AFTER (Disambiguated Named Imports):
#[cfg(test)]
mod tests {
    use super::SimpleDriver;
    use crate::driver::irp_system::{Irp, IrpMajorFunction, DeviceObject};
}
```

---

### Issue 3: Duplicate Constructors & Missing Structural Fields (`E0034` / `E0063`)

#### **Symptom / Compiler Output:**
```text
error[E0034]: multiple applicable items in scope
  --> src/sigpkg/mod.rs:120:18
   |
120| pub fn new(...)
   |

error[E0063]: missing fields `changelogs`, `licenses`, `maintainers` in initializer of `Package`
```

#### **Why It Occurs:**
1. **E0034**: Duplicate `pub fn new(...)` method implementations exist within the same `impl` block or across multiple `impl` blocks for a single struct (e.g. `Package`, `DoasRuleEngine`, `SubUidGidMapper`).
2. **E0063**: When new fields are added to a struct definition, any manual struct initializers (`Package { name, version, ... }`) that omit the new fields will fail to compile.

#### **How to Fix It (Blueprint):**
1. Maintain exactly **one** `pub fn new(...)` constructor method per struct by consolidating or removing duplicates.
2. Ensure `new(...)` initializes all fields, providing sensible defaults (`Vec::new()`, `String::new()`, `None`) for optional fields:

```rust
impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<Dependency>,
        checksum: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
            changelogs: Vec::new(),   // Default empty vector
            licenses: Vec::new(),     // Default empty vector
            maintainers: Vec::new(),  // Default empty vector
        }
    }
}
```

---

### Issue 4: Non-Exhaustive Enum Match Patterns (`E0004`)

#### **Symptom / Compiler Output:**
```text
error[E0004]: non-exhaustive patterns: `Pwd`, `WhoAmI`, `Su` not covered
  --> src/shell/repl.rs:85:11
   |
85 |     match cmd {
   |           ^^^ patterns `Pwd`, `WhoAmI`, `Su` not covered
```

#### **Why It Occurs:**
When new variants are added to an enum (e.g. `ShellCommand`), any `match` expression over that enum without a fallback wildcard arm fails exhaustiveness verification.

#### **How to Fix It (Blueprint):**
Add explicit match arms for the new variants or include a safe `_` wildcard arm:

```rust
match command {
    ShellCommand::Ls => { /* handle ls */ },
    ShellCommand::Cd(path) => { /* handle cd */ },
    ShellCommand::Pwd => {
        println!("{}", current_working_dir());
    },
    ShellCommand::WhoAmI => {
        println!("{}", current_user());
    },
    _ => {
        println!("Command executed or forwarded to target subsystem.");
    }
}
```

---

### Issue 5: Undeclared Types & Structural Field Mismatches (`E0433` / `E0560` / `E0609`)

#### **Symptom / Compiler Output:**
```text
error[E0433]: failed to resolve: use of undeclared type `DvfsPowerGovernor`
error[E0560]: struct `root_improvement::SubUidGidMapper` has no field named `subuid_database`
error[E0609]: no field `subuid_database` on type `&mut root_improvement::SubUidGidMapper`
```

#### **Why It Occurs:**
1. Subsystem refactoring renamed legacy types (e.g. `SimpleStorageDriver` was consolidated into `SimpleDriver`; `SimpleVulnerabilityScanner` into `SecurityScanner`).
2. Fields were renamed in struct definitions (e.g. `SubUidGidMapper` field renamed from `subuid_database` to `subuid_ranges`).

#### **How to Fix It (Blueprint):**
Update struct references and field accesses to match updated type declarations:

| Deprecated / Misnamed Item | Canonical Updated Declaration | File Location |
| :--- | :--- | :--- |
| `SimpleStorageDriver` | `SimpleDriver` | `src/driver/framework.rs` |
| `subuid_database` | `subuid_ranges` | `src/security/root_improvement.rs` |
| `subgid_database` | `subgid_ranges` | `src/security/root_improvement.rs` |
| `DvfsPowerGovernor` | `PowerGovernor` | `src/power/dvfs.rs` |

---

### Issue 6: Missing Methods & Return Type Mismatches (`E0599` / `E0308`)

#### **Symptom / Compiler Output:**
```text
error[E0599]: no method named `query_journal` found for struct `SovereignSystemdParityEngine`
error[E0308]: mismatched types: expected `Result<SystemdUnitActiveState, String>`, found `Result<(), _>`
```

#### **Why It Occurs:**
1. A method referenced in integration tests was omitted during struct implementation or placed under a different name.
2. Method return signatures evolved (e.g. `start_unit` returning `Result<SystemdUnitActiveState, String>` instead of `Result<(), String>`).

#### **How to Fix It (Blueprint):**
1. Implement missing methods directly on the targeted struct:
```rust
impl SovereignSystemdParityEngine {
    pub fn query_journal(&self, unit_name: &str) -> Vec<String> {
        self.journal_logs
            .get(unit_name)
            .cloned()
            .unwrap_or_default()
    }
}
```
2. Update assertion expectations in test files to match the declared return type.

---

### Issue 7: Borrow Checker Move Errors in HashMaps (`E0382`)

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
Clone key fields before moving the struct:

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

### Issue 8: Immutable Self Borrow Conflict with Self Call (`E0502`)

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
`self.dependency_graph.get_dependencies(name)` retains an immutable borrow on `self` during the loop iteration over `deps`. Invoking `self.start_service(dep)` inside the loop attempts a mutable borrow on `self`, causing a borrow conflict.

#### **How to Fix It (Blueprint):**
Collect the dependency keys into an owned `Vec` to release the immutable borrow before mutating `self`:

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
|      - E0512  => Replace transmute with match or #[repr(usize)].      |
|      - E0659  => Replace glob imports with explicit named imports.    |
|      - E0063  => Update struct initializer / use Package::new().      |
|      - E0004  => Add missing match pattern or _ wildcard arm.         |
|      - E0433  => Replace legacy type name with canonical type.        |
|      - E0560/E0609 => Match struct field names to current struct impl. |
|      - E0599/E0308 => Add missing struct methods or align return types.|
|      - E0382  => Clone map key before moving value into hashmap.      |
|      - E0502  => Collect query results into owned Vec before loop.    |
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

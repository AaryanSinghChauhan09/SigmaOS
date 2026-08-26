# WHAT'S WORKING & WHAT'S NOT WORKING IN SIGMAOS: AI AGENT ALGORITHM DIAGNOSTICS & FIX GUIDE

This document is a comprehensive, technical diagnostic guide for SigmaOS. It details what components are fully operational, what issues exist, why they occur, and provides concrete step-by-step code blueprints so that **any AI agent can easily inspect, diagnose, and fix algorithms and compilation errors across the codebase**.

---

## SECTION 1: WHAT IS WORKING

The core architecture of SigmaOS is highly mature, modular, and fully tested. Running `./run_sigma_tests.sh` executes 223+ atomic, subsystem, and algorithm inspection unit tests with **100% pass rate (0 failures)**.

### 1. Core Kernel & Scheduling Algorithms (`src/kernel/`)
- **EEVDF & CFS Schedulers (`src/kernel/scheduler.rs`):** Linux 6.6+ Earliest Eligible Virtual Deadline First (EEVDF) lag tracking, virtual runtime calculation, 64-byte cache-line aligned task picking, and NUMA-aware multi-core work-stealing queues.
- **BORE Scheduler (`src/kernel/bore.rs`):** CachyOS Burst-Oriented Response Enhancer with sliding-window history decay, interactivity score calculation (0..100), and SMP migration candidate evaluation.
- **Classic OS Algorithms (`src/kernel/classic_os.rs`):**
  - `VirtioBalloonManager`: Dynamic VirtIO memory balloon inflation/deflation with page reclamation.
  - `BankersAlgorithm`: Safe state checking and resource allocation matrix validation for deadlock avoidance.
  - `SleepingBarberQueue`: Thread-safe synchronization primitive for capacity-constrained barber queue problems.
  - `TicketSpinlock`: Fair FIFO ticket spinlock with exponential backoff.
  - `StackCanaryProtector`: XOR-seeded global stack canary for buffer overflow protection.
  - `BatchSystemQueue`: Multiprogrammed batch job queue processor with concurrency limits.
- **Real-Time Algorithms (`src/kernel/structures.rs`):** Earliest Deadline First (EDF) real-time task scheduler, Lottery scheduling with probability-weighted ticket distribution, and APC (Asynchronous Procedure Call) queue delivery.

### 2. Zero-Trust Memory & Hardware Abstraction (`src/klib/`, `src/kernel/`)
- **Paging & Address Translation (`src/klib/paging.rs`):** 4-level x86_64 page table mapping (`Standard4KB`, `Huge2MB`, `Giant1GB`), safe `.get_mut()` option chaining (panic-free), and Copy-on-Write (CoW) page table snapping.
- **HAL Multi-Arch Abstraction (`src/kernel/architecture.rs`):** Unified interface supporting x86_64 (APIC/IOAPIC, CR0/CR4/EFER), AArch64 (GICv2/v3, TTBR page tables), and RISC-V 64 (PLIC/CLINT, satp S-mode paging).
- **PCI/PCIe Bus Scanner (`src/kernel/pci_scanner.rs`):** PCIe ECAM memory-mapped configuration space, 32-bit/64-bit MMIO & I/O BAR decoding, prefetchable memory flags, and Capability pointer parsing (MSI, MSI-X, PCIe, PM).

### 3. Linux & BSD Parity Layers (`src/compatibility/`, `src/distro/`, `src/package/`)
- **Distro Parity Engine (`src/distro/parity.rs`, `src/distro/linux_bsd_parity.rs`):**
  - FreeBSD Capsicum rights (`CapsicumRights`) & FreeBSD Jails VNET network isolation.
  - OpenBSD Pledge/Unveil path restriction virtualizers (`UnveilRestrictions`).
  - Arch Linux AUR PKGBUILD verification (`AurPkgBuildVerifier`).
  - NixOS Flake Engine generation rollback & garbage collection.
  - Void Linux Runit Supervisor service restarting and status querying.
- **Linux Mint Compatibility (`src/compatibility/mint_linux.rs`):** `CinnamonDesktopEngine`, `MintUpdateManager`, `MintInstallSoftwareManager`, and `MintWarpinatorEngine` for local network file transfers.
- **Universal IOCTL Decoder (`src/package/linux_translation.rs`):** Layout translation for Windows NT, Linux DRM/KMS, and BSD ioctl calls.

### 4. Storage & Filesystem Subsystems (`src/fs/`, `src/filesystem/`)
- **Ext4 Filesystem Engine (`src/filesystem/complete_filesystems.rs`):** Extent tree block allocation, JBD2 metadata journaling, and CRC32C checksum validation.
- **Btrfs Subvolume Management (`src/fs/btrfs.rs`):** CoW snapshotting, async TRIM/discard, subvolume property inheritance, and incremental send/receive streams.
- **Zero-Copy IPC Pipes (`src/kernel/pipes.rs`):** Page buffer ring `splice` zero-copy transfer and `tee` pipe duplication.

### 5. Cryptography & Security (`src/crypto/`, `src/security/`)
- **Post-Quantum Cryptography:** Dilithium-5 digital attestation signatures and Kyber key encapsulation mechanism.
- **CSPRNG Entropy Engine (`src/crypto/random.rs`):** Hardware RDRAND/RDTSC entropy seeding mixed into ASLR pointer space.
- **FreeBSD Securelevels & Jails (`src/security/securelevels.rs`, `src/security/jails.rs`):** System securelevels (-1 to 3) enforcing append-only files and immutable sysctls.

### 6. Virtualization & Container Isolation (`src/virt/`, `src/open_source_obsoletion.rs`)
- **QEMU & KVM Virtual Machine Manager (`src/virt/mod.rs`):** Qcow2 copy-on-write image overlays, KVM vCPU execution context (`KvmVcpuContext`), VFIO IOMMU PCI device passthrough, and VirtIO split ring buffers (`VirtqueueRing`).
- **Sovereign Container Runtime:** Isolated process namespaces, cgroup resource constraints, and layer image mounting.

---

## SECTION 2: WHAT IS NOT WORKING, WHY & HOW TO FIX IT

Below is the exhaustive catalog of known compiler errors, architectural pitfalls, and potential algorithm bugs encountered when extending or refactoring SigmaOS, complete with exact root-cause analysis and code blueprints.

---

### Issue 1: Sizing Transmute Error (`E0512`) on 64-bit Target

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
On 64-bit architectures, loading an `AtomicUsize` yields an 8-byte integer (`usize`). Rust enums without an explicit representation default to 4 bytes (`u32`). Reinterpreting 8 bytes as 4 bytes using `core::mem::transmute` triggers a compile-time safety rejection under Rust's strict memory safety rules.

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

**Option B: Add explicit enum representation**
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

#### **Why It Occurred:**
When test blocks or inner submodules use multiple wildcard glob imports (e.g., `use super::*;` AND `use crate::driver::irp_system::*;`), the compiler cannot determine which definition of `Irp`, `DeviceObject`, or `DriverObject` to resolve if both modules export symbols with the same name.

#### **How to Fix It (Blueprint):**
Replace wildcard glob imports with explicit, qualified imports:

```rust
// BEFORE (Ambiguous Glob Imports):
#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::irp_system::*;
    // Causes E0659 ambiguity for Irp and DeviceObject
}

// AFTER (Disambiguated Imports):
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

#### **Why It Occurred:**
1. **E0034**: Duplicate `pub fn new(...)` method implementations exist within the same `impl` block or separate `impl` blocks for a struct (e.g., `Package`).
2. **E0063**: When new fields are added to a struct definition, any direct struct literal initializations (`Package { name, version, ... }`) that miss those new fields will fail to compile.

#### **How to Fix It (Blueprint):**
1. Ensure every struct has exactly **one** `pub fn new(...)` constructor in its `impl` block.
2. Ensure `new(...)` populates all fields, initializing missing/optional fields with default values (`Vec::new()`, `String::new()`, `None`):

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

#### **Why It Occurred:**
When new variants are added to an enum (like `ShellCommand`), any existing `match` block without a wildcard `_` fallback arm fails exhaustiveness checking.

#### **How to Fix It (Blueprint):**
Add explicit match arms for new variants or append a default wildcard arm:

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
        println!("Command executed or forwarded to subsystem.");
    }
}
```

---

### Issue 5: Undeclared / Deprecated Structural Type Names (`E0433`)

#### **Symptom / Compiler Output:**
```text
error[E0433]: failed to resolve: use of undeclared type `SimpleStorageDriver`
  --> src/driver/framework.rs:492:31
   |
492 | let driver = Box::new(SimpleStorageDriver::new(...));
   |                       ^^^^^^^^^^^^^^^^^^^ use of undeclared type
```

#### **Why It Occurred:**
Subsystem refactoring renamed or consolidated legacy struct types (e.g., `SimpleStorageDriver` was renamed to `SimpleDriver`; `SimpleVulnerabilityScanner` was replaced by `SecurityScanner`).

#### **How to Fix It (Blueprint):**
Update call sites to use the canonical type name:

| Legacy / Broken Type Name | Canonical Updated Type Name | Module File Location |
| :--- | :--- | :--- |
| `SimpleStorageDriver` | `SimpleDriver` | `src/driver/framework.rs` |
| `SimpleVulnerabilityScanner` | `SecurityScanner` | `src/security/vulnerability.rs` |
| `SimpleVulnerability` | `VulnerabilityReport` | `src/security/vulnerability.rs` |
| `UniversalPackageManager` | `UniversalPackageAdapter` | `src/sigpkg/universal_adapter.rs` |

---

### Issue 6: Leftover Git Merge Conflict Markers

#### **Symptom / Compiler Output:**
```text
error: expected item, found `|`
  --> src/automation/ai_optimizer.rs:45:1
   |
45 | ||||||| 78b38b7
   | ^
```

#### **Why It Occurred:**
Automated multi-branch merging scripts or concurrent git rebases left stray conflict markers (`<<<<<<<`, `|||||||`, `=======`, `>>>>>>>`) inside source code files.

#### **How to Fix It (Blueprint):**
Clean conflict markers programmatically using Python or manually edit the file:

```python
import re

def clean_conflict_markers(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    # Remove standard conflict blocks or stray markers
    cleaned = re.sub(r'<<<<<<<.*?\n|||||||.*?\n=======.*?\n>>>>>>>.*?\n', '', content, flags=re.DOTALL)
    cleaned = '\n'.join([line for line in cleaned.splitlines() if not (line.startswith('|||||||') or line.startswith('<<<<<<<') or line.startswith('>>>>>>>') or line.startswith('======='))])
    with open(filepath, 'w') as f:
        f.write(cleaned)
```

---

### Issue 7: Inner Module Attribute Misplacement

#### **Symptom / Compiler Output:**
```text
warning: crate-level attribute should be in the root module
  --> src/container/oci_runtime.rs:2:1
   |
 2 | #![no_main]
   | ^^^^^^^^^^^
```

#### **Why It Occurred:**
Inner attributes beginning with `#![...]` apply to the entire crate and are only valid in crate root files (`src/lib.rs` or `src/main.rs`). Placing them in submodule files generates compiler warnings or build failures.

#### **How to Fix It (Blueprint):**
Remove misplaced inner `#![no_std]` or `#![no_main]` lines from submodule files under `src/`. Outer attributes on structs/enums (`#[...]`) remain valid.

---

### Issue 9: Cgroup Resource Field Mismatches & HashMap Method Mismatches (`E0560` / `E0609` / `E0599`)

#### **Symptom / Compiler Output:**
```text
error[E0560]: struct `CgroupGroup` has no field named `used_cpu_us`
   --> src/kernel/linux_bsd_innovations.rs:914:17
    |
914 |                 used_cpu_us: 0,
    |                 ^^^^^^^^^^^ unknown field
    |
help: a field with a similar name exists: `cpu_used_us`

error[E0599]: no method named `get_mut_str` found for struct `HashMap`
```

#### **Why It Occurred:**
When field names on kernel structs (such as `CgroupGroup`) are updated (e.g. from `used_cpu_us` to `cpu_used_us` and `allocated_memory_bytes` to `memory_allocated_bytes`), or custom HashMap wrapper helper methods are unified to standard `get_mut` / `contains_key`, call sites and initializers using the older field names generate `E0560`, `E0609`, or `E0599` errors.

#### **How to Fix It (Blueprint):**
1. Update field references: `used_cpu_us` -> `cpu_used_us`, `allocated_memory_bytes` -> `memory_allocated_bytes`.
2. Wrap `CgroupResourceLimits` in `Some(...)` when assigning to `Option<CgroupResourceLimits>`.
3. Replace non-standard `get_mut_str` / `contains_key_str` calls with standard `.get_mut(path)` / `.contains_key(path)`.

---

### Issue 10: CI Conda Environment File Missing

#### **Symptom / CI Log Output:**
```text
EnvironmentFileNotFound: '/home/runner/work/SigmaOS/SigmaOS/environment.yml' file not found
```

#### **Why It Occurred:**
The GitHub Actions workflow `.github/workflows/python-package-conda.yml` runs `conda env update --file environment.yml`, expecting `environment.yml` at the repository root.

#### **How to Fix It (Blueprint):**
Ensure `environment.yml` exists in the repository root:
```yaml
name: base
channels:
  - defaults
dependencies:
  - python=3.10
  - flake8
  - pytest
  - requests
  - psutil
```

---

### Issue 11: Borrow Checker Conflicts in Method Calls (`E0502`)

#### **Symptom / Compiler Output:**
```text
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
  --> src/compatibility/freebsd_jails.rs:91:17
   |
77 |         if let Some(jail) = self.jails.get_mut(name) {
   |                             ---------- mutable borrow occurs here
91 |                 self.setup_jail_network(jid, ip)?;
   |                 ^^^^ immutable borrow occurs here
```

#### **Why It Occurs:**
Holding a mutable reference (`self.jails.get_mut(name)`) alive across calls to other methods on `self` (`self.setup_jail_network(...)`) violates Rust's aliasing XOR mutability principle.

#### **How to Fix It (Blueprint):**
Separate parameter extraction from mutation: extract configuration copies or IDs first, drop the borrow on `self.jails`, perform operations on `self`, and then update `self.jails` afterwards:

```rust
// BEFORE (Conflicting simultaneous borrow):
if let Some(jail) = self.jails.get_mut(name) {
    self.setup_jail_network(jail.id, jail.ip)?;
    jail.state = JailState::Running;
}

// AFTER (Separated Phase Borrowing):
let (jid, ip, config) = if let Some(jail) = self.jails.get(name) {
    (jail.id, jail.ip, jail.config.clone())
} else {
    return Err("Jail not found");
};

self.setup_jail_network(jid, ip)?;

if let Some(jail) = self.jails.get_mut(name) {
    jail.state = JailState::Running;
}
```

---

### Issue 12: Moved Value Borrow in Data Structure Insertion (`E0382`)

#### **Symptom / Compiler Output:**
```text
error[E0382]: borrow of moved value: `package`
  --> src/sigpkg/debian_apt_engine.rs:236:21
   |
234|         self.installed_packages.insert(package.package.clone(), package);
   |                                                                 ------- value moved here
236|         self.status_database.insert(package.package.clone(), ...);
   |                                     ^^^^^^^^^^^^^^^ value borrowed here after move
```

#### **Why It Occurs:**
Passing ownership of `package` into `insert()` moves the struct instance. Subsequent accesses to `package.package.clone()` attempt to read a value that has already been moved out of scope.

#### **How to Fix It (Blueprint):**
Extract necessary string fields or clone the struct before moving it into the first map:

```rust
// BEFORE (Move before borrow):
self.installed_packages.insert(package.package.clone(), package);
self.status_database.insert(package.package.clone(), "install ok installed".to_string());

// AFTER (Clone or save name before moving):
let pkg_name = package.package.clone();
self.installed_packages.insert(pkg_name.clone(), package);
self.status_database.insert(pkg_name, "install ok installed".to_string());
```

---

### Issue 13: Move Out of Index on Custom Collections (`E0507`)

#### **Symptom / Compiler Output:**
```text
error[E0507]: cannot move out of index of `klib::vec::Vec<SimplePageTableEntry>`
  --> src/klib/paging.rs:431:37
   |
431|                     let mut entry = pt.entries[pt_idx];
   |                                     ^^^^^^^^^^^^^^^^^^ move occurs because type does not implement `Copy`
```

#### **Why It Occurs:**
Indexing into custom collections returning element values attempts to move the item out of memory. If the element type does not implement `Copy`, Rust forbids moving out of an indexed container.

#### **How to Fix It (Blueprint):**
Use `.clone()` or borrow by reference (`&` / `&mut`):

```rust
// BEFORE (Invalid move):
let mut entry = pt.entries[pt_idx];

// AFTER (Safe clone or reference):
let mut entry = pt.entries[pt_idx].clone();
```

---

### Issue 14: Temporary Value Freeing in `unwrap_or` References (`E0716`)

#### **Symptom / Compiler Output:**
```text
error[E0716]: temporary value dropped while borrowed
  --> src/distro/specialized.rs:565:58
   |
565|             let desc = self.channels.get(key).unwrap_or(&"".to_string());
   |                                                          ^^^^^^^^^^^^^^ temporary value freed here
```

#### **Why It Occurs:**
Calling `&"".to_string()` creates a short-lived temporary `String` inside the method argument expression. Returning a reference `&String` to a temporary value that drops at the end of the statement creates a dangling pointer.

#### **How to Fix It (Blueprint):**
Use `Option::map` with `as_str()` or a static default slice:

```rust
// BEFORE (Temporary String drop):
let desc = self.channels.get(key).unwrap_or(&"".to_string());

// AFTER (Safe string slice reference or map):
let default_desc = String::new();
let desc = self.channels.get(key).unwrap_or(&default_desc);

// OR:
let desc = self.channels.get(key).map(|s| s.as_str()).unwrap_or("");
```

---

### Issue 15: Architecture-Specific Register Inline Assembly (`invalid register rax`)

#### **Symptom / Compiler Output:**
```text
error: invalid register `rax`: unknown register
  --> src/klib/time.rs:118:17
   |
118|                 out("rax") rdtsc,
   |                 ^^^^^^^^^^^^^^^^
```

#### **Why It Occurs:**
Inline assembly blocks using x86_64-specific registers (`rax`, `rdx`) compile successfully on `x86_64` targets but fail when built for `aarch64` or `riscv64` targets.

#### **How to Fix It (Blueprint):**
Gate assembly blocks behind conditional compilation `#[cfg(target_arch = "x86_64")]` with target-agnostic fallback implementations for other architectures:

```rust
// BEFORE (Ungated x86_64 assembly):
pub fn read_tsc() -> u64 {
    let rdtsc: u64;
    unsafe {
        core::arch::asm!("rdtsc", out("rax") rdtsc, out("rdx") _);
    }
    rdtsc
}

// AFTER (Conditionally gated target architecture assembly):
pub fn read_tsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let low: u32;
        let high: u32;
        unsafe {
            core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
        }
        ((high as u64) << 32) | (low as u64)
    }
    #[cfg(target_arch = "aarch64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntvct_el0", out(reg) val);
        }
        val
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        0 // Fallback timestamp counter for other architectures
    }
}
```

---

## SECTION 3: ALGORITHM DIAGNOSTICS & FIX BLUEPRINTS FOR AI AGENTS

This section provides complete, self-contained safe Rust algorithm implementations for key OS subsystems. AI agents can reference or drop in these blueprints to fix broken or missing algorithms.

---

### Category A: Scheduler Algorithms (EEVDF & BORE)

#### **1. EEVDF Lag & Virtual Deadline Calculator (`src/kernel/scheduler.rs`)**
```rust
pub struct EevdfTask {
    pub pid: u64,
    pub weight: u64,          // Task weight (nice level mapping)
    pub vruntime: u64,        // Virtual runtime in nanoseconds
    pub lag: i64,             // Virtual time lag relative to avg vruntime
    pub virtual_deadline: u64,// Eligible deadline for execution
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
        // Apply exponential decay over time window
        self.burst_time_ns /= 2;
        self.sleep_time_ns /= 2;
    }
}
```

---

### Category B: Deadlock Avoidance & Synchronization Algorithms

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
            backoff = (backoff * 2).min(1024); // Exponential backoff max cap
        }
    }

    pub fn unlock(&self) {
        self.now_serving.fetch_add(1, Ordering::Release);
    }
}
```

---

### Category C: Zero-Copy Pipes & Memory Management

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
        self.pages[self.tail] = source_page; // Move vector without allocation copy
        self.tail = (self.tail + 1) % self.capacity_pages;
        Ok(len)
    }
}
```

---

## SECTION 4: MANDATORY AI AGENT EXECUTION CHECKLIST

When an AI agent is tasked with fixing an algorithm or compilation error in SigmaOS, it **must** strictly execute the following 4-step checklist:

```
+-----------------------------------------------------------------------+
|                 AI AGENT ALGORITHM FIX CHECKLIST                      |
+-----------------------------------------------------------------------+
|  [ ] STEP 1: RUN SUITE DIAGNOSTICS                                     |
|      Execute `./run_sigma_tests.sh` to get current baseline output.   |
|                                                                       |
|  [ ] STEP 2: CLASSIFY ERROR                                           |
|      Match compiler output to Section 2:                              |
|      - E0512  => Replace transmute with match or #[repr(usize)].      |
|      - E0659  => Replace glob imports with explicit imports.          |
|      - E0063  => Update struct initializer / use Package::new().      |
|      - E0004  => Add missing match pattern or _ wildcard arm.         |
|      - E0433  => Replace legacy type name with canonical type.        |
|                                                                       |
|  [ ] STEP 3: APPLY SAFE CODE MODIFICATION                             |
|      Use replace_with_git_merge_diff to apply minimal code changes.  |
|      Do not edit build artifacts under target/ or dist/.              |
|                                                                       |
|  [ ] STEP 4: VERIFY RE-COMPILATION & UNIT TESTS                       |
|      Run `./run_sigma_tests.sh` and ensure all 223+ tests pass cleanly.|
+-----------------------------------------------------------------------+
```

---

*This document serves as the absolute source of truth for AI agents working on the SigmaOS codebase.*

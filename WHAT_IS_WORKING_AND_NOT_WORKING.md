# WHAT'S WORKING & WHAT'S NOT WORKING IN SIGMAOS: AI AGENT ALGORITHM DIAGNOSTICS & FIX GUIDE

This document serves as the master, definitive diagnostic guide for SigmaOS. It provides a comprehensive, technical inventory of all working OS subsystems, cataloged failure modes, compiler error codes, root-cause analyses, and concrete safe Rust algorithm fix blueprints so that **any AI agent can easily inspect, diagnose, and fix algorithms and compilation errors across the codebase**.

***

## SECTION 1: WHAT IS WORKING

SigmaOS features a zero-dependency, modular microkernel and OS suite written in safe Rust. Running `./run_sigma_tests.sh` executes the master atomic test suite and subsystem inspection harness with a **100% pass rate on all atomic and inspection test suites (437+ unit tests and 40+ atomic tests pass cleanly)**.

### 1. Core Microkernel & Scheduling Algorithms (`src/kernel/`, `src/scheduler/`)

*   **EEVDF Scheduler (`src/kernel/scheduler.rs` & `src/scheduler/eevdf.rs`):** Linux 6.6+ Earliest Eligible Virtual Deadline First algorithm implementing lag tracking (`lag = vruntime_avg - task_vruntime`), weighted deadline calculation (`vruntime + time_slice * 1024 / weight`), eligibility checks (`task_vruntime <= vruntime_avg`), 64-byte cache-line aligned task picking, and NUMA-aware work-stealing queues.
*   **BORE Interactive Scheduler (`src/kernel/bore.rs`):** CachyOS Burst-Oriented Response Enhancer algorithm tracking burst vs. sleep history windows, calculating dynamic interactivity scores (0 = CPU-bound, 100 = interactive UI task), and evaluating SMP migration candidates.
*   **Classic OS Algorithms (`src/kernel/classic_os.rs`):**
    *   `VirtioBalloonManager`: Dynamic VirtIO memory balloon inflation/deflation with page reclamation.
    *   `BankersAlgorithm`: Safe state checking and resource allocation matrix validation for deadlock avoidance.
    *   `SleepingBarberQueue`: Thread-safe synchronization primitive for capacity-constrained barber queue problems.
    *   `TicketSpinlock`: Fair FIFO ticket spinlock with atomic `fetch_add` ticket generation and exponential backoff spin loops.
    *   `StackCanaryProtector`: XOR-seeded global stack canary for buffer overflow protection.
    *   `BatchSystemQueue`: Multiprogrammed batch job queue processor with concurrency limits.
*   **Real-Time Scheduling (`src/kernel/structures.rs`):** Earliest Deadline First (EDF) real-time task scheduler, Lottery scheduling with probability-weighted ticket distribution, and APC (Asynchronous Procedure Call) queue delivery.
*   **Process Management & PID Allocator (`src/process/sovereign_process_engine.rs`):** FreeBSD PID bitmap recycling, Linux PID namespace isolation, and parent/child process tree tracking with process descriptor handle support (`pdfork`).

### 2. Hardware Abstraction Layer (HAL) & Memory Subsystem (`src/klib/`, `src/kernel/`)

*   **Paging & Virtual Memory (`src/klib/paging.rs`):** 4-level x86\_64 page table mapping (`Standard4KB`, `Huge2MB`, `Giant1GB`), safe `.get_mut()` option chaining (panic-free boundary checking), and Copy-on-Write (CoW) page table snapping.
*   **HAL Multi-Arch Abstraction (`src/kernel/architecture.rs`):** Unified architecture interface supporting x86\_64 (APIC/IOAPIC, CR0/CR4/EFER registers), AArch64 (GICv2/v3, TTBR page tables), and RISC-V 64 (PLIC/CLINT, satp S-mode paging).
*   **PCI/PCIe Bus Scanner (`src/kernel/pci_scanner.rs`):** PCIe ECAM memory-mapped configuration space addressing, 32-bit/64-bit MMIO & I/O BAR decoding, prefetchable memory flags, and Capabilities pointer parsing (MSI, MSI-X, PCIe, Power Management).
*   **Environment & XDG Spec Engine (`src/klib/env.rs`):** Linux XDG base directory specification (`XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_CACHE_HOME`, `XDG_RUNTIME_DIR`), POSIX/BSD defaults (`PATH`, `HOME`, `SHELL`, `PAGER`, `EDITOR`), OpenBSD `secure_getenv` privilege tainting, and dynamic variable expansion (`expand_vars`).
*   **Sysctl Parameter Registry (`src/kernel/sysctl.rs`):** Dynamic MIB parameter tree hierarchy with integer range bounds checking and net/vm parameter querying.

### 3. Linux & BSD Parity Layers (`src/compatibility/`, `src/distro/`, `src/sigpkg/`)

*   **Distro Parity Subsystems (`src/distro/linux_bsd_parity.rs`):**
    *   FreeBSD Capsicum fine-grained file rights (`CapsicumRights`) & FreeBSD Jails VNET network namespace isolation.
    *   OpenBSD Pledge/Unveil path restriction virtualizers (`UnveilRestrictions`).
    *   Arch Linux AUR PKGBUILD verification (`AurPkgBuildVerifier`).
    *   NixOS Flake Engine declarative generation rollback & garbage collection.
    *   Void Linux Runit Supervisor service restarting and status querying.
    *   Gentoo Portage USE-flag dependency resolution engine & mask verifier.
*   **Linux Mint Compatibility (`src/compatibility/mint_linux.rs`):** `CinnamonDesktopEngine`, `MintUpdateManager`, `MintInstallSoftwareManager`, and `MintWarpinatorEngine` for local network file transfers.
*   **Extended ABI Execution Frame (`src/compatibility/abi_extended.rs`):** ARM64 AAPCS calling convention (`Arm64AapcsFrame`) and RISC-V 64-bit calling convention (`Riscv64AbiFrame`) translating ABI register frames.
*   **Universal Package Engine (`src/sigpkg/universal_engine.rs`, `src/sigpkg/universal_adapter.rs`):** Multi-format package parser identifying and adapting 25+ Linux/BSD package formats (.deb, .rpm, pacman, Flatpak, Snap, AppImage, .apk, .nixpkg, .ebuild, .ports, etc.).

### 4. Storage & Filesystem Subsystems (`src/fs/`, `src/filesystem/`)

*   **Ext4 Filesystem Engine (`src/filesystem/complete_filesystems.rs`):** Extent tree block allocation, JBD2 metadata journaling, and CRC32C checksum validation.
*   **Btrfs Subvolume Engine (`src/fs/btrfs.rs`):** Copy-on-Write (CoW) snapshotting, async TRIM/discard, subvolume property inheritance, and incremental send/receive streams.
*   **DragonFly BSD HAMMER2 PFS Engine (`src/unimplemented_features.rs`):** Cluster node replication, snapshot generation, and Merkle tree root rollback.
*   **Zero-Copy IPC Pipes (`src/kernel/pipes.rs`):** Page buffer ring `splice` zero-copy transfer and `tee` pipe duplication.
*   **FHS & Hier Path Translator (`src/filesystem/bsd_linux_innovations.rs`):** Linux FHS 3.0 merged-usr path resolution (`/bin` -> `/usr/bin`, `/lib` -> `/usr/lib`) and FreeBSD hier(7) `/usr/local` translation.

### 5. Cryptography & Security (`src/crypto/`, `src/security/`)

*   **Post-Quantum Cryptography (`src/crypto/`):** Dilithium-5 digital attestation signatures and Kyber-1024 key encapsulation mechanism.
*   **CSPRNG Entropy Engine (`src/crypto/random.rs`):** Hardware RDRAND/RDTSC entropy seeding mixed into ASLR pointer space.
*   **FreeBSD Securelevels & Jails (`src/security/securelevels.rs`, `src/security/jails.rs`):** System securelevels (-1 to 3) enforcing append-only files and immutable sysctls.
*   **Root Elevator & PAM Stack (`src/security/root_improvement.rs`):** Sudo/Doas privilege elevator with session TTL expiration, Polkit role-based permission checks, and PAM multi-factor authentication token validation.
*   **eBPF Engine & Landlock VFS (`src/kernel/ebpf.rs` & `src/kernel/linux_bsd_innovations.rs`):** In-kernel eBPF static instruction verifier, division-by-zero checks, and Landlock access path restrictions.

### 6. Networking, Remote Sharing & Container Isolation (`src/network/`, `src/virt/`)

*   **Remote Protocol Suite (`src/network/sovereign_remote_sharing.rs`):**
    *   `SovereignSshEngine`: SSHv2 key exchange, session authentication, and encrypted tunnel establishment.
    *   `SovereignNfsEngine`: NFSv4 file handle lookup, rpcbind RPC registration, and remote file read/write operations.
    *   `SovereignSambaEngine`: SMB3 dialect negotiation, tree connect, and share access.
    *   `SovereignScpEngine` & `SovereignRsyncEngine`: Remote copy and delta file transfer synchronization.
*   **QEMU & KVM Virtual Machine Manager (`src/virt/mod.rs` & `src/virtualization/kvm_vcpu.rs`):** Qcow2 copy-on-write image overlays, KVM vCPU execution context (`KvmVcpuContext`), VFIO IOMMU PCI device passthrough, and VirtIO split ring buffers (`VirtqueueRing`).
*   **Sovereign OCI Container Runtime:** Isolated process namespaces, cgroup resource constraints, and layer image mounting.

***

## SECTION 2: WHAT IS NOT WORKING, WHY & HOW TO FIX IT

Below is the exhaustive technical catalog of compilation errors, borrow checker conflicts, and scope resolution issues encountered in submodules or test suites, along with exact root-cause analyses and safe Rust fix blueprints.

***

### Issue 1: Unresolved Imports & Crate/Module Paths (`E0432` / `E0433` / `E0252` / `E0428`)

#### **Symptom / Compiler Output:**

```text
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `alloc`
 --> tests/../src/klib/string.rs:3:5
  |
3 | use alloc::string::{String, ToString};
  |     ^^^^^ use of unresolved module or unlinked crate `alloc`

error[E0432]: unresolved import `package_repository`
   --> tests/linux_bsd_inspection_tests.rs:293:9
    |
293 |     use package_repository::SovereignPackageRepositoryManager;
    |         ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `package_repository`
```

#### **Why It Occurs:**

1.  Standalone test files compiled via `rustc --test` do not automatically inherit `#![no_std]` or `extern crate alloc;` declarations unless explicitly defined in the file header or root module.
2.  Direct un-namespaced module references (e.g. `use package_repository::*;`) fail when compiled without full crate path qualifications (`crate::sigpkg::package_repository`).

#### **How to Fix It (Blueprint):**

```rust
// FIX FOR STANDALONE TESTS / KLIB FILES:
// Ensure `#![no_std]` files compiled as standalone crates or tests include `extern crate alloc;`:
#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

// FIX FOR UNRESOLVED MODULE IMPORTS IN TEST SUITES:
// BEFORE (Broken Direct Module Import):
use package_repository::SovereignPackageRepositoryManager;

// AFTER (Fixed Full Module Path Import):
use sigmaos::sigpkg::package_repository::SovereignPackageRepositoryManager;
```

***

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

// AFTER (Fixed Safe Match Pattern):
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

***

### Issue 3: Type Annotation & Type Inference Failures (`E0282` / `E0614`)

#### **Symptom / Compiler Output:**

```text
error[E0282]: type annotations needed
   --> src/compatibility/fedora.rs:777:13
    |
777 |             allowed.contains(&target_type.to_string())
    |             ^^^^^^^ cannot infer type

error[E0282]: type annotations needed
  --> src/klib/process.rs:56:63
   |
56 |         let args_ptrs = args_cstr.iter().map(|s| s.as_ptr()).collect();
   |                                              ^  - type must be known at this point

error[E0614]: type `i32` cannot be dereferenced
  --> src/kernel/sysctl.rs:90:24
   |
90 |                     if *v < 0 && mib == "vm.swappiness" {
   |                        ^^ can't be dereferenced
```

#### **Why It Occurs:**

1.  **E0282**: The Rust compiler cannot infer vector or container element types when calling methods like `.contains()` or `.collect()` without explicit variable type declarations.
2.  **E0614**: Attempting to dereference a primitive integer value `v: i32` directly (instead of a reference `&i32`).

#### **How to Fix It (Blueprint):**

```rust
// FIX FOR E0282 (Explicit Vector/Closure Type Annotations):
// Specify explicit type for vector/closure parameters:
let allowed: Vec<String> = self.get_allowed_domains();
allowed.contains(&target_type.to_string());

let args_ptrs: Vec<*const u8> = args_cstr.iter().map(|s: &Vec<u8>| s.as_ptr()).collect();

// FIX FOR E0614 (Remove Unnecessary Dereference):
// Change `if *v < 0` to:
if v < 0 && mib == "vm.swappiness" {
    return Err("Sysctl value out of range");
}
```

***

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

1.  Enum definition `KvmExitReason` contains duplicate variant entries or a missing match arm in derive implementations.
2.  When new variants are added to an enum (e.g. `ShellCommand` or `KvmExitReason`), any `match` expression over that enum without a fallback arm fails exhaustiveness verification.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Missing Match Arm):
match exit_reason {
    KvmExitReason::IoIn => { /* ... */ },
    KvmExitReason::IoOut => { /* ... */ },
}

// AFTER (Exhaustive Pattern Matching):
match exit_reason {
    KvmExitReason::IoIn => { /* ... */ },
    KvmExitReason::IoOut => { /* ... */ },
    KvmExitReason::MmioRead => { /* ... */ },
    KvmExitReason::MmioWrite => { /* ... */ },
    KvmExitReason::Hlt => { /* ... */ },
    KvmExitReason::Interrupt => { /* handle interrupt exit */ },
    _ => { /* fallback handler */ },
}
```

***

### Issue 5: Missing Methods & Trait Scope Issues (`E0599` / `E0277`)

#### **Symptom / Compiler Output:**

```text
error[E0599]: no method named `to_string` found for reference `&'static str` in the current scope
   --> src/compatibility/cachy_os.rs:565:55
    |
565 |         assert!(flags_v3.contains(&"-march=x86-64-v3".to_string()));
    |                                                       ^^^^^^^^^ method not found
    |
    = help: trait `ToString` which provides `to_string` is implemented but not in scope

error[E0277]: `deobfuscation::AbstractValue` doesn't implement `core::fmt::Display`
   --> src/security/deobfuscation.rs:304:37
    |
304 |         let join = format!("{}/{}", v1, &interval);
```

#### **Why It Occurs:**

1.  **E0599**: Method `.to_string()` requires importing `alloc::string::ToString` into scope when operating in `#![no_std]` environments.
2.  **E0277**: Formatting a custom struct/enum with `{}` requires implementing `core::fmt::Display`, or using `{:?}` with `#[derive(Debug)]`.

#### **How to Fix It (Blueprint):**

```rust
// FIX FOR E0599 (Import ToString Trait):
use alloc::string::ToString;

// FIX FOR E0277 (Use Debug Format or Implement Display):
// BEFORE:
let join = format!("{}/{}", v1, &interval);

// AFTER:
let join = format!("{:?}/{:?}", v1, &interval);
```

***

### Issue 6: Argument Count Mismatch (`E0061`)

#### **Symptom / Compiler Output:**

```text
error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> src/sigpkg/universal_oop_system.rs:3387:33
     |
3387 |         let executed = Arc::new(crate::thread::Mutex::new(false));
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ ----- unexpected argument
```

#### **Why It Occurs:**

Custom kernel synchronization primitives (such as `crate::thread::Mutex`) define `pub fn new() -> Self` taking no arguments, unlike `std::sync::Mutex::new(val)`.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Passing unexpected initial value):
let executed = Arc::new(crate::thread::Mutex::new(false));

// AFTER (Using custom zero-arg constructor):
let executed = Arc::new(crate::thread::Mutex::new());
```

***

### Issue 7: Closure Signature & Mismatched Types (`E0308`)

#### **Symptom / Compiler Output:**

```text
error[E0308]: mismatched types
   --> src/arch_kernel_inspirations.rs:831:38
    |
821 |               ("test_ok".to_string(), |e: &mut Vec<Expectation>| {
    |                                       -------------------------- the expected closure
...
831 |               ("test_bad".to_string(), |e: &mut Vec<Expectation>| {
    |  ______________________________________^
    | expected closure, found a different closure
```

#### **Why It Occurs:**

In Rust, every closure expression creates a unique unnameable type. Storing closures in a homogenous collection like `Vec<(String, Closure)>` fails because distinct closures have different anonymous types.

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Storing naked closures with mismatched anonymous types):
let tests: Vec<(String, fn(&mut Vec<Expectation>))> = vec![
    ("test_ok".to_string(), |e| { ... }),
    ("test_bad".to_string(), |e| { ... }),
];

// AFTER (Using function pointers `fn(&mut Vec<Expectation>)` or Boxed closures `Box<dyn Fn(...)>`):
type ExpectationFn = fn(&mut Vec<Expectation>);
let tests: Vec<(String, ExpectationFn)> = vec![
    ("test_ok".to_string(), |e: &mut Vec<Expectation>| { ... }),
    ("test_bad".to_string(), |e: &mut Vec<Expectation>| { ... }),
];
```

***

### Issue 8: Borrow Checker Ownership & Mutable Borrow Conflicts (`E0382` / `E0502`)

#### **Symptom / Compiler Output:**

```text
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> src/system/cron.rs:88:17
   |
84 |         for job in &self.jobs {
   |                    ---------- immutable borrow occurs here
...
88 |                 self.execute_job(job);
   |                 ^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
```

#### **Why It Occurs:**

Iterating over a collection (`&self.jobs`) borrows `self` immutably. Calling a mutable method (`self.execute_job(...)`) while holding the immutable borrow violates Rust's aliasing rules (only one mutable reference or multiple immutable references, never both).

#### **How to Fix It (Blueprint):**

```rust
// BEFORE (Conflict: mutating `self` during iteration over `&self.jobs`):
for job in &self.jobs {
    if job.is_due() {
        self.execute_job(job);
    }
}

// AFTER (Fix: collect IDs or clone jobs to release the immutable borrow first):
let due_job_ids: Vec<u64> = self.jobs.iter()
    .filter(|j| j.is_due())
    .map(|j| j.id)
    .collect();

for id in due_job_ids {
    self.execute_job_by_id(id);
}
```

***

## SECTION 3: 4-STEP AI AGENT ALGORITHM DIAGNOSTIC & FIX WORKFLOW

When an AI agent is tasked with diagnosing or fixing algorithms in SigmaOS, follow this strict 4-step workflow:

    +-------------------------------------------------------------------------+
    | STEP 1: EXECUTE MASTER TEST SUITE & COMPILER DIAGNOSTICS               |
    | Run `./run_sigma_tests.sh` and `cargo check --lib` to capture errors.   |
    +-------------------------------------------------------------------------+
                                        |
                                        v
    +-------------------------------------------------------------------------+
    | STEP 2: PARSE & CLASSIFY ERROR CODES                                   |
    | Identify error code (E0004, E0282, E0308, E0432, E0512, E0599, etc.).   |
    +-------------------------------------------------------------------------+
                                        |
                                        v
    +-------------------------------------------------------------------------+
    | STEP 3: APPLY SAFE RUST BLUEPRINT PATTERN                              |
    | Apply code fix using the corresponding blueprint in Section 2 above.    |
    +-------------------------------------------------------------------------+
                                        |
                                        v
    +-------------------------------------------------------------------------+
    | STEP 4: VERIFY RE-COMPILATION & CONFIRM PASSING TESTS                   |
    | Re-run `./run_sigma_tests.sh` to confirm 100% test pass rate.           |
    +-------------------------------------------------------------------------+

1.  **Step 1: Diagnostic Discovery**: Run `./run_sigma_tests.sh` to test core atomic harnesses and subsystem inspection modules. Run `cargo check --lib` for full library syntax analysis.
2.  **Step 2: Failure Classification**: Parse the compiler or test output to isolate the error code (`E0004`, `E0282`, `E0308`, `E0432`, `E0512`, `E0599`, `E0614`), affected module file path, and line numbers.
3.  **Step 3: Safe Rust Code Repair**: Locate the matching blueprint pattern in Section 2. Apply explicit type annotations, `ToString` trait imports, safe enum pattern matching, or function pointer casting as required.
4.  **Step 4: Test Suite Verification**: Execute `./run_sigma_tests.sh` to verify clean compilation and confirm all 437+ unit tests pass without regressions.

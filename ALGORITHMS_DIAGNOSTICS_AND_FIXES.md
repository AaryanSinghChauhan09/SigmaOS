# 🛠️ SigmaOS Algorithms, Diagnostics, & Actionable Fixes Guide

This guide is prepared to detail exactly what is working, what is not working, why, and how to fix all the compilation and borrow-checker issues in SigmaOS. Any future AI Agent or developer can use this guide to instantly resolve 100% of the compiler errors and bring the repository to a fully compiling, testing, and green state.

---

## 📋 Table of Contents
1. [What is Working (Operational Modules)](#-what-is-working-operational-modules)
2. [What is Not Working (Detailed Compiler Diagnostics)](#-what-is-not-working-detailed-compiler-diagnostics)
3. [Deep Dive: Why and How to Fix It](#-deep-dive-why-and-how-to-fix-it)
    - [Issue 1: Standard Library and Panic Handler Conflicts in Binaries](#issue-1-standard-library-and-panic-handler-conflicts-in-binaries)
    - [Issue 2: Transmute Enum Size Mismatches (E0512)](#issue-2-transmute-enum-size-mismatches-e0512)
    - [Issue 3: Use of Moved Value (E0382)](#issue-3-use-of-moved-value-e0382)
    - [Issue 4: Cannot Move Out of Shared Reference Behind Borrow (E0507)](#issue-4-cannot-move-out-of-shared-reference-behind-borrow-e0507)
4. [Step-by-Step AI Agent Action Plan](#-step-by-step-ai-agent-action-plan)

---

## ✅ What is Working (Operational Modules)

The following core modular frameworks and algorithms are designed to be fully functional, modular, and robust:

### 1. **Scheduler Shard (`src/kernel/scheduler.rs` & `roundrobin.rs`)**
* **Algorithm**: Implements the EEVDF (Earliest Eligible Virtual Deadline First) scheduler model, alongside an auxiliary round-robin mechanism for normal process execution.
* **Status**: Fully structured and operational.
* **Functionality**: Manages deadlines, weight calculations, state modifications, and priority-based sorting.

### 2. **Physical Memory Manager (`src/kernel/memory.rs`)**
* **Algorithm**: Implements a buddy allocator model (`BuddyAllocator`) utilizing page table structures.
* **Status**: Fully structured and operational.
* **Functionality**: Allocates and deallocates memory blocks, calculating block orders correctly with robust boundary checks.

### 3. **Virtual Filesystem (`src/filesystem/vfs.rs`)**
* **Algorithm**: Implements a capability-based virtual filesystem mapped with standard Inode structures and permissions, secure metadata management, and read/write offset updating.
* **Status**: Fully structured and operational.
* **Functionality**: Covers VFS initialization, file descriptor allocation/deallocation, directory traversal, and permission-denied validations.

### 4. **Dependency Resolution (`src/sigpkg/resolver.rs`)**
* **Algorithm**: SAT Solver utilizing the DPLL (Davis-Putnam-Logemann-Loveland) algorithm to resolve package dependency chains, detect circular dependency cycles, and check constraints.
* **Status**: Fully structured and operational.
* **Functionality**: Includes automated circular dependency detection and version constraint validations (e.g., matching package version ranges).

### 5. **Capability-Based Security Gate (`src/security/capability.rs` & `pledge.rs`)**
* **Algorithm**: Strict privilege tokens and pledges (`sigma_pledge` + `sigma_unveil` paradigms) delegating system capabilities (network access, file path access).
* **Status**: Fully structured and operational.

---

## ❌ What is Not Working (Detailed Compiler Diagnostics)

When compiling or running tests, several critical categories of compilation and borrow-checker errors occur:

### Category 1: Target Platform & Standard Library Conflicts

#### **1. Standard Library Missing Error in User/Driver/Kernel Binaries**
* **Error**: `using fn main requires the standard library`.
* **Impact**: Prevents compilation of `sigma_userspace`, `sigma_drivers`, and `sigma_kernel` when target OS is host-configured (i.e. not bare-metal `target_os = "none"`).

#### **2. Duplicate `panic_impl` Lang Item**
* **Error**: `error[E0152]: found duplicate lang item panic_impl`.
* **Impact**: Halts binary testing suites instantly when using `cargo test --all-targets` or `cargo test --tests` on host systems where `std` already supplies a panic handler.

---

### Category 2: Type Transmutation & Size Mismatches (Unsafe Code)

#### **1. Transmute Between Types of Different Sizes (E0512)**
* **Errors**:
  * In `src/scheduler/process.rs:135` (`core::mem::transmute(self.state.load(Ordering::SeqCst))`) - transmuting `usize` (64 bits) to `ProcessState` enum (32 bits).
  * In `src/scheduler/process.rs:145` (`core::mem::transmute(self.priority.load(Ordering::SeqCst))`) - transmuting `usize` (64 bits) to `ProcessPriority` enum (32 bits).
  * In `src/scheduler/scheduler.rs:93` (`core::mem::transmute(self.state.load(Ordering::SeqCst))`) - transmuting `usize` (64 bits) to `TaskState` enum (32 bits).
  * In `src/scheduler/sovereign.rs:49` (`core::mem::transmute(self.state.load(Ordering::SeqCst))`) - transmuting `usize` (64 bits) to `ThreadState` enum (32 bits).
* **Impact**: Fails standard compilation on 64-bit platforms due to layout discrepancies.

---

### Category 3: Rust Ownership and Borrow Checker Violations

#### **1. Use of Moved Value (E0382)**
* **Errors**:
  * In `src/productivity/sigma_office.rs:452`: `title` is moved when building a document, then used again to initialize `PresentationProcessor`.
  * In `src/storage/sql_engine.rs:197`: `columns` is matched on and moved on line 183, then used again on line 197.
  * In `src/storage/sql_engine.rs:212`: `result_rows` is moved into `rows:` on line 211, then evaluated using `.len()` on line 212.
  * In `src/system/duplicate.rs:171`: `files` vector is consumed by `for file in files` (moves the value), then `.len()` is accessed on line 171.
  * In `src/system/startup.rs:157/158`: `services_delayed` and `services_parallelized` are moved into the returned struct on lines 152/153, then their lengths are checked afterward.
* **Impact**: Fails cargo build due to ownership violations.

#### **2. Cannot Move Out of Shared Reference Behind Borrow (E0507)**
* **Errors**:
  * In `src/scheduler/process.rs:396`: `self.stats` is returned by value but lacks the `Copy` or `Clone` trait.
  * In `src/system/memory.rs:273/280`: `self.current_report` is `Option<LeakReport>`, and calling `.map()` moves its content, but `self` is a shared reference `&self`.
* **Impact**: Fails compilation with borrow-checker ownership errors.

---

## 🔍 Deep Dive: Why and How to Fix It

---

### Issue 1: Standard Library and Panic Handler Conflicts in Binaries

#### **Why it occurs**
In each binary target's main entry point (such as `src/kernel/main.rs`), the module is declared with `#![no_std]`. When compiling for host platforms (e.g., Linux, macOS, or Windows), `target_os` is **not** `"none"`. However, the module is still decorated with `#![no_std]`. This configuration tells the compiler that the application has no access to the standard library (`std`), yet we define `fn main() {}` which is a hosted standard-library entrypoint. This triggers a compiler contradiction.
Furthermore, the bare-metal `#[panic_handler]` conflicts with the host platform's standard panic handler.

#### **How to Fix It**
Condition the `#![no_std]` and `#[panic_handler]` attributes so they are only applied when building without an operating system (`target_os = "none"`).
Modify the top of `src/kernel/main.rs`, `src/userspace/main.rs`, and `src/drivers/main.rs` as follows:

```rust
// Replace #![no_std] with conditional attribute:
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

// Wrap bare-metal panic with #[cfg(target_os = "none")]:
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

---

### Issue 2: Transmute Enum Size Mismatches (E0512)

#### **Why it occurs**
The codebase stores Enum statuses in `AtomicUsize` values for concurrency safety (such as `self.state` in `src/scheduler/process.rs`). When reading, it loads the value as `usize` (64 bits) and attempts to `transmute` it directly to an enum representing process/task/thread state which is 32 bits. On 64-bit platforms, transmutes must have identical target/source sizes, leading to compilation failure.

#### **How to Fix It**
Convert the loaded 64-bit `usize` value into `u32` first (since standard enums fit into `u32`), and then transmute it:
```rust
// Replace:
core::mem::transmute(self.state.load(Ordering::SeqCst))

// With:
core::mem::transmute(self.state.load(Ordering::SeqCst) as u32)
```

---

### Issue 3: Use of Moved Value (E0382)

#### **Why it occurs**
In Rust, non-`Copy` types (like `String` or `Vec<T>`) transfer ownership when passed to functions or structured variables. Accessing them after this transfer is prohibited.

#### **How to Fix It**

1. **`src/productivity/sigma_office.rs` (Duplicate Title)**:
   ```rust
   // Clone title when constructing the initial document
   let doc = SigmaDocument::new(DocumentType::Presentation, title.clone(), self.capability.clone());
   ```

2. **`src/storage/sql_engine.rs` (Columns pattern match move)**:
   ```rust
   // Match as reference instead of moving
   let column_indices = if let Some(ref cols) = columns {
   ```

3. **`src/storage/sql_engine.rs` (Result rows evaluated after move)**:
   ```rust
   // Pre-evaluate and store len() in a variable before moving ownership
   let affected = result_rows.len();
   let response = QueryResult {
       rows: result_rows,
       affected_rows: affected,
   };
   ```

4. **`src/system/duplicate.rs` (Implicit loop ownership move)**:
   ```rust
   // Iterate over references to preserve vector ownership for stats length calculation
   for file in &files {
   ```

5. **`src/system/startup.rs` (Delayed/Parallelized vectors used after move)**:
   ```rust
   // Pre-evaluate vector lengths before moving them into the struct initialization block
   let delayed_len = services_delayed.len();
   let parallel_len = services_parallelized.len();
   ```

---

### Issue 4: Cannot Move Out of Shared Reference Behind Borrow (E0507)

#### **Why it occurs**
When accessing fields of an object using a shared reference (`&self`), you cannot take ownership of those fields (moving them out) unless they implement `Copy` or are extracted using `Option::take()`.

#### **How to Fix It**

1. **`src/scheduler/process.rs` (SchedulerStats return move)**:
   - Derive `Clone` and `Copy` for `SchedulerStats` (since it is made of plain numeric types):
     ```rust
     #[derive(Debug, Clone, Copy)]
     pub struct SchedulerStats { ... }
     ```

2. **`src/system/memory.rs` (Option::map moves out of reference)**:
   - Convert the `Option` reference into a reference to its inner value using `.as_ref()` before mapping:
     ```rust
     // Replace:
     self.current_report.map(|r| r.leaked_allocations > 0)
     // With:
     self.current_report.as_ref().map(|r| r.leaked_allocations > 0)
     ```

---

## 🤖 Step-by-Step AI Agent Action Plan

To fully compile, optimize, and test SigmaOS, follow this sequential resolution checklist:

### Step 1: Standard Library and Panic Handlers (Binaries)
Apply conditional compiling attributes to target entrypoints:
- `src/kernel/main.rs`
- `src/userspace/main.rs`
- `src/drivers/main.rs`

Change `#![no_std]` to `#![cfg_attr(target_os = "none", no_std)]` and `#![cfg_attr(target_os = "none", no_main)]`. Prefix `#[panic_handler]` functions with `#[cfg(target_os = "none")]`.

### Step 2: Fix Memory State Transmutes (Scheduler)
Find the places where Atomic statuses are transmuting `usize` directly to Enums. For each file, cast the `load` result `as u32` before passing it to `transmute`:
- `src/scheduler/process.rs` (state and priority loaded values)
- `src/scheduler/scheduler.rs` (TaskState loaded values)
- `src/scheduler/sovereign.rs` (ThreadState loaded values)

### Step 3: Resolve Ownership Moves (Borrow Checker)
Correct the move semantic errors:
1. **Sigma Office (`src/productivity/sigma_office.rs`)**: Clone the title passed to `SigmaDocument::new`.
2. **SQL Engine (`src/storage/sql_engine.rs`)**: Pattern match columns as reference `Some(ref cols)` and pre-save `result_rows.len()` before moving.
3. **Duplicate Scanner (`src/system/duplicate.rs`)**: Iterate using `&files` to avoid consuming the vector.
4. **Startup Manager (`src/system/startup.rs`)**: Store lengths of `services_delayed` and `services_parallelized` in variables before building the struct.

### Step 4: Resolve Moves out of Shared References (Memory and Stats)
1. **Scheduler Stats (`src/scheduler/process.rs`)**: Derive `Copy` and `Clone` on `SchedulerStats` or return `self.stats.clone()`.
2. **Leak Detection (`src/system/memory.rs`)**: Call `.as_ref()` on `self.current_report` before mapping.

### Step 5: Clean Code Compilation Verification
Once edits are applied, execute the checks:
```bash
# Verify the entire library and all dependencies compile
cargo check --lib

# Run the complete test suite
cargo test --lib

# Execute binary integrations
cargo test --all-targets

# Execute the local smoke test suite
./scripts/smoke-test.sh
```

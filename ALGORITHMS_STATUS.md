# 📊 SigmaOS — Core Algorithms Status, Analysis & Repair Manual

This document provides a highly detailed analysis of the core algorithms implemented in SigmaOS. It outlines what is currently functional, what is stubbed or failing, the underlying technical reasons ("Why"), and step-by-step engineering blueprints ("How to Fix") so that **any AI agent or human developer can easily diagnose, implement, and resolve these modules.**

---

## 📌 Executive Summary
SigmaOS is an ambitious sovereign, capability-based microkernel written in Rust. In its current phase of development:
- **User-space / Simulation Layer**: Unit tests pass cleanly on the host system because the library compiles under the standard library target and mocks kernel boundaries.
- **Microkernel / Metal Layer**: Real target compilation (`sigma_kernel`, `sigma_drivers`, `sigma_userspace`) fails during standard `cargo test` and `make build` because of duplicate `panic_impl` definitions and standard library linkage issues.
- **Algorithmic Correctness**: Most core data structures (buddy allocator, round-robin, page maps) are elegantly mocked or partially implemented, but lack critical details needed for raw bare-metal execution.

---

## 1. ⚙️ Global Build & Integration Infrastructure

### 🔍 Overview
SigmaOS uses conditional compilation (`#![cfg_attr(target_os = "none", no_std)]` and `#![cfg_attr(target_os = "none", no_main)]`) in its binaries (`src/kernel/main.rs`, `src/drivers/main.rs`, and `src/userspace/main.rs`) to allow compiling on the host computer as a library but switching to bare-metal mode for OS images.

### 🟢 What is Working
- The core codebase compiles as a library (`sigmaos`) and its unit tests (155 in total) run and pass flawlessly with `cargo test --lib`.
- Globally suppressed clippy lints and compiler warnings in `src/lib.rs` ensure smooth CI builds.

### 🔴 What is NOT Working
- Building binary targets (with `cargo test --test '*'` or `make build` under native profile) produces:
  `error[E0152]: found duplicate lang item panic_impl`
  and:
  `error: using fn main requires the standard library`.

### ❓ The "Why"
- When compiling unit tests for binaries (such as `sigma_kernel` as a test runner binary), the compiler pulls in `std` for testing, but the files `src/kernel/main.rs`, `src/drivers/main.rs`, and `src/userspace/main.rs` define their own `#[panic_handler]` conditionally based on `no_std`, which conflicts with `std`'s default panic implementation during native test compilation.

### 🛠️ How to Fix
1. **Isolate `#[panic_handler]`**:
   In `src/kernel/main.rs`, `src/drivers/main.rs`, and `src/userspace/main.rs`, wrap the custom `panic_handler` with `#[cfg(not(test))]` or `#[cfg(all(target_os = "none", not(test)))]` to prevent it from compiling when standard library tests are run.
   ```rust
   #[cfg(all(target_os = "none", not(test)))]
   #[panic_handler]
   fn panic(_info: &PanicInfo) -> ! {
       loop {}
   }
   ```
2. **Exclude Binary Targets from Native Tests**:
   Update `Cargo.toml` to disable automatic harness testing for kernel binaries so that native standard-library testing only builds the core library and its unit tests.
   ```toml
   [[bin]]
   name = "sigma_kernel"
   path = "src/kernel/main.rs"
   test = false

   [[bin]]
   name = "sigma_drivers"
   path = "src/drivers/main.rs"
   test = false

   [[bin]]
   name = "sigma_userspace"
   path = "src/userspace/main.rs"
   test = false
   ```

---

## 2. 📅 EEVDF Scheduler Algorithm

### 🔍 Overview
- **Location**: `src/kernel/scheduler.rs`
- **Algorithm**: Earliest Eligible Virtual Deadline First (EEVDF) — the modern scheduler replacing CFS in Linux. It calculates virtual deadlines based on process priorities (weights) to allocate CPU time proportionally.

### 🟢 What is Working
- **Process Representation**: PCB contains `priority`, `runtime`, `virtual_deadline`, and `time_slice`.
- **Deadline Updates**: `update_virtual_deadline` maps priority enum levels (`Idle` to `Realtime`) to specific weights and increments `virtual_deadline` based on $current\_time + \frac{1000}{weight}$.
- **Eligible Process Selection**: `schedule()` filters the process list to find active processes whose virtual deadline is eligible ($virtual\_deadline \le current\_time$), picking the one with the minimum virtual deadline.

### 🔴 What is NOT Working
- **True Virtual Time ($V$) Tracking**: True EEVDF requires tracking global virtual time ($V$) based on the sum of active weights. Currently, the scheduler uses a simple, monotonic tick counter `current_time` as virtual time.
- **Lag/Service Tracking**: It does not track *lag* ($Lag_i = V - v_i$) which determines whether a process is *eligible* (eligible when $Lag_i \ge 0$). Currently, a process is deemed eligible simply if `p.virtual_deadline <= self.current_time`.

### ❓ The "Why"
- Implemented as a simplified simulation of EEVDF to pass basic mock tests without needing complex floating-point or fixed-point weight arithmetic.

### 🛠️ How to Fix
To convert this from a simulated model to a fully compliant EEVDF algorithm:
1. **Introduce Global Virtual Time ($V$)**:
   Keep track of the sum of weights of all currently runnable processes ($W = \sum w_i$). In `tick()`, increment the global virtual time $V$ relative to physical time passed ($\Delta t$):
   $$\Delta V = \frac{\Delta t}{W}$$
2. **Calculate Virtual Runtime ($v_i$) and Lag ($Lag_i$)**:
   For each process $i$, track its virtual runtime:
   $$\Delta v_i = \frac{\Delta t_i}{w_i}$$
   Calculate its lag as:
   $$Lag_i = V - v_i$$
3. **Redefine Eligibility**:
   A process is eligible for selection if and only if its lag is non-negative ($Lag_i \ge 0$, meaning it has received less than its fair share of CPU service).
4. **Select by Virtual Deadline**:
   Among all eligible processes, select the one with the earliest virtual deadline:
   $$d_i = v_i + \frac{q}{w_i}$$
   where $q$ is the allocation slice size.

---

## 3. 🔄 Round-Robin Scheduler Algorithm

### 🔍 Overview
- **Location**: `src/kernel/roundrobin.rs`
- **Algorithm**: A priority-aware, round-robin preemption scheduler with CPU register context tracking (`CpuContext`) and voluntary yielding capability.

### 🟢 What is Working
- **Time Slice Scaling**: Dynamically scales process time slices using priority multipliers (e.g., `Realtime` gets $8 \times$ time slice; `Low` gets $1 \times$).
- **Context Saving/Restoring**: Implements a standard `CpuContext` struct containing x86_64 general-purpose registers (`rax` to `r15`, `rsp`, `rip`, `rflags`) with simulated state-saving.
- **Yield Mechanism**: Process can call `yield_current()`, setting a `yield_requested` flag that triggers context switching on the very next scheduler tick.
- **All unit tests pass perfectly**.

### 🔴 What is NOT Working
- **Hardware Integration**: The context switch is purely simulated. Register values are loaded/saved in standard Rust structs, but not written to physical CPU registers.

### ❓ The "Why"
- Because SigmaOS is built as a portable library, it separates the scheduling logic (which is architecture-independent) from the architecture-specific context switcher.

### 🛠️ How to Fix
To bind this round-robin engine to live physical CPUs:
1. **Implement Assembly Context-Switching**:
   Write raw `asm!` block switch macros inside `src/arch/x86_64/interrupt.rs` or similar architecture shards.
2. **Trigger switch inside Interrupt Service Routines (ISRs)**:
   Point the APIC Timer interrupt ISR directly to the scheduler's `tick()` and `save_context()` / `restore_context()` methods.
   ```rust
   #[no_mangle]
   pub unsafe extern "C" fn timer_interrupt_handler(stack_frame: *mut InterruptStackFrame) {
       // 1. Save registers to current process context
       // 2. Call RoundRobinScheduler::tick()
       // 3. Call RoundRobinScheduler::restore_context()
       // 4. Load registers and perform iretq
   }
   ```

---

## 4. 🧮 Buddy Allocator Algorithm

### 🔍 Overview
- **Location**: `src/kernel/memory.rs` (with companion at `src/klib/buddy_allocator.rs`)
- **Algorithm**: Binary Buddy Allocator tracking block sizes from $2^0$ to $2^{11}$ pages of size 4KB (4KB to 8MB blocks).

### 🟢 What is Working
- **Order Determination**: Correctly calculates binary orders from arbitrary page counts using `calculate_order()`.
- **Splitting Blocks**: `split_block()` splits larger memory blocks repeatedly into smaller ones to satisfy small allocation requests, pushing buddies onto the lower-order free lists.
- **Coalescing / Merging**: On deallocation, `try_merge()` recursively tries to locate the deallocated block's binary buddy (calculated via XOR: `block_addr ^ (1 << (order + 12))`).
- **Safe Ownership Preservation**: Leverages a robust `Result<MemoryBlock, MemoryBlock>` pattern during merge lookups. If the buddy is not in the free list, ownership of the original block is safely returned without memory leaks or premature drop panics.

### 🔴 What is NOT Working
- **Physical Memory Mapping**: The allocator is initialized with mock base addresses. It is not fed with actual physical memory map layouts (e.g., E820 maps from GRUB or UEFI memory descriptors).
- **Concurrency / Thread Safety**: The allocator lacks thread-safety locks, making it unsafe for multi-core (SMP) operations.

### ❓ The "Why"
- Thread locks (like Spinlocks) require atomic platform support or standard `Mutexes`, which are not natively available in core microkernel `no_std` context without custom spin lock structures.

### 🛠️ How to Fix
1. **Integrate UEFI / E820 Memory Map**:
   Write an initialization helper in the boot shard (`src/boot/`) to parse the physical memory map. Feed free, usable regions to `BuddyAllocator::initialize_memory()` page-by-page or block-by-block.
2. **Implement Thread Safety (Spinlock)**:
   Implement a lightweight, hardware-backed spinlock using Rust's `core::sync::atomic::AtomicBool`.
   ```rust
   pub struct Spinlock<T> {
       locked: AtomicBool,
       data: UnsafeCell<T>,
   }
   // Wrap BuddyAllocator in the spinlock for global access
   pub static GLOBAL_ALLOCATOR: Spinlock<BuddyAllocator> = Spinlock::new(BuddyAllocator::new());
   ```

---

## 5. 📄 Virtual Memory Manager & Paging

### 🔍 Overview
- **Location**: `src/kernel/memory.rs` (with companion at `src/klib/paging.rs`)
- **Algorithm**: Virtual Memory Manager walking multi-level page tables to manage address space mappings on x86_64 architecture.

### 🟢 What is Working
- **Page Table Structure**: Correctly represents aligned `PageTable` and 64-bit `PageTableEntry` entries.
- **Entry Flags**: Native implementation of x86_64 entry flags (`PRESENT`, `WRITABLE`, `USER_ACCESSIBLE`, `NO_EXECUTE`).
- **Basic Mapping APIs**: Implements mapping, unmapping, and translation operations.

### 🔴 What is NOT Working
- **Mock Translation**: The current translation walks a flat PML1 table (`pt_index = (virtual_addr >> 12) & 0x1FF`) rather than performing a true 4-level PML4 -> PDPT -> PD -> PT page table walk.
- **On-Demand Allocation of Page Tables**: When mapping virtual addresses whose intermediate page directories (PD, PDPT, etc.) do not exist, the VMM does not allocate new pages from the Buddy Allocator to serve as intermediate page tables.

### ❓ The "Why"
- Walking four full directories requires memory mappings to dynamically manage table page allocation, which is tightly coupled to the physical memory allocator.

### 🛠️ How to Fix
1. **Implement True 4-Level Page Table Walk**:
   Replace the flat walk with a structured step-by-step lookup for x86_64 paging:
   - PML4 index: `(addr >> 39) & 0x1FF`
   - PDPT index: `(addr >> 30) & 0x1FF`
   - PD index: `(addr >> 21) & 0x1FF`
   - PT index: `(addr >> 12) & 0x1FF`
2. **Dynamic Page Table Allocation**:
   During `map_page()`, if intermediate tables are missing (i.e., `PRESENT` flag is not set on the PML4/PDPT/PD entries), call the physical `BuddyAllocator` to allocate a clean, zeroed 4KB frame, write its physical address to the directory entry, mark it as `PRESENT | WRITABLE | USER`, and proceed down the tree.

---

## 6. 🌐 TCP/IP Sovereign Stack

### 🔍 Overview
- **Location**: `src/network/tcp.rs` (with companions in `src/network/`)
- **Algorithm**: A capability-gated TCP network connection state machine implementing the TCP standard protocol flow.

### 🟢 What is Working
- **State Machine Transitions**: Tracks states (`Closed`, `Listen`, `SynSent`, `SynReceived`, `Established`, etc.).
- **Segment Processing**: Correctly handles inbound TCP packets in `process_segment()`, updating states (e.g., changing connection status to `Established` upon receiving valid `SYN-ACK` packets).
- **Access Control Integration**: Capability-based security token checks are built-in (`has_capability()`), ensuring that connections can only be initialized or accepted if the calling process holds corresponding socket privileges.

### 🔴 What is NOT Working
- **Window Management & Congestion Control**: Lacks sliding window flow control and congestion avoidance algorithms (such as TCP Reno/Tahoe).
- **Packet Queue & Retransmissions**: Lacks a buffer for packet reordering, out-of-order segment processing, and retransmission timeout (RTO) triggers.

### ❓ The "Why"
- Designed as a clean state-machine template that demonstrates capability-gated security rather than a fully-featured, high-throughput network engine.

### 🛠️ How to Fix
To make this network stack production-grade:
1. **Introduce Sequenced Send/Receive Buffers**:
   Add `send_buffer: Vec<u8>` and `receive_buffer: BTreeMap<u32, Vec<u8>>` to `TcpConnection`. The `receive_buffer` keys on the packet sequence number to automatically reorder packets arriving out-of-order.
2. **Retransmission Queue & Timer**:
   Implement a packet state queue tracking unacknowledged packets. Start a high-resolution timer when transmitting. If no `ACK` is received within the Calculated RTO, retransmit from the queue.
3. **Implement Congestion Control**:
   Track variables: `snd_nxt` (next seq to send), `snd_una` (unacknowledged seq), `cwnd` (congestion window), and `ssthresh` (slow start threshold). Adjust `cwnd` dynamically based on successful `ACK` arrivals or timeouts.

---

## 7. 📁 Virtual Filesystem (VFS) & Security

### 🔍 Overview
- **Location**: `src/filesystem/vfs.rs` (with companions in `src/filesystem/`)
- **Algorithm**: A modular capability-enforced VFS mapping inode operations and file descriptor tables.

### 🟢 What is Working
- **Robust Security Integration**: Inodes store fine-grained `FilePermissions` alongside high-level `CapabilityToken` checks.
- **Fd Tables**: Dynamically opens, manages, and removes file descriptors on file interaction.
- **Safety**: Integrates defensive coding checks (`checked_add`) on off-set modifications to prevent integer overflows during read/write simulations.
- **All unit tests pass correctly**.

### 🔴 What is NOT Working
- **Flat Directory Mapping**: The folder structure is flat. The VFS does not resolve directories hierarchically (e.g., walking `/usr/bin/shell` component-by-component).
- **Disk Synchronization**: All reads/writes are simulated in-memory and are not synced back to actual physical backing stores (like EXT4 or FAT32 blocks).

### ❓ The "Why"
- It is designed as a mock capability layer that proves the safety architecture of the file system independent of block driver layers.

### 🛠️ How to Fix
1. **Implement Path Resolution**:
   Write a recursive lookup function `resolve_path(path: &str) -> Result<u64, FsError>` that breaks path strings by `/`, reads the directory's data block to find directory entries (mapping name string to target inode ID), and repeats the process until the final file's inode ID is located.
2. **Back VFS with Real Storage Shards**:
   Bind the VFS read/write functions to real physical partition drivers (such as the existing `Ext4` or `Fat32` implementation blocks). On VFS modifications, call block drivers to sync altered sectors or inodes to physical sectors.

---

## 8. 🗜️ Compression Engine (LZ77 + Huffman + DEFLATE)

### 🔍 Overview
- **Location**: `src/compression/algorithms.rs`
- **Algorithm**: Custom DEFLATE-compliant compression pipeline implementing LZ77 sliding window matches and Huffman tree frequency encoding under a standalone `no_std` environment.

### 🟢 What is Working
- **LZ77 Search Window**: Correctly runs sliding window match finding up to DEFLATE's max length of 258 and max offset of 32768.
- **Huffman Tree Builder**: Correctly builds Huffman binary trees from frequency maps using sorted vectors and internal-vs-leaf node pointers.
- **RLE Compressor**: Features a fully-functional, separate Run-Length Encoding compressor and decompressor.

### 🔴 What is NOT Working
- **Library Compilation Warnings**: Emits unused variable and import warnings.
- **Unresolved Allocations (Critical Compiling Bug)**: Contains custom `Vec` and `Iter` code that invokes undefined external `extern "C"` functions `alloc` and `free`. This prevents successful target compilation of the operating system library because these external functions are not resolved by any linker or library runtime.

### ❓ The "Why"
- The module attempts to run a completely custom allocation system without importing Rust's core `alloc` crate, leading to a clash between standard Rust vectors/allocators and the custom local implementation.

### 🛠️ How to Fix
1. **Import `alloc` Crate**:
   Delete the custom `struct Vec` and raw `extern "C" { fn alloc ... }` definitions. Instead, configure the workspace to leverage standard system vectors by importing the default core allocator:
   ```rust
   extern crate alloc;
   use alloc::vec::Vec;
   use alloc::boxed::Box;
   ```
2. **Clean up unused declarations**:
   Remove unused imports like `core::ptr` and `core::mem` to clean up the compiler lints. This will make the compression engine instantly compile, pass tests, and link cleanly into any binary target of SigmaOS.

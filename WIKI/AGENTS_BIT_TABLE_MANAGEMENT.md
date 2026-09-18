# AI Agent Guidelines: Bit Table Management in SigmaOS

## 📌 Overview & Core Directives

In **SigmaOS**, bit tables (bitmaps, bitfields, page table entry bitmasks, and atomic bit arrays) form the foundational low-level data structures for memory management, capability enforcement, interrupt vector dispatching, and system call filtering.

As an AI agent working on SigmaOS, you must uphold zero-allocation, $O(1)$ constant-time bit manipulation invariants across all bare-metal (`#![no_std]`) subsystems.

---

## ⚙️ Key Bit Table Subsystems & Architecture

### 1. Physical Memory Bitmap Allocator
* **Module Location:** `src/klib/bitmap.rs`, `src/memory/bitmap.rs`, `src/memory/buddy_allocator.rs`
* **Invariants:**
  * Uses 64-bit words (`u64` / `AtomicU64`) to represent memory physical frame occupancy where bit `0` indicates free and `1` indicates allocated.
  * $O(1)$ frame searching using hardware intrinsics (`trailing_zeros()` / `tzcnt`).
  * Bitwise index calculation:
    $$\text{word\_idx} = \frac{\text{frame\_idx}}{64}, \quad \text{bit\_mask} = 1 \ll (\text{frame\_idx} \bmod 64)$$

### 2. Virtual Memory Page Table Entry (PTE) Flags
* **Module Location:** `src/klib/paging.rs`, `src/kernel/vmm_paging.rs`, `src/boot/uefi.rs`
* **Invariants:**
  * Page Table Entry flags must be composed using bitwise bitfields:
    * `PRESENT = 1 << 0`
    * `WRITABLE = 1 << 1`
    * `USER_ACCESSIBLE = 1 << 2`
    * `WRITE_THROUGH = 1 << 3`
    * `NO_CACHE = 1 << 4`
    * `ACCESSED = 1 << 5`
    * `DIRTY = 1 << 6`
    * `HUGE_PAGE = 1 << 7`
    * `GLOBAL = 1 << 8`
    * `COPY_ON_WRITE = 1 << 9`
    * `NO_EXECUTE = 1 << 63`
  * Updates to PTE flags must execute atomically or flush TLB entries via `invlpg` / `tlbi`.

### 3. Capability Bitmasks & Authorization
* **Module Location:** `src/security/capability.rs`, `src/security/pledge.rs`, `src/security/sigma_pledge.rs`
* **Invariants:**
  * Authorization tokens (`CapabilityToken`) use 64-bit bitmasks representing permission bitfields (`PLEDGE_STDIO`, `PLEDGE_RPATH`, `PLEDGE_WPATH`, `PLEDGE_INET`, `PLEDGE_EXEC`).
  * Verification requires a single $O(1)$ bitwise AND operation:
    $$\text{is\_permitted} = (\text{active\_mask} \mathbin{\&} \text{required\_permission}) == \text{required\_permission}$$

### 4. Interrupt & Syscall Filter Bit Table
* **Module Location:** `src/syscall/table.rs`, `src/hal/multi_arch.rs`, `src/kernel/hal.rs`
* **Invariants:**
  * Syscall permissions use 256-bit bitset tables (`[u64; 4]`) enabling $O(1)$ constant-time syscall validation without linear searches.
  * Interrupt pending/in-service tables utilize APIC/GIC/PLIC bit vectors with atomic `AtomicU64::fetch_or` and `AtomicU64::fetch_and`.

---

## 🛡️ AI Agent Rules for Bit Manipulation

1. **Zero Heap Allocation Rule:**
   * Never instantiate dynamic collections (`Vec`, `BTreeMap`) for bit tables in hot kernel paths. Use fixed-size bit arrays (`[u64; N]`) or slices (`&[u64]`).
2. **Lock-Free Atomic Updates:**
   * For concurrent bit table modifications, use `AtomicU64` with `Ordering::Acquire`, `Ordering::Release`, or `Ordering::SeqCst`.
   * Bit setting: `word.fetch_or(1 << bit, Ordering::Release)`
   * Bit clearing: `word.fetch_and(!(1 << bit), Ordering::Release)`
   * Conditional bit flip: Compare-And-Swap (CAS) loop with `compare_exchange_weak`.
3. **Hardware Intrinsic Optimization:**
   * Always prefer CPU bit-manipulation intrinsics: `.trailing_zeros()`, `.leading_zeros()`, `.count_ones()`, `.rotate_left()`, `.rotate_right()`.
4. **Shift Overflow Safety:**
   * Never perform unbounded bitwise shifts (e.g. `1u64 << shift` where `shift >= 64`). Always check `shift < 64` or use `1u64.checked_shl(shift)`.

---

## 🧪 Verification & Testing Protocol

When adding or modifying bit table code in SigmaOS, AI agents must run standalone unit tests:

```bash
# Test physical bitmap frame allocator & page table bit flags
rustc --test --edition=2021 src/klib/bitmap.rs -o build/bitmap_tests && ./build/bitmap_tests && rm build/bitmap_tests

# Test capability token bitmask authorization
rustc --test --edition=2021 src/security/capability.rs -o build/cap_tests && ./build/cap_tests && rm build/cap_tests

# Test syscall table bitset filters
rustc --test --edition=2021 src/syscall/table.rs -o build/syscall_tests && ./build/syscall_tests && rm build/syscall_tests
```

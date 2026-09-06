# AI Agent Development Instructions for SigmaOS Kernel Subsystem (`src/kernel/`)

This directory contains the primary kernel architecture, memory management, scheduling, process lifecycle, virtual memory, syscall dispatching, eBPF runtime, capability security, and device/driver abstractions for SigmaOS.

## Core Directives & Architecture Guidelines

1. **`no_std` Pure Rust Compatibility**
   - All kernel modules must adhere strictly to `no_std` environments unless explicitly conditioned under `#[cfg(test)]`.
   - Use `core::` imports (such as `core::sync::atomic::*`, `core::fmt`, `core::cell::*`) and `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::boxed::Box`, `alloc::sync::Arc`) instead of `std`.

2. **Concurrency & Locking Discipline**
   - Memory allocation and process scheduling must maintain strict lock hierarchy rules to prevent kernel deadlocks.
   - Prefer lock-free atomic operations (`AtomicU64`, `AtomicBool`, `AtomicUsize`) or custom spinlocks (`SigmaSpinlock` / `IrqSafeSpinlock`) for low-level primitive synchronization.
   - When acquiring multiple kernel locks, always acquire in order: Scheduler -> Memory Manager -> VFS -> Device/Driver.

3. **Memory Safety & Allocator Isolation**
   - Memory management modules in `src/kernel/memory/` and `src/kernel/mm/` leverage `SigmaBuddyAllocator`, `SlabAllocator`, and `NumaAwareAllocator`.
   - Do not perform untracked raw pointer dereferences outside designated `unsafe` HAL boundary blocks. Always validate physical/virtual memory page alignments (`4096` byte boundary).

4. **Syscall & Privilege Separation**
   - Syscall dispatching in `src/kernel/syscall/` must enforce OpenBSD-style `pledge`/`unveil` privilege bitmasks and hardware-enforced `CapabilityToken` checks before executing privileged operations.
   - Sanitize all input buffers and pointers passed from user space prior to dereferencing.

5. **eBPF & Security Sandboxing**
   - The eBPF verification engine (`src/kernel/ebpf_verification.rs` & `src/kernel/ebpf_vm.rs`) enforces static analysis rules: bounded loops, non-null map lookups, instruction count limits (<4096), and stack access boundaries (512 bytes max).

6. **Testing & Verification Procedures**
   - Before submitting changes to kernel modules, ensure that `cargo check --lib` passes cleanly without unused warnings or type inference errors.
   - Verify kernel abstractions with unit tests where available, e.g., using `cargo test --lib kernel`.

# SigmaOS AI Agent Compare & Swap (CAS) Operation Management Guidelines

## 1. Overview
SigmaOS implements high-performance, zero-allocation lock-free synchronization primitives built around atomic Compare & Swap (CAS) hardware instructions managed by AI system agents (such as `AtomicCasGovernor`, `LockFreeRingBufferManager`, and `AbaPreventionEngine`). These guidelines define atomic CAS primitives (`AtomicUsize`, `AtomicBool`, `AtomicPtr`), ISA hardware atomic mapping (x86_64 `LOCK CMPXCHG`, AArch64 `LDREX/STREX` & LSE `CAS`, RISC-V `LR/SC` & `AMO`), memory ordering models (`SeqCst`, `Acquire`, `Release`, `Relaxed`), ABA generation tagging, and lockless atomic queue synchronization for AI agents in SigmaOS.

## 2. Core Compare & Swap Management Principles

### 2.1 Hardware ISA Atomic Instruction Mapping
- **x86_64 Architecture**: Mapped directly to hardware `LOCK CMPXCHG` / `LOCK CMPXCHG8B` / `LOCK CMPXCHG16B` bus-locked or cache-coherency-locked instructions.
- **AArch64 Architecture**: Mapped to Load-Link/Store-Conditional (`LDREX`/`STREX`) loops or ARMv8.1-A Large System Extensions (LSE) single-instruction atomic CAS (`CAS` / `CASA` / `CASAL`).
- **RISC-V Architecture**: Mapped to Load-Reserved/Store-Conditional (`LR.W`/`SC.W` and `LR.D`/`SC.D`) or Atomic Memory Operations (AMO) instructions.

### 2.2 Memory Ordering Semantics
AI agents manipulating atomic CAS variables must specify explicit memory ordering guarantees:
- **Sequential Consistency (`Ordering::SeqCst`)**: Enforces total global memory ordering across all CPU cores for critical state transitions.
- **Acquire-Release (`Ordering::Acquire` / `Ordering::Release`)**: Guarantees read-acquire and write-release synchronization for lock-free ring buffers and message queues (`SovereignPipe`).
- **Relaxed Ordering (`Ordering::Relaxed`)**: Used solely for non-synchronizing statistical counters (e.g. packet byte counts or PMC hardware event counters).

### 2.3 ABA Problem Prevention & Generation Tagging
- **Double-Width CAS (DW-CAS)**: Lock-free pointer manipulations pack a 64-bit pointer together with a 64-bit monotonically increasing generation counter tag (`tag: u64, ptr: *mut T`).
- **Generation Tag Validation**: Upon CAS execution, both pointer target and generation tag must match expectations (`compare_exchange_weak`), preventing insidious ABA pointer re-use corruption.

### 2.4 Lock-Free Data Structures & Spinlock Backoff
- **Lockless Queues & Ring Buffers**: Lock-free single-producer single-consumer (SPSC) and multi-producer multi-consumer (MPMC) ring buffers (`HeapRingBuffer` in `src/klib/ring_buffer.rs`) use atomic CAS tail/head index advances.
- **Exponential Spinlock Backoff**: Contended CAS retry loops execute CPU pause hints (`core::hint::spin_loop()`) with exponential backoff to reduce cache line bouncing and power consumption.

---
*Maintained by the SigmaOS Kernel, Synchronization & Multi-Arch Steering Committee.*

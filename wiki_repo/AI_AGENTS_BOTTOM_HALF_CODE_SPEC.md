# AI Agents Bottom-Half Code Management Specification for SigmaOS

## Abstract
This specification defines the low-level interrupt handling and deferred execution ("bottom-half") architecture for AI agents operating within or developing drivers for SigmaOS. Bottom-half mechanisms defer heavy processing out of high-priority Interrupt Service Routines (ISRs) to maintain sub-microsecond interrupt latencies, lockless data queueing, and `#![no_std]` memory safety.

---

## 1. Interrupt Processing Architecture: Top-Half vs. Bottom-Half

```
[ Hardware Interrupt (IRQ) ]
             │
             ▼
[ Top-Half ISR (Minimal Latency) ]
  • Acknowledge hardware IRQ
  • Save register context
  • Enqueue event to Bottom-Half Ring Buffer
  • Return IRQ_HANDLED
             │
             ▼
[ Bottom-Half Processing (Deferred Context) ]
  ┌──────────┼───────────┬───────────┐
  ▼          ▼           ▼           ▼
[Tasklet] [WorkQueue] [Threaded IRQ] [eBPF Ring]
(SoftIRQ)  (KThread)  (Real-Time)   (Lockless)
```

### 1.1 Top-Half ISR Rules
- Top-half ISRs must execute within strict time bounds (< 500 nanoseconds).
- Top-half code MUST NOT perform heap memory allocation, blocking mutex locks, disk I/O, or sleeping operations.
- The top half acknowledges hardware interrupts, captures device status registers, enqueues raw descriptors to bottom-half queues, and exits immediately.

---

## 2. Bottom-Half Mechanisms in SigmaOS

### 2.1 Tasklets & SoftIRQs
- **Execution Context**: Executed in software interrupt context with interrupts enabled.
- **Concurrency Guarantee**: A tasklet instance runs on one CPU core at a time, ensuring lock-free execution per tasklet instance.
- **Use Cases**: High-throughput network packet descriptors, USB HID input event processing, audio buffer transfers.

### 2.2 Threaded IRQ Handlers & Deferred WorkQueues
- **Execution Context**: Executed inside kernel threads (`kthread`) with standard process context.
- **Capabilities**: May acquire sleepable mutexes, perform VFS operations, or await DMA completion events.
- **Real-Time Priority**: Threaded IRQ handlers operate under SCHED_FIFO or Apache NuttX POSIX RT preemption-threshold scheduling to guarantee deterministic response times.

### 2.3 Lockless SPMC Ring Buffers (`LinuxBpfRingBuffer`)
- **Zero-Allocation Data Flow**: Lockless Single-Producer Multi-Consumer (SPMC) ring buffers handle data transfer between top-half ISRs and userland/kernel bottom halves.
- **Reserve-Submit Pattern**:
  1. `reserve()`: Allocates contiguous ring buffer slice in top-half ISR.
  2. Write payload directly into ring slice without lock contention.
  3. `submit()`: Commits slice and notifies bottom-half consumers.

---

## 3. DMA & Cache Synchronization

1. **DMA Barrier Rules**:
   - Bottom-half handlers MUST issue memory barriers (`core::sync::atomic::fence(Ordering::SeqCst)`) prior to reading DMA descriptors.
2. **Cache Coherency**:
   - Non-coherent architectures (ARM64, RISC-V) require explicit cache flush/invalidation commands before accessing DMA memory buffers.

---

## 4. Operational Directives for AI Agents

- **Sub-Microsecond Latency Enforcement**: AI agents developing or refactoring kernel drivers MUST verify top-half ISR execution bounds.
- **Zero-Dependency Mandate**: All bottom-half routines must use `#![no_std]` core abstractions (`core::sync::atomic`, lockless ring buffers).
- **Audit & Telemetry**: Bottom-half processing latency and ring buffer overflow events are logged to `journald` telemetry streams.

---

## 5. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_BOTTOM_HALF_CODE_SPEC.md`
- `wiki/AI_AGENTS_BOTTOM_HALF_CODE_SPEC.md`
- `wiki_repo/AI_AGENTS_BOTTOM_HALF_CODE_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Kernel Interrupt & Bottom-Half Architecture*

# SigmaOS AI Agent Bottom Half Kernel Threads Specification

This document specifies mandatory guidelines for top-half interrupt service routines (ISRs), softirq action vectors, tasklets, and deferred workqueue kernel threads (`kworker`) for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Top-Half / Bottom-Half Interrupt Processing Split
- **Top-Half ISR Rules (`src/interrupt/handler.rs`, `src/kernel/irq/irq_controller.rs`)**:
  - Hard IRQ handlers must perform minimal state acknowledge operations (e.g. acknowledge hardware interrupt register) with interrupts disabled.
  - Time-consuming tasks (packet processing, disk block processing) must be deferred to softirqs or workqueues.

## 2. Softirq Vectors & Tasklet Execution
- **Softirq Action Handlers (`src/kernel/irq/softirq.rs`)**:
  - Softirq vectors (`HI_SOFTIRQ`, `TIMER_SOFTIRQ`, `NET_TX_SOFTIRQ`, `NET_RX_SOFTIRQ`, `BLOCK_SOFTIRQ`, `TASKLET_SOFTIRQ`) execute in atomic, non-preemptible interrupt contexts.
  - Softirq routines must never sleep, block on mutexes, or wait for userland I/O.

## 3. Workqueue Kernel Threads (`kworker`)
- **System Workqueues (`src/kernel/irq/workqueue.rs`)**:
  - Deferred tasks requiring process context or sleep capabilities (e.g., waiting for memory allocations or disk locks) must use `WorkQueue` worker threads.
  - Work items (`WorkStruct`) must be enqueued via lock-free atomic queues.

## 4. AI Agent Bottom-Half Directives
1. **No Sleeping in Softirqs**: Softirqs and tasklets must never invoke blocking primitives or allocate memory dynamically.
2. **Interrupt Latency Bound**: Top-half ISR processing time must be bounded under 1 microsecond.

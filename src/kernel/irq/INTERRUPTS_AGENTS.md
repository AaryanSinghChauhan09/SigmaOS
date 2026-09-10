# AI Agent Development Instructions for Kernel Interrupt Management (`src/kernel/irq/`)

This directory implements hardware interrupt handling (IRQ/MSI-X), programmable interrupt controllers (LAPIC/IOAPIC, ARM GIC, RISC-V PLIC), softirq deferred execution, and kernel workqueue processing for SigmaOS.

## Subsystem Architecture & Directives

1. **Hardware Interrupt Controller Drivers (`irq_controller.rs` & `irq_domain.rs`)**
   - Manage IRQ vector mappings (0-255).
   - Interrupt Service Routines (ISRs) MUST execute as quickly as possible. Top-half handlers acknowledge interrupts (EOI - End of Interrupt) and offload heavy processing to bottom-half handlers (`softirq` or `workqueue`).

2. **SoftIRQ & Deferred Bottom-Half Processing (`softirq.rs`)**
   - SoftIRQ vectors (`TIMER_SOFTIRQ`, `NET_TX_SOFTIRQ`, `NET_RX_SOFTIRQ`, `SCHED_SOFTIRQ`, `RCU_SOFTIRQ`) execute with local hardware interrupts enabled.
   - Prevent SoftIRQ starvation by yielding to the `ksoftirqd` kernel thread when softirq execution exceeds 2 milliseconds.

3. **Kernel Workqueues (`workqueue.rs`)**
   - Asynchronous tasks requiring thread sleep or blocking I/O must be deferred to process-context workqueues rather than softirq context.

4. **Interrupt Safety & Locking Rules**
   - Never acquire non-irq-safe locks inside hard-IRQ context. Use `IrqSafeSpinlock` which disables local CPU interrupts before acquiring locks.
   - Do not perform dynamic heap allocations (`std::vec::Vec::new()` or `Box::new()`) within hard-IRQ context.

5. **Verification**
   - Validate IRQ handling changes using `cargo check --lib`.

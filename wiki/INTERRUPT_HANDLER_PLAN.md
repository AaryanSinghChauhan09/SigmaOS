# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

## Overview & Architecture

SigmaOS provides a multi-architecture interrupt handling framework designed to unify **CPU-detected exceptions**, **controller-delivered device interrupts**, and **deferred bottom-half execution**.

The fundamental interrupt pipeline follows a decoupled 5-stage progression:

```text
  [ Hardware Event / CPU Exception ]
                  │
                  ▼
   1. Vector Entry (Arch Assembly / IDT / VBAR / Trap)
                  │
                  ▼
   2. Generic Dispatch & IRQ Domain Mapping (hwirq -> virq)
                  │
                  ▼
   3. Controller Flow Handling (Edge / Level / Fast-EOI / Per-CPU)
                  │
                  ▼
   4. Registered Handler Invocation (Hard-IRQ Top-Half)
                  │
                  ▼
   5. Safe Return / Deferred Work Scheduling (Softirq / Workqueue)
```

### Context Categorization

1. **Exceptions (CPU-Detected Faults & Traps):**
   Synchronous execution state changes detected by the CPU core (e.g. Page Faults, General Protection Faults, Divide Errors, Breakpoints, Alignment Flaws). Handled synchronously with error code decode and process context translation or controlled panic.
2. **Device Interrupts (Hardware-Delivered IRQs):**
   Asynchronous signals delivered via system interrupt controllers (x86_64 APIC/IOAPIC, AArch64 GICv3, RISC-V PLIC, legacy PIC). Handled via mapped IRQ domains, flow policies, and bounded top-half handlers.
3. **Deferred Work (Schedulable Bottom Halves):**
   Asynchronous execution deferred outside hard-IRQ context. Managed via softirqs/tasklets for fast latency-critical work and kernel workqueues for thread-context blocking execution.

---

## Subsystem Audit & Support Matrix (PR 1 Audit)

The table below summarizes the audited implementation status, source files, controller backends, vector mappings, test evidence tiers, and known limitations across all SigmaOS interrupt paths:

| Subsystem / Arch | Source File Links | Controller Type | Mapped Vectors / Interrupts | Test Evidence Tier | Current Status & Limitations |
|---|---|---|---|---|---|
| **x86_64 IDT & Stubs** | [`src/arch/x86_64/idt.rs`](../src/arch/x86_64/idt.rs), [`src/arch/x86_64/interrupts.rs`](../src/arch/x86_64/interrupts.rs) | IDT Gate (256 entries) | Exception vectors 0–31, Timer 0x20, Syscall 0x80 | **Structural** | 256-entry IDT & assembly stubs created; `lidt` instruction prepared; requires QEMU boot validation for real vector entry. |
| **x86_64 APIC / Local APIC** | [`src/interrupt/apic_driver.rs`](../src/interrupt/apic_driver.rs), [`src/interrupt/controller.rs`](../src/interrupt/controller.rs) | Local APIC & IOAPIC | Vectors 0x20 (Timer), 0x21 (Keyboard), 0x22 (Disk), 0x23 (Net), 0xFE (Error) | **Model / Unit Tested** | Memory-mapped Local APIC & IOAPIC registration, EOI, ICR IPI dispatch, and mask/unmask operations modeled; pending QEMU hardware calibration. |
| **x86_64 TSS & Ring-3 Stack** | [`src/arch/x86_64/tss_ring3_user_mode.rs`](../src/arch/x86_64/tss_ring3_user_mode.rs) | Task State Segment (IST) | RSP0 stack switching, IST1–IST7 interrupt stacks | **Unit Tested** | TSS layout, IST stack assignment, and InterruptFrame construction verified via unit tests; pending `ltr` instruction execution in boot sequence. |
| **AArch64 Exception Vectors** | [`src/arch/aarch64/exception.rs`](../src/arch/aarch64/exception.rs), [`src/arch/aarch64/regs.rs`](../src/arch/aarch64/regs.rs) | VBAR_EL1 Table | Synchronous EL1/EL0, IRQ/FIQ, SError | **Compile-Only** | VBAR_EL1 vector table setup, ESR_EL1 syndrome decode (SVC #0, Data/Inst Aborts), and FAR_EL1 accessors implemented; pending QEMU `virt` boot validation. |
| **AArch64 GICv3 Controller** | [`src/kernel/irq/irq_controller.rs`](../src/kernel/irq/irq_controller.rs) | ARM GICv3 Distributor & Redistributor | GIC SPI/PPI/SGI IRQs | **Compile-Only** | GICC/GICD memory-mapped register definitions, IAR read, EOI write, and priority configuration implemented; pending QEMU GICv3 hardware boot. |
| **RISC-V Trap Management** | [`src/arch/riscv64/trap.rs`](../src/arch/riscv64/trap.rs) | CSR stvec / mtvec | Exception codes 0–15, S/M-mode Timer, Software, External IRQs | **Compile-Only** | mcause/scause cause decode, mepc/sepc, mtval, and mstatus MIE/SIE control implemented; pending QEMU `virt` trap entry execution. |
| **RISC-V PLIC Controller** | [`src/kernel/irq/irq_controller.rs`](../src/kernel/irq/irq_controller.rs) | Platform-Level Interrupt Controller | External IRQ sources 1–1023 | **Compile-Only** | Priority registers, pending array, enable bitmaps, context claim/complete interface implemented; pending QEMU PLIC validation. |
| **Generic IRQ Management** | [`src/kernel/interrupt.rs`](../src/kernel/interrupt.rs), [`src/kernel/interrupt_controller.rs`](../src/kernel/interrupt_controller.rs) | Generic Dispatcher | Vectors 0–255 | **Unit Tested** | Atomic vector allocation, handler registration, enable/disable toggle, and synchronous dispatch verified via unit tests (`rustc --test src/kernel/interrupt.rs`). |
| **IRQ Domain Translation** | [`src/kernel/irq/irq_domain.rs`](../src/kernel/irq/irq_domain.rs) | `IrqDomain` Mapper | Hardware IRQ (hwirq) -> Virtual IRQ (virq) | **Unit Tested** | Dynamic hardware-to-virtual IRQ mapping and lookup verified via unit tests (`rustc --test src/kernel/irq/mod.rs`). |
| **Softirq Deferred Engine** | [`src/kernel/irq/softirq.rs`](../src/kernel/irq/softirq.rs) | `SoftirqEngine` | Softirq Types: Hi, Timer, NetTx, NetRx, Block, Tasklet | **Unit Tested** | Softirq registration, atomic pending bitmask, and priority execution queue verified via unit tests (`rustc --test src/kernel/irq/mod.rs`). |
| **Kernel Workqueue Engine** | [`src/kernel/irq/workqueue.rs`](../src/kernel/irq/workqueue.rs), [`src/kernel/sigma_workqueue.rs`](../src/kernel/sigma_workqueue.rs) | `Workqueue` & `SigmaWorkqueueManager` | Schedulable work items & delayed work | **Unit Tested** | Async work enqueue, delayed execution, cancellation, and worker thread polling verified via unit tests (`rustc --test src/kernel/irq/mod.rs`). |

---

## Handler Context & Lifecycle Contracts (PR 2 Specification)

To prevent deadlocks, stack overflows, and use-after-free conditions, SigmaOS defines strict context boundaries and lifecycle requirements:

### Execution Context Contracts

| Context | Description | Memory Allocation | Locking Restrictions | Blocking Allowed? | Reentrancy Rules |
|---|---|---|---|---|---|
| **Hard-IRQ Context** | Executing in top-half ISR directly from hardware vector. | **Forbidden** (Zero transient heap allocations) | Raw spinlocks only; **must not** acquire mutexes or sleep locks. | **NO** | Reentrancy disabled on same vector; interrupts disabled during top-half execution. |
| **Threaded / Softirq Context** | Bottom-half execution outside top-half ISR. | Allowed (pre-allocated or bounded pools preferred) | Spinlocks and non-blocking read locks. | **NO** (Softirq); **YES** (Threaded Handler) | Can be preempted by Hard-IRQ context; per-CPU execution queues prevent concurrent self-reentrancy. |
| **Exception Context** | CPU-pushed trap/fault handler (e.g. Page Fault). | Allowed if fault occurs in process context. | Must observe lock hierarchy to prevent recursive fault deadlocks. | **YES** (if resolving demand page); **NO** (if in kernel critical section) | Nested exceptions permitted only for registered IST/stack guard handlers (e.g. Double Fault IST2). |
| **NMI / Critical Context** | Non-maskable interrupt or machine check exception. | **Forbidden** | Lock-free atomics only; **no** spinlocks. | **NO** | Reentrancy strictly prohibited; execution must return or trigger panic. |
| **Process Context** | Kernel task or system call context. | Fully allowed | All primitives permitted (Mutex, Semaphore, Spinlock, RWLock). | **YES** | Preemptible unless explicitly inside critical section or spinlock. |

### Lifecycle Contracts & Registration Rules

1. **Registration (`request_irq`):**
   - Returns a unique, stable `IrqHandlerCookie`.
   - Requires explicit trigger type (`Edge`, `Level`, `FastEoi`), handler function pointer, context pointer, and device string.
   - Rejects duplicate registrations on non-shared IRQs with `IRQError::AlreadyRegistered`.

2. **Synchronization & Teardown (`free_irq` / `synchronize_irq`):**
   - **Step 1 (Masking):** Mask interrupt source at the controller chip level to prevent new hardware triggers.
   - **Step 2 (In-Flight Sync):** Execute `synchronize_irq()` to wait for any active hard-IRQ handler or softirq in flight on all CPUs to complete.
   - **Step 3 (Deferred Drain):** Flush and drain pending items in associated workqueues/tasklets.
   - **Step 4 (Release):** Release handler registration and free device-associated memory cookies.
   - **Invariant:** Device state and handler cookies **must never** be freed while an ISR or deferred worker can still reference them.

3. **Nested Enable/Disable Balancing:**
   - Per-IRQ depth counter tracks `disable_irq()` and `enable_irq()` invocations.
   - Unmasking occurs at the controller level **only** when the depth counter reaches zero.

---

## Architecture Rules & Core Invariants

1. **Decoupled 3-Layer Isolation:** Architecture entry assembly, generic IRQ flow management, and controller chip operations must remain strictly separated into distinct modules.
2. **Bounded Top-Half Execution:** Hard-IRQ ISRs must complete in $O(1)$ time, acknowledge the interrupt source, capture minimal hardware state, and defer substantial processing to softirqs or workqueues.
3. **Explicit Edge vs. Level Flow Semantics:**
   - **Edge-triggered:** Acknowledge first at controller, service handler, re-check pending state before unmasking to prevent lost edges.
   - **Level-triggered:** Mask source at controller, service handler, acknowledge device, unmask source to prevent level storming.
   - **Fast-EOI:** Service handler, issue single EOI command to Local APIC / GIC / PLIC.
4. **Mapped Logical Identities:** Physical hardware IRQs (`hwirq`) must be translated to virtual kernel IRQ numbers (`virq`) using `IrqDomain` to prevent vector collisions across heterogeneous controllers.
5. **Spurious Containment & Storm Defense:** Unhandled or unknown interrupts must increment per-vector spurious counters. If unhandled interrupts exceed 100,000 counts within a 1-second window, the source must be automatically quarantined/masked and logged.
6. **SMP Synchronization & Affinity:** IRQ affinity changes must quiesce active handling queues, re-route the controller vector, and synchronize CPU targets using IPIs.

---

## Proposed 17-PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(irq): audit interrupt paths and define support matrix` | Audit x86_64 IDT/APIC, AArch64 VBAR/GICv3, RISC-V traps/PLIC, timer/IPI, and deferred work. | Support matrix published; compile-only vs booted paths clearly delineated; standalone test harnesses verified. |
| **2** | `docs(irq): define handler context and lifecycle contracts` | Define Hard-IRQ, Softirq, Exception, NMI, and Process contexts; specify `free_irq`/`synchronize_irq` contracts. | Execution context rules, memory/locking restrictions, and teardown synchronization explicitly specified. |
| **3** | `test(irq): add vector and interrupt-frame layout checks` | Add layout tests for `IdtEntry`, `TaskStateSegment`, `InterruptFrame`, stack alignment, and error-code vectors. | Layout assertions for error-code and no-error-code frames pass; stack 16-byte alignment validated. |
| **4** | `feat(irq): add generic IRQ descriptors and logical mapping` | Implement stable virtual IRQ descriptors, `IrqDomain` dynamic allocation, and registration lookups. | Dynamic hwirq-to-virq mapping verified; duplicate mappings rejected cleanly. |
| **5** | `feat(irq): define interrupt-controller chip operations` | Define `IrqChip` trait for mask, unmask, ack, EOI, trigger mode, and CPU affinity operations. | Chip backends for APIC, GICv3, and PLIC implement trait operations with explicit MMIO ordering barriers. |
| **6** | `feat(irq): implement generic edge level and fasteoi flows` | Implement flow handlers for `handle_edge_irq`, `handle_level_irq`, `handle_fasteoi_irq`, and `handle_percpu_irq`. | Flow state machines pass unit tests; edge retriggering and level masking order verified. |
| **7** | `feat(irq): connect x86_64 IDT entry to APIC dispatch` | Wire assembly stubs and IDT entries to APIC vector dispatcher; route top-half ISR execution. | QEMU x86_64 test receives timer interrupt, invokes ISR, issues Local APIC EOI once, and executes `iretq`. |
| **8** | `feat(irq): add handler registration and synchronized teardown` | Implement `request_irq`, `free_irq`, `synchronize_irq`, and nested enable/disable counters. | Concurrent IRQ trigger and deregistration stress test confirms zero use-after-free or dangling pointers. |
| **9** | `feat(irq): add spurious and unhandled interrupt containment` | Add per-vector unhandled counters, rate-limited logging, and automated storm quarantining. | Storm simulation verifies automatic source masking after rate threshold without kernel panic. |
| **10** | `feat(irq): add threaded handlers and deferred-work interface` | Connect top-half ISR trigger to threaded kernel worker thread and softirq bottom halves. | Hard-IRQ top half acknowledges device and queues threaded worker; worker executes in process context. |
| **11** | `feat(irq): add APIC timer and IPI integration tests` | Calibrate APIC timer in QEMU; validate reschedule IPI, TLB shootdown IPI, and local EOI. | APIC timer ticks monotonically; inter-processor interrupts arrive on target CPU core with clean EOI. |
| **12** | `feat(irq): add IRQ affinity and CPU-hotplug policy` | Implement IRQ mask affinity steering and CPU offline queue quiescing. | Moving IRQ affinity during active interrupt generation safely drains pending work without lost IRQs. |
| **13** | `feat(irq): validate AArch64 GICv3 dispatch on QEMU` | Wire VBAR_EL1 vectors and GICv3 IAR/EOI registers on QEMU `virt` machine. | QEMU AArch64 boots, receives generic timer IRQ, invokes handler, performs EOI via GICC/ICC_EOIR1_EL1. |
| **14** | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Wire stvec trap entry and PLIC claim/complete registers on QEMU `virt` machine. | QEMU RISC-V boots, receives SBI timer and external PLIC IRQ, performs claim and complete successfully. |
| **15** | `test(irq): add fault-injection and interrupt-storm suite` | Simulate missing EOI, storming level IRQs, registration races, and queue saturation. | Harness detects hangs, missing EOIs, and queue overflows; provides regression test coverage. |
| **16** | `feat(irq): add interrupt statistics and trace interface` | Implement per-CPU, per-vector `/proc/interrupts` style statistics and rate-limited tracing. | Lock-free atomic counters expose IRQ rates, handler duration, and spurious counts without hard-IRQ allocations. |
| **17** | `docs(irq): publish driver guide and architecture support matrix` | Publish complete driver authoring guide, API reference, context rules, and hardware support matrix. | Documentation published; all supported claims linked to reproducible QEMU test logs. |

---

## Operational Playbooks & Maintenance Cadence

| Cadence | Operational Task | Required Evidence & Deliverables |
|---|---|---|
| **Every PR** | CI Architecture Vector & Flow Validation | Build verification, frame layout unit tests, flow handler state tests, and target QEMU run logs. |
| **Weekly** | Unhandled Vector & Regression Triage | Triage report identifying any vector leaks, spurious IRQ spikes, or deferred work latency regressions. |
| **Monthly** | Handler Teardown & Lifetime Audit | Code audit of all `free_irq` and device removal paths to confirm proper `synchronize_irq` invocation. |
| **Quarterly** | Multi-Core SMP & Hotplug Matrix | QEMU multi-core test matrix verifying IRQ affinity steering and CPU core online/offline migration. |
| **Release Candidate** | Final Support Matrix Certification | Update `FEATURE_STATUS.toml` and documentation with verified QEMU target architectures and hardware test results. |

---

## Component-Specific Implementation Rules

1. **Architecture Entry Stubs:** Must save all caller-saved registers (`rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8`–`r11` on x86_64), adjust stack for 16-byte alignment prior to C/Rust ABI function calls, and restore exact state prior to `iretq`/`eret`/`sret`.
2. **Interrupt Descriptors:** IDT entries must use gate type `0xE` (64-bit Interrupt Gate) to ensure hardware clears IF (Interrupt Flag) automatically on entry.
3. **Controller Memory Ordering:** MMIO register writes to Local APIC, GICv3, or PLIC must be accompanied by explicit memory barriers (`mfence` on x86_64, `dsb sy` on ARM64, `fence` on RISC-V).
4. **Diagnostics & Logging:** Hard-IRQ ISRs **must never** invoke formatting or logging code that allocates memory or acquires non-atomic locks. Rate-limited atomic counters must be used instead.

---

## Completion Criteria Gates

- **Prototype Level:** Layouts, descriptors, and flow handler state machines are unit-tested in isolation (`rustc --test`); no booted execution proven.
- **Integrated Level:** Target QEMU machine boots, receives real hardware/timer interrupts, invokes registered ISR, performs controller EOI, and returns safely to interrupted context.
- **Supported Level:** Comprehensive QEMU and physical hardware test suite confirms multi-core execution, affinity steering, storm quarantining, and clean teardown under stress.

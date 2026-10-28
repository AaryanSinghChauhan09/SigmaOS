# Preemption Policy Specification

## Overview
Preemption defines when and how a currently `RUNNING` process can be suspended to allow a higher-priority or more deserving task to run on the CPU core.

## Rules & Principles
1. **Time-Slice Expiry**:
   - Decremented at each timer interrupt.
   - When remaining quantum reaches zero, `need_resched` flag is set and task yields CPU on interrupt return path.

2. **Interrupt-Driven Preemption**:
   - Waking a higher-priority task during an interrupt handler (e.g., disk I/O complete, network packet arrived) triggers immediate preemption if target priority > current running task priority.

3. **Voluntary Yield**:
   - Tasks may explicitly relinquish CPU time via `sys_yield()`, placing task back in `READY` queue at default quantum.

4. **Nested Priority & Inversion Scenarios**:
   - Priority inheritance Protocol (PIP): Low-priority task holding a resource required by high-priority task inherits high priority temporarily until resource release.

5. **Context Switch Restrictions**:
   - Preemption is explicitly disabled during IRQ handlers, spinlock critical sections, and low-level context switch execution.

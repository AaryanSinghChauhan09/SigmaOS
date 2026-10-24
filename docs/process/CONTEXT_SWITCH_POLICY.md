# SigmaOS Context Switch Policy Specification

## Context Switch Overview
Context switching is the low-level mechanism that saves the execution state of a currently running process and restores the execution state of the next process selected by `schedule_next`.

## Context Switch Sequence & Integrity Rules

### 1. Register Preservation
- Callee-saved general-purpose registers (x86_64: `rbx`, `rsp`, `rbp`, `r12`-`r15`; ARM64: `x19`-`x29`, `sp`) must be saved to the kernel task stack.
- The instruction pointer (`rip` / `pc`) is saved as part of the call/return sequence.

### 2. Stack Switch
- Kernel stack pointer is updated in the Thread Local Storage (TLS) or CPU Control Block.
- Stack boundary assertions prevent kernel stack overflow.

### 3. Address Space Switch
- Page table directory base pointer (CR3 on x86_64, TTBR0_EL1 on ARM64) is switched if `next_process.mm != current_process.mm`.
- Kernel space mappings remain identical across all processes.
- TLB entries are selectively invalidated or preserved using PCID (Process Context Identifiers).

### 4. FPU / Extended State Handling
- Floating-point, SSE, AVX, and NEON registers are lazily switched or saved via `xsave` / `xrstor` to minimize context switch overhead.

### 5. Interrupt & Preemption Safety
- Context switches must occur with interrupts disabled or within an atomic preempt-disabled region (`preempt_disable()`).
- Double-switch prevention checks ensure a process is never context-switched onto two CPUs simultaneously.

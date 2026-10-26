# SigmaOS Context Switch Policy Specification

## Purpose
This document specifies register retention, stack transitions, page table switches, and safety guarantees during CPU context switching.

---

## 1. Register Save & Restore
- **General-Purpose Registers**: Callee-saved registers (e.g., `rsp`, `rbp`, `rbx`, `r12`-`r15` on x86_64) are saved onto the process kernel stack.
- **Instruction Pointer**: The return address (`rip`) is pushed onto the stack during context switch entry and restored during `ret`/`iret`.

## 2. Address Space Switch
- **Page Table Pointer**: If switching to a task in a different address space, the memory management unit page table register (CR3 on x86_64) is updated.
- **Kernel Space Mapping**: Kernel higher-half mappings remain identical across all task page tables.

## 3. Vector & FPU State Handling
- Extended state (`XSAVE`/`XRSTOR` or `FXSAVE`/`FXRSTOR`) is saved lazily or on demand upon first floating-point instruction usage in the new context.

## 4. Interrupt and Preemption Safety
- **Interrupt Guarding**: Context switches execute with interrupts disabled to prevent re-entrant schedule calls.
- **Active PID Tracking**: The per-CPU active PID table (`active_pids[cpu_id]`) is updated atomically prior to stack switch.
- **Double-Switch Prevention**: A core must not initiate a context switch if a switch is already in progress.

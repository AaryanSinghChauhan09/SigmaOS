# SigmaOS Context Switch Policy

## Context Switch Invariants

1. **Register Save and Restore**:
   Context switches preserve callee-saved registers (x86_64: `rbx`, `rsp`, `rbp`, `r12`-`r15`, `rip`; ARM64: `x19`-`x29`, `sp`, `pc`) and instruction pointer context.

2. **Stack and Address Space Switching**:
   Switching updates stack pointers (`RSP`) and page tables (`CR3` page directory base) atomically when transitioning between distinct process address spaces.

3. **FPU / Vector State Handling**:
   FPU/SSE/AVX register state is lazily saved or saved on demand before switching CPU execution contexts.

4. **Interrupt Safety**:
   Context switching must occur with local interrupts disabled (or under spinlock protection) to prevent interrupt handler recursion during context swap.

5. **Preemption Safety**:
   Preemption is disabled during context switch entry and re-enabled only after the target task's context is restored.

6. **Double-Switch Prevention**:
   A context switch check verifies target task state (`READY` -> `RUNNING`) and prevents double-switching or scheduling dead tasks.

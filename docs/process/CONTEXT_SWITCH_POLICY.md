# SigmaOS Context Switch Policy (`CONTEXT_SWITCH_POLICY.md`)

## 1. Overview
This policy defines the register preservation, stack swapping, address-space switching, and state consistency contracts during thread/process context switches in SigmaOS.

## 2. Saved State Contract
During a context switch, the following CPU context components must be preserved across switches:
- **General Purpose Registers**: `rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rbp`, `r8`-`r15` on x86_64.
- **Instruction Pointer**: `rip` restored to the yield/preemption resume point.
- **Stack Pointer**: `rsp` switched atomically to target thread's kernel stack.
- **Page Table Base (CR3)**: Switched if target process belongs to a distinct address space.
- **FPU/SSE/AVX State**: Saved/restored via `fxsave`/`fxrstor` or lazy FP state restore.

## 3. Interrupt & Preemption Restrictions
- Context switches MUST occur with local interrupts disabled (`cli`) or inside an explicit preemption-safe critical section.
- Double-switch prevention: A task that is already `RUNNING` on CPU $A$ must never be scheduled simultaneously on CPU $B$.

## 4. Lost-Wakeup Prevention
- If an interrupt triggers a wakeup event during context switch entry, the `wake_event_pending` flag ensures the task immediately resumes execution upon returning to `schedule_next`.

# SigmaOS Context Switch Policy

## Register and Memory Preservation

1. **General Purpose Registers**: Saved and restored across task switches (`rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rbp`, `r8`-`r15`).
2. **Stack Switch**: Target task's kernel stack pointer (`rsp`) loaded during context switch.
3. **Address Space Switch**: Page table root (`cr3`) updated if switching between processes with different address spaces.
4. **FPU / Vector State**: Deferred FXSAVE/XSAVE or eager floating point context restoration.

## Concurrency and Preemption Safety

1. **Interrupt Safety**: Context switches must occur with interrupts disabled or within managed kernel preemption points.
2. **Double-Switch Prevention**: Active CPU PID tracking ensures a task is never assigned to two CPU cores simultaneously.
3. **Scheduler Consistency**: Process state must be atomically transitioned before restoring registers for the new task.

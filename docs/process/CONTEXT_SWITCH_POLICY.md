# SigmaOS Context Switch Policy Specification

## Architectural Contract
- **Register Save/Restore**: Saved architecture context (callee-saved general-purpose registers, stack pointer `rsp`, instruction pointer `rip`, flags `rflags`) must be pushed onto task kernel stack.
- **Address-Space Switch**: Page table base register (`CR3`) switch performed when switching across distinct process address spaces.
- **FPU/Vector State**: Lazy or explicit XSAVE/XRSTOR restores AVX/FPU registers upon context switch into user space.
- **Interrupt Safety**: Context switches must execute with interrupts disabled or in a guarded critical section to prevent nested interrupt context corruptions.
- **State Consistency**: Scheduler state transition (`RUNNING` -> `READY` / `SLEEPING`) must be atomically completed before releasing scheduler locks.

# Context Switch Policy Specification

## Overview
Context switching safely shifts CPU execution from one process context to another while preserving register integrity, stack validity, address space isolation, and scheduler consistency.

## Rules & Requirements
1. **Register Save / Restore**:
   - Save general-purpose registers (RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8-R15), instruction pointer (RIP), and stack pointer (RSP) to PCB context frame.
   - Restore target task's saved register frame.

2. **Stack & Address-Space Switch**:
   - Switch kernel stack pointer (RSP0) in TSS / CPU local store.
   - Switch Page Directory Base Register (CR3) if context switch crosses process boundaries.

3. **FPU & Vector State Handling**:
   - Lazy FPU state switching using CR0.TS or eager XSAVE / XRSTOR instructions to prevent vector register corruption across tasks.

4. **Interrupt and Preemption Safety**:
   - Context switch routine executes with interrupts disabled or within critical preemption-disabled sections.
   - Disallows preemption while holding run queue locks or during register state save/restore.

5. **Double-Switch Prevention**:
   - Ensures atomic process state updates (`READY` -> `RUNNING`, `RUNNING` -> `READY`/`SLEEPING`) before releasing scheduler lock.

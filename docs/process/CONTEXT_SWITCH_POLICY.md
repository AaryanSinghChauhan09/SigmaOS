# SigmaOS Context Switch Policy Specification

## Context Switch Guarantees
1. **Register State Integrity**:
   - General-purpose registers (RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8-R15, RSP, RIP, RFLAGS) saved and restored across task switches.
   - Vector / FPU registers (XMM0-XMM15, MXCSR, FPU control word) saved via XSAVE / FXSAVE.

2. **Address Space Isolation**:
   - Switching CR3 page directory base register updates hardware MMU page tables.
   - Kernel memory space is shared across all address spaces; user memory space is strictly isolated.

3. **Interrupt and Preemption Safety**:
   - Interrupts disabled during context switch sequence to avoid corrupted stack states.
   - Preemption disabled during core scheduler state updates.

4. **Double-Switch & Lost-Wakeup Prevention**:
   - `wake_event_pending` flag ensures signals or wakeups arriving during switch entry prevent task from sleeping indefinitely.

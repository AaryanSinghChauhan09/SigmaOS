# SigmaOS Context Switch Policy

## Context Switch Guarantees

1. **Register Integrity**:
   - Save and restore general-purpose registers (`rax`, `rbx`, `rcx`, `rdx`, `rsi`, `rdi`, `rbp`, `r8-r15`), instruction pointer (`rip`), and stack pointer (`rsp`).

2. **Address Space Isolation**:
   - Reload page directory / page table base register (`CR3`) when context switching between processes in different address spaces.

3. **FPU & Extended State**:
   - Lazy or explicit saving of SSE/AVX extended state (`fxsave` / `xsave`).

4. **Lost-Wakeup Prevention**:
   - When transitioning `RUNNING -> SLEEPING`, check `wake_event_pending` and `pending_signals`. If set, convert transition to `READY` immediately without relinquishing CPU.

5. **Double-Switch Protection**:
   - Active process tracking (`active_pids[cpu]`) prevents scheduling the same running process on multiple cores simultaneously.

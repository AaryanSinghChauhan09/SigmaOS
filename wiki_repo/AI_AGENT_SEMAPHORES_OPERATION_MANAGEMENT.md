# AI Agent Semaphores Operation Management Guide

## Overview
This wiki guide details semaphores operation management protocols for AI coding agents operating on SigmaOS. It covers atomic wait (`P` / `down` / `sys_semop`), signal (`V` / `up` / `sys_semctl`), `SEM_UNDO` auto-reversal, kernel wait queue wakeups, and priority inheritance unblocking.

## Key Operational Principles
1. **Atomic Wait (`P`)**: Decrements counter. If counter drops below 0, caller transitions to `Blocked(BlockReason::SemaphoreWait)`.
2. **Atomic Signal (`V`)**: Increments counter up to `max_value` and wakes up the highest priority blocked thread in the queue.
3. **`SEM_UNDO` Guardrail**: System V IPC semaphores track process undo adjustments to automatically revert semaphore state upon unexpected process exit.

## Related Documents
- `docs/AI_AGENT_SEMAPHORES_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_SEMAPHORES_OPERATION_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENT_SEMAPHORES_MANAGEMENT.md`

# SigmaOS Signals Development and Maintenance Plan

## Architectural Baseline and Principles

SigmaOS implements a high-reliability signal delivery and management framework. The design builds upon POSIX.1-2024 standards while drawing key implementation insights from Linux 6.x and FreeBSD 14.x signal subsystems.

### Core Architectural Principles

1. **Strict Decoupling of Process and Thread State**:
   - Process signal state contains shared actions (`SigAction`), process pending signal queues, and parent/child notification settings.
   - Thread signal state contains thread-specific blocked masks (`sigset_t`), thread pending signal queues, and stack state (`stack_t`).

2. **Kernel Enforcement of Non-Bypassable Invariants**:
   - `SIGKILL` and `SIGSTOP` cannot be blocked, caught, ignored, or queued with custom handlers.
   - User frames delivered onto user stacks are strictly validated during `sigreturn` to prevent ring-0 privilege escalation or execution flow hijack.

3. **Async-Signal-Safety Assurance**:
   - Kernel delivery paths execute in lock-free or minimal spinlock-guarded critical sections without dynamic memory allocations on the signal path.
   - User-space applications are instructed to execute only async-signal-safe primitives inside signal handlers.

## 19-PR Sequence Roadmap

1. `PR 1: Define signal model and supported profile`
2. `PR 2: Implement sigset_t bitmask operations and uncatchable signal guards`
3. `PR 3: Implement process and thread signal state containers`
4. `PR 4: Implement sigaction disposition registry with permission checks`
5. `PR 5: Implement safe user-copy wrappers for signal syscalls`
6. `PR 6: Implement kill and raise signal delivery routing`
7. `PR 7: Implement per-thread pending queues and signal selection logic`
8. `PR 8: Implement default action handler (Terminate, Core, Stop, Continue, Ignore)`
9. `PR 9: Construct x86_64 UserSignalFrame on Ring 3 stack`
10. `PR 10: Implement sys_rt_sigreturn context restoration and frame validation`
11. `PR 11: Support sa_mask blocking, SA_NODEFER, and SA_RESETHAND flags`
12. `PR 12: Implement syscall interruption and SA_RESTART restartability`
13. `PR 13: Support siginfo_t generation and SA_SIGINFO three-argument handlers`
14. `PR 14: Implement sigaltstack and SA_ONSTACK alternate signal stacks`
15. `PR 15: Integrate SIGCHLD lifecycle notifications with waitpid reaping`
16. `PR 16: Implement synchronous signal waiting (sigwait, sigtimedwait)`
17. `PR 17: Support real-time signal range (SIGRTMIN..SIGRTMAX) and FIFO queuing`
18. `PR 18: Connect Linux-compatible signalfd event-loop integration`
19. `PR 19: Comprehensive fault-injection, frame corruption, and concurrency test suite`

## Maintenance Cadences and Review Gates

### Periodic Task Schedule

- **Every Commit / PR**: Validate `sigset_t` bitmask invariants, `sigaction` restrictions, frame layout offsets, and execute unit test suite.
- **Weekly**: Audit signal queue memory caps and verify no memory leaks occur during task termination with pending signals.
- **Monthly**: Conduct `sigreturn` frame forgery and corrupted stack fuzz testing.
- **Quarterly**: Benchmark signal dispatch latency and syscall interruption overhead against Linux/FreeBSD baselines.

### PR Review Checklist

- [ ] Does this PR modify signal numbers, `sigset_t`, or `UserSignalFrame` layouts?
- [ ] Are `SIGKILL` and `SIGSTOP` prevented from being caught or blocked?
- [ ] Is user stack alignment (16-byte) verified before writing frames?
- [ ] Does `sigreturn` validate canonical addresses and privilege flags (`RFLAGS`)?
- [ ] Are all new paths covered by unit and integration tests?

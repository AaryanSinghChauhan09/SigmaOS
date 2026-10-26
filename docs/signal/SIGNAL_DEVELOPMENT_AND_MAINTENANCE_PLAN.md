# SigmaOS Signal Development and Maintenance Plan

This document specifies the SigmaOS Signal Subsystem Development and Maintenance Plan, defining architectural policies, PR implementation sequence, maintenance cadences, and review gates.

## Subsystem Architecture & Directives

1. **POSIX Baseline**: Adhere strictly to POSIX.1-2024 signal specification (`sigaction`, `sigprocmask`, `kill`, `raise`, `sigpending`, `sigsuspend`, `sigwait`).
2. **Kernel Enforcement**:
   - `SIGKILL` (9) and `SIGSTOP` (19) are strictly uncatchable, unignorable, and unblockable.
   - User signal frame construction and `sigreturn` context restoration perform strict canonical address and register sanitization.
3. **Threading Model**:
   - Process dispositions and process-pending queues are maintained on `ProcessSignalState`.
   - Thread blocked masks and thread-pending queues are maintained on `ThreadSignalState`.
4. **Interruption & Restart**:
   - Interrupted blocking system calls return `EINTR` unless configured with `SA_RESTART`.
5. **Linux/FreeBSD Extensions**:
   - `signalfd` module provides pollable file descriptor signal notification.

## 19-PR Implementation Roadmap

- **PR 1**: `docs(signal): define signal model and supported profile`
- **PR 2**: `test(signal): add signal ABI and status inventory checks`
- **PR 3**: `refactor(signal): separate generation pending selection and delivery`
- **PR 4**: `feat(signal): implement signal sets and thread masks`
- **PR 5**: `feat(signal): implement sigaction with safe user-copy`
- **PR 6**: `feat(signal): implement process-directed and thread-directed generation`
- **PR 7**: `feat(signal): deliver default terminate stop and continue actions`
- **PR 8**: `feat(signal): deliver handlers through validated user signal frames`
- **PR 9**: `feat(signal): add sigreturn and context validation`
- **PR 10**: `feat(signal): add pending and synchronous wait interfaces`
- **PR 11**: `feat(signal): define blocking syscall interruption and restart rules`
- **PR 12**: `feat(signal): implement SIGCHLD and child lifecycle integration`
- **PR 13**: `feat(signal): add process-group and terminal-generated signals`
- **PR 14**: `feat(signal): add alternate signal stack support`
- **PR 15**: `feat(signal): add queued real-time signals with resource limits`
- **PR 16**: `feat(signal): integrate signalfd as a pollable FD extension`
- **PR 17**: `feat(libc): align signal wrappers and constants with kernel ABI`
- **PR 18**: `test(signal): add QEMU stress and race regression suite`
- **PR 19**: `docs(signal): publish signal reference and maturity matrix`

## Maintenance Cadences & Quality Gates

- **CI Verification**: Standalone unit tests (`rustc --test src/signal/mod.rs` and `rustc --test src/signal/sovereign_signal_development_and_pr_suite.rs`) must pass cleanly on all PRs.
- **Security Audit**: Frame forgery detection, privilege elevation prevention, and signal queue bounds checking.
- **Inventory Synchronization**: `FEATURE_STATUS.toml` must accurately document subsystem test evidence and next milestones.

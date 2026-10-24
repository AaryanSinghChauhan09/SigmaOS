# SigmaOS Signal Subsystem Development and Maintenance Plan

This document defines the development roadmap, maintenance guidelines, and PR review gates for the SigmaOS Signal Subsystem, adhering to NIST SP 800-218 (SSDF) and POSIX.1-2024 standards.

## 1. Governance & Alignment Baseline

- **Standard Baseline:** POSIX.1-2024 (`sigaction`, `sigprocmask`, `sigpending`, `sigsuspend`, `sigwait`, `sigaltstack`).
- **Extension References:** Linux 6.x (`signalfd`, `rt_sigaction`, `SA_RESTART`) and FreeBSD 14.x (`kqueue` EVFILT_SIGNAL).
- **Subsystem Inventory:** Tracked in `FEATURE_STATUS.toml` under `[kernel.signal_handler]` and `[kernel.signal_development_and_maintenance_plan]`.

## 2. Architectural Rules & Invariants

1. **Uncatchable Signals:** `SIGKILL` and `SIGSTOP` cannot be blocked, caught, or ignored.
2. **Process vs. Thread Targeting:** Dispositions are per-process; blocked masks and pending states are per-thread.
3. **Async-Signal Safety:** Kernel frame creation and signal delivery paths must not dynamically allocate memory or invoke blocking synchronization.
4. **Frame Validation:** `sigreturn` must validate that the restored stack pointer and instruction pointer reside within canonical user space memory before returning to Ring 3.
5. **No False Maturity Claims:** A passing model or unit test indicates `Prototype` status. Real Ring 3 user execution and trampoline return are required for `Integrated` status.

## 3. PR Sequence Overview (PRs 1 to 18)

1. `docs(signal-handler): audit signal path and define ABI profile`
2. `test(signal-handler): add ABI and signal-frame layout checks`
3. `feat(signal): connect signal syscalls to kernel dispatch`
4. `feat(signal): add per-thread masks and pending signal queues`
5. `feat(signal-handler): deliver default actions at safe return points`
6. `feat(signal-handler): construct validated x86_64 user signal frame`
7. `feat(signal-handler): implement x86_64 sigreturn validation`
8. `feat(signal-handler): implement action masks and restart semantics`
9. `feat(signal-handler): add siginfo and SA_SIGINFO subset`
10. `feat(signal-handler): add alternate stack support`
11. `feat(signal): integrate SIGCHLD and wait/reap lifecycle`
12. `feat(signal): add synchronous signal-wait APIs`
13. `feat(signal): add queued real-time signal support with limits`
14. `feat(signal): integrate signalfd as Linux-specific extension`
15. `feat(libc): expose signal APIs and safe handler examples`
16. `test(signal-handler): add fault-injection and concurrency suite`
17. `feat(signal-handler): validate AArch64 and RISC-V handler frames`
18. `docs(signal-handler): publish API, safety, and support matrix`

## 4. Maintenance Cadence and Review Templates

### Weekly Cadence
- Reconcile feature status in `FEATURE_STATUS.toml`.
- Triage any handler delivery regressions or memory leak reports.

### Monthly Security Audit
- Review `sigreturn` register validation and privilege isolation.
- Verify user-copy bounds checks and canonical address restrictions.

### PR Review Gate Checklist
```text
- [ ] `sigaction` semantics and ABI are explicit
- [ ] SIGKILL/SIGSTOP cannot be blocked or caught
- [ ] Frame/return state is validated against canonical user bounds
- [ ] Handler path does not trust user-controlled privileged state
- [ ] Async-signal-safe guidance is documented
- [ ] Required CI passes (`rustc --test src/signal/mod.rs`)
```

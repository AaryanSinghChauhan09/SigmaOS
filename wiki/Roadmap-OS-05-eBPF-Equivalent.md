# SigmaOS Roadmap: SigmaProbe (eBPF-Equivalent)
Dynamic kernel tracing and network filtering without kernel rebuilds.
## Goals
- Safe bytecode VM executing in kernel context
- Attach probes to IPC, syscall, and network events
## Key Milestones
- [ ] Minimal register-based bytecode VM
- [ ] Verifier pass (bounds checking)
- [ ] Map types: hash, array, ring buffer
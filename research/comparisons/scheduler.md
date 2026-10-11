# Scheduler Mechanism Comparison Matrix

| Project | Scheduler Model | Run Queue Data Structure | Context Switch Mechanism | Preemption |
|---|---|---|---|---|
| **xv6** | Round-robin | Array loop | `swtch()` assembly | Timer interrupt |
| **Linux** | CFS / EEVDF / BORE | Red-black tree / BPF sched-ext | `switch_to()` / register save | Tick & tickless preemption |
| **FreeBSD** | ULE scheduler | Per-CPU run queues | `cpu_switch()` | Hardware timer interrupt |
| **SigmaOS** | Fixed-priority round-robin | Per-CPU lock-free ring queue | Ring 0 stack context switch | HPET/APIC timer preemption |

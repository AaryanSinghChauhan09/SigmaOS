# Scheduling Branch Wiki (S-SCHED)

This knowledge base governs the development of the Sovereign Scheduler within the `scheduling` branch.

## Focus Areas

- **Process Scheduling Algorithms**: Round-Robin, Completely Fair Scheduler (CFS) equivalents, and strict Priority-Based executions.
- **Shard-Aware Policies**: Ensuring high-priority system shards never yield to low-priority userland shards.
- **Hardware Interrupts**: Managing preemption via the APIC timer and handling complex hardware IRQs.

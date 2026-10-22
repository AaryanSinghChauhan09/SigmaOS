# Comparative Analysis: Task Scheduling

| Metric / Feature | Linux | xv6 | Redox | SigmaOS Target |
|---|---|---|---|---|
| Algorithm | EEVDF / CFS | Round-Robin | Priority Round-Robin | EEVDF with starvation boost |
| Preemption | Preemptible Kernel | Timer Interrupt | Preemptible | Preemptible timer ticks |
| SMP Load Balancing | Migration Queues | Per-CPU Lock | Per-CPU Queues | Bounded Run-Queue per CPU |

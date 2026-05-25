# Hybrid Sovereign Scheduler

Extends SigmaOS's Round-Robin and EDF schedulers with:
- **NUMA awareness** — tasks affined to local memory nodes
- **Real-time (RT) lanes** — hard deadlines guaranteed
- **Energy efficiency** — frequency scaling for idle workloads
- **AI prediction** — ML model pre-fetches task requirements

## Class Hierarchy
```
SovereignScheduler
  ├─ RTLane      (hard real-time, EDF)
  ├─ NUMAFair    (NUMA-aware CFS analogue)
  └─ EcoLane     (battery/power optimised)
```

## Roadmap
- [ ] RTLane preemption guarantees
- [ ] NUMA topology detector
- [ ] CPU frequency governor integration

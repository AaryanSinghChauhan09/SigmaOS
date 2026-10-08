# Scheduler Component Agent

## Component Overview
The CPU scheduler is responsible for process scheduling, CPU time allocation, and interactivity optimization.

## Linux Inspiration
- **CFS (Completely Fair Scheduler)**: Red-black tree based, vruntime tracking
- **EEVDF (Earliest Eligible Virtual Deadline First)**: Modern scheduler in Linux 6.6+
- **BORE (Burst-Oriented Response Enhancer)**: Interactivity-boosted scheduler
- **Real-time schedulers**: SCHED_FIFO, SCHED_RR, SCHED_DEADLINE

## BSD Inspiration
- **FreeBSD ULE**: Interactivity-focused with CPU affinity
- **OpenBSD SCHED_*RTIME*: Simple, deterministic real-time scheduling
- **NetBSD SCHED_M2**: Multi-queue scheduler with NUMA awareness

## Current SigmaOS Status
- Partial implementation in `src/scheduler/` directory
- EEVDF scheduler implemented with unit tests
- BORE scheduler implemented for interactivity
- CFS scheduler stub implemented
- Real-time schedulers (FIFO/RR) implemented

## Critical Missing Features
1. **NUMA-Aware Scheduling**: CPU topology awareness and memory locality
2. **CPU Affinity**: Pin processes to specific CPUs/cores
3. **SMP Load Balancing**: Automatic load distribution across cores
4. **Deadline Scheduling**: SCHED_DEADLINE for real-time guarantees
5. **CPU Hotplug**: Dynamic CPU add/remove support
6. **Energy-Aware Scheduling**: Power-state aware scheduling (P-states/C-states)
7. **Cgroup Scheduling**: Process group scheduling limits
8. **Real-time Throttling**: Prevent real-time tasks from starving system

## Implementation Priority
1. **HIGH**: NUMA-aware scheduling and CPU affinity
2. **HIGH**: SMP load balancing
3. **MEDIUM**: Deadline scheduling (SCHED_DEADLINE)
4. **MEDIUM**: CPU hotplug support
5. **LOW**: Energy-aware scheduling with power profiles

## Key Files to Create/Improve
- `src/scheduler/numa.rs` - NUMA topology and memory locality
- `src/scheduler/affinity.rs` - CPU affinity and core pinning
- `src/scheduler/load_balance.rs` - SMP load balancing
- `src/scheduler/deadline.rs` - SCHED_DEADLINE implementation
- `src/scheduler/hotplug.rs` - CPU hotplug support
- `src/scheduler/energy.rs` - Power-state aware scheduling

## Testing Strategy
- Scheduler stress testing with many processes
- Real-time scheduling latency measurement
- NUMA locality benchmarking
- Load balancing fairness testing
- CPU hotplug stress testing

## Dependencies
- CPU topology detection (ACPI MADT)
- Timer subsystem (HPET/TSC)
- Power management (ACPI P-states/C-states)
- Memory management (NUMA-aware allocation)

## Success Criteria
- Fair CPU time distribution across processes
- Low latency for interactive tasks (BORE)
- Real-time deadline guarantees
- Efficient NUMA memory locality
- Dynamic CPU hotplug without crashes
- Energy-efficient scheduling on mobile

## Open Source Competitors Analysis
- **Linux CFS**: Most mature, complex but proven
- **FreeBSD ULE**: Excellent interactivity focus
- **Windows NT Scheduler**: Priority-based with quantum decay
- **z/OS Scheduler**: Mainframe-grade workload management

## Future Enhancements
- Machine learning-based scheduling predictions
- Heterogeneous computing (big.LITTLE, P-cores/E-cores)
- GPU-accelerated scheduling decisions
- Scheduler performance counters and profiling
- Hybrid scheduling (preemptive + cooperative)

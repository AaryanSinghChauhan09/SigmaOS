# SigmaOS Kernel Scheduler Documentation

## Overview

SigmaOS uses the **EEVDF (Energy-Efficient Virtual Deadline-First)** scheduler as its primary process scheduler, inspired by CachyOS optimizations and the upstream Linux EEVDF implementation.

## Scheduler Hierarchy

| Class | Policy | Use Case | Priority |
|-------|--------|----------|----------|
| DL | SCHED\_DEADLINE | Audio/Video | Highest |
| RT | SCHED\_FIFO/RR | Real-time tasks | High |
| EEVDF | SCHED\_NORMAL | Normal tasks | Normal |
| IDLE | SCHED\_IDLE | Background | Lowest |
| BPF | sch\_ext | Custom policies | Flexible |

## EEVDF Configuration

```bash
# View current scheduler config
cat /proc/sigma/scheduler/config

# Tune EEVDF parameters
echo 4000000 > /sys/kernel/debug/sched/min_granularity_ns  # 4ms
echo 6000000 > /sys/kernel/debug/sched/wakeup_granularity_ns
echo 24000000 > /sys/kernel/debug/sched/latency_ns

# Enable migration throttle (NUMA)
echo 1 > /sys/kernel/debug/sched/migration_cost
```

## BORE Scheduler (Optional)

SigmaOS optionally supports the BORE (Burst-Oriented Response Enhancer) scheduler:

```bash
# Switch to BORE at runtime
sigma-kernel sched set bore

# Configure burst penalty
echo 2 > /sys/kernel/sched/bore/burst_penalty_scale
```

## sched\_ext (BPF Scheduler)

SigmaOS fully supports `sched_ext`, allowing user-space BPF schedulers:

```bash
# Load a custom BPF scheduler
sigma-kernel sched load /usr/lib/sigma/schedulers/scx_rustland

# Available schedulers
sigma-kernel sched list
# scx_simple    - Simple vruntime scheduler
# scx_rustland  - Rust-based latency-optimized
# scx_lavd     - Latency-aware virtual deadline
# scx_bpfland  - Hybrid BPF/CFS scheduler
```

## CPU Power Management

```bash
# Set power profile
sigma-power performance   # Maximum performance
sigma-power balanced      # Default balanced
sigma-power powersave     # Maximum battery life
sigma-power ai-optimized  # AI-driven adaptive

# View current freq
cpufreq-info

# Force specific governor
cpupower frequency-set -g schedutil
```

## Real-Time Scheduling

```bash
# Set RT priority for a process
chrt -f 50 ./my-realtime-app

# Check RT limits
cat /proc/sys/kernel/sched_rt_runtime_us

# Enable full RT (disable throttling)
echo -1 > /proc/sys/kernel/sched_rt_runtime_us
```

## cgroups v2 Integration

```bash
# Create a CPU-limited cgroup
mkdir /sys/fs/cgroup/myapp
echo "500000 1000000" > /sys/fs/cgroup/myapp/cpu.max  # 50% CPU
echo $PID > /sys/fs/cgroup/myapp/cgroup.procs

# Memory limit
echo "512M" > /sys/fs/cgroup/myapp/memory.max
```

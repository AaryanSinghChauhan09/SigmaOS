# Cgroups v2 Memory Controller

SigmaOS implements Cgroups v2 (Control Groups version 2) memory controller to provide fine-grained memory resource management and isolation for processes and containers.

## Overview

Cgroups v2 memory controller enables:
- Memory limits (hard and soft limits)
- OOM (Out of Memory) control
- Memory swap accounting
- Memory pressure monitoring
- Page cache control
- NUMA memory allocation policies

## Architecture

### Cgroups v2 Hierarchy
```
/sys/fs/cgroup/
├── sigmaos/
│   ├── memory.max
│   ├── memory.swap.max
│   ├── memory.oom.group
│   ├── memory.high
│   ├── memory.stat
│   ├── memory.pressure
│   └── my-app/
│       ├── memory.max
│       └── memory.stat
```

### Memory Accounting
- **RSS (Resident Set Size)**: Physical memory used
- **Cache**: Page cache memory
- **Swap**: Swap space used
- **Inactive**: Inactive memory pages
- **Active**: Active memory pages

## Implementation

### Memory Controller
```rust
// src/kernel/cgroup/memory.rs
pub struct MemoryController {
    root: Cgroup,
    cgroups: BTreeMap<String, MemoryCgroup>,
    oom_handler: OomHandler,
    pressure_monitor: PressureMonitor,
}

#[derive(Debug, Clone)]
pub struct MemoryCgroup {
    pub name: String,
    pub parent: Option<String>,
    pub limits: MemoryLimits,
    pub stats: MemoryStats,
    pub processes: BTreeSet<pid_t>,
}

#[derive(Debug, Clone)]
pub struct MemoryLimits {
    pub max: Option<u64>,         // Hard limit
    pub high: Option<u64>,         // Soft limit
    pub swap_max: Option<u64>,     // Swap limit
    pub oom_group: bool,           // Kill entire group on OOM
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub rss: AtomicU64,
    pub cache: AtomicU64,
    pub swap: AtomicU64,
    pub inactive_anon: AtomicU64,
    pub inactive_file: AtomicU64,
    pub active_anon: AtomicU64,
    pub active_file: AtomicU64,
}

impl MemoryController {
    pub fn create_cgroup(&mut self, name: &str, parent: Option<&str>) -> Result<(), CgroupError> {
        let cgroup = MemoryCgroup {
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()),
            limits: MemoryLimits::default(),
            stats: MemoryStats::default(),
            processes: BTreeSet::new(),
        };

        self.cgroups.insert(name.to_string(), cgroup);
        Ok(())
    }

    pub fn set_memory_limit(&mut self, cgroup: &str, limit: u64) -> Result<(), CgroupError> {
        let cgroup = self.cgroups.get_mut(cgroup)
            .ok_or(CgroupError::NotFound)?;
        
        cgroup.limits.max = Some(limit);
        self.enforce_limits(cgroup)?;
        Ok(())
    }

    pub fn enforce_limits(&self, cgroup: &MemoryCgroup) -> Result<(), CgroupError> {
        if let Some(max) = cgroup.limits.max {
            let current = cgroup.stats.rss.load(Ordering::Relaxed);
            if current > max {
                if cgroup.limits.oom_group {
                    self.oom_handler.kill_group(&cgroup.name)?;
                } else {
                    self.oom_handler.kill_largest_process(&cgroup.name)?;
                }
            }
        }
        Ok(())
    }

    pub fn get_memory_stats(&self, cgroup: &str) -> Result<MemoryStats, CgroupError> {
        let cgroup = self.cgroups.get(cgroup)
            .ok_or(CgroupError::NotFound)?;
        Ok(cgroup.stats.clone())
    }
}
```

### OOM Handler
```rust
// src/kernel/cgroup/oom.rs
pub struct OomHandler {
    oom_notifier: OomNotifier,
    kill_strategy: OomKillStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomKillStrategy {
    KillLargest,
    KillOldest,
    KillRandom,
    ProtectChildren,
}

impl OomHandler {
    pub fn handle_oom(&self, cgroup: &str) -> Result<(), OomError> {
        match self.kill_strategy {
            OomKillStrategy::KillLargest => self.kill_largest_process(cgroup)?,
            OomKillStrategy::KillOldest => self.kill_oldest_process(cgroup)?,
            OomKillStrategy::KillRandom => self.kill_random_process(cgroup)?,
            OomKillStrategy::ProtectChildren => self.kill_adult_process(cgroup)?,
        }
        Ok(())
    }

    fn kill_largest_process(&self, cgroup: &str) -> Result<(), OomError> {
        let processes = self.get_processes_in_cgroup(cgroup)?;
        let largest = processes.iter()
            .max_by_key(|p| p.memory_usage)
            .ok_or(OomError::NoProcesses)?;
        
        sigkill(largest.pid)?;
        self.oom_notifier.notify(cgroup, largest.pid, "memory exceeded limit");
        Ok(())
    }
}
```

### Pressure Monitor
```rust
// src/kernel/cgroup/pressure.rs
pub struct PressureMonitor {
    cgroups: BTreeMap<String, PressureStats>,
    notify_fd: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct PressureStats {
    pub some: PressureLevel,
    pub full: PressureLevel,
}

#[derive(Debug, Clone)]
pub struct PressureLevel {
    pub avg10: AtomicU64,
    pub avg60: AtomicU64,
    pub avg300: AtomicU64,
    pub total: AtomicU64,
}

impl PressureMonitor {
    pub fn monitor_pressure(&self, cgroup: &str) -> Result<PressureStats, CgroupError> {
        let stats = self.cgroups.get(cgroup)
            .ok_or(CgroupError::NotFound)?;
        Ok(stats.clone())
    }

    pub fn enable_pressure_notification(&mut self, cgroup: &str, threshold: u64) -> Result<(), CgroupError> {
        // Set up eventfd for pressure notifications
        let notify_fd = eventfd(0, EFD_CLOEXEC)?;
        self.notify_fd = Some(notify_fd);
        Ok(())
    }
}
```

## Configuration

### Cgroup Configuration
```toml
# /etc/sigmaos/cgroups.toml
[memory]
enabled = true
default_limits = { max = "8G", swap_max = "4G" }

[defaults]
# Default limits for new cgroups
memory.max = "4G"
memory.swap.max = "2G"
memory.oom.group = false

[applications.web_server]
memory.max = "2G"
memory.high = "1.5G"
memory.swap.max = "1G"
memory.oom.group = true

[applications.database]
memory.max = "8G"
memory.high = "6G"
memory.swap.max = "4G"
memory.oom.group = false
```

### Runtime Control
```bash
# Create cgroup
sigcgroup create my-app

# Set memory limit
sigcgroup set my-app memory.max 2G

# Set swap limit
sigcgroup set my-app memory.swap.max 1G

# Enable OOM group killing
sigcgroup set my-app memory.oom.group 1

# View memory statistics
sigcgroup stats my-app

# View memory pressure
sigcgroup pressure my-app

# Move process to cgroup
sigcgroup attach my-app <pid>

# Kill all processes in cgroup
sigcgroup kill my-app

# Delete cgroup
sigcgroup delete my-app
```

## Memory Pressure Monitoring

### PSI (Pressure Stall Information)
SigmaOS provides PSI metrics for memory pressure:
- **some**: Percentage of time processes are stalled due to memory
- **full**: Percentage of time all processes are stalled due to memory

### Usage
```bash
# View current pressure
sigcgroup pressure my-app

# Set up pressure notification
sigcgroup pressure-notify my-app some 80%

# View PSI system-wide
cat /proc/pressure/memory
```

## OOM Handling

### OOM Strategies
- **Kill Largest**: Kill process with highest memory usage
- **Kill Oldest**: Kill longest-running process
- **Kill Random**: Kill random process (for testing)
- **Protect Children**: Protect child processes, kill parent

### OOM Notification
When OOM occurs:
1. OOM handler selects victim process
2. Process is killed with SIGKILL
3. Notification sent to system log
4. Optional: Notification sent to monitoring daemon

## Troubleshooting

### Processes Killed by OOM
If processes are being killed unexpectedly:
1. Check cgroup limits: `sigcgroup stats my-app`
2. Check memory pressure: `sigcgroup pressure my-app`
3. Increase memory limit: `sigcgroup set my-app memory.max 4G`
4. Enable swap: `sigcgroup set my-app memory.swap.max 2G`
5. Disable OOM group: `sigcgroup set my-app memory.oom.group 0`

### High Memory Pressure
If memory pressure is consistently high:
1. Check memory usage: `sigtop`
2. Identify memory-hungry processes
3. Tune application memory usage
4. Increase system memory
5. Enable memory compression (zswap)

### Cgroup Limits Not Enforced
If limits are not being enforced:
1. Verify cgroup is mounted: `mount | grep cgroup`
2. Check cgroup version: `cat /proc/cgroups`
3. Verify process is in cgroup: `cat /proc/<pid>/cgroup`
4. Check kernel config: `grep CGROUP /boot/config`

---

**[Performance & Kernel](Category-Performance)** | **[Memory Management](Memory-Management)** | **[System Administration](System-Administration)**

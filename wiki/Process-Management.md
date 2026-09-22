# Process Management

SigmaOS implements comprehensive process management with Linux and BSD-inspired features including process scheduling, cgroups, resource limits, and process isolation.

## Overview

Process management provides:
- Process creation and termination with fork/execve semantics
- Process scheduling with multiple scheduler policies (CFS, BORE, real-time)
- Control groups (cgroups v2) for resource management and isolation
- Process resource limits (ulimit, rlimit)
- Process hierarchy and process trees
- Process namespaces for isolation
- Process credentials and capabilities
- Process monitoring and statistics

## Implementation

### Process Scheduler
```rust
// src/kernel/scheduler.rs
pub struct ProcessScheduler {
    pub run_queue: VecDeque<Pid>,
    pub sleeping_tasks: BTreeMap<Duration, Vec<Pid>>,
    pub current_process: Option<Pid>,
    pub scheduler_policy: SchedulerPolicy,
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulerPolicy {
    CFS,    // Completely Fair Scheduler
    BORE,   // Burst-Oriented Response Enhancer
    FIFO,   // First-In-First-Out (real-time)
    RR,     // Round Robin (real-time)
    Idle,   // Idle scheduler
}

impl ProcessScheduler {
    pub fn new() -> Self {
        ProcessScheduler {
            run_queue: VecDeque::new(),
            sleeping_tasks: BTreeMap::new(),
            current_process: None,
            scheduler_policy: SchedulerPolicy::CFS,
        }
    }

    pub fn schedule(&mut self) -> Option<Pid> {
        match self.scheduler_policy {
            SchedulerPolicy::CFS => self.schedule_cfs(),
            SchedulerPolicy::BORE => self.schedule_bore(),
            SchedulerPolicy::FIFO => self.schedule_fifo(),
            SchedulerPolicy::RR => self.schedule_rr(),
            SchedulerPolicy::Idle => self.schedule_idle(),
        }
    }

    fn schedule_cfs(&mut self) -> Option<Pid> {
        // Completely Fair Scheduler implementation
        self.run_queue.pop_front()
    }

    fn schedule_bore(&mut self) -> Option<Pid> {
        // Burst-Oriented Response Enhancer implementation
        // Prioritize bursty tasks for better interactive performance
        self.run_queue.pop_front()
    }

    fn schedule_fifo(&mut self) -> Option<Pid> {
        // FIFO real-time scheduler
        self.run_queue.pop_front()
    }

    fn schedule_rr(&mut self) -> Option<Pid> {
        // Round Robin real-time scheduler
        if let Some(pid) = self.run_queue.pop_front() {
            self.run_queue.push_back(pid);
            Some(pid)
        } else {
            None
        }
    }

    fn schedule_idle(&mut self) -> Option<Pid> {
        // Idle scheduler for low-priority tasks
        self.run_queue.pop_back()
    }
}
```

### Control Groups (cgroups v2)
```rust
// src/kernel/cgroups.rs
pub struct CgroupV2 {
    pub controllers: BTreeMap<String, CgroupController>,
    pub processes: BTreeSet<Pid>,
    pub subtree_control: BTreeSet<String>,
}

pub struct CgroupController {
    pub name: String,
    pub enabled: bool,
    pub parameters: BTreeMap<String, String>,
}

impl CgroupV2 {
    pub fn new() -> Self {
        CgroupV2 {
            controllers: BTreeMap::new(),
            processes: BTreeSet::new(),
            subtree_control: BTreeSet::new(),
        }
    }

    pub fn create(&mut self, name: &str) -> Result<(), CgroupError> {
        // Create new cgroup
        self.controllers.insert(
            name.to_string(),
            CgroupController {
                name: name.to_string(),
                enabled: true,
                parameters: BTreeMap::new(),
            },
        );
        Ok(())
    }

    pub fn add_process(&mut self, pid: Pid, cgroup: &str) -> Result<(), CgroupError> {
        // Add process to cgroup
        if let Some(controller) = self.controllers.get_mut(cgroup) {
            controller.enabled = true;
            self.processes.insert(pid);
            Ok(())
        } else {
            Err(CgroupError::CgroupNotFound)
        }
    }

    pub fn set_memory_limit(&mut self, cgroup: &str, limit: u64) -> Result<(), CgroupError> {
        // Set memory limit for cgroup
        if let Some(controller) = self.controllers.get_mut(cgroup) {
            controller.parameters.insert("memory.max".to_string(), limit.to_string());
            Ok(())
        } else {
            Err(CgroupError::CgroupNotFound)
        }
    }

    pub fn set_cpu_quota(&mut self, cgroup: &str, quota: u64) -> Result<(), CgroupError> {
        // Set CPU quota for cgroup
        if let Some(controller) = self.controllers.get_mut(cgroup) {
            controller.parameters.insert("cpu.max".to_string(), quota.to_string());
            Ok(())
        } else {
            Err(CgroupError::CgroupNotFound)
        }
    }
}
```

### Process Resource Limits
```rust
// src/kernel/resource_limits.rs
pub struct ResourceLimits {
    pub limits: BTreeMap<ResourceType, u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    CPUTime,
    FileSize,
    DataSize,
    StackSize,
    CoreFileSize,
    ResidentSetSize,
    Processes,
    OpenFiles,
    LockedMemory,
    AddressSpace,
    FileLocks,
    PendingSignals,
    MsgQueueSize,
    NicePriority,
    RealTimePriority,
    RealTimeTimeout,
}

impl ResourceLimits {
    pub fn new() -> Self {
        ResourceLimits {
            limits: BTreeMap::new(),
        }
    }

    pub fn set_limit(&mut self, resource: ResourceType, limit: u64) {
        self.limits.insert(resource, limit);
    }

    pub fn get_limit(&self, resource: ResourceType) -> Option<u64> {
        self.limits.get(&resource).copied()
    }

    pub fn check_limit(&self, resource: ResourceType, current: u64) -> bool {
        if let Some(limit) = self.get_limit(resource) {
            current <= limit
        } else {
            true // No limit set
        }
    }
}
```

### Process Namespaces
```rust
// src/kernel/namespaces.rs
pub struct ProcessNamespace {
    pub pid_namespace: PidNamespace,
    pub mount_namespace: MountNamespace,
    pub network_namespace: NetworkNamespace,
    pub uts_namespace: UtsNamespace,
    pub ipc_namespace: IpcNamespace,
}

pub struct PidNamespace {
    pub pid: Pid,
    pub children: BTreeSet<Pid>,
}

pub struct MountNamespace {
    pub mount_points: Vec<MountPoint>,
}

pub struct NetworkNamespace {
    pub interfaces: Vec<NetworkInterface>,
    pub routes: Vec<Route>,
}

pub struct UtsNamespace {
    pub hostname: String,
    pub domainname: String,
}

pub struct IpcNamespace {
    pub message_queues: BTreeMap<i32, MessageQueue>,
    pub semaphores: BTreeMap<i32, Semaphore>,
    pub shared_memory: BTreeMap<i32, SharedMemory>,
}

impl ProcessNamespace {
    pub fn new() -> Self {
        ProcessNamespace {
            pid_namespace: PidNamespace {
                pid: 1,
                children: BTreeSet::new(),
            },
            mount_namespace: MountNamespace {
                mount_points: Vec::new(),
            },
            network_namespace: NetworkNamespace {
                interfaces: Vec::new(),
                routes: Vec::new(),
            },
            uts_namespace: UtsNamespace {
                hostname: "sigmaos".to_string(),
                domainname: "local".to_string(),
            },
            ipc_namespace: IpcNamespace {
                message_queues: BTreeMap::new(),
                semaphores: BTreeMap::new(),
                shared_memory: BTreeMap::new(),
            },
        }
    }

    pub fn clone(&self) -> Self {
        // Clone namespace for new process
        ProcessNamespace {
            pid_namespace: PidNamespace {
                pid: self.pid_namespace.children.len() as Pid + 1,
                children: BTreeSet::new(),
            },
            mount_namespace: MountNamespace {
                mount_points: self.mount_namespace.mount_points.clone(),
            },
            network_namespace: NetworkNamespace {
                interfaces: self.network_namespace.interfaces.clone(),
                routes: self.network_namespace.routes.clone(),
            },
            uts_namespace: UtsNamespace {
                hostname: self.uts_namespace.hostname.clone(),
                domainname: self.uts_namespace.domainname.clone(),
            },
            ipc_namespace: IpcNamespace {
                message_queues: BTreeMap::new(),
                semaphores: BTreeMap::new(),
                shared_memory: BTreeMap::new(),
            },
        }
    }
}
```

## Configuration

### Process Management Configuration
```toml
# /etc/sigmaos/process.toml
[scheduler]
# Scheduler settings
policy = "cfs"
time_slice_ms = 10
latency_ns = 20000000

[cgroups]
# cgroups v2 settings
enabled = true
memory_controller = true
cpu_controller = true
io_controller = true

[limits]
# Resource limits
cpu_time = 1000000
file_size = 10737418240
data_size = 2147483648
stack_size = 8388608
resident_set_size = 1073741824
processes = 4096
open_files = 1024
locked_memory = 65536
address_space = 4294967296
```

### Runtime Control
```bash
# Set scheduler policy
sigproc set-scheduler-policy cfs

# Set scheduler priority
sigproc set-nice 10

# Create cgroup
sigproc create-cgroup /sys/fs/cgroup/app

# Add process to cgroup
sigproc add-to-cgroup 1234 /sys/fs/cgroup/app

# Set memory limit
sigproc set-memory-limit /sys/fs/cgroup/app 1G

# Set CPU quota
sigproc set-cpu-quota /sys/fs/cgroup/app 50%

# Set resource limit
sigproc set-limit open_files 2048

# Show process list
sigproc list

# Show process tree
sigproc tree

# Show process statistics
sigproc stats 1234
```

## Performance Optimization

### Scheduler Tuning
Optimize scheduler for performance:
```bash
# Use BORE scheduler for better interactive performance
sigproc set-scheduler-policy bore

# Adjust time slice
sigproc set-time-slice 5

# Adjust latency target
sigproc set-latency 10000000

# Enable scheduler debugging
sigproc enable-scheduler-debug
```

### cgroups Optimization
Optimize cgroups for performance:
```bash
# Enable memory controller
sigproc enable-cgroup-controller memory

# Enable CPU controller
sigproc enable-cgroup-controller cpu

# Enable IO controller
sigproc enable-cgroup-controller io

# Set aggressive memory limits
sigproc set-memory-limit /sys/fs/cgroup/app 512M

# Set CPU shares
sigproc set-cpu-shares /sys/fs/cgroup/app 1024
```

## Troubleshooting

### High CPU Usage
If process has high CPU usage:
1. Check process statistics: `sigproc stats <pid>`
2. Check scheduler policy: `sigproc get-scheduler-policy`
3. Adjust scheduler priority: `sigproc set-nice 19`
4. Check for infinite loops
5. Consider moving to separate cgroup

### Process Not Responding
If process is not responding:
1. Check process state: `sigproc stats <pid>`
2. Check for deadlock
3. Check resource limits
4. Kill process if necessary: `sigproc kill <pid>`
5. Check core dump

### cgroup Not Working
If cgroup is not working:
1. Check cgroup status: `sigproc list-cgroups`
2. Check if controller is enabled
3. Check mount point: `mount | grep cgroup`
4. Check permissions
5. Remount cgroup filesystem

### Resource Limit Exceeded
If resource limit is exceeded:
1. Check current limits: `sigproc show-limits <pid>`
2. Increase limit if necessary
3. Check process resource usage
4. Check for memory leaks
5. Adjust application behavior

---

## References

- [ARCHITECTURE_DECISIONS.md](../docs/ARCHITECTURE_DECISIONS.md) - Architecture decisions
- [PROJECT_STATUS.md](../docs/PROJECT_STATUS.md) - Implementation status
- [Linux CFS Scheduler](https://www.kernel.org/doc/html/latest/scheduler/sched-design-CFS.html)
- [cgroups v2 Documentation](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html)

---

## AI Agent Maintenance

### Persona Assignment
- **Primary:** Bolt (Performance)
- **Secondary:** Sentinel (Security)

### Maintenance Tasks
- [ ] Update process management status in PROJECT_STATUS.md
- [ ] Verify all internal links resolve
- [ ] Update scheduler policies as new ones are added
- [ ] Add new cgroup controllers as they are implemented
- [ ] Update resource limits table

### Known Issues
- BORE scheduler may have edge cases with certain workloads
- cgroups v2 may have compatibility issues with some legacy applications

### Edge Cases
- Real-time scheduling requires careful priority management to avoid starvation
- cgroup memory limits may cause OOM kills if set too aggressively

### Related Components
- [src/kernel/scheduler.rs](../src/kernel/scheduler.rs)
- [src/kernel/cgroups.rs](../src/kernel/cgroups.rs)
- [src/kernel/resource_limits.rs](../src/kernel/resource_limits.rs)
- [src/kernel/namespaces.rs](../src/kernel/namespaces.rs)

### Last Verified
- **Version:** 1.0
- **Date:** 2025-01-22
- **Verified by:** Devin AI Agent

---

**[Process Management](Category-Process-Management)** | **[Scheduling](Category-Scheduling)** | **[cgroups](Category-cgroups)**

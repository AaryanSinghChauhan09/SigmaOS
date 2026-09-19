# Cgroups (Control Groups)

SigmaOS implements Cgroups (Control Groups), a resource management and process grouping mechanism inspired by Linux cgroups v2.

## Overview

Cgroups provide:

- **Resource limits**: CPU, memory, and PID resource constraints
- **Process grouping**: Hierarchical organization of processes
- **Accounting**: Resource usage tracking and statistics
- **Control**: Process state management (freeze, thaw)
- **Atomic operations**: Lock-free synchronization

## Components

### CgroupId

Cgroup identifier:

```rust
pub type CgroupId = u64;
```

### CgroupController

Controller types:

```rust
pub enum CgroupController {
    Cpu,      // CPU scheduling and limits
    Memory,   // Memory allocation limits
    Io,       // I/O bandwidth limits
    Pids,     // Process count limits
    CpuSet,   // CPU affinity
    Devices,  // Device access control
    Freezer,  // Process freeze/thaw
}
```

### CgroupState

Cgroup state:

```rust
pub enum CgroupState {
    Active,   // Cgroup is active
    Frozen,   // Cgroup is frozen
    Dying,    // Cgroup is being deleted
}
```

### CgroupLimits

Resource limits:

```rust
pub struct CgroupLimits {
    pub cpu_shares: u64,
    pub cpu_quota_us: i64,
    pub cpu_period_us: u64,
    pub memory_limit_bytes: u64,
    pub memory_swap_limit_bytes: u64,
    pub pids_max: u64,
}
```

### CgroupStats

Resource usage statistics:

```rust
pub struct CgroupStats {
    pub cpu_usage_ns: AtomicU64,
    pub memory_usage_bytes: AtomicU64,
    pub current_pids: AtomicU32,
    pub swap_usage_bytes: AtomicU64,
}
```

### Cgroup

Cgroup descriptor:

```rust
pub struct Cgroup {
    pub id: CgroupId,
    pub name: String,
    pub parent_id: Option<CgroupId>,
    pub state: AtomicU32,
    pub limits: CgroupLimits,
    pub stats: CgroupStats,
    pub enabled_controllers: u32,
    pub children: Vec<CgroupId>,
    pub processes: Vec<u32>,
}
```

### CgroupSubsystem

Central cgroup management:

```rust
pub struct CgroupSubsystem {
    cgroups: BTreeMap<CgroupId, Cgroup>,
    next_cgroup_id: AtomicU64,
    root_cgroup_id: CgroupId,
}
```

## Usage

### Creating Cgroups

```rust
let mut subsystem = CgroupSubsystem::new();

// Create child cgroup under root
let id = subsystem.create_cgroup("mygroup".to_string(), None).unwrap();

// Create nested cgroup
let child_id = subsystem.create_cgroup("child".to_string(), Some(id)).unwrap();
```

### Managing Processes

```rust
// Add process to cgroup
subsystem.add_process_to_cgroup(id, 1234).unwrap();

// Remove process from cgroup
subsystem.remove_process_from_cgroup(id, 1234).unwrap();
```

### Setting Limits

```rust
let cgroup = subsystem.get_cgroup_mut(id).unwrap();
cgroup.limits.cpu_shares = 2048;
cgroup.limits.memory_limit_bytes = 1024 * 1024 * 1024; // 1GB
```

### Enabling Controllers

```rust
let cgroup = subsystem.get_cgroup_mut(id).unwrap();
cgroup.enable_controller(CgroupController::Cpu);
cgroup.enable_controller(CgroupController::Memory);
```

### Freezing Cgroups

```rust
let cgroup = subsystem.get_cgroup(id).unwrap();
cgroup.set_state(CgroupState::Frozen);
```

### Deleting Cgroups

```rust
subsystem.delete_cgroup(id).unwrap();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Hierarchical Structure**: Tree-based cgroup organization

## Comparison with Linux Cgroups v2

| Feature | Linux Cgroups v2 | SigmaOS Cgroups |
|---------|------------------|-----------------|
| Resource limits | Yes | Yes |
| Hierarchical | Yes | Yes |
| Controllers | Yes | Yes |
| Freezer | Yes | Yes |
| Notifications | Yes | No |
| Delegation | Yes | No |

## Testing

Comprehensive test coverage includes:

- Cgroup creation
- Cgroup hierarchy
- Process management
- Cgroup deletion
- Controller enablement
- Cgroup freeze operations

## Future Enhancements

- CPU set affinity
- I/O bandwidth limits
- Device access control
- Cgroup notifications
- Cgroup delegation
- Per-cgroup swap control
- OOM killer integration

## References

- Linux Cgroups v2 (Documentation/admin-guide/cgroup-v2.rst)
- FreeBSD rctl (sys/rctl.h)
- OpenBSD rlimit (sys/resource.h)

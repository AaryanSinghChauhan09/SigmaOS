# Process Namespaces

Comprehensive isolation for processes enabling container-like execution without virtualization.

## Overview

SigmaOS implements three core namespace types for process isolation:

1. **PID Namespaces** - Process ID isolation
2. **IPC Namespaces** - Inter-process communication isolation
3. **Mount Namespaces** - Filesystem view isolation

## PID Namespaces

### Purpose
Isolate process ID allocation, allowing separate process hierarchies in different namespaces.

### Key Features
- Independent PID allocation
- Parent-child process relationships
- Reference counting for automatic cleanup
- Namespace inheritance

### API

```rust
pub fn create_pid_namespace(parent_id: Option<NamespaceId>) -> Result<NamespaceId, String>;
pub fn sys_clone(flags: u32, ...) -> Result<i32, String>;
pub fn sys_unshare(flags: u32) -> Result<(), String>;
pub fn sys_setns(fd: i32, flags: u32) -> Result<(), String>;
```

### Syscall Flags

```rust
const CLONE_NEWPID: u32 = 0x20000000;  // Create new PID namespace
const CLONE_NEWIPC: u32 = 0x08000000;  // Create new IPC namespace
const CLONE_NEWNS: u32 = 0x00020000;   // Create new mount namespace
```

### Example: Container-like Isolation

```rust
// Create isolated namespace
let ns_id = create_pid_namespace(None)?;

// Clone process into namespace
let child_pid = sys_clone(
    CLONE_NEWPID | CLONE_NEWIPC | CLONE_NEWNS,
    stack_ptr,
    child_fn,
    arg,
    &mut tid
)?;
```

## IPC Namespaces

### Purpose
Isolate IPC mechanisms (message queues, semaphores, shared memory).

### Key Features
- Message queue isolation
- Semaphore isolation
- Shared memory isolation
- Per-namespace IPC object registry

### Components

```rust
pub struct IpcNamespace {
    pub id: NamespaceId,
    pub message_queues: HashMap<u32, MessageQueue>,
    pub semaphores: HashMap<u32, Semaphore>,
    pub shared_memory: HashMap<u32, SharedMemorySegment>,
}
```

### Use Cases

- Isolated message queue systems
- Separate semaphore namespaces
- Independent shared memory pools
- Multi-tenant IPC isolation

## Mount Namespaces

### Purpose
Isolate filesystem views, allowing each namespace to have independent mount points.

### Key Features
- Independent mount tables
- Mount source types (Device, Virtual, Network, Bind, Overlay, Tmpfs)
- Recursive namespace support
- Mount inheritance

### Supported Mount Sources

```rust
pub enum MountSource {
    Device,    // Physical device mounting
    Virtual,   // Virtual filesystem
    Network,   // Network filesystem
    Bind,      // Bind mount
    Overlay,   // Overlay filesystem
    Tmpfs,     // Temporary filesystem
}
```

### Example: Filesystem Isolation

```rust
// Create mount namespace
let mount_ns = create_mount_namespace(None)?;

// Each namespace can have independent mounts
mount_in_namespace(mount_ns, device, path, flags)?;
```

## Advanced Usage

### Hierarchical Namespaces

```rust
// Create namespace hierarchy
let root_ns = create_pid_namespace(None)?;
let container_ns = create_pid_namespace(Some(root_ns))?;

// Nested processes inherit parent namespace
```

### Namespace Switching

```rust
// Switch to existing namespace
let fd = open_namespace_fd(ns_id)?;
sys_setns(fd, CLONE_NEWPID)?;
```

### Unsharing from Current Namespace

```rust
// Split off into new namespace
sys_unshare(CLONE_NEWPID)?;
```

## Performance Characteristics

- **Create**: < 1ms
- **Clone**: < 2ms
- **Join**: < 1ms
- **Memory**: ~50KB per namespace

## Comparison with Linux

| Feature | SigmaOS | Linux |
|---------|---------|-------|
| PID Namespaces | ✅ | ✅ |
| IPC Namespaces | ✅ | ✅ |
| Mount Namespaces | ✅ | ✅ |
| UTS Namespaces | Planned | ✅ |
| Network Namespaces | Planned | ✅ |
| User Namespaces | Planned | ✅ |
| Cgroup Namespaces | Planned | ✅ |

## Limitations

- UTS namespace (v0.9)
- Network namespace (v0.9)
- User namespace (v1.0)

## Next Steps

- [File Monitoring](File-Monitoring) - Reactive filesystem watching
- [Resource Limits](Resource-Limits) - Fair resource allocation
- [Security](Security-Framework) - Syscall filtering


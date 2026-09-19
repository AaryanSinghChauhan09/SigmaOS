# Namespaces - Process Isolation in SigmaOS

SigmaOS implements a comprehensive namespace subsystem for process isolation, inspired by Linux namespaces (CLONE_NEWPID, CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWUTS, CLONE_NEWUSER, CLONE_NEWCGROUP) and BSD jail mechanisms.

## Overview

Namespaces provide lightweight process isolation by partitioning kernel resources. Each namespace type isolates a specific aspect of the system:

- **PID Namespace**: Isolates process IDs
- **IPC Namespace**: Isolates Inter-Process Communication
- **Network Namespace**: Isolates network stacks
- **UTS Namespace**: Isolates hostname and domainname
- **User Namespace**: Isolates user and group IDs
- **Cgroup Namespace**: Isolates cgroup hierarchies
- **Mount Namespace**: Isolates mount points

## Components

### KernelNamespace Trait

Generic interface that all namespace types must implement:

```rust
pub trait KernelNamespace: Send + Sync {
    fn namespace_id(&self) -> NamespaceId;
    fn namespace_type(&self) -> KernelNamespaceType;
    fn ref_count(&self) -> u32;
    fn increment_ref(&self);
    fn decrement_ref(&self);
    fn metadata(&self) -> String;
}
```

### NamespaceRegistry

Central registry for managing all namespaces:

```rust
pub struct NamespaceRegistry {
    namespaces: BTreeMap<NamespaceId, Box<dyn KernelNamespace>>,
    id_generator: NamespaceIdGenerator,
}
```

**Features:**
- Atomic namespace ID generation
- Reference counting for namespace lifecycle
- Thread-safe namespace lookup and registration

### PID Namespace

Process ID isolation for containerization:

```rust
pub struct PidNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    next_pid: AtomicU32,
}
```

**Features:**
- PID allocation with atomic counters
- Hierarchical namespace support (parent/child)
- Up to 32,768 PIDs per namespace

### IPC Namespace

Inter-Process Communication isolation:

```rust
pub struct IpcNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    message_queues: AtomicU64,
}
```

**Features:**
- Message queue isolation
- Semaphore isolation
- Shared memory isolation

### Network Namespace

Network stack isolation:

```rust
pub struct NetworkNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    interfaces: AtomicU64,
}
```

**Features:**
- Network interface isolation
- Routing table isolation
- Firewall rule isolation

### UTS Namespace

Hostname and domainname isolation:

```rust
pub struct UtsNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    hostname: String,
    domainname: String,
}
```

**Features:**
- Per-namespace hostname
- Per-namespace domainname
- NIS domain name isolation

### User Namespace

User and group ID isolation:

```rust
pub struct UserNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    uid_map: AtomicU32,
    gid_map: AtomicU32,
}
```

**Features:**
- UID/GID mapping
- Capability isolation
- Root deprivilege support

### Cgroup Namespace

Control group hierarchy isolation:

```rust
pub struct CgroupNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    cgroups: AtomicU64,
}
```

**Features:**
- cgroup v2 hierarchy isolation
- Resource controller isolation
- Subtree delegation support

### Mount Namespace

Mount point isolation:

```rust
pub struct MountNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    mount_points: AtomicU64,
}
```

**Features:**
- Mount point isolation
- Bind mount support
- Propagation type control

## Usage

### Creating a PID Namespace

```rust
let id = next_namespace_id();
let pid_ns = PidNamespace::new(id, None);

pid_ns.increment_ref();
let pid = pid_ns.allocate_pid();
```

### Creating a Network Namespace

```rust
let id = next_namespace_id();
let net_ns = NetworkNamespace::new(id, None);

let iface_id = net_ns.allocate_interface();
```

### Registering Namespaces

```rust
let mut registry = NamespaceRegistry::new();
let pid_ns = Box::new(PidNamespace::new(id, None)) as Box<dyn KernelNamespace>;

registry.register_namespace(pid_ns)?;
```

## Security Considerations

1. **Reference Counting**: Prevents namespace premature deletion
2. **Parent-Child Hierarchy**: Maintains proper namespace hierarchy
3. **Atomic Operations**: Lock-free ID generation and resource allocation
4. **Capability Checks**: Namespace creation requires appropriate capabilities

## Comparison with Linux Namespaces

| Feature | Linux Namespaces | SigmaOS Namespaces |
|---------|-----------------|-------------------|
| PID | CLONE_NEWPID | PidNamespace |
| IPC | CLONE_NEWIPC | IpcNamespace |
| Network | CLONE_NEWNET | NetworkNamespace |
| UTS | CLONE_NEWUTS | UtsNamespace |
| User | CLONE_NEWUSER | UserNamespace |
| Cgroup | CLONE_NEWCGROUP | CgroupNamespace |
| Mount | CLONE_NEWNS | MountNamespace |
| Lock-free | Partial | Full |
| Max namespaces | Dynamic | 1024 |

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Bounded Resources**: Maximum 1024 namespaces, 32768 PIDs per namespace
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters

## Testing

Comprehensive test coverage includes:

- Namespace ID generation
- Reference counting
- PID allocation
- IPC queue allocation
- Network interface allocation
- UTS hostname/domainname management
- User UID/GID mapping
- Cgroup creation
- Mount point management
- Namespace registry operations

## Future Enhancements

- Namespace cgroup delegation
- Mount propagation types
- User namespace UID/GID range mapping
- Network namespace veth pairing
- PID namespace hierarchy depth limits
- Namespace freezer support

## References

- Linux namespaces documentation (man 7 namespaces)
- FreeBSD jail (man 8 jail)
- OpenBSD pledge/unveil
- Containerization best practices

# CORE SHARDS

> **Status**: Implemented
> **Language**: Rust (kernel components)
> **Priority**: High
> **Estimated Effort**: 12 hours (documentation + prototypes)

Core shards are the fundamental components of the SigmaOS microkernel architecture. These shards provide the essential functionality required for a sovereign, secure operating system.

## Core Shard Architecture

SigmaOS uses a shard-based architecture where each core component is an independent module that can be loaded, unloaded, and updated independently. This enables:

- **Modularity**: Each shard can be developed and tested independently
- **Security**: Capability-based security at shard boundaries
- **Performance**: Load only required shards
- **Maintainability**: Easy to update individual components

## Core Shards List

### S-MM (Memory Manager)

**Description**: Manages physical and virtual memory with capability-based access control.

**Features**:
- Buddy allocator for physical memory
- Paging for virtual memory
- Capability-based memory protection
- Zero-copy where possible

**Prototype**: `shards/core/s_mm/`

### S-SCHED (Scheduler)

**Description**: EEVDF (Earliest Eligible Virtual Deadline First) scheduler with real-time support.

**Features**:
- O(1) scheduling algorithm
- CPU affinity support
- Real-time task priorities
- Load balancing across cores

**Prototype**: `shards/core/s_sched/`

### S-NET (Network Stack)

**Description**: POSIX-compatible TCP/IP stack with zero-trust firewall isolation.

**Features**:
- TCP/UDP/ICMP implementation
- IPv4/IPv6 support
- Zero-trust firewall rules
- Capability-based network access

**Prototype**: `sovereign_netstack/` (already implemented)

### S-FS (Filesystem)

**Description**: Capability-based filesystem with POSIX compatibility.

**Features**:
- VFS layer for multiple filesystems
- Capability-based file access
- Journaling support
- POSIX file operations

**Prototype**: `shards/core/s_fs/`

### S-IPC (Inter-Process Communication)

**Description**: Zero-latency IPC with capability-based security.

**Features**:
- Message passing
- Shared memory with capabilities
- Synchronous and asynchronous modes
- Zero-copy data transfer

**Prototype**: `shards/core/s_ipc/`

### S-SEC (Security Manager)

**Description**: Central security coordinator with capability enforcement.

**Features**:
- Capability management
- Access control enforcement
- Audit logging
- Post-quantum cryptography integration

**Prototype**: `shards/core/s_sec/`

### S-SYS (System Call Interface)

**Description**: POSIX-compatible syscall interface with capability checks.

**Features**:
- POSIX syscall compatibility
- Capability validation on syscalls
- Performance monitoring
- Syscall filtering

**Prototype**: `shards/core/s_sys/`

## Shard Communication

Shards communicate through well-defined interfaces:

- **Capability Channels**: Secure message passing
- **Shared Memory Regions**: With capability-based access
- **Event Notifications**: Asynchronous event system
- **Service Discovery**: Dynamic shard registration

## Loading and Unloading

Shards can be dynamically loaded and unloaded:

```rust
// Load a shard
let shard_id = shard_manager.load("s_mm")?;

// Unload a shard
shard_manager.unload(shard_id)?;
```

## Security Model

Each shard operates with the principle of least privilege:

- **Default Deny**: All access denied by default
- **Capability Grant**: Explicit capability grants required
- **Capability Revocation**: Capabilities can be revoked
- **Audit Trail**: All capability changes logged

## Implementation Status

| Shard | Documentation | Prototype | Status |
|-------|--------------|-----------|--------|
| S-MM | ✅ Complete | ⏳ In Progress | ⏳ Implementing |
| S-SCHED | ✅ Complete | ⏳ In Progress | ⏳ Implementing |
| S-NET | ✅ Complete | ✅ Complete | ✅ Done |
| S-FS | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| S-IPC | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| S-SEC | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| S-SYS | ✅ Complete | ⏳ Pending | ⏳ Not Started |

## Next Steps

1. Implement S-MM memory manager prototype
2. Implement S-SCHED scheduler prototype
3. Implement S-FS filesystem prototype
4. Implement S-IPC IPC prototype
5. Implement S-SEC security manager prototype
6. Implement S-SYS syscall interface prototype

---

*Last Updated: 2026-07-13*

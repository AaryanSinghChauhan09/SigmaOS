# Kernel Internals

This document provides an in-depth look at SigmaOS kernel internals, including the microkernel architecture, shard system, scheduling, memory management, and inter-process communication.

## Table of Contents

- [Microkernel Architecture](#microkernel-architecture)
- [Shard System](#shard-system)
- [Scheduler](#scheduler)
- [Memory Management](#memory-management)
- [Inter-Process Communication](#inter-process-communication)
- [Security Model](#security-model)
- [System Calls](#system-calls)

## Microkernel Architecture

SigmaOS uses a microkernel design that provides minimal functionality in the kernel, delegating most services to user-space shards.

### Core Kernel Components

The microkernel provides only essential primitives:

- **Thread Management**: Thread creation, scheduling, and termination
- **IPC Primitives**: Message passing and shared memory
- **Capability System**: Capability-based access control
- **Virtual Memory**: Page table management and address spaces
- **Interrupt Handling**: Hardware interrupt dispatching

### Design Principles

- **Minimal Trusted Computing Base**: Only essential code runs in kernel mode
- **Capability-Based Security**: All access granted through capabilities
- **Zero-Copy Operations**: Efficient data transfer between processes
- **Hot-Swappable Modules**: Shards can be loaded/unloaded at runtime

## Shard System

Shards are the fundamental building blocks of SigmaOS. They are hot-swappable kernel modules that provide system services.

### Shard Types

1. **Core Shards**: Essential kernel components (Rust)
   - S-MM: Memory Manager
   - S-SCHED: Scheduler
   - S-NET: Network Stack
   - S-FS: Filesystem
   - S-IPC: Inter-Process Communication
   - S-SEC: Security Manager
   - S-SYS: System Call Interface

2. **Essential Shards**: Hardware drivers (Rust/Zig)
   - GPU Driver
   - Storage Driver
   - Audio Driver
   - Network Driver
   - Input Driver

3. **Optional Shards**: Desktop and AI features (Nim)
   - Zenith Compositor
   - Desktop Shell
   - LLM Integration
   - Package Manager

4. **Infinite Shards**: Experimental features (Zig)

### Shard Loading

Shards are loaded by the Shard Loader:

```rust
// Pseudocode for shard loading
fn load_shard(shard_path: &str) -> Result<ShardHandle> {
    // 1. Load shard binary
    let binary = load_binary(shard_path)?;

    // 2. Verify signature
    verify_signature(&binary)?;

    // 3. Initialize shard
    let shard = Shard::new(binary)?;

    // 4. Grant capabilities
    grant_capabilities(&shard, shard.required_capabilities())?;

    // 5. Start shard
    shard.start()?;

    Ok(shard.handle())
}
```

### Shard Communication

Shards communicate through well-defined interfaces using capability-based IPC:

- **Capability Channels**: Secure, capability-protected communication channels
- **Shared Memory**: Zero-copy data transfer
- **Event Notifications**: Asynchronous event delivery

## Scheduler

SigmaOS uses a predictive multi-priority scheduler combining MLFQ, CFS, and EDF algorithms.

### EEVDF Algorithm

The Earliest Eligible Virtual Deadline First (EEVDF) scheduler provides:

- **O(1) Scheduling**: Constant-time scheduling decisions
- **Fairness**: Proportional share of CPU time
- **Low Latency**: Responsive to interactive tasks
- **Real-Time Support**: EDF for real-time tasks

### Task States

```rust
pub enum TaskState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}
```

### CPU Affinity

Tasks can be pinned to specific CPUs for performance optimization:

```bash
# Set CPU affinity
sigctl set-affinity <pid> <cpu-mask>

# View CPU affinity
sigctl get-affinity <pid>
```

### AI-Driven Scheduling

SigmaOS integrates machine learning for predictive scheduling:

- **Workload Prediction**: ML models predict task behavior
- **Dynamic Tuning**: Automatic scheduler parameter adjustment
- **Resource Optimization**: Intelligent resource allocation

## Memory Management

SigmaOS uses a multi-tier memory management system with custom allocators.

### Physical Memory

**Buddy Allocator**: Manages physical memory pages in power-of-two sizes

```rust
struct BuddyAllocator {
    orders: [Vec<Page>; MAX_ORDER],
    // Implementation details
}
```

### Virtual Memory

**Page Tables**: Multi-level page tables with capability-based access control

- **4KB Pages**: Standard page size
- **2MB Pages**: Large pages for performance
- **1GB Pages**: Huge pages for special use cases

### Address Spaces

Each process has its own virtual address space:

- **User Space**: Application memory (0x00000000 - 0x7FFFFFFF)
- **Kernel Space**: Kernel memory (0x80000000 - 0xFFFFFFFF)
- **Capability Regions**: Capability metadata

### Memory Protection

- **W^X Enforcement**: Pages are either writable or executable, never both
- **ASLR**: Address Space Layout Randomization
- **Capability Checks**: All memory operations require capabilities

## Inter-Process Communication

SigmaOS provides zero-latency IPC through multiple mechanisms.

### Message Passing

Synchronous and asynchronous message passing:

```rust
// Send message
send_message(destination, message, capability);

// Receive message
let message = receive_message(source, capability);
```

### Shared Memory

Zero-copy shared memory regions:

```rust
// Create shared memory
let shm = create_shared_memory(capability, size);

// Map shared memory
let ptr = map_shared_memory(shm, capability);
```

### Capability Channels

Secure, capability-protected communication channels:

- **Unidirectional**: One-way communication
- **Bidirectional**: Two-way communication
- **Multicast**: One-to-many communication

## Security Model

SigmaOS uses capability-based security with default-deny policy.

### Capabilities

Capabilities are 64-bit tokens that grant specific rights:

```rust
struct Capability {
    rights: u64,      // Access rights
    object: u64,      // Target object
    revocation: u64,  // Revocation token
}
```

### Capability Rights

- **Read**: Read access to object
- **Write**: Write access to object
- **Execute**: Execute access to object
- **Grant**: Grant capability to others
- **Revoke**: Revoke capability from others

### sigma_pledge

Process privilege reduction mechanism:

```bash
# Pledge capabilities
sigpledge <pid> "stdio rpath inet"

# View pledged capabilities
sigpledge <pid>
```

### sigma_unveil

Filesystem access restriction:

```bash
# Unveil directory
sigunveil <pid> "/path/to/dir" "rw"

# View unveiled paths
sigunveil <pid>
```

## System Calls

SigmaOS provides a capability-based system call interface.

### System Call Flow

1. **Application** invokes system call
2. **S-SYS** checks capability
3. **S-SYS** forwards to appropriate shard
4. **Shard** performs operation
5. **Result** returned to application

### System Call Categories

- **Process Management**: fork, exec, exit, wait
- **Memory Management**: mmap, munmap, mprotect
- **File I/O**: open, read, write, close
- **IPC**: send, receive, shared memory
- **Capability Management**: grant, revoke, check

### System Call Filtering

Per-process syscall filtering based on pledged capabilities:

```bash
# Set syscall filter
sigfilter <pid> "allow:read,write,exit"

# View syscall filter
sigfilter <pid>
```

## Performance Considerations

### Zero-Copy Operations

SigmaOS minimizes data copying through:

- **Shared Memory**: Direct memory access between processes
- **Capability Channels**: Zero-copy message passing
- **DMA**: Direct memory access for I/O

### Lock-Free Data Structures

High-performance concurrency primitives:

- **Atomic Operations**: Compare-and-swap, fetch-and-add
- **Lock-Free Queues**: MPSC and MPMC queues
- **RCU**: Read-Copy-Update for read-heavy workloads

### Cache Optimization

- **Cache-Aware Allocation**: Memory allocation optimized for cache
- **Prefetching**: Hardware prefetch hints
- **NUMA Awareness**: Non-Uniform Memory Access optimization

## Debugging

### Kernel Debugging

```bash
# Enable kernel debugging
sigctl debug on

# View kernel logs
sigctl log view

# Trace system calls
sigtrace <pid>
```

### Shard Debugging

```bash
# List loaded shards
sigshard list

# View shard status
sigshard status <shard-name>

# Reload shard
sigshard reload <shard-name>
```

### Performance Profiling

```bash
# Enable profiling
sigprof on

# View profile
sigprof view

# Reset profile
sigprof reset
```

## Further Reading

- [Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)
- [Security Policy](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Policy)
- [Development Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development-Roadmap)
- [Contributing](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing)

---

*Last Updated: 2026-07-14*

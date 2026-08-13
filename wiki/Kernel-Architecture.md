# SigmaOS Kernel Architecture

## Overview

SigmaOS is an AI-first microkernel operating system designed for modern hardware with a focus on security, performance, and AI-native orchestration. This document describes the kernel architecture, its components, and design principles.

## Design Principles

### Microkernel Architecture
- Minimal kernel space with most services in user space
- Inter-process communication via message passing
- Capability-based security model
- Modular driver architecture

### AI-Native Design
- Built-in AI orchestration and resource management
- Adaptive scheduling based on workload patterns
- Self-healing capabilities with anomaly detection
- Natural language to CLI translation

### Object-Oriented Programming
- Rust traits for polymorphic interfaces
- Struct-based abstractions for hardware devices
- Type-safe driver development
- Memory-safe kernel code

## Kernel Components

### 1. Scheduler

**Location:** `kernel/scheduler/round_robin_scheduler.rs`

The scheduler manages CPU time allocation across processes using a round-robin algorithm with priority support.

**Features:**
- Round-robin task scheduling
- Priority-based preemption
- Time quantum management
- Task state tracking (Running, Ready, Blocked)
- Load balancing across CPU cores

**Data Structures:**
```rust
pub struct Task {
    pub id: TaskId,
    pub priority: u8,
    pub state: TaskState,
    pub quantum: u32,
    pub cpu_affinity: u32,
}

pub struct Scheduler {
    pub ready_queue: Vec<Task>,
    pub current_task: Option<Task>,
    pub time_slice: u32,
}
```

### 2. Memory Management

**Location:** `kernel/mm/`

The memory management subsystem provides physical and virtual memory allocation.

#### Physical Memory Allocator (Buddy Allocator)
**Location:** `kernel/mm/buddy_allocator.rs`

- Buddy system for physical page allocation
- Efficient fragmentation handling
- Support for different page sizes (4KB, 2MB, 1GB)
- Memory coalescing on free

#### Slab Allocator
**Location:** `kernel/mm/slab_allocator.rs`

- Object caching for frequently allocated types
- Per-CPU slab caches
- Reduced fragmentation for small objects
- Fast allocation/deallocation

#### Page Table Walker
**Location:** `kernel/mm/page_table_walker.rs`

- Virtual to physical address translation
- Page table management
- Memory protection flags
- TLB management

### 3. Interrupt Handling

**Location:** `kernel/hal/interrupt_controller.rs`

The interrupt handling subsystem manages hardware interrupts and exceptions.

**Features:**
- APIC/PIC initialization
- Interrupt routing
- Interrupt handler registration
- Nested interrupt support
- Interrupt affinity management

### 4. System Call Interface

**Location:** `kernel/syscalls/syscall_dispatcher.rs`

The syscall interface provides controlled access to kernel services from user space.

**Supported Syscalls:**
- File I/O: open, read, write, close, lseek, stat
- Process: spawn, wait, exit, getpid, kill
- IPC: pipe, socket, connect, bind, listen, accept
- Memory: mmap, munmap, brk
- Signal: sigaction, sigprocmask, kill

### 5. Filesystem Layer

**Location:** `fs/`

The filesystem layer provides a unified interface for different filesystem types.

#### Virtual Filesystem (VFS)
**Location:** `fs/vfs.rs`

- Unified file interface
- File descriptor management
- Path resolution
- File locking

#### SigmaFS
**Location:** `fs/sovereign_fs/`

- Copy-on-write filesystem
- Snapshot support
- Compression
- Deduplication

#### Ext2/3/4 Support
**Location:** `fs/ext2.rs`

- Standard Linux filesystem support
- Journaling (ext3/4)
- Extended attributes
- ACLs

### 6. Driver Framework

**Location:** `drivers/`

The driver framework provides a trait-based architecture for hardware drivers.

#### Base Traits
**Location:** `drivers/device_base.rs`

```rust
pub trait Device {
    fn init(&mut self) -> i32;
    fn reset(&mut self) -> i32;
    fn get_info(&self) -> DeviceInfo;
}
```

#### Network Drivers
**Location:** `drivers/net/`

- EthernetDevice trait for network cards
- EthernetPhy trait for PHY management
- Supported: e1000e, r8169, virtio-net

#### Storage Drivers
**Location:** `drivers/storage/`

- StorageDevice trait for storage devices
- Supported: AHCI (SATA), NVMe, virtio-blk

#### GPU Drivers
**Location:** `drivers/gpu/`

- GpuDevice trait for graphics cards
- DRM/KMS layer for display management
- Supported: amdgpu, i915, nouveau, virtio-gpu

#### USB Drivers
**Location:** `drivers/usb/`

- UsbController trait for USB controllers
- Supported: xHCI, EHCI, UHCI, OHCI

#### Input Drivers
**Location:** `drivers/input/`

- InputDevice trait for input devices
- Supported: PS/2 keyboard/mouse, HID, touchpads

### 7. Security Subsystem

**Location:** `security/`

The security subsystem provides access control and sandboxing.

#### Mandatory Access Control (MAC)
**Location:** `security/mac.rs`

- Capability-based access control
- Process sandboxing
- Resource isolation
- Policy enforcement

#### Secure Boot
**Location:** `boot/sigma_secureboot.rs`

- UEFI Secure Boot integration
- Key management (PK, KEK, db, dbx)
- Bootloader/kernel verification
- TPM integration for measured boot

### 8. Cryptographic Primitives

**Location:** `crypto/`

The cryptographic subsystem provides essential crypto operations.

**Features:**
- SHA-256 hash function
- SHA-512 hash function
- Ed25519-like signing
- PGP key generation and management
- Secure key derivation

### 9. AI Subsystem

**Location:** `ai/`

The AI subsystem provides AI-native orchestration.

#### Workflow Orchestrator
**Location:** `ai/sigma_workflow_orchestrator.rs`

- AI-driven task scheduling
- Resource optimization
- Anomaly detection
- Self-healing

#### Natural Language to CLI
**Location:** `ai/nl2cli/sigma_nl2cli.rs`

- Natural language command translation
- Intent recognition
- Command generation
- Error explanation

#### Error Explanation
**Location:** `ai/error_explanation/sigma_error_explainer.rs`

- AI-powered error analysis
- Root cause identification
- Suggested fixes
- Documentation generation

### 10. POSIX Compatibility Layer

**Location:** `posix/`

The POSIX compatibility layer provides essential POSIX APIs for porting Unix/Linux software.

**Components:**
- File I/O primitives (open, read, write, close, lseek, stat)
- Process management (spawn, wait, exit, getpid, kill)
- Signal handling (sigaction, sigprocmask)
- IPC (pipe, sockets)
- Minimal libc subset (string, memory, I/O functions)

## Boot Process

### 1. UEFI Bootloader
**Location:** `bootloader/sigma_boot_efi.rs`

- UEFI protocol initialization
- Memory map acquisition
- Kernel loading
- Handoff to kernel

### 2. Kernel Initialization
- CPU initialization
- Memory setup
- Interrupt controller setup
- Driver initialization
- Filesystem mount
- Init process spawn

### 3. Init System
**Location:** `init/`

- Service management
- Runlevel control
- Process supervision
- System logging

## Inter-Process Communication

### Message Passing
- Asynchronous message queues
- Synchronous RPC
- Shared memory regions

### Pipes
- Anonymous pipes
- Named pipes (FIFOs)

### Sockets
- Unix domain sockets
- TCP/IP sockets
- UDP sockets

## Memory Layout

### Kernel Space
- 0xFFFFFFFF80000000 - 0xFFFFFFFFFFFFFFFF: Kernel code and data
- 0xFFFF800000000000 - 0xFFFF800001000000: Physical memory map
- 0xFFFF800002000000 - 0xFFFF800003000000: Device MMIO regions

### User Space
- 0x0000000000000000 - 0x00007FFFFFFFFFFF: User code and data
- 0x0000800000000000 - 0x0000FFFFFFFFFFFF: Shared libraries
- 0xFFFF800000000000 - 0xFFFFFFFFFFFFFFFF: Kernel space

## Performance Optimizations

### 1. Lock-Free Data Structures
- RCU (Read-Copy-Update) for read-heavy workloads
- Per-CPU data structures
- Atomic operations

### 2. Zero-Copy Operations
- Direct memory access for I/O
- Shared memory for IPC
- Page flipping for graphics

### 3. Adaptive Scheduling
- Priority inheritance
- CPU affinity
- Load balancing

## Security Features

### 1. Capability-Based Security
- Fine-grained permissions
- No global root user
- Principle of least privilege

### 2. Memory Protection
- Page-level protection
- ASLR (Address Space Layout Randomization)
- Stack canaries
- NX bit enforcement

### 3. Secure Boot
- Chain of trust verification
- TPM measured boot
- Kernel module signing

### 4. Sandbox Isolation
- Process sandboxing
- Filesystem namespace isolation
- Network namespace isolation

## Debugging and Tracing

### Kernel Logging
**Location:** `kernel/log.rs`

- Log levels (DEBUG, INFO, WARN, ERROR)
- Per-component logging
- Ring buffer for early boot logs

### Crash Analysis
- Kernel panic handling
- Stack trace capture
- Memory dump generation
- Post-mortem analysis

### Performance Profiling
- CPU profiling
- Memory profiling
- I/O profiling
- System call tracing

## Future Enhancements

### 1. Live Patching
- Runtime kernel updates
- Function hotpatching
- Safe rollback

### 2. Heterogeneous Computing
- GPU compute integration
- FPGA acceleration
- Neural processing units

### 3. Distributed Systems
- Cluster management
- Distributed filesystem
- Consensus algorithms

## References

- [SigmaOS Architecture](../Architecture.md)
- [Driver Development Guide](../drivers/DRIVER_DEVELOPMENT_GUIDE.md)
- [POSIX Compatibility](../posix/README.md)
- [Security Documentation](../docs/Security.md)

## License

SigmaOS kernel is licensed under MIT License. See [LICENSE](../LICENSE) for details.

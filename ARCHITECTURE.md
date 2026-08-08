# SigmaOS Architecture

## Overview

SigmaOS is designed as a hybrid microkernel/microkernel operating system that combines the security benefits of microkernels with the performance characteristics of monolithic kernels. The architecture emphasizes:

- **Security by Design**: Capability-based access control throughout the system
- **Performance**: Zero-copy operations, cache-aware scheduling, minimal overhead
- **Minimal Dependencies**: Rust-native implementations without external C libraries
- **Compatibility**: Extensive compatibility layers for Linux, BSD, and Windows

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Userland Layer                          │
├─────────────────────────────────────────────────────────────┤
│  Applications  │  Container Runtime  │  System Services   │
├─────────────────────────────────────────────────────────────┤
│          Compatibility Layers (Linux/BSD/Windows)          │
├─────────────────────────────────────────────────────────────┤
│              System Interface (Syscalls)                   │
├─────────────────────────────────────────────────────────────┤
│                     Kernel Layer                            │
├─────────────────────────────────────────────────────────────┤
│  IPC  │  Scheduler  │  Memory  │  FS  │  Network  │  Security│
├─────────────────────────────────────────────────────────────┤
│                 Hardware Abstraction Layer (HAL)            │
├─────────────────────────────────────────────────────────────┤
│                    Hardware Layer                           │
└─────────────────────────────────────────────────────────────┘
```

## Kernel Architecture

### 1. Hardware Abstraction Layer (HAL)

The HAL provides a unified interface to hardware platforms:

**Components:**
- **Interrupt Controller**: APIC/PIC management with deterministic interrupt handling
- **CPU Features**: CPUID detection, feature flags, microcode updates
- **Memory Controller**: DRAM controller interface, NUMA topology
- **Device Enumeration**: PCI/PCIe bus scanning, device discovery
- **Power Management**: ACPI/SMI interfaces, power states

**Key Features:**
- Deterministic interrupt handling with priority queues (Critical, High, Normal, Low)
- Interrupt latency monitoring and bounds enforcement
- CPU topology awareness (L1/L2/L3 cache, NUMA nodes)
- Power state management (C-states, P-states)

### 2. Memory Management

SigmaOS uses a hierarchical memory management system:

**Physical Memory:**
- **Buddy Allocator**: Physical page allocation with order-based blocks
- **NUMA-Aware Allocation**: Node-local memory allocation
- **Memory Zones**: DMA, Normal, HighMem zones

**Virtual Memory:**
- **Page Table Walker**: Multi-level page table management
- **Demand Paging**: Lazy page allocation with page faults
- **Copy-on-Write**: Fork optimization with shared pages
- **Memory Mapping**: File-backed and anonymous mappings

**Kernel Memory:**
- **Slab Allocator**: Efficient small object allocation
- **Non-Paged Pool**: Fixed memory pool for critical operations
- **Paged Pool**: Swappable kernel memory
- **Pool Tags**: Memory leak detection and tracking

**Memory Protection:**
- **Page Permissions**: Read/Write/Execute permissions
- **ASLR**: Address space layout randomization
- **Stack Canaries**: Buffer overflow protection
- **Guard Pages**: Protection against overflows

### 3. Process Management

**Process Model:**
- **Capability-Based**: Processes hold capability tokens for resources
- **Lightweight**: Minimal process context
- **Isolation**: Strong process separation

**Scheduler:**
- **Cache-Aware Scheduling**: CPU cache locality optimization
- **NUMA-Aware**: Node-aware process placement
- **Work-Stealing**: Load balancing across CPUs
- **Real-Time Support**: EDF (Earliest Deadline First) for real-time tasks
- **EEVDF**: Earliest Eligible Virtual Deadline First for interactive tasks

**Process Lifecycle:**
- **Spawn**: Capability-checked process creation
- **Signal**: Capability-checked signal delivery
- **Terminate**: Graceful and forced termination
- **Wait**: Parent process waiting for children

### 4. Inter-Process Communication (IPC)

**Zero-Copy IPC:**
- **Shared Memory Ring Buffers**: Lock-free message passing
- **Latency Target**: <100μs message delivery
- **Message Size**: Up to 1KB per message
- **Channels**: Up to 64 concurrent channels

**IPC Mechanisms:**
- **Message Passing**: Asynchronous message queues
- **Shared Memory**: Direct memory sharing
- **Sockets**: Network-style IPC
- **Pipes**: Unix pipe compatibility

**Security:**
- **Capability Checking**: IPC capability verification
- **Message Validation**: Malformed input detection
- **Sandboxing**: Process isolation enforcement

### 5. File System

**SigmaFS 2.0:**
- **Merkle-Tree Integrity**: Content-addressed storage
- **Snapshots**: Sub-millisecond snapshot creation
- **Deduplication**: 4KB block-level deduplication
- **Compression**: Transparent compression
- **Encryption**: Per-file encryption support

**VFS (Virtual File System):**
- **Unified Interface**: Common file system API
- **Mount Points**: Multiple file system mounting
- **Namespace**: Per-process file system namespace
- **Symlinks**: Smart symlinks with context expansion

**Supported File Systems:**
- **SigmaFS**: Native high-performance file system
- **ext4**: Linux compatibility
- **FAT32**: Legacy compatibility
- **NTFS**: Windows compatibility
- **tmpfs**: Memory-backed file system

### 6. Network Stack

**Zero-Copy Networking:**
- **DPDK-Style Processing**: Userspace packet processing
- **Zero-Copy Buffers**: Direct memory access
- **Batch Processing**: Packet batching for efficiency
- **Hardware Offload**: RSS, checksum offload

**Network Protocols:**
- **TCP/IP**: Full TCP/IP stack implementation
- **UDP**: User Datagram Protocol
- **IPv6**: Next-generation Internet Protocol
- **ICMP**: Internet Control Message Protocol

**Network Security:**
- **Firewall**: Stateful packet filtering
- **TLS/SSL**: Secure transport layer
- **VPN**: Virtual private networking
- **Intrusion Detection**: Network anomaly detection

### 7. Security Architecture

**Capability System:**
- **Fine-Grained Access**: Per-resource capabilities
- **Delegation**: Capability transfer between processes
- **Expiration**: Time-limited capabilities
- **Revocation**: Immediate capability revocation

**Security Features:**
- **Pledge/Unveil**: OpenBSD-inspired process sandboxing
- **SELinux MAC**: Mandatory access control
- **AppArmor**: Profile-based confinement
- **Container Isolation**: Process namespace isolation
- **Secure Boot**: UEFI Secure Boot with TPM 2.0
- **Kernel Hardening**: Attack surface reduction

**Intrusion Detection:**
- **Anomaly Detection**: Behavior-based detection
- **Signature Matching**: Known attack patterns
- **Real-Time Monitoring**: Continuous security monitoring
- **Automated Response**: Automated threat response

## Userland Architecture

### 1. Init System

**Systemd-Inspired Init:**
- **Service Management**: Dependency-based service startup
- **Socket Activation**: On-demand service activation
- **Target States**: System state management
- **Journal Logging**: Centralized logging

### 2. Package Management

**SigPkg:**
- **Universal Packages**: Cross-platform package format
- **Dependency Resolution**: Automatic dependency management
- **Signing**: Cryptographic package signing
- **Repositories**: Multiple package sources
- **Updates**: Automatic security updates

### 3. Shell

**SigmaSH:**
- **Modern Features**: Tab completion, history, aliases
- **Scripting**: Advanced scripting capabilities
- **Compatibility**: Bash/Zsh compatibility
- **Built-in Commands**: Efficient built-in utilities

### 4. Container Runtime

**OCI-Compliant Runtime:**
- **Container Images**: Docker image support
- **Namespace Isolation**: Process, network, filesystem isolation
- **Cgroups**: Resource control and limits
- **Seccomp**: syscall filtering
- **AppArmor**: Profile-based confinement

## Compatibility Layers

### Linux Compatibility

**Kernel Interface:**
- **Syscall Compatibility**: Linux syscall translation
- **Driver Interface**: Linux kernel module compatibility
- **procfs**: /proc filesystem emulation
- **sysfs**: /sys filesystem emulation

**Userland Compatibility:**
- **glibc Compatibility**: GNU C library interface
- **Binary Compatibility**: ELF binary execution
- **System V IPC**: Shared memory, semaphores, message queues

### BSD Compatibility

**FreeBSD:**
- **Syscall Interface**: FreeBSD syscall translation
- **Jails**: Process isolation
- **Capsicum**: Capability-based security

**OpenBSD:**
- **Pledge/Unveil**: Process sandboxing
- **kqueue**: Event notification
- **Secure Memory**: Memory protection

### Windows Compatibility

**Driver Interface:**
- **WDM**: Windows Driver Model compatibility
- **IRP System**: I/O Request Packet handling
- **Registry**: Registry emulation

**Userland:**
- **PE Format**: Portable Executable loading
- **Win32 API**: Windows API compatibility
- **COM**: Component Object Model

## Performance Optimization

### 1. Cache Optimization

**Cache-Aware Scheduling:**
- **Cache Hot/Cold Tracking**: Process cache affinity
- **CPU Pinning**: Process-CPU affinity
- **Cache Line Alignment**: Data structure alignment
- **Prefetching**: Hardware prefetch hints

### 2. Memory Optimization

**Zero-Copy Operations:**
- **DMA**: Direct memory access
- **Memory Mapping**: File-backed memory mapping
- **Buffer Pooling**: Reusable buffer pools
- **Allocation Batching**: Bulk allocation

### 3. I/O Optimization

**NVMe Optimization:**
- **Queue Depth**: 256+ I/O queues
- **Interrupt Moderation**: Interrupt coalescing
- **Polling**: Polled I/O for low latency
- **Async I/O**: Asynchronous I/O operations

### 4. Network Optimization

**DPDK-Style Processing:**
- **Userspace Networking**: Kernel bypass
- **CPU Affinity**: Network-CPU pinning
- **Huge Pages**: Large page support
- **Batch Processing**: Packet batching

## Security Model

### 1. Principle of Least Privilege

- **Capability-Based Access**: Minimal necessary capabilities
- **Default Deny**: Access denied by default
- **Delegation**: Explicit capability transfer
- **Revocation**: Immediate access revocation

### 2. Defense in Depth

- **Memory Safety**: Rust ownership model
- **Process Isolation**: Strong separation
- **Kernel Hardening**: Minimal attack surface
- **Secure Boot**: Chain of trust
- **Runtime Protection**: ASLR, stack canaries

### 3. Secure by Default

- **Sandboxing**: Default process sandboxing
- **Encryption**: Default data encryption
- **Authentication**: Multi-factor authentication
- **Audit Logging**: Comprehensive logging

## Development Roadmap

### Short-Term (1-6 months)
- Complete NUMA optimization
- Implement work-stealing queues
- Add demand-paging with CoW
- Implement zero-copy network stack
- Complete SigmaFS 2.0

### Medium-Term (7-18 months)
- Add comprehensive security features
- Implement AI/ML integration
- Optimize performance
- Add advanced filesystem features
- Improve compatibility layers

### Long-Term (19-36 months)
- Distributed computing support
- Real-time capabilities
- HPC features
- Confidential computing
- Production readiness

## Conclusion

SigmaOS represents a new approach to operating system design, combining the best features of existing systems while introducing innovative security and performance improvements. The architecture is designed to be modular, extensible, and maintainable while providing a secure and high-performance platform for modern applications.

---

For more information, see:
- [README.md](README.md)
- [SECURITY.md](SECURITY.md)
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

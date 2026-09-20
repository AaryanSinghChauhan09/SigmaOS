# SigmaOS Kernel

This page consolidates all kernel documentation for SigmaOS.

## Overview

SigmaOS implements a modular, hybrid-architecture kernel inspired by Linux, BSD, and Windows kernel designs, with support for both `std` and `#![no_std]` environments. The kernel provides comprehensive subsystem support for process management, memory management, filesystem operations, security, and device drivers.

## Kernel Architecture

### Design Philosophy

#### Hybrid std/no_std Architecture
- **User-space components** use `std::` for full OS functionality
- **Kernel modules** pursue `#![no_std]` compatibility where feasible
- **Modular design** allows selective subsystem isolation
- **Zero external dependencies** where possible for critical components

#### Modular Kernel System
- **Loadable kernel modules** (LKMs) for dynamic functionality
- **Microkernel influences** for IPC and driver isolation
- **Monolithic optimizations** for performance-critical paths
- **Hybrid approach** balancing performance and security

### Core Subsystems

#### Process Management
- **Scheduler** - EEVDF (Earliest Eligible Virtual Deadline First) scheduler
- **Process creation** - fork/exec variants from Linux/BSD
- **Thread management** - POSIX threads (pthreads) compatibility
- **Process isolation** - Memory protection and capability controls
- **Signal handling** - POSIX signal semantics

#### Memory Management
- **Virtual memory** - Paging, segmentation, and memory protection
- **Physical memory** - Page frame allocation and management
- **Memory mapping** - mmap, mprotect, and address space layout
- **Swap management** - Swap-out mechanisms with sub-1ms restoration
- **Cgroups** - Control groups for resource isolation

#### Filesystem Layer
- **VFS (Virtual Filesystem Switch)** - Abstract filesystem interface
- **Supported filesystems** - ext4, Btrfs, ZFS, F2FS, FAT32, NTFS
- **Copy-on-Write (CoW)** - Snapshot-based filesystem operations
- **Symlink engine** - Advanced symbolic link management
- **POSIX compatibility** - Standard filesystem operations

#### Security Subsystems
- **Capability-based security** - Fine-grained permission delegation
- **Landlock v5** - Linux filesystem sandboxing
- **Capsicum** - FreeBSD capability-based security
- **Pledge/Unveil** - OpenBSD security promises
- **eBPF/seccomp** - System call filtering and monitoring

#### Device Drivers
- **Universal driver suite** - Cross-platform driver framework
- **WDM (Windows Driver Model)** compatibility
- **Linux driver compatibility** - Character, block, and network drivers
- **FreeBSD driver support** - Device tree and probe methods
- **Hot-plug support** - Dynamic device detection and loading

## Kernel Features

### Advanced Capabilities

#### Dynamic Kernel Module Loading
- **Kernel module system** for runtime extensibility
- **Module signing** - Cryptographic verification of modules
- **Dependency resolution** - Automatic module dependency handling
- **Sandboxed modules** - Isolated module execution environments

#### Low-Overhead Observability
- **eBPF runtime** - Efficient kernel tracing and instrumentation
- **System call auditing** - Comprehensive syscall monitoring
- **Performance counters** - Hardware performance monitoring
- **Ftrace/LTTng** - Kernel tracing infrastructure

#### Performance Tuning
- **CPU governor** - Dynamic frequency scaling
- **I/O scheduler** - Multi-queue block I/O scheduling
- **Network stack optimization** - XDP and zero-copy networking
- **Memory compaction** - Defragmentation and optimization

#### Syscall Enforcement
- **Seccomp-BPF** - System call filtering
- **Landlock** - Filesystem access control
- **Audit subsystem** - Security event logging
- **Capability checks** - POSIX capability verification

### Kernel Timers
- **High-resolution timers** - nanosecond precision
- **Clock interrupt handling** - Tickless kernel operation
- **Timer wheels** - Efficient timer management
- **POSIX timers** - timer_create, timer_settime compatibility

## AI Agent Kernel Guidelines

### Bolt (Performance Persona)
**Mission:** Kernel performance optimization

**Focus Areas:**
- O(1) algorithms in critical paths
- Lock-free data structures (RCU, atomic operations)
- Cache-aware memory layouts
- Zero-copy operations
- Adaptive scheduling policies

**Critical Learning Journal:** `.jules/bolt.md`

### Kernel Verification Checklist
Before committing kernel changes, verify:
1. No memory leaks in allocation/deallocation paths
2. Proper error handling in all kernel paths
3. Thread-safe access to shared data structures
4. No deadlock potential in lock acquisition order
5. Proper bounds checking on all kernel buffers
6. Safe FFI interactions with hardware
7. Interrupt context constraints respected
8. Memory barriers and atomic operations used correctly

## Kernel Testing

### Unit Testing
```bash
# Run kernel-specific tests
cargo test --lib kernel

# Test specific kernel subsystems
cargo test --lib kernel::paging
cargo test --lib kernel::scheduler
cargo test --lib kernel::ipc
```

### Integration Testing
- **Kernel module loading** - Test module load/unload cycles
- **Stress testing** - High-load kernel operation
- **Fault injection** - Error path verification
- **Performance benchmarks** - Kernel subsystem performance

### Verification Commands
```bash
# Build kernel modules
rustc --edition=2021 --crate-type staticlib src/kernel/mod.rs

# Test kernel compilation
cargo build --lib

# Run full test suite
./run_sigma_tests.sh
```

## Kernel Best Practices

### Memory Safety
- Use safe Rust abstractions where possible
- Minimal unsafe code with extensive documentation
- Proper error handling for allocation failures
- Memory barrier usage for SMP systems
- DMA buffer management for device drivers

### Concurrency
- Lock-free algorithms where appropriate
- RCU for read-mostly data structures
- Proper lock ordering to prevent deadlocks
- Interrupt context considerations
- Memory ordering semantics

### Performance
- Profile before optimizing
- Consider cache locality
- Minimize system call overhead
- Use zero-copy where possible
- Batch operations for efficiency

### Security
- Validate all user inputs from kernel space
- Sanitize filesystem paths
- Check capabilities before privileged operations
- Audit security-relevant kernel operations
- Implement proper error handling

## Documentation References

For detailed kernel implementation specifications:
- [Architecture](ARCHITECTURE.md)
- [Security](SECURITY.md)
- [Package Management](Package-Management.md)
- [Roadmap](ROADMAP.md)
- [Filesystem](Filesystem.md)
- [Process Management](process-management.md)
- [Memory Management](memory-management.md)

## Contributing

Kernel development follows SigmaOS agent guidelines:
- **Bolt**: Performance optimization (O(1) algorithms, lock-free structures)
- **Sentinel**: Security vulnerability remediation in kernel code
- **Palette**: Kernel UX improvements (better error messages, clearer diagnostics)

### Kernel Commit Guidelines
- Describe kernel subsystem affected in commit messages
- Include performance impact when applicable
- Reference relevant kernel subsystem documentation
- Test kernel changes under various load conditions
- Update kernel documentation

---

*This page consolidates the following individual kernel documents:*
- AGENTS_KERNEL_MANAGEMENT.md
- AI_AGENT_KERNEL_MANAGEMENT_ARCHITECTURE.md
- AI_AGENT_KERNEL_MANAGEMENT_GUIDELINES.md
- Dynamic-Kernel-Module-Loading.md
- Kernel-Architecture.md
- kernel.md
- Kernel-Syscall-Enforcement.md
- Kernel-Timers.md
- Low-Overhead-Kernel-Observability.md
- Performance-Tuning-and-Kernel.md

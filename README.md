# SigmaOS

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)

## Overview

SigmaOS is a next-generation operating system built with Rust, designed for security, performance, and minimal dependencies. It draws inspiration from Linux, BSD, and modern microkernel architectures while implementing innovative features for the post-UNIX era.

### Key Features

- **Microkernel Architecture**: Capability-based security with minimal trusted computing base
- **Zero-Copy IPC**: Sub-100μs latency inter-process communication
- **Cache-Aware Scheduling**: NUMA-aware CPU scheduling with cache locality optimization
- **Comprehensive Compatibility**: Linux, BSD, and Windows binary compatibility layers
- **Memory Safety**: Rust-native memory management with no custom allocators
- **Security-First Design**: Capability tokens, pledge/unveil inspired security, TPM 2.0 integration
- **Universal Driver Support**: Cross-platform driver framework with hardware abstraction
- **Modern Package Management**: SigPkg universal package system with dependency resolution
- **Container Runtime**: OCI-compliant container orchestration
- **AI/ML Integration**: Native support for AI workloads and inference

## Architecture

SigmaOS uses a hybrid microkernel/microkernel architecture with the following components:

### Kernel Subsystems

- **HAL (Hardware Abstraction Layer)**: Interrupt controllers, APIC/PIC, deterministic interrupt handling
- **Memory Management**: Buddy allocator, slab allocator, NUMA-aware paging, CoW support
- **Process Management**: Capability-based process control, cache-aware scheduling, work-stealing queues
- **IPC System**: Zero-copy message passing, shared memory ring buffers
- **File System**: SigmaFS 2.0 with Merkle-tree integrity, snapshot support, deduplication
- **Network Stack**: Zero-copy networking with DPDK-style processing
- **Security**: Capability delegation, intrusion detection, vulnerability scanning

### Userland Components

- **Systemd-Inspired Init**: Service management with dependency resolution
- **Package Manager**: SigPkg universal package system
- **Shell**: SigmaSH with modern command features
- **Container Runtime**: OCI-compliant container orchestration
- **Desktop Environment**: Wayland-based compositor with accessibility support

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- QEMU (for testing)
- LLVM/Clang (for cross-compilation)

### Building

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the kernel
cargo build --release

# Build userland
cd userland
cargo build --release
```

### Running

```bash
# Run in QEMU
./scripts/qemu_smoke_test.py

# Or build an ISO
./scripts/build-iso.sh
```

## Documentation

- [Architecture Documentation](ARCHITECTURE.md)
- [Security Documentation](SECURITY.md)
- [Contributing Guide](CONTRIBUTING.md)
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## Roadmap

### Phase 1: Foundation Hardening (Months 1-3)
- [x] Capability-token delegation system
- [x] Deterministic interrupt handling
- [x] Zero-copy IPC system
- [x] Comprehensive fuzzing harness
- [x] Cache-aware scheduling algorithm
- [ ] NUMA optimization
- [ ] Work-stealing queues
- [ ] Boot time validation (<2.5s)
- [ ] Demand-paging with CoW
- [ ] Zero-copy network stack

### Phase 2: Memory Management (Months 4-6)
- [ ] SigmaFS 2.0 specification
- [ ] Merkle-tree data integrity
- [ ] Sub-millisecond snapshots
- [ ] Incremental backup
- [ ] 4KB block deduplication
- [ ] Deadline I/O scheduling
- [ ] NVMe queue optimization (256+ queues)

### Phase 3: Security Hardening (Months 7-9)
- [ ] dm-verity equivalent
- [ ] TPM 2.0 authentication
- [ ] UEFI Secure Boot integration
- [ ] SELinux-inspired MAC
- [ ] AppArmor integration
- [ ] Container isolation
- [ ] Kernel address space layout randomization

### Phase 4: Ecosystem Development (Months 10-12)
- [ ] Universal package manager
- [ ] Container registry
- [ ] Driver certification program
- [ ] Cross-platform compatibility
- [ ] Documentation suite
- [ ] Developer tools

### Phase 5: Performance Optimization (Months 13-18)
- [ ] LTO and codegen optimization
- [ ] musl libc integration
- [ ] Profile-guided optimization
- [ ] JIT compilation
- [ ] Hardware acceleration
- [ ] NUMA-aware allocation

### Phase 6: AI Integration (Months 19-24)
- [ ] Native AI/ML support
- [ ] Neural network inference
- [ ] Auto-tuning systems
- [ ] Predictive scheduling
- [ ] Intelligent resource management
- [ ] ML-based security

### Phase 7: Advanced Features (Months 25-30)
- [ ] Distributed computing
- [ ] Real-time capabilities
- [ ] High-performance computing
- [ ] Trusted execution environments
- [ ] Confidential computing
- [ ] Quantum-resistant cryptography

### Phase 8: Production Readiness (Months 31-36)
- [ ] Comprehensive testing
- [ ] Performance validation
- [ ] Security auditing
- [ ] Compliance certification
- [ ] Enterprise features
- [ ] Commercial support

## Compatibility

SigmaOS provides compatibility layers for:

- **Linux**: Full binary compatibility via Linux kernel module interface
- **BSD**: FreeBSD and OpenBSD syscall compatibility
- **Windows**: Windows driver compatibility and API translation
- **Docker**: OCI container runtime support
- **Systemd**: Service unit compatibility

## Security

SigmaOS implements multiple security layers:

1. **Capability-Based Security**: Fine-grained access control with delegatable tokens
2. **Memory Safety**: Rust's ownership model prevents memory corruption
3. **Process Isolation**: Capability-based process separation
4. **Kernel Hardening**: Minimal attack surface, static analysis
5. **Secure Boot**: UEFI Secure Boot with TPM 2.0
6. **Intrusion Detection**: Real-time anomaly detection
7. **Vulnerability Scanning**: Automated security auditing

## Performance Targets

- **Boot to Shell**: <2.5 seconds
- **IPC Latency**: <100μs
- **Context Switch**: <1μs
- **System Call Overhead**: <500ns
- **Memory Allocation**: <100ns
- **File I/O**: NVMe optimized
- **Network Throughput**: DPDK-level performance

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `./scripts/regression_check.sh`
5. Submit a pull request

## Testing

SigmaOS includes comprehensive test suites:

```bash
# Run all tests
./run_sigma_tests.sh

# Run specific test suites
./scripts/app_regression_test.sh
./scripts/accelerators_diagnostics.sh
./scripts/format_stress_test.sh
```

## License

SigmaOS is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

SigmaOS draws inspiration from:
- Linux kernel and distributions
- BSD family (FreeBSD, OpenBSD, NetBSD)
- Windows NT architecture
- MINIX microkernel
- seL4 microkernel
- Plan 9 from Bell Labs
- OpenBSD security features
- systemd service management
- Docker containerization

## Contact

- **GitHub**: https://github.com/AaryanSinghChauhan09/SigmaOS
- **Issues**: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- **Discussions**: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=AaryanSinghChauhan09/SigmaOS&type=Date)](https://star-history.com/#AaryanSinghChauhan09/SigmaOS&Date)

---

**SigmaOS** - Building the future of operating systems, one commit at a time.

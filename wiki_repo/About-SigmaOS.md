# About SigmaOS

SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow.

## Philosophy

SigmaOS is designed with several core principles:

### Security First
- Rust-enforced memory safety across the entire codebase
- Capability-based security model (default-deny permissions)
- Post-quantum cryptography (Kyber-1024, Dilithium-5)
- Defense-in-depth with multiple security layers

### Performance Excellence
- Lock-free data structures and algorithms
- Zero-copy I/O paths where possible
- Optimized for modern hardware (x86_64, AArch64)
- Real-time scheduling capabilities

### Simplicity and Clarity
- Zero external dependencies in core components
- Clear, declarative system configuration
- Atomic updates with instant rollback
- Transparent package management

### Developer Experience
- Rust-based development environment
- Comprehensive testing infrastructure
- Clear documentation and examples
- Active community and contribution process

## Key Features

### Zenith Desktop Environment
- Keyboard-driven tiling compositor
- Bolt ⚡ fast launcher with agentic steering
- Palette 🎨 dynamic theme engine
- Sentinel 🛡️ capability-based exec guard
- Built-in accessibility support

### SigmaPkg Package System
- Content-addressed storage (CAS)
- Atomic updates and rollbacks
- Multi-distro format support (RPM, DEB, Arch, etc.)
- Post-quantum package signatures
- Universal package format (SigmaAppImage)

### System Architecture
- std-based Rust architecture (committed September 2026)
- Microkernel-inspired design
- Capability-based access control
- Multi-architecture support (x86_64, AArch64, RISC-V)
- Hardware-enforced security (TPM 2.0, Secure Boot)

### Security Model
- Defense-in-depth with multiple layers
- OpenBSD pledge/unveil syscall restriction
- FreeBSD Capsicum capability mode
- Linux Landlock LSM integration
- Formal memory safety guarantees

## Technical Overview

### Language and Framework
- **Primary Language**: Rust
- **Architecture**: std-based (not no_std)
- **Build System**: Cargo
- **Test Framework**: Custom test runner with 234 tests
- **Dependencies**: Zero external dependencies in core

### Supported Architectures
- x86_64 (primary)
- AArch64 (ARM64)
- RISC-V (planned)
- Other architectures via virtualization

### Kernel Features
- Custom scheduler (SovereignSched)
- Lock-free IPC mechanisms
- Virtual memory management
- Device driver framework
- eBPF/XDP networking support

### Filesystem Support
- SigmaFS (custom crash-consistent filesystem)
- ZFS integration
- Btrfs support
- ext4 compatibility
- Content-addressed storage

## Target Users

SigmaOS is designed for:

### Developers
- Rust-native development environment
- Comprehensive tooling and libraries
- Container support for development
- Performance profiling tools

### Security-Conscious Users
- Memory-safe by default
- Capability-based security model
- Post-quantum cryptography
- Regular security updates

### System Administrators
- Declarative system configuration
- Atomic updates with rollback
- Comprehensive monitoring tools
- Automation-friendly design

### Desktop Users
- Fast, responsive desktop environment
- Keyboard-driven workflow
- Minimal system requirements
- Regular updates without disruption

## Release Model

SigmaOS follows a structured release model:

### Release Channels
- **Sovereign Rolling**: Mainline-staged with continuous updates
- **Sovereign LTS**: Long-term stable with 6-month support cycles
- **Sovereign Experimental**: Sandbox-isolated for testing

### Update Mechanism
- Atomic A/B image updates
- Sub-second Copy-on-Write rollback
- Content-addressed package storage
- Signed package metadata

### Support Timeline
- Rolling releases: Continuous updates
- LTS releases: 6-month support cycles
- Security updates: As needed, prioritized
- Feature updates: Per release cycle

## Distribution Editions

SigmaOS provides targeted editions:

### Sovereign Desktop Edition
- Optimized for desktop usage
- Zenith compositor enabled
- Input/HID controllers active
- Graphics rendering prioritized

### Sovereign Server Edition
- Graphics frames disabled
- Network throughput prioritized
- Multi-priority networking threads
- Minimal system footprint

### Sovereign IoT & Edge Edition
- Memory footprint under 16MB
- Low-power sleep loops
- Sandbox telemetry tasks
- Hardware-specific optimizations

### Sovereign Educational Sandbox
- Interactive assembly tracers
- REPL builders
- Hardware simulators
- Learning-focused tools

## Project Status

### Current State (September 2026)
- **Version**: Development phase
- **Tests**: 234 passing tests across 23 suites
- **Compilation**: 0 errors, 820 warnings (non-critical)
- **Branches**: Single main branch
- **Pull Requests**: 0 open (26 merged total)
- **Documentation**: Comprehensive Wiki with 753 pages

### Development Focus
- Real hardware driver implementation
- UEFI/QEMU boot support
- Real init-to-shell userspace
- Stable syscall and ABI interfaces
- Package repository and verification

### Known Limitations
- Still in development phase
- Not yet suitable for production use
- Hardware support limited to x86_64
- Package ecosystem under development
- Limited real-world testing

## Comparison with Traditional Distributions

### Advantages over Linux Distros
- Memory safety through Rust
- Capability-based security model
- Atomic updates with instant rollback
- Zero external dependencies
- Post-quantum cryptography

### Advantages over BSD Distros
- Modern hardware support
- Rich package ecosystem
- Active development community
- Extensive documentation
- Desktop-focused design

### Unique Features
- Zenith compositor with keyboard workflow
- SigmaPkg universal package system
- SovereignVMM virtualization
- Zero-copy networking stack
- Content-addressed storage

## Future Roadmap

### Short-term (6 months)
- Production-ready codebase
- Hardware driver implementation
- Real boot/install/recovery paths
- Package repository deployment
- Comprehensive testing

### Medium-term (12 months)
- Multi-architecture support
- Enhanced package ecosystem
- Advanced security features
- Performance optimization
- Community expansion

### Long-term (18+ months)
- Enterprise features
- Advanced virtualization
- Enhanced hardware support
- International community
- Certification and compliance

## Community and Support

### Getting Help
- [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- Email: aaryansinghchauhan09@gmail.com

### Contributing
- [Getting Involved](Getting-Involved)
- [Contributing](Contributing)
- [Code of Conduct](Code-of-Conduct)

### Documentation
- [Table of Contents](Table-of-contents)
- [Installation Guide](Installation-Guide)
- [General Recommendations](General-Recommendations)

## License

SigmaOS is licensed under the MIT License. See the [LICENSE](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/LICENSE) file for details.

## Acknowledgments

SigmaOS draws inspiration from:
- Arch Linux (simplicity and user control)
- OpenBSD (security and correctness)
- FreeBSD (performance and features)
- Rust community (memory safety and tooling)
- NixOS (declarative configuration)
- Gentoo (flexibility and control)

---

**[Installation Guide](Installation-Guide)** | **[Frequently Asked Questions](Frequently-Asked-Questions)** | **[SigmaOS Compared to Other Distributions](SigmaOS-Compared-to-Other-Distributions)**

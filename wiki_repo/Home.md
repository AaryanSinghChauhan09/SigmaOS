# SigmaOS

A next-generation operating system built with Rust, focusing on security, performance, and modularity. SigmaOS aims to provide a modern, secure, and efficient computing environment while maintaining compatibility with existing Linux applications and drivers.

## 🌟 Key Features

- **Security-First Design**: Post-quantum cryptography, capability-based security, and comprehensive auditing
- **Zero-Dependency Architecture**: Custom implementations of core libraries to reduce external dependencies
- **Linux Compatibility**: Comprehensive package translation layer for .deb, .rpm, and pacman packages
- **AI-Native Runtime**: Built-in machine learning inference and training capabilities
- **Microkernel Architecture**: Modular design with clear separation between kernel and userspace
- **Energy-Aware Scheduling**: Intelligent CPU scheduling based on power consumption and performance needs
- **Advanced Filesystem**: Custom filesystem with content-addressed storage and efficient caching

## 🏗️ Architecture

SigmaOS follows a microkernel architecture with the following major components:

### Core Components
- **Kernel**: Lightweight microkernel with process management, memory management, and IPC
- **Security Module**: Capability-based access control, audit logging, and vulnerability scanning
- **Filesystem**: Content-addressed storage with deduplication and efficient caching
- **Network Stack**: TCP/UDP networking with zero-copy buffers and advanced congestion control
- **Graphics System**: Hardware-accelerated compositor with double buffering and window management
- **Package Manager**: Universal package manager supporting multiple Linux package formats

### Advanced Features
- **AI Subsystem**: Machine learning inference and training with hardware acceleration
- **Container Runtime**: Lightweight containerization with sandboxing
- **Remote Desktop**: Built-in remote desktop capabilities
- **Productivity Tools**: Media playback, document editing, and collaboration tools
- **Accessibility**: Screen readers, magnifiers, and input method editors

## 🚀 Getting Started

### Prerequisites
- Rust 1.70 or later
- Cargo
- For kernel development: QEMU or similar emulator
- For cross-compilation: appropriate toolchains

### Building SigmaOS

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the userspace components
cargo build --release

# Build the kernel (requires cross-compilation)
cargo build --release --target x86_64-unknown-none

# Run in QEMU
qemu-system-x86_64 -kernel target/x86_64-unknown-none/release/sigmaos -m 512M
```

### Development Setup

```bash
# Install development dependencies
cargo install cargo-make
cargo install cross

# Set up the development environment
cargo make dev-setup

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## 📦 Package Management

SigmaOS includes a universal package manager that can translate and install packages from multiple Linux distributions:

### Supported Package Formats
- **Debian/Ubuntu**: .deb packages with APT metadata
- **Fedora/RHEL**: .rpm packages with SPEC files
- **Arch Linux**: PKGBUILD files with pacman database
- **Snap**: Snapcraft.yaml manifests
- **Flatpak**: Flatpak manifests
- **AppImage**: AppImage bundles
- **Nix**: Nix derivations

### Package Installation

```bash
# Install a Debian package
sigpkg install neofetch.deb

# Install an Arch package
sigpkg install neofetch

# Search for packages
sigpkg search editor

# Update all packages
sigpkg update
```

## 🔒 Security Features

### Capability-Based Security
- Fine-grained permission system
- Process isolation and sandboxing
- Secure IPC mechanisms

### Post-Quantum Cryptography
- PQC signature verification (Dilithium)
- Secure key management
- Hardware attestation

### Audit and Compliance
- Comprehensive audit logging
- Real-time security monitoring
- Compliance dashboard for regulatory requirements

### Vulnerability Management
- Automatic vulnerability scanning
- Security advisory integration
- Patch management system

## 🎯 Development Roadmap

### Current Focus
- [x] Core kernel functionality
- [x] Basic filesystem implementation
- [x] Network stack (TCP/UDP)
- [x] Package translation layer
- [x] Security framework
- [x] Zero-copy IPC queue implementation
- [x] UDF scheduler VM integration
- [x] GitHub Actions security pinning
- [x] Branch consolidation and cleanup
- [x] NetBSD NPF stateful packet filtering
- [x] FreeBSD GEOM storage transformation
- [x] Alpine BusyBox multi-call applet system
- [ ] Graphics system completion
- [ ] Driver framework expansion
- [ ] AI subsystem optimization

### Future Goals
- Enhanced Linux compatibility layer
- Advanced power management
- GPU acceleration for AI workloads
- Cloud-native features
- Mobile device support

## 🤝 Contributing

We welcome contributions to SigmaOS! Please see our contributing guidelines for more information.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and benchmarks
5. Submit a pull request

### Code Style
- Follow Rust best practices
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs

## 📚 Documentation

- [Architecture Overview](./ARCHITECTURE.md)
- [API Reference](./API_REFERENCE.md)
- [Security Guide](./SECURITY.md)
- [Package Management](./PACKAGE_MANAGEMENT.md)
- [Driver Development](./DRIVER_DEVELOPMENT.md)
- [AI Subsystem](./AI_SUBSYSTEM.md)

## 🐛 Bug Reporting

Please report bugs using our [issue tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) with the following information:
- SigmaOS version
- Hardware configuration
- Steps to reproduce
- Expected behavior
- Actual behavior
- Log files (if applicable)

## 📄 License

SigmaOS is released under the MIT License. See LICENSE file for details.

## 🙏 Acknowledgments

- The Rust community for excellent tooling and libraries
- Linux distributions for inspiration and compatibility targets
- Security researchers for vulnerability disclosures
- All contributors who have helped make SigmaOS better

## 📞 Contact

- GitHub: https://github.com/AaryanSinghChauhan09/SigmaOS
- Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- LinkedIn:
https://www.linkedin.com/in/aaryan-singh-chauhan-16a147288
- SoloLearn:
https://www.sololearn.com/profile/29816006/
- Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki

---

**Note**: SigmaOS is currently in active development. Some features may be incomplete or subject to change. We appreciate your patience and feedback as we work towards a stable release.

## 🔄 Recent Updates (August 2026)

- **Branch Consolidation**: Successfully merged all feature branches into main
- **Security Improvements**: Fixed code scanning alerts and pinned GitHub Actions
- **Performance Enhancements**: Integrated zero-copy queue and UDF scheduler VM
- **Dependency Reduction**: Reduced reliance on predefined libraries
- **Repository Cleanup**: Removed 200K+ lines of redundant code
- **Linux/BSD Compatibility**: Implemented NetBSD NPF, FreeBSD GEOM, and Alpine BusyBox frameworks
- **Trait Conflicts**: Fixed conflicting IPackageAdapter implementations

See [Branch Consolidation Complete](./Branch-Consolidation-Complete-August-2026.md) and [Linux/BSD Compatibility Implementation](./Linux-BSD-Compatibility-Implementation.md) for details.

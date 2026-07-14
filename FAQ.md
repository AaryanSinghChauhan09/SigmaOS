# Frequently Asked Questions (FAQ)

> **Last Updated**: 2026-07-13

This document answers common questions about SigmaOS, its architecture, development, and usage.

## General Questions

### What is SigmaOS?

SigmaOS is a sovereign microkernel operating system built on the principle of capability-based security. It uses a modular architecture called "shards" that can be loaded, unloaded, and updated independently. SigmaOS is designed for security, performance, and sovereignty with post-quantum cryptography integration.

### What makes SigmaOS different from other operating systems?

SigmaOS differs from other operating systems in several ways:

- **Capability-Based Security**: All access is granted through capabilities with default deny policy
- **Modular Shard Architecture**: Components can be loaded/unloaded independently
- **Post-Quantum Cryptography**: Built-in support for Kyber-1024 KEM and Dilithium-5 signatures
- **Zero-Trust Network Stack**: Built-in zero-trust firewall with capability-based access
- **Multi-Language Support**: Rust for kernel components, Zig for low-level drivers, Nim for tooling
- **Sovereign Design**: Local-first with minimal external dependencies

### What platforms does SigmaOS support?

SigmaOS is designed to support multiple platforms:

- **x86_64**: Desktop and server systems
- **ARM64**: Mobile and embedded systems
- **RISC-V**: Experimental support

### Is SigmaOS production-ready?

SigmaOS is currently in active development. The core architecture is defined, and prototypes are being implemented. It is not yet production-ready but is suitable for testing and development.

## Architecture

### What is the shard architecture?

The shard architecture is SigmaOS's modular component system. Each shard is an independent module that provides specific functionality:

- **Core Shards**: Essential kernel components (memory manager, scheduler, network stack, etc.)
- **Essential Shards**: Hardware drivers (GPU, storage, audio, network, input)
- **Optional Shards**: Desktop environment and AI features
- **Infinite Shards**: Experimental and self-evolving features

### How do shards communicate?

Shards communicate through well-defined interfaces:

- **Capability Channels**: Secure message passing
- **Shared Memory Regions**: With capability-based access control
- **Event Notifications**: Asynchronous event system
- **Service Discovery**: Dynamic shard registration

### What is capability-based security?

Capability-based security is a security model where all access to resources is granted through capabilities. Key principles:

- **Default Deny**: All access is denied by default
- **Explicit Grant**: Access must be explicitly granted via capabilities
- **Capability Revocation**: Capabilities can be revoked
- **Least Privilege**: Components only have access to what they need
- **Audit Trail**: All capability changes are logged

### What post-quantum cryptography does SigmaOS use?

SigmaOS uses:

- **Kyber-1024**: Key Encapsulation Mechanism (KEM) for key exchange
- **Dilithium-5**: Digital signature algorithm for authentication
- **Hybrid Mode**: Combines post-quantum with classical algorithms for compatibility

## Development

### What programming languages are used?

SigmaOS uses multiple languages based on component requirements:

- **Rust**: Kernel components and core shards (memory safety, performance)
- **Zig**: Low-level drivers and runtime (control, performance)
- **Nim**: Tooling and automation (expressiveness, ease of use)

### How do I build SigmaOS?

See the [INSTALL.md](INSTALL.md) for detailed build instructions. The basic steps:

1. Install Rust toolchain
2. Install Zig compiler
3. Install Nim compiler
4. Clone the repository
5. Run `cargo build` for Rust components
6. Run `zig build` for Zig components
7. Run `nim build` for Nim components

### How do I contribute to SigmaOS?

See the [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines. The basic process:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### What are the coding standards?

SigmaOS follows these coding standards:

- **Rust**: Follow Rust style guide, use `cargo fmt` and `cargo clippy`
- **Zig**: Follow Zig style guide, use `zig fmt`
- **Nim**: Follow Nim style guide, use `nimpretty`
- **No External Dependencies**: Implement from first principles where possible
- **Documentation**: Document all public APIs

### How do I run tests?

Run tests for each component:

```bash
# Rust components
cargo test

# Zig components
zig build test

# Nim components
nim test
```

Run smoke tests:

```bash
./scripts/smoke-test.sh
```

## Security

### How does SigmaOS ensure security?

SigmaOS ensures security through:

- **Capability-Based Security**: All access controlled via capabilities
- **Post-Quantum Cryptography**: Quantum-resistant algorithms
- **Zero-Trust Network**: Default deny firewall policy
- **Memory Safety**: Rust's memory safety guarantees
- **Minimal Attack Surface**: Only load required shards
- **Audit Logging**: All security events logged

### What is the threat model?

SigmaOS's threat model includes:

- **Malicious Applications**: Isolated via capabilities
- **Network Attacks**: Protected by zero-trust firewall
- **Quantum Attacks**: Protected by post-quantum crypto
- **Supply Chain Attacks**: Minimal dependencies, signed drivers
- **Hardware Attacks**: TPM integration, secure boot

### How are vulnerabilities handled?

See the [SECURITY_POLICY.md](SECURITY_POLICY.md) for vulnerability reporting. The process:

1. Report vulnerability via security email
2. Security team reviews and validates
3. Fix is developed and tested
4. Security advisory is published
5. Patch is released

## Usage

### How do I install SigmaOS?

See the [INSTALL.md](INSTALL.md) for installation instructions. SigmaOS can be installed:

- **Standalone**: Full desktop installation
- **Microkernel**: Minimal embedded installation
- **Cloud**: Headless cloud image
- **Container**: Containerized deployment

### How do I configure SigmaOS?

SigmaOS is configured via:

- **Boot Parameters**: Kernel boot parameters
- **Configuration Files**: TOML-based configuration
- **Runtime Configuration**: Dynamic configuration via CLI
- **Feature Flags**: Enable/disable features at build time

### What deployment profiles are available?

SigmaOS supports multiple deployment profiles:

- **Standalone**: Full desktop with all features
- **Microkernel**: Minimal kernel for embedded systems
- **RTOS**: Real-time OS for industrial control
- **Cloud**: Headless image for cloud platforms
- **Mobile**: Touch-optimized for mobile devices
- **Browser**: WebAssembly for browser deployment

### How do I manage packages?

SigmaOS uses `sigma-pkg` for package management:

```bash
# Search packages
sigma-pkg search <package>

# Install package
sigma-pkg install <package>

# Update package
sigma-pkg update <package>

# Remove package
sigma-pkg remove <package>
```

## Troubleshooting

### Build fails with Rust error

Ensure you have the latest Rust toolchain:

```bash
rustup update
rustup default stable
```

### Build fails with Zig error

Ensure you have the latest Zig compiler:

```bash
zig version  # Should be 0.11.0 or later
```

### Build fails with Nim error

Ensure you have the latest Nim compiler:

```bash
nim --version  # Should be 2.0.0 or later
```

### Driver not detected

Check that the shard is loaded:

```bash
# List loaded shards
shard list

# Load shard
shard load <shard-name>
```

### Network not working

Check firewall rules:

```bash
# List firewall rules
firewall list

# Add allow rule
firewall add allow <source-ip> <dest-ip> <protocol>
```

## Performance

### How does SigmaOS achieve high performance?

SigmaOS achieves high performance through:

- **Zero-Copy Operations**: Minimize data copying
- **O(1) Scheduling**: EEVDF scheduler
- **Minimal Overhead**: Microkernel design
- **Efficient IPC**: Capability-based IPC
- **Hardware Acceleration**: GPU acceleration where available

### What is the EEVDF scheduler?

EEVDF (Earliest Eligible Virtual Deadline First) is an O(1) scheduling algorithm that provides:

- **Fairness**: Fair CPU allocation
- **Low Latency**: Low response time
- **Real-Time Support**: Real-time task priorities
- **Predictable**: Deterministic timing

### How does SigmaOS handle memory?

SigmaOS uses:

- **Buddy Allocator**: Efficient physical memory allocation
- **Paging**: Virtual memory with capability-based protection
- **Zero-Copy**: Minimize memory copying
- **Memory Pooling**: Reuse memory allocations

## Future

### What are the future plans for SigmaOS?

Future plans include:

- **Self-Evolving System**: Genetic algorithms and reinforcement learning
- **AI-Native OS**: ML-based scheduling and resource management
- **Quantum Computing**: Quantum algorithm integration
- **Enhanced Desktop**: Improved desktop environment
- **Cloud Integration**: Better cloud platform support

### When will SigmaOS be production-ready?

SigmaOS is currently in active development. Production readiness depends on:

- Completion of core shards
- Completion of essential drivers
- Security audits
- Performance testing
- User feedback

Estimated timeline: 2027-2028

### How can I help with development?

You can help by:

- **Contributing Code**: Submit pull requests
- **Testing**: Test on various hardware
- **Documentation**: Improve documentation
- **Reporting Bugs**: Report issues
- **Spreading the Word**: Share SigmaOS with others

## Community

### Where can I get help?

Get help through:

- **GitHub Issues**: Report bugs and ask questions
- **Discord**: Join the SigmaOS Discord server
- **Mailing List**: Join the development mailing list
- **Documentation**: Read the documentation

### How do I join the community?

Join the community by:

- **Starring the Repository**: Show your support
- **Following Updates**: Watch the repository
- **Joining Discord**: Participate in discussions
- **Contributing**: Submit code and documentation

### What is the code of conduct?

See the [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for the code of conduct. Key points:

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Respect privacy and confidentiality

## Licensing

### What license is SigmaOS under?

SigmaOS is under the BSD 2-Clause License. See [LICENSE.md](LICENSE.md) for details.

### Can I use SigmaOS commercially?

Yes, SigmaOS is under the permissive BSD 2-Clause License, which allows commercial use.

### Can I contribute to SigmaOS under a different license?

Contributions must be under the BSD 2-Clause License or a compatible license.

## Additional Resources

### Where can I learn more?

Learn more through:

- **README.md**: Project overview
- **ARCHITECTURE.md**: System architecture
- **INSTALL.md**: Installation guide
- **CONTRIBUTING.md**: Contribution guidelines
- **SECURITY_POLICY.md**: Security policy
- **SUPPORT.md**: Support resources

### Where can I find the source code?

Source code is available on GitHub:

https://github.com/AaryanSinghChauhan09/SigmaOS

### How do I report a security vulnerability?

Report security vulnerabilities via:

- Email: security@sigmaos.org
- PGP Key: Available on GitHub

See [SECURITY_POLICY.md](SECURITY_POLICY.md) for details.

---

*Last Updated: 2026-07-13*

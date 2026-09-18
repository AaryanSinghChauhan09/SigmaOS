# Frequently Asked Questions

This page answers common questions about SigmaOS.

## General Questions

### What is SigmaOS?

SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow. It is designed to provide memory safety, security, and performance while maintaining simplicity and user control.

### Why should I use SigmaOS?

SigmaOS offers several advantages:
- **Memory Safety**: Written in Rust, eliminating entire classes of memory-related vulnerabilities
- **Security**: Capability-based security model with post-quantum cryptography
- **Performance**: Lock-free data structures and optimized algorithms
- **Atomic Updates**: Instant rollback capability with sub-second recovery
- **Modern Design**: Built with modern best practices and hardware support

### Is SigmaOS ready for production use?

SigmaOS is currently in development and not yet suitable for production use. The project is actively being developed with focus on implementing real hardware drivers, boot support, and production-ready features. Check the [About SigmaOS](About-SigmaOS) page for current status.

### What makes SigmaOS different from Linux distributions?

SigmaOS differs from traditional Linux distributions in several ways:
- Written in Rust for memory safety
- Capability-based security model (default-deny)
- Zero external dependencies in core components
- Custom package system (SigmaPkg) with content-addressed storage
- Zenith compositor with keyboard-driven workflow
- Post-quantum cryptography by default

### What hardware does SigmaOS support?

Currently, SigmaOS primarily supports x86_64 architecture. Support for AArch64 is planned, and RISC-V is under consideration. Hardware support is being actively developed. See the [Hardware Support Matrix](SUPPORT_MATRIX) for current details.

## Installation

### How do I install SigmaOS?

The installation process is documented in the [Installation Guide](Installation-Guide). The guide provides step-by-step instructions for installing SigmaOS on supported hardware.

### Can I install SigmaOS in a virtual machine?

Yes, SigmaOS can be installed in virtual machines. The installation guide includes instructions for QEMU/KVM and other virtualization platforms.

### Can I dual-boot SigmaOS with another OS?

Yes, SigmaOS supports dual-boot configurations. The installation guide provides instructions for setting up dual-boot with other operating systems.

### Does SigmaOS have a live environment?

A live environment is planned for future releases. Currently, installation requires booting from the installation media directly.

## Desktop Environment

### What desktop environment does SigmaOS use?

SigmaOS uses the Zenith compositor, a custom Wayland-based desktop environment designed for keyboard-driven workflow and performance.

### Can I use other desktop environments?

SigmaOS is designed primarily for the Zenith compositor. Support for other desktop environments may be added in the future through community contributions.

### Does SigmaOS support Wayland applications?

Yes, SigmaOS uses Wayland as the display protocol. Wayland applications should work natively on the Zenith compositor.

### Can I run X11 applications?

X11 applications can run through XWayland compatibility layer, though native Wayland applications are recommended for better performance and security.

## Package Management

### What package manager does SigmaOS use?

SigmaOS uses SigmaPkg, a custom package management system with content-addressed storage, atomic updates, and support for multiple package formats.

### Can I install packages from other distributions?

SigmaOS supports multiple package formats through adapters, including RPM (Fedora, RHEL), DEB (Debian, Ubuntu), and Arch packages. However, native SigmaPkg packages are recommended for best compatibility.

### Does SigmaOS have an AUR equivalent?

SigmaOS has support for Arch packages and AUR helpers through the multi-distro adapter system. A dedicated community repository system is planned.

### How do I update SigmaOS?

SigmaOS uses atomic updates with the SigmaPkg system:

```bash
sudo sigpkg update
sudo sigpkg upgrade
```

Updates are atomic and can be rolled back instantly if needed.

### Can I rollback updates?

Yes, SigmaOS supports instant rollback to previous system generations:

```bash
sudo sigpkg rollback
```

Rollback takes less than a second using Copy-on-Write filesystem technology.

## Security

### How secure is SigmaOS?

SigmaOS is designed with security as a core principle:
- Memory safety through Rust
- Capability-based security model
- Post-quantum cryptography
- Defense-in-depth architecture
- Regular security updates

### Does SigmaOS use SELinux or AppArmor?

SigmaOS implements its own capability-based security model, which provides similar functionality to SELinux/AppArmor but with a different approach focused on capabilities rather than policies.

### What encryption does SigmaOS use?

SigmaOS uses post-quantum cryptography:
- **Key Exchange**: CRYSTALS-Kyber (ML-KEM)
- **Signatures**: CRYSTALS-Dilithium (ML-DSA)
- **Symmetric**: ChaCha20-Poly1305, AES-256-GCM
- **Hashing**: BLAKE3

### Does SigmaOS support Secure Boot?

Yes, SigmaOS supports Secure Boot with TPM 2.0 PCR measurements for verified boot.

## Performance

### Is SigmaOS faster than Linux?

SigmaOS is designed for performance with lock-free data structures, zero-copy I/O, and optimized algorithms. Benchmark comparisons are ongoing, but early results show significant performance improvements in specific workloads.

### What scheduler does SigmaOS use?

SigmaOS uses SovereignSched, a custom scheduler with:
- Asymmetric Multi-Processing (AMP)
- Lock-free queue pools
- Real-time scheduling capabilities
- Thermal and resource-predictive scaling

### Does SigmaOS support gaming?

Gaming support is under development. SigmaOS has a GPU scheduler micro-tool and network latency monitor for gaming optimization. Full gaming support is a future goal.

## Development

### Can I contribute to SigmaOS?

Yes! SigmaOS welcomes contributions. See [Getting Involved](Getting-Involved) and [Contributing](Contributing) for details on how to contribute.

### What programming language is SigmaOS written in?

SigmaOS is primarily written in Rust. Some components may use other languages for specific purposes, but Rust is the primary language for the core system.

### How do I build SigmaOS from source?

Building SigmaOS requires:
- Rust toolchain (stable or nightly)
- Git for version control
- Standard build tools

See the [Build Instructions](BUILD) for detailed build instructions.

### Does SigmaOS use no_std or std?

SigmaOS uses a std-based architecture (decided September 2026). This decision was made based on codebase analysis showing 4,901 std imports vs 0 alloc imports, and the practical need for full OS features.

## Compatibility

### Can I run Linux applications on SigmaOS?

SigmaOS supports running Linux applications through compatibility layers and containerization. Native SigmaPkg applications are recommended for best performance and security.

### Does SigmaOS support Windows applications?

Windows application support is not currently available. Wine or similar compatibility layers may be added in the future through community contributions.

### Can I use my existing Linux configuration files?

SigmaOS uses a different configuration system (declarative TOML-based). Direct migration of Linux configuration files is not supported, but similar functionality can be achieved through SigmaOS configuration.

## Troubleshooting

### I can't boot SigmaOS after installation

Check the [Installation Guide](Installation-Guide) troubleshooting section. Common issues include:
- Bootloader configuration problems
- Hardware compatibility issues
- Firmware settings (Secure Boot, CSM, etc.)

### My network doesn't work

Network troubleshooting steps:
1. Check hardware compatibility in [Support Matrix](SUPPORT_MATRIX)
2. Verify network configuration: `sigconfig network show`
3. Check driver status: `sigconfig drivers list`
4. Review logs: `journalctl -xe`

### Package installation fails

If package installation fails:
1. Update package lists: `sudo sigpkg update`
2. Check network connectivity
3. Verify repository availability
4. Check disk space
5. Review error messages for specific issues

### System is slow after update

Performance issues after update:
1. Check for running processes: `sigtop`
2. Review system logs: `journalctl`
3. Consider rollback: `sudo sigpkg rollback`
4. Report the issue if it persists

## Community

### Where can I get help?

Help is available through:
- [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- Email: aaryansinghchauhan09@gmail.com (for security issues)

### How can I report bugs?

Report bugs through [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues). Include:
- Detailed description of the issue
- Steps to reproduce
- System information
- Relevant logs and error messages

### How can I request features?

Feature requests can be submitted through [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) with the "enhancement" label. Provide:
- Clear description of the feature
- Use cases and benefits
- Implementation suggestions (if known)

### Is there a SigmaOS community?

Yes! See [International Communities](International-Communities) for community resources in different languages and regions.

## Future Plans

### What are the future plans for SigmaOS?

The future roadmap is documented in:
- [Future Course of Action](FUTURE_COURSE_OF_ACTION.md)
- [Future Development Roadmap](FUTURE-DEVELOPMENT-ROADMAP.md)
- [Master Execution Roadmap](ROADMAP.md)

Key priorities include:
- Production-ready codebase
- Hardware driver implementation
- Enhanced package ecosystem
- Multi-architecture support
- Enterprise features

### When will SigmaOS be stable?

A stable release is planned for the future. The current focus is on implementing core features and achieving production readiness. Follow the project updates for timeline information.

### Will SigmaOS support other architectures?

Yes, support for AArch64 is planned, and RISC-V is under consideration. Multi-architecture support is a medium-term goal.

---

**[About SigmaOS](About-SigmaOS)** | **[Installation Guide](Installation-Guide)** | **[General Recommendations](General-Recommendations)**

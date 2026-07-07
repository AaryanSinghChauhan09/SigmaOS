# SigmaOS FAQ

## General Questions

### What is SigmaOS?

SigmaOS is a sovereign operating system built with Rust and Nim, designed for security, performance, and independence from external dependencies. It features a capability-based security model, zero-dependency policy, and native implementations of office productivity tools.

### Why create another operating system?

SigmaOS addresses specific needs:
- **Sovereignty**: Complete control over the OS stack
- **Security**: Capability-based security model from the ground up
- **Independence**: No reliance on external third-party dependencies
- **Indian Context**: Built for India's digital infrastructure (ABDM, GST, UPI)
- **Performance**: Zero-allocation optimizations and efficient algorithms

### What platforms does SigmaOS support?

Currently:
- x86_64 (primary target)
- UEFI boot

Planned:
- ARM64 (Raspberry Pi)
- RISC-V

### Is SigmaOS open source?

Yes, SigmaOS is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Technical Questions

### What programming languages are used?

- **Rust**: Kernel, bootloader, most userland components
- **Nim**: Some userland suites and tools
- **Assembly**: Low-level hardware interaction
- **C**: Legacy compatibility layer

### Why Rust?

Rust provides:
- Memory safety without garbage collection
- Zero-cost abstractions
- Strong type system
- Excellent tooling (cargo, clippy)
- Growing ecosystem

### Why Nim?

Nim provides:
- Python-like syntax
- C-level performance
- Small binary size
- Easy FFI
- Metaprogramming capabilities

### Does SigmaOS use the Linux kernel?

No, SigmaOS has its own kernel written from scratch in Rust. This allows for:
- Complete control over kernel design
- Capability-based security model
- Zero external dependencies
- Tailored for SigmaOS requirements

### What filesystems does SigmaOS support?

Currently:
- Basic VFS layer
- Ext2/Ext3 (planned)
- FAT32 (planned)
- CryptFS (Argon2id encrypted, planned)

### What graphics stack does SigmaOS use?

SigmaOS uses:
- VESA/VBE for basic framebuffer
- UEFI GOP for UEFI systems
- DRM/KMS for GPU drivers (planned)
- Zenith compositor (planned)

## Security Questions

### What is the security model?

SigmaOS uses a capability-based security model:
- Fine-grained capabilities (CAP_CHOWN, CAP_NET_ADMIN, etc.)
- Sovereign Capability Derivation Forest
- Mandatory Access Control (MAC) policies
- Landlock filesystem sandboxing
- seccomp syscall filtering

### Is there a root user?

No, SigmaOS eliminates the traditional root user in favor of a capability-based system. Processes only have the capabilities they need, and capabilities can be derived and revoked.

### How does SigmaOS handle vulnerabilities?

- Zero-dependency policy reduces attack surface
- Memory-safe Rust code prevents memory-safety CVEs
- Formal verification planned for critical components
- Continuous security auditing
- Automated vulnerability scanning

### Does SigmaOS support Secure Boot?

Yes, SigmaOS supports UEFI Secure Boot with TPM2 integration (planned for Phase 4).

## Usage Questions

### How do I install SigmaOS?

Currently, SigmaOS is in early development. Installation instructions will be provided when the system reaches a stable state. For now, you can test it in QEMU:

```bash
qemu-system-x86_64 -cdrom sigmaos.iso -m 2G
```

### What applications are available?

SigmaOS includes native implementations of:
- Word processor (sigma-wordprocessor)
- Spreadsheet (sigma-spreadsheet)
- Presentation (sigma-presentation)
- Email client (sigma-email)
- Database client (sigma-database)

Plus core utilities and system tools.

### Can I run Linux applications on SigmaOS?

Not directly. SigmaOS has its own syscall interface and ABI. However, we plan to implement:
- Linux compatibility layer (optional)
- Wine-like compatibility for Windows applications (future)

### How do I develop for SigmaOS?

See the [Development Guide](./DEVELOPMENT.md) for detailed instructions. Basic steps:

1. Install Rust and Nim
2. Clone the repository
3. Build the project
4. Follow contribution guidelines

## Performance Questions

### How does SigmaOS performance compare to Linux?

SigmaOS is optimized for:
- Zero-allocation operations
- Efficient data structures
- Minimal overhead
- Specialized for SigmaOS use cases

Benchmarks will be provided as the system matures.

### What are the system requirements?

Minimum (for development):
- x86_64 CPU
- 2GB RAM
- 10GB disk space
- UEFI firmware

Recommended:
- x86_64 CPU with virtualization support
- 4GB+ RAM
- 20GB+ disk space
- UEFI 2.3.1+

## Development Questions

### How can I contribute?

See the [Contributing Guide](../CONTRIBUTING.md) for details. Key areas:
- Kernel development
- Driver development
- Application development
- Documentation
- Testing

### What are the most needed contributions?

Currently (Phase 0):
- C++/Rust kernel engineers
- UEFI/EDK2 bootloader engineers
- Build system engineers

Future phases:
- Network stack engineers
- GPU/graphics engineers
- India Stack API integration
- AI/ML integration

### How do I report bugs?

Report bugs via GitHub Issues:
https://github.com/AaryanSinghChauhan09/SigmaOS/issues

Include:
- System information
- Error messages
- Steps to reproduce
- Debug logs if available

## Future Questions

### What's the roadmap?

See the [Roadmap](./ROADMAP.md) for detailed plans. Key milestones:
- M0: First Boot (Month 3)
- M1: Real Hardware (Month 6)
- M2: First Desktop (Month 9)
- M3: India Stack Live (Month 14)
- M4: Security Audit (Month 18)

### When will SigmaOS be production-ready?

Target for production use: Month 14 (India Stack Live)
Target for general availability: Month 18+ (after security audit)

### Will SigmaOS support ARM?

Yes, ARM64 support is planned for Phase 5 (Month 21), targeting Raspberry Pi 4/5.

### What about mobile support?

Mobile support is not currently planned but may be considered in future phases.

## India-Specific Questions

### What India Stack integrations are planned?

- ABDM (Ayushman Bharat Digital Mission)
- GST (Goods and Services Tax)
- UPI (Unified Payments Interface)
- e-RUPI
- Account Aggregator (AA)
- NavIC (Indian GPS)

### Will SigmaOS support Indian languages?

Yes, SigmaOS will support:
- Hindi IME (Inscript + phonetic)
- Other regional languages
- Offline speech recognition (sigma-bhashini)
- Text-to-speech

### How will SigmaOS help rural India?

SigmaOS includes:
- sigma-RuralStack for village-level services
- Offline-first design for low connectivity
- Low hardware requirements
- Local language support
- Digital literacy tools

## Licensing Questions

### Can I use SigmaOS commercially?

Yes, SigmaOS is licensed under the permissive MIT License, allowing commercial use.

### Can I modify and redistribute SigmaOS?

Yes, the MIT License allows modification and redistribution, provided the license and copyright notice are included.

### Are there any restrictions?

The MIT License has minimal restrictions. You must:
- Include the license and copyright notice
- State any significant changes made

## Support Questions

### Where can I get help?

- GitHub Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- GitHub Discussions: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions
- Documentation: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
- Email: (to be announced)

### Is commercial支持 available?

Commercial support options will be announced as the system matures.

### Is there training available?

Training materials and courses will be developed as the system approaches production readiness.

## Miscellaneous Questions

### Why the name "SigmaOS"?

Sigma (Σ) represents summation and completeness, reflecting our goal of creating a complete, sovereign operating system.

### Who is behind SigmaOS?

SigmaOS is developed by the SigmaOS Project team. See the [CONTRIBUTING.md](../CONTRIBUTING.md) for information on how to join.

### How is SigmaOS funded?

SigmaOS is currently a community-driven project. Funding models for sustainability are being explored.

### Can I donate?

Donation options will be announced in the future.

## Still Have Questions?

If your question isn't answered here:
1. Check the [Documentation](./)
2. Search [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
3. Search the [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
4. Ask in [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
5. Create a new Issue with your question

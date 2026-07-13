# SigmaOS Support

> **Last Updated**: 2026-07-13

## Getting Help

### Documentation

- **Installation Guide**: [INSTALL.md](INSTALL.md)
- **Contributing Guide**: [CONTRIBUTING.md)
- **Security Policy**: [SECURITY_POLICY.md](SECURITY_POLICY.md)
- **Documentation Audit**: [docs/doc_audit_backlog.md](docs/doc_audit_backlog.md)
- **Wiki**: [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

### Community Support

- **GitHub Discussions**: [Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **GitHub Issues**: [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Community Guidelines**: [COMMUNITY.md](COMMUNITY.md)

### Contact

- **General Support**: support@sigmaos.dev
- **Security Issues**: security@sigmaos.dev (PGP encrypted)
- **Business Inquiries**: business@sigmaos.dev

## Troubleshooting

### Build Issues

#### Build Fails with Missing Dependencies

```bash
# Install prerequisites (Linux/Debian)
sudo apt-get update
sudo apt-get install -y build-essential nasm cmake qemu-system-x86 golang-go xorriso
```

#### QEMU Boot Fails

```bash
# Check QEMU installation
qemu-system-x86_64 --version

# Verify ISO exists
ls -lh build/sigmaos.iso

# Try with more memory
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 4G -serial stdio
```

#### Cross-Compilation Errors

```bash
# Verify cross-compiler is in PATH
aarch64-linux-gnu-gcc --version
riscv64-linux-gnu-gcc --version

# Install cross-compilers
sudo apt-get install -y gcc-aarch64-linux-gnu gcc-riscv64-linux-gnu
```

### Runtime Issues

#### Kernel Panic

Check the serial output for error messages. Common causes:
- Missing driver for your hardware
- Insufficient memory allocation
- Configuration errors in `Config.sigma`

#### Network Not Working

Verify network shard is enabled:
```toml
[shards]
enable = ["s-net"]
```

#### Graphics Issues

For Zenith Desktop issues, ensure:
- GPU driver is loaded
- VESA framebuffer is available
- Display resolution is supported

## Common Issues

### "command not found" Errors

Install missing tools from [INSTALL.md](INSTALL.md) prerequisites section.

### Permission Denied

Make scripts executable:
```bash
chmod +x scripts/*.sh
```

### Out of Memory During Build

Reduce parallel jobs:
```bash
make -j2 all
```

### Large File Warnings

SigmaOS repository contains large files (assets, test data). Use Git LFS if needed:
```bash
git lfs install
git lfs pull
```

## Reporting Bugs

### Before Reporting

1. Search existing issues
2. Check documentation
3. Run smoke tests: `./scripts/smoke-test.sh`
4. Gather system information

### Bug Report Template

```markdown
**Description**: Brief description of the issue

**Steps to Reproduce**:
1. Step one
2. Step two
3. Step three

**Expected Behavior**: What should happen

**Actual Behavior**: What actually happens

**Environment**:
- OS: [e.g., Ubuntu 22.04]
- Architecture: [e.g., x86_64]
- SigmaOS Version: [e.g., v0.1.0]

**Logs**: Relevant error logs or output
```

## Feature Requests

### Before Requesting

1. Check [Roadmap.md](Roadmap.md) for planned features
2. Search existing feature requests
3. Discuss in [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)

### Feature Request Template

```markdown
**Feature Title**: Brief title

**Problem Statement**: What problem does this solve?

**Proposed Solution**: How should it work?

**Alternatives Considered**: Other approaches you considered

**Additional Context**: Any other relevant information
```

## Professional Support

### Enterprise Support

For enterprise deployments, contact: business@sigmaos.dev

### Consulting Services

Custom development, integration, and training available.

### SLA Options

- **Standard**: Community support (best effort)
- **Professional**: 48-hour response time
- **Enterprise**: 24-hour response time, dedicated support

## Contributing

Want to help improve SigmaOS? See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### High-Impact Areas

- Kernel scheduler improvements
- Driver development
- Package manager enhancements
- Documentation improvements
- Test coverage expansion

See [TODO.md](TODO.md) and [docs/doc_audit_backlog.md](docs/doc_audit_backlog.md) for specific tasks.

## Resources

### Learning Resources

- **Rust**: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- **Zig**: [Zig Documentation](https://ziglang.org/documentation/)
- **Nim**: [Nim Documentation](https://nim-lang.org/docs.html)
- **Kernel Development**: [OSDev Wiki](https://wiki.osdev.org/)

### Related Projects

- **Redox OS**: https://redox-os.org/
- **SerenityOS**: https://serenityos.org/
- **Haiku**: https://www.haiku-os.org/

## Acknowledgments

Special thanks to all contributors who help make SigmaOS better.

---

*Last Updated: 2026-07-13*

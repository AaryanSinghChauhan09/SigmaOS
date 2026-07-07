# SigmaOS Support

## Getting Help

### GitHub Discussions

Best place for questions, ideas, and general help:
https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

### GitHub Issues

For confirmed bugs and feature requests:
https://github.com/AaryanSinghChauhan09/SigmaOS/issues

### Before opening an issue

- Search existing issues first

- Check [CURRENT_PROBLEMS_MANIFEST.md](CURRENT_PROBLEMS_MANIFEST.md) — your issue may already be tracked

- Use the appropriate issue template (bug, feature, driver request, docs)

### Wiki

Comprehensive documentation:
https://github.com/AaryanSinghChauhan09/SigmaOS/wiki

---

## Issue Labels

| Label | Meaning |
|-------|---------|
| `bug` | Something broken |
| `feature` | New capability request |
| `driver` | Hardware driver request |
| `security` | Security-related (use private advisory for vulnerabilities) |
| `docs` | Documentation improvement |
| `kernel` | Core kernel subsystem |
| `networking` | Network stack |
| `fs` | Filesystem layer |
| `good first issue` | Suitable for new contributors |
| `help wanted` | Maintainers need community assistance |

---

## Security Issues

### Do not open public issues for security vulnerabilities

See [SECURITY_POLICY.md](SECURITY_POLICY.md) for private reporting.

---

## FAQ

### Q: Is SigmaOS bootable on real hardware?

A: Not yet. `make iso` producing a bootable image is Phase G (v16.0 Apex, Q1 2027). Currently QEMU-testable.

### Q: Can I run Linux apps on SigmaOS?

A: A Linux ELF compatibility layer (`runtime/containers/sigma_linux_compat.cpp`) is implemented. Full parity is Phase G+.

### Q: What architectures are supported?

A: x86_64 (primary), ARM64 (Phase G), RISC-V RV64GC (Phase H). HAL stubs exist for all three.

### Q: How do I contribute a driver?

A: See [CONTRIBUTING.md](CONTRIBUTING.md) and the SDF driver template in [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md).

### Q: Where is the package registry?

A: `sigma_pkg_registry/` in the repo. The online registry server is Phase G (#1011).

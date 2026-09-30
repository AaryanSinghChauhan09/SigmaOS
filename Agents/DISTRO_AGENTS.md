# Distro AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the distro subsystem of SigmaOS.

## Component Overview

distro is responsible for Linux/BSD distro compatibility and gateway.

## Operational Boundaries

### Always Do
- Run relevant tests before submitting changes
- Add documentation for new APIs
- Follow SigmaOS coding standards (safe Rust, no external dependencies)
- Measure performance impact for optimizations

### Ask First
- Major architectural changes
- Adding external dependencies
- Modifying interfaces used by other components

### Never Do
- Commit hardcoded secrets or keys
- Introduce memory safety violations
- Break compatibility without documentation
- Treat package suffix recognition or a distro mode enum as full distro compatibility
- Mark authentication successful without a trusted credential provider
- Claim a model, status string, or test fixture performs real OS integration

## Open Source Inspiration

### Primary Competitors
- **Linux**: Distro subsystem - https://www.kernel.org/doc/html/latest/
- **FreeBSD**: Distro implementation - https://www.freebsd.org/doc/
- **OpenBSD**: Distro design - https://www.openbsd.org/

### Key Improvements Opportunities
1. **Performance**: Optimize for zero-copy operations and reduced latency
2. **Security**: Enhance capability-based security and input validation
3. **Compatibility**: Improve Linux/BSD compatibility layers

## Implementation Status

### Current State
- **Verified**: Report distro support only for call paths integrated with the target system and tested against its real interfaces.
- **Models**: Distro mode maps and compatibility bridges describe policy or behavior; they are not host OS integrations by themselves.
- **Unavailable**: Authentication and package operations must fail closed when trusted providers and runtime adapters are absent.
- **Planned**: Track missing distro integrations and validation per subsystem rather than treating a mode selector as parity.

### Testing
- **Unit Tests**: Implemented for core functions
- **Integration Tests**: In progress
- **Performance Tests**: Planned

## Architecture Notes

### Key Structures
- Main manager/engine structs for component control
- Configuration enums for component behavior
- Error handling enums for graceful failure

### Dependencies
- **Internal**: Depends on kernel core and memory management
- **External**: No external dependencies (zero-dependency philosophy)

## Development Workflow

### Verification Commands
```bash
# Run component tests
cargo test --lib distro

# Check compilation
cargo check --lib

# Format code
cargo fmt
```

### Common Patterns
- Use safe Rust patterns
- Prefer alloc:: over std:: for kernel code
- Implement comprehensive error handling
- Keep authentication unsupported until credentials are checked by a trusted PAM/BSD-auth/systemd-homed provider.
- Verify new distro-mode mappings with focused tests and preserve unavailable behavior for unintegrated security operations.
- Update the canonical Wiki topic, repository mirrors, and `COMPLETION_STATUS.md`; label compatibility models separately from runtime integration.

## Known Issues

- Integration with multi-distro compatibility layers
- Performance optimization opportunities

## Future Roadmap

### Short Term
- Complete integration testing
- Add performance benchmarks

### Long Term
- Enhanced security features
- Improved compatibility layers

## References

- Linux Kernel Documentation: https://www.kernel.org/doc/html/latest/
- FreeBSD Handbook: https://www.freebsd.org/doc/handbook/
- OpenBSD FAQ: https://www.openbsd.org/faq/

---

*Generated for SigmaOS distro component*

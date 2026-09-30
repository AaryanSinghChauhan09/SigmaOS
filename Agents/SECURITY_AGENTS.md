# Security AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the security subsystem of SigmaOS.

## Component Overview

security is responsible for Security framework including capabilities, seccomp, and sandboxing.

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
- Accept authentication based on nonempty input, a fixed byte prefix, a trusted name, or a simulated state transition
- Claim enforcement when a policy is only constructed or modeled and not installed at the system boundary
- Replace failed authentication, entropy, or provider checks with permissive defaults

## Open Source Inspiration

### Primary Competitors
- **Linux**: Security subsystem - https://www.kernel.org/doc/html/latest/
- **FreeBSD**: Security implementation - https://www.freebsd.org/doc/
- **OpenBSD**: Security design - https://www.openbsd.org/

### Key Improvements Opportunities
1. **Performance**: Optimize for zero-copy operations and reduced latency
2. **Security**: Enhance capability-based security and input validation
3. **Compatibility**: Improve Linux/BSD compatibility layers

## Implementation Status

### Current State
- **Verified only**: Describe a control as implemented only when its execution path is connected to the kernel or trusted provider and the enforcement behavior is tested.
- **Prototype/model**: Compatibility bridges and policy models may aid development but are not runtime controls without integration evidence.
- **Unavailable**: Authentication, randomness, encryption, signatures, or audit claims must fail closed when trusted providers are missing.
- **Planned**: Track missing provider integrations and end-to-end enforcement checks explicitly.

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
cargo test --lib security

# Check compilation
cargo check --lib

# Format code
cargo fmt
```

### Common Patterns
- Use safe Rust patterns
- Prefer alloc:: over std:: for kernel code
- Implement comprehensive error handling
- Preserve deny-by-default behavior on missing credentials, entropy, provider, or policy integration.
- Do not log passwords, keys, tokens, or sensitive request data; use bounded inputs and checked lengths.
- Update the canonical security Wiki topic and its mirrors, state limitations, and give future maintainers focused verification commands.

## Known Issues

- Trusted credential, password-hashing, cryptographic, and kernel-enforcement integrations are incomplete in multiple compatibility modules.
- Some security-facing modules are standalone models and are not linked into the crate or system runtime.
- Integration with multi-distro compatibility layers

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

*Generated for SigmaOS security component*

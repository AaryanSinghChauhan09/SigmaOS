# Crypto AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the crypto subsystem of SigmaOS.

## Component Overview

crypto is responsible for Cryptographic operations and encryption.

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
- Implement replacement ciphers, signatures, hashes, KDFs, password hashes, or CSPRNGs for production use
- Return fabricated keys, signatures, ciphertext, authentication success, or cryptographic audit results
- Treat a checksum, nonempty token, fixed byte prefix, timestamp, or hardware counter as authentication or secure entropy

## Open Source Inspiration

### Primary Competitors
- **Linux**: Crypto subsystem - https://www.kernel.org/doc/html/latest/
- **FreeBSD**: Crypto implementation - https://www.freebsd.org/doc/
- **OpenBSD**: Crypto design - https://www.openbsd.org/

### Key Improvements Opportunities
1. **Performance**: Optimize for zero-copy operations and reduced latency
2. **Security**: Enhance capability-based security and input validation
3. **Compatibility**: Improve Linux/BSD compatibility layers

## Implementation Status

### Current State
- **Provider boundary**: Several crypto-shaped APIs are compatibility prototypes and return provider-unavailable errors. Their names do not establish cryptographic behavior.
- **Implemented**: Only claim an algorithm after known-answer tests, provider review, correct key lifecycle, and call-path integration are verified.
- **Unavailable**: Production-grade password hashing, signatures, key exchange, authenticated encryption, and CSPRNG integration remain unavailable unless a vetted provider is wired in and verified.
- **Planned**: Provider integration, key management, and end-to-end runtime checks must be tracked as plans until complete.

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
cargo test --lib crypto

# Check compilation
cargo check --lib

# Format code
cargo fmt
```

### Common Patterns
- Use safe Rust patterns
- Prefer alloc:: over std:: for kernel code
- Implement comprehensive error handling
- Propagate provider-unavailable errors; never downgrade to plaintext or a deterministic fallback and never mark output encrypted or authenticated.
- Check algorithm behavior with independent known-answer vectors, boundary lengths, malformed input, and tamper cases. Keep secret material out of logs and tests.
- Update `wiki/07-Security.md`, `WIKI/07-Security.md`, and `COMPLETION_STATUS.md` with verified behavior and limitations.

## Known Issues

- Vetted cryptographic providers and end-to-end key management are not integrated for multiple exposed APIs.
- Many crypto-named modules are models or prototypes; do not present them as runtime security controls.
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

*Generated for SigmaOS crypto component*

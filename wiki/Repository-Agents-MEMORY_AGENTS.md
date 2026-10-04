> Imported repository document from [`Agents/MEMORY_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/MEMORY_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# Memory AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the memory subsystem of SigmaOS.

## Component Overview

memory is responsible for Memory management including buddy allocator, slab allocator, and paging.

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

## Open Source Inspiration

### Primary Competitors
- **Linux**: Memory subsystem - https://www.kernel.org/doc/html/latest/
- **FreeBSD**: Memory implementation - https://www.freebsd.org/doc/
- **OpenBSD**: Memory design - https://www.openbsd.org/

### Key Improvements Opportunities
1. **Performance**: Optimize for zero-copy operations and reduced latency
2. **Security**: Enhance capability-based security and input validation
3. **Compatibility**: Improve Linux/BSD compatibility layers

## Implementation Status

### Current State
- **Implemented**: Core functionality is implemented
- **In Progress**: Advanced features and optimizations
- **Planned**: Additional distro compatibility layers

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
cargo test --lib memory

# Check compilation
cargo check --lib

# Format code
cargo fmt
```

### Common Patterns
- Use safe Rust patterns
- Prefer alloc:: over std:: for kernel code
- Implement comprehensive error handling

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

*Generated for SigmaOS memory component*

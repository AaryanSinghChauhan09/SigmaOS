# Kernel AI Agent Guidelines

This document provides specialized guidelines for AI agents working on the kernel subsystem of SigmaOS.

## Component Overview

kernel is responsible for Core kernel subsystems including scheduling, interrupts, and syscalls.

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
- **Linux**: Kernel subsystem - https://www.kernel.org/doc/html/latest/
- **FreeBSD**: Kernel implementation - https://www.freebsd.org/doc/
- **OpenBSD**: Kernel design - https://www.openbsd.org/

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
cargo test --lib kernel

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

*Generated for SigmaOS kernel component*

---

## BSD-Inspired Kernel Techniques

### OpenBSD
- **W^X** (implemented: `src/kernel/wx_pte_hardening.rs`): Every page is writable OR executable, never both. Enforced via PTE flags.
- **pledge(2)** (implemented: `src/security/pledge_unveil.rs`): Restrict process to declared syscall promises.
- **unveil(2)** (implemented: `src/security/pledge_unveil.rs`): Restrict filesystem visibility per path.
- **KARL**: Kernel Address Layout Randomization — relink kernel on each boot.
- **retguard**: Return address protection on every function call.

### FreeBSD
- **Capsicum** (implemented: `src/security/capsicum.rs`): Capability-based security, least-privilege sandboxing.
- **VIMAGE** (implemented: `src/networking/sovereign_net.rs`): Virtualized network stack per jail/container.
- **Netmap** (implemented: `src/net/zero_copy.rs`): Zero-copy packet I/O via memory-mapped rings.
- **bhyve** (implemented: `src/virtualization/`): Type-2 hypervisor for running VMs.
- **UTS Namespaces** (implemented: `src/syscall/uts_syscalls.rs`): Per-process hostname/domainname.

### NetBSD
- **pkgsrc** (implemented: `src/package/`): Cross-platform source package build system.
- **rump kernels**: Userspace kernel driver testing framework.
- **npf**: NetBSD Packet Filter, nftables-compatible ruleset.

### Linux
- **EEVDF Scheduler** (implemented: `src/kernel/scheduler.rs`): Earliest Eligible Virtual Deadline First.
- **CFI** (implemented: `src/kernel/cfi.rs`): Control Flow Integrity for forward/backward edges.
- **kptr_restrict** (implemented: `src/kernel/kptr_restrict.rs`): Prevent kernel address leaks.
- **cgroup v2** (implemented: `src/resource/cgroup_v2.rs`): Unified resource controller hierarchy.
- **eBPF/XDP**: Zero-copy packet processing hook — partially implemented.

## AI Agent Maintenance Instructions

When modifying kernel code:
1. Run `cargo check 2>&1 | grep '^error' | wc -l` — must be 0 before commit
2. Add tests in `#[cfg(test)]` blocks for every new public function
3. Document syscall numbers matching Linux ABI (`src/syscall/`)
4. Ensure W^X invariants are maintained in any memory mapping code
5. Reference: https://www.kernel.org/doc/html/latest/
6. No `unsafe` blocks without `// SAFETY:` comment explaining the invariant

---
## BSD/Linux-Inspired Techniques (Oct 2026 additions)
- **W^X** (OpenBSD): `src/kernel/wx_pte_hardening.rs` — PageFlags struct with WRITE/EXECUTE/ACCESSED/NO_EXECUTE constants
- **EEVDF Scheduler** (Linux 6.6+): `src/kernel/scheduler.rs` — CfsScheduler + RtScheduler
- **CFI** (Linux): `src/kernel/cfi.rs` — forward-edge control flow integrity
- **UTS Namespaces** (Linux): `src/syscall/uts_syscalls.rs` — per-process hostname/domainname
- **POSIX compat stubs**: `src/syscall/posix_compat.rs` — prctl, madvise, pread64, pwrite64, sigaction
## AI Agent Maintenance
Before any kernel commit: `cargo check 2>&1 | grep '^error' | wc -l` must be 0.
Every unsafe block needs `// SAFETY:` comment. Syscall numbers match Linux x86_64 ABI.

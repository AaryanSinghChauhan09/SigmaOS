# SigmaOS AI Agent Guidelines & Tri-Agent Framework

This document provides comprehensive guidelines for AI agents working on SigmaOS, defining the Tri-Agent Framework and operational boundaries for autonomous development.

## Tri-Agent Framework

SigmaOS employs a three-agent autonomous continuous development framework where each agent operates under strict operational boundaries and focused micro-PR constraints.

### ⚡ Bolt: The Performance-OBSESSED Agent

**Core Mission**: Identify and implement focused, measurable performance improvements that make SigmaOS faster, lighter, and more memory-efficient.

**Operational Boundaries**:
- **Always Do**:
  - Run test suite (`cargo check --lib`, `run_sigma_tests.sh`, `pytest tests/`) before submitting PRs
  - Add concise comments explaining performance optimizations
  - Measure and document expected performance impact (latency reduction, memory saving, cycle efficiency)
- **Ask First**:
  - Adding any external crate or dependency
  - Making major architectural changes
- **Never Do**:
  - Modify build manifests (`Cargo.toml`) without instruction
  - Introduce breaking API changes
  - Optimize cold paths prematurely without actual bottlenecks
  - Sacrifice code readability for unmeasurable micro-optimizations

**Philosophy**: Speed is a feature. Every millisecond and CPU cycle counts. **Measure first, optimize second.** Never sacrifice maintainability or correctness for micro-optimizations.

**Journaling Rules (`.jules/bolt.md`)**: Record only critical insights, such as:
- Codebase-specific performance bottlenecks
- Optimizations that unexpectedly failed or regressed latency
- Rejected optimizations with valuable architectural lessons

**Daily Process Workflow**:
1. **🔍 PROFILE**: Identify lock contention, inefficient memory layouts, redundant allocations, unnecessary clones, O(N²) iterations, missing zero-copy abstractions, or unindexed lookups
2. **⚡ SELECT**: Pick a high-impact optimization cleanly implementable in < 50 lines
3. **🔧 OPTIMIZE**: Write clean, self-explaining, lock-free or memory-efficient code
4. **✅ VERIFY**: Run cargo tests, benchmark benchmarks, and verify functional correctness
5. **🎁 PRESENT**: Submit PR with title format `⚡ Bolt: [performance improvement]`

### 🎨 Palette: The UX & Accessibility Agent

**Core Mission**: Enhance Zenith Desktop, Web UI, and CLI user interfaces with accessible, intuitive, and delightful user interactions.

**Operational Boundaries**:
- **Always Do**:
  - Test keyboard navigation and focus visibility
  - Add proper ARIA labels, roles, and contrast guarantees
  - Maintain clean separation between styling and application state
  - Keep changes strictly under 50 lines
- **Ask First**:
  - Major UI design or global design token changes
- **Never Do**:
  - Make complete page/component redesigns without approval
  - Add heavy UI dependencies
  - Change core performance or security backend logic

**Philosophy**: Users notice micro-details. Accessibility (a11y) is mandatory, not optional. Every interaction should feel smooth, responsive, and clear.

**Journaling Rules (`.jules/palette.md`)**: Record critical UX/a11y insights, such as component-specific contrast issues, keyboard focus bugs, or reusable accessibility patterns.

### 🛡️ Sentinel: The Security & Hardening Agent

**Core Mission**: Protect SigmaOS kernel and userland from security vulnerabilities, privilege escalation, memory unsafety, and data leaks.

**Operational Boundaries**:
- **Always Do**:
  - Run full security verification and regression test suites
  - Validate and sanitize all userland inputs at system call boundaries
  - Use constant-time cryptography and memory zeroization
  - Keep fixes focused and under 50 lines
- **Ask First**:
  - Modifying authentication, capabilities, or access control models
- **Never Do**:
  - Commit API keys, tokens, or hardcoded secrets
  - Expose raw kernel stack traces or memory addresses to userland

**Philosophy**: Security is foundational. Defense in depth: validate at every boundary. Fail safely and zeroize sensitive memory immediately.

**Journaling Rules (`.jules/sentinel.md`)**: Record critical security learnings, vulnerability patterns, and mitigation strategies.

## Universal Agent Guidelines

### Code Quality Standards

1. **Language**: All kernel and system code must be written in **safe Rust** (unsafe only when strictly necessary for hardware access)
2. **Style**: Run `cargo fmt` before committing
3. **Tests**: Add tests for new functionality
4. **Documentation**: Include doc comments for all public APIs
5. **No external runtimes**: No Python, Node.js, Java, or Go dependencies in the kernel

### Development Workflow

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/SigmaOS.git

# Create a feature branch
git checkout -b feature/your-feature

# Make changes and verify
cargo check --lib
cargo test

# Commit and push
git add .
git commit -m "feat: add your feature description"
git push origin feature/your-feature

# Open a Pull Request
```

### Pre-Commit Verification Protocol

Before submitting any code or documentation changes, all agents must complete the pre-commit protocol:

1. **Static Analysis & Compilation**: Execute `cargo check --lib` to ensure zero compilation warnings or errors
2. **Unit Test Verification**: Run target module unit tests using `rustc --test` or `cargo test`
3. **Integration Test Suite**: Run `./run_sigma_tests.sh` and `pytest tests/` to confirm 100% test pass rate
4. **Mirror Parity Check**: Confirm that all modified documentation is reflected across `docs/`, `wiki/`, `WIKI/`, and `wiki_repo/`

### Error Resolution Algorithms

#### Algorithm A: Workspace Crate Compilation Resolution Protocol

```
INPUT: Compiler output from `cargo check`
OUTPUT: Clean crate build with 0 errors

STEP 1: Run `cargo check 2>&1 | grep "error[E"` to generate the exact collision list.
STEP 2: For each collision error:
   a. IF Error is "duplicate definition of struct/trait X":
      i. Locate both definitions using `grep -rn "struct X" src/`.
      ii. Keep the canonical implementation in its primary domain module.
      iii. Convert secondary definitions to re-exports: `pub use crate::canonical_module::X;`.
   b. IF Error is "duplicate test name Y":
      i. Rename the secondary test function with a distinct descriptive suffix.
   c. IF Error is "conflicting implementations of trait Z":
      i. Introduce a newtype wrapper `struct SpecificWrapper(TargetType);` or use conditional compilation attributes `#[cfg(feature = "...")]`.
STEP 3: Verify fix by re-running `cargo check`.
```

#### Algorithm B: Subsystem Parity Gap Closure Protocol

```
INPUT: Subsystem feature request or missing syscall/API
OUTPUT: Native Safe-Rust `klib` implementation with 100% test coverage

STEP 1: Identify target shard and module in `src/`.
STEP 2: Implement state struct, configuration enum, and error handling enum using zero third-party dependencies.
STEP 3: Implement main processing engine and public API gateway.
STEP 4: Re-export engine in target module's `mod.rs` and `src/lib.rs`.
STEP 5: Add comprehensive `#[cfg(test)]` unit test suite in target file.
STEP 6: Verify with `./run_sigma_tests.sh`.
```

### Security Hardening Rules

1. **No Hardcoded Cryptographic Values**: Never use hardcoded keys, passwords, salts, or initialization vectors. Use randomly generated key material.
2. **Memory Safety**: Ensure all memory operations are bounds-checked and use safe Rust patterns.
3. **Input Validation**: Validate all user inputs at system call boundaries.
4. **Least Privilege**: Implement capability-based security models using pledge/unveil where appropriate.
5. **Secure Defaults**: Default to secure configurations rather than convenient ones.

### Zero-Dependency Philosophy

SigmaOS is designed to eliminate dependencies on:
- External package managers (pacman, apt, rpm, etc. - these are handled by SigmaPkg)
- Third-party libraries in kernel space
- External runtimes (Python, Node.js, Java, Go)
- Predefined wrappers and abstractions

All data structures and algorithms should be implemented directly using:
- Raw hardware pointers and bare-metal memory pages (kernel space)
- Safe Rust standard library (user space when std is available)
- Custom `klib` primitives for no_std compatibility

### Documentation Standards

1. **Public APIs**: Every public function, struct, and enum must have doc comments
2. **Examples**: Include usage examples in doc comments
3. **Safety**: Document unsafe blocks with safety invariants
4. **Panics**: Document conditions that cause panics
5. **Error Handling**: Document all possible error conditions

## Repository Organization

### Key Directories

- `src/kernel/` - Core kernel subsystems
- `src/memory/` - Memory management (Buddy, Slab, Paging)
- `src/vfs/` - Virtual Filesystem layer
- `src/sigpkg/` - Universal package manager
- `src/desktop/` - Zenith desktop environment
- `src/security/` - Security framework
- `src/klib/` - Kernel library (no_std compatible primitives)
- `tests/` - Test suites
- `docs/` - Documentation
- `.github/workflows/` - CI/CD workflows

### Documentation Files

- `README.md` - Project overview and quick start
- `AGENTS.md` - This file (agent guidelines)
- `WHAT_IS_WORKING_AND_NOT_WORKING.md` - Component status tracker
- `FUTURE-DEVELOPMENT-ROADMAP.md` - Roadmap and specifications
- `FUTURE_LINUX_BSD_MISSING_COMPONENTS_AGENTS.md` - Agent guidelines for closing Linux & BSD distro component feature gaps
- `SIGMAOS_MASTER_PLAN_TRI_AGENT_500_REPOS_ABSORPTION.md` - Tri-agent framework details
- `CAPABILITY_MATRIX.toml` - Machine-readable capability matrix
- `FEATURE_STATUS.toml` - Feature implementation status

## Continuous Integration

SigmaOS uses GitHub Actions for CI/CD:
- `pr_fast_checks.yml` - Fast PR validation (format, clippy, basic tests)
- `security.yml` - Security scanning and dependency checks
- `qemu-boot-smoke-test.yml` - QEMU boot smoke tests
- `documentation-checks.yml` - Documentation validation

## Performance Benchmarks

When making performance changes, always:
1. Establish baseline measurements
2. Run controlled benchmarks
3. Document the improvement
4. Ensure no regression in other areas

## Testing Requirements

1. **Unit Tests**: Every module must have unit tests
2. **Integration Tests**: Cross-module functionality tests
3. **Regression Tests**: Tests for previously fixed bugs
4. **Coverage**: Aim for high test coverage on critical paths

## Collaboration Guidelines

1. **Respect Boundaries**: Each agent should respect the domain of other agents
2. **Communication**: Use clear commit messages and PR descriptions
3. **Review**: Participate in code reviews constructively
4. **Documentation**: Update documentation when changing behavior
5. **Stability**: Prioritize stability over speed of development

## Emergency Procedures

In case of critical issues:
1. **Security Vulnerabilities**: Immediately escalate, do not attempt fixes without authorization
2. **Data Loss**: Stop all operations, investigate root cause
3. **Build Failures**: Rollback to last known good state
4. **Performance Regression**: Revert if unexplained, investigate before reapplying

## Version Control Policy

1. **Branch Naming**: Use descriptive branch names (e.g., `feature/bolt-memory-optimization`)
2. **Commit Messages**: Follow conventional commits format
3. **PR Titles**: Clear, descriptive, and scoped
4. **Merge Strategy**: Use squash merges for clean history
5. **Release Tags**: Follow semantic versioning

## AI Agent Specific Instructions

### When to Ask for Help

- If unsure about architectural implications
- If a change might affect multiple subsystems
- If security implications are unclear
- If performance impact is uncertain
- If documentation is missing or unclear

### When to Proceed Independently

- Clear bug fixes with known solutions
- Performance optimizations with measurable impact
- Documentation improvements
- Test additions
- Code style fixes

### Quality Gates

Before considering a task complete:
1. All tests pass
2. Code compiles without warnings
3. Documentation is updated
4. Performance is measured (if applicable)
5. Security review is passed (if applicable)

## Future Development Roadmap: Linux & BSD Missing Component Gap Closure

This section defines the strategic future development roadmap for autonomous AI agents working on closing missing component gaps in SigmaOS relative to mainstream Linux distributions (Arch, Debian, Fedora, Alpine, NixOS, Gentoo, Void, CachyOS, Omarchy) and BSD operating systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Illumos/Solaris).

### 🚀 Milestone 1: Kernel & System Architecture Gaps
- **Linux eBPF / XDP Subsystem Enhancement**:
  - Full x86_64 JIT compiler (`compile_x86_64_jit`) for eBPF bytecodes.
  - Zero-copy AF_XDP socket maps (`XSK`) and BPF_MAP_TYPE_RINGBUF event passing.
- **BSD Memory & Kernel Architecture**:
  - OpenBSD KARL (Kernel Address Randomized Link) and W^X memory page protection allocator.
  - NetBSD Rump Kernel userland driver host bridge for isolated device driver execution.
  - DragonFly BSD Lockless Per-CPU Netpoll Ring & Variant Symlinks (`varsyms`) resolution.

### 🛡️ Milestone 2: Security & Sandboxing Gaps
- **Landlock v5 Network Guard & OpenBSD Pledge/Unveil**:
  - Full path-based unveil restriction locking and socket port binding/connect controls.
  - FreeBSD Capsicum descriptor capability rights and IOMMU DMA fault containment.
- **Post-Quantum Cryptography & Isolation**:
  - Dilithium / Falcon / ML-KEM PQC signature verification in package manager transactions (`sigma-pkg`).
  - Firejail / Bubblewrap container sandboxing profiles for developer environments.

### 💾 Milestone 3: Storage & Filesystem Gaps
- **Self-Healing Copy-On-Write Storage**:
  - Bcachefs multi-tier storage engine with automatic SSD promotion and cold HDD demotion.
  - FreeBSD OpenZFS pool integration with Fletcher-4 checksum verification and zero-copy dataset clones.
  - DragonFly HAMMER2 MVCC B-Tree snapshotting and cluster quorum consensus.

### 📦 Milestone 4: Package Management & Build Infrastructure
- **Universal Package Parity**:
  - Transpilation gateway for `.deb`, `.rpm`, `PKGBUILD`, `.ebuild`, `APKBUILD`, `.xbps`, and `.nix` Flakes into native `.sigpkg`.
  - Content-Addressed Storage (CAS) with reachability mark-and-sweep garbage collection.
  - SAT dependency solver using Davis-Putnam-Logemann-Loveland (DPLL) with cycle detection.

### 🎨 Milestone 5: Desktop & Developer Tools Gaps
- **Wayland Direct KMS Scanout & Zenith Desktop**:
  - Holographic 3D LUT HDR color transformations and sub-millisecond Wayland window scanout.
  - Cinnamon/XApp desktop integration (Desklets, Warpinator LAN transfer, Timeshift snapshots, Hypnotix IPTV).
  - Omarchy Developer Tools Suite (Theme Switcher, Stow Dotfiles Manager, Hyprland Binds, Fastfetch, Herdr AI Orchestrator).

---

## Appendix: Quick Reference

### Common Commands

```bash
# Build check
cargo check --lib

# Run tests
cargo test
./run_sigma_tests.sh

# Format code
cargo fmt

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Clean build
make clean
```

### Useful Patterns

```rust
// Safe Rust with error handling
pub fn safe_function(input: &str) -> Result<Output, Error> {
    // Implementation
}

// Custom error type
#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    NotFound,
    PermissionDenied,
}

// Zero-copy patterns
pub fn process_data(data: &[u8]) -> Result<&[u8], Error> {
    // Process without allocation
}
```

---

*End of Agent Guidelines*

---

## Linux & BSD Distro Inspiration Guidelines

When implementing new components, AI agents MUST draw inspiration from these proven systems:

### Kernel Subsystems
- **Scheduler**: Implement EEVDF (Linux 6.6+) or CFS with W^X memory hardening (OpenBSD KARL)
- **Memory**: Buddy allocator + slab allocator patterns (Linux mm/), ASLR (PaX/grsecurity)
- **IPC**: HelenOS async IPC, Mach IPC ports, FreeBSD Capsicum capabilities
- **Networking**: Linux netfilter/nftables, FreeBSD VIMAGE network stacks, Netmap zero-copy

### Security Hardening (mandatory for all new code)
- **Pledge/Unveil**: OpenBSD-style syscall restriction (`src/security/pledge_unveil.rs`)
- **Seccomp-BPF**: Linux seccomp filters (`src/security/seccomp_filter.rs`)
- **W^X Enforcement**: No page simultaneously writable and executable (`src/kernel/wx_pte_hardening.rs`)
- **CFI**: Control Flow Integrity (`src/kernel/cfi.rs`)
- **kptr_restrict**: Prevent kernel pointer leaks (`src/kernel/kptr_restrict.rs`)
- **No hardcoded crypto**: Always use `src/crypto/entropy.rs` for random values

### Package Management
- Support all 33+ package formats via `SovereignUniversalPackageFormatMasterEngine`
- Always add BsdPkg, FreeBsdPkg, OpenBsdPkg variants when adding package format enums
- Use `sigpkg` as the unified CLI for all package operations

### Build Rules (non-negotiable)
- Run `cargo check 2>&1 | grep '^error' | wc -l` → must be **0** before every commit
- Zero errors required before pushing to main
- Use `#[allow(dead_code)]` only with a justification comment
- No `std::` imports in `no_std` modules — use `alloc::` instead
- No external crate dependencies — implement everything from scratch

### Branch Policy
- Only ONE branch: **main**. All work goes directly to main via commits.
- Delete feature branches immediately after merging.
- PRs are auto-merged if build passes and no conflicts.

### Architecture Targets (all must compile)
- x86_64 bare-metal (primary)
- AArch64 / ARM (secondary)
- RISC-V 64-bit (tertiary)
- Must support both CISC (x86) and RISC (ARM, RISC-V) instruction set philosophies

---
## Linux/BSD Architecture Inspiration Guidelines (Oct 2026)

### Build Rules (non-negotiable)
- `cargo check 2>&1 | grep '^error' | wc -l` → must be **0** before every commit
- No `std::` in no_std modules — use `alloc::`
- No external crate deps — implement from scratch

### Branch Policy
- Only **main** branch. Delete feature branches immediately after merging.

### New Modules Added
- `src/crypto/entropy.rs` — XorShift64 PRNG entropy pool (RDRAND-ready)
- `src/syscall/posix_compat.rs` — POSIX compat stubs (prctl, madvise, pread64, sigaction)

### Architecture Targets
- x86_64 bare-metal (primary), AArch64/ARM (secondary), RISC-V 64 (tertiary)

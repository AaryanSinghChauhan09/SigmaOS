# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS — the world's most advanced sovereign, bare-metal operating system.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Philosophy](#development-philosophy)
4. [Repository Structure](#repository-structure)
5. [How to Contribute](#how-to-contribute)
6. [Coding Standards](#coding-standards)
7. [Testing Requirements](#testing-requirements)
8. [Submitting a Pull Request](#submitting-a-pull-request)
9. [Security Vulnerability Reporting](#security-vulnerability-reporting)
10. [Issue Guidelines](#issue-guidelines)
11. [Documentation Standards](#documentation-standards)
12. [Contact](#contact)

---

## Code of Conduct

SigmaOS is committed to providing a welcoming environment for all contributors. We expect all participants to:

- Be respectful and constructive in all interactions
- Focus on what is best for the project and the community
- Accept constructive criticism gracefully
- Show empathy toward other community members

Harassment, discrimination, and personal attacks will not be tolerated.

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (nightly recommended for kernel development)
- `git` 2.30+
- [GitHub CLI](https://cli.github.com/) `gh` (optional but recommended)
- QEMU for testing (optional)

### Setting Up the Development Environment

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust nightly (for no_std kernel features)
rustup install nightly
rustup component add rust-src --toolchain nightly

# Build the project
cargo build 2>&1

# Run tests
cargo test --lib
```

---

## Development Philosophy

SigmaOS follows these core principles, which all contributions must respect:

### 1. Zero Dependency / Minimal Predefined Libraries
SigmaOS uses its own `klib` (kernel library) instead of the Rust standard library for kernel components:
- Use `core::` instead of `std::` in kernel modules
- Prefer `klib::*` implementations over external crates
- All new kernel code should compile with `#![no_std]`

### 2. OOP-Inspired Rust Architecture
SigmaOS uses a trait-based object-oriented design:
- Define behavior via `trait` interfaces
- Implement behavior via `struct` + `impl Trait for Struct`
- Use `Box<dyn Trait>` for dynamic dispatch where needed

### 3. Security First
- All new features must have a documented threat model
- Never use `unsafe` without a documented safety comment
- Capability-check all privileged operations

### 4. Linux Distro Inspiration
Draw from the best Linux distros:
- Arch Linux: rolling releases, minimal base, user-centric
- NixOS: reproducible builds, atomic upgrades
- Debian: stability, long-term support
- Alpine: minimal footprint, musl-like philosophy
- QubesOS: compartmentalization, disposable VMs
- Parrot OS: security tooling

---

## Repository Structure

```
SigmaOS/
├── src/                    # Core OS source
│   ├── kernel/             # Microkernel core (scheduler, memory, IPC)
│   ├── security/           # Capability system, isolation, crypto
│   ├── filesystem/         # VFS, Btrfs/ZFS/SigFS implementations
│   ├── network/            # TCP/IP stack, TLS, DNS
│   ├── driver/             # Driver framework (HAL)
│   ├── drivers/            # Concrete driver implementations
│   ├── klib/               # Custom kernel library (no_std)
│   ├── distro/             # Linux distro-inspired features
│   ├── sigpkg/             # Package manager
│   ├── shell/              # Sigma Shell (REPL)
│   ├── accessibility/      # A11y subsystem
│   └── lib.rs              # Crate root
├── tests/                  # Integration tests
├── docs/                   # Documentation
├── wiki/                   # Wiki source files (synced to GitHub Wiki)
├── tools/                  # Build tools and compat utils
├── zenith_desktop/         # Zenith Desktop Environment
└── Cargo.toml              # Workspace manifest
```

---

## How to Contribute

### Types of Contributions Welcome

1. **Bug fixes** — Fixes for compile errors, logic bugs, security issues
2. **New features** — Kernel modules, drivers, security features
3. **Documentation** — Improving docs, adding examples, wiki pages
4. **Tests** — Unit tests, integration tests, fuzzing harnesses
5. **Performance** — Optimizations, profiling, benchmarks
6. **Distro ideas** — Implementing ideas from Linux distributions
7. **Security hardening** — Fixing CodeQL alerts, adding hardening

### Workflow

1. **Fork** the repository on GitHub
2. **Create** a feature branch: `git checkout -b feature/my-improvement`
3. **Make** your changes following the coding standards below
4. **Test** your changes thoroughly
5. **Commit** with a descriptive message: `git commit -m "feat(kernel): add BORE scheduling"`
6. **Push** to your fork: `git push origin feature/my-improvement`
7. **Open** a Pull Request against `main`

---

## Coding Standards

### Rust Style

```rust
// ✅ Good: Explicit, clear, no stdlib in kernel code
use core::sync::atomic::{AtomicU64, Ordering};

pub struct CapabilityToken {
    bits: u64,
}

impl CapabilityToken {
    /// Creates a new capability token with no permissions.
    pub fn new() -> Self {
        Self { bits: 0 }
    }
}

// ❌ Bad: Using std in kernel code
use std::collections::HashMap; // NOT allowed in kernel modules
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `security`

Examples:
```
feat(kernel): implement BORE CPU scheduler with burst detection
fix(security): resolve capability bitmask overlap in privilege escalation
docs(arch): update architecture diagram with AI scheduler
security(capability): fix privilege escalation via bitmask overflow
perf(simd): optimize vector addition with zip iterator chains
```

### Code Comments

```rust
/// Public function documentation (shown in rustdoc)
///
/// # Arguments
/// * `bits` - The 64-bit capability bitmask
///
/// # Returns
/// A new CapabilityToken with the given permissions
///
/// # Safety
/// The caller must ensure `bits` only contains valid capability flags.
pub fn from_bits(bits: u64) -> Self {
    Self { bits }
}

// Internal comment explaining non-obvious logic
// Using SeqCst ordering to prevent reordering with other atomic operations
self.current.store(token.bits(), Ordering::SeqCst);
```

### Error Handling

```rust
// ✅ Use Result<T, E> with descriptive error types
pub enum KernelError {
    OutOfMemory,
    PermissionDenied,
    InvalidArgument,
}

pub fn allocate(size: usize) -> Result<*mut u8, KernelError> {
    if size == 0 {
        return Err(KernelError::InvalidArgument);
    }
    // ...
}

// ❌ Never panic in kernel code
// panic!("out of memory"); // NOT ALLOWED
```

### Unsafe Code

```rust
// Every unsafe block must have a SAFETY comment explaining why it's safe
unsafe {
    // SAFETY: ptr is guaranteed to be non-null and aligned by the
    // allocator contract established at construction time.
    core::ptr::write(ptr, value);
}
```

---

## Testing Requirements

All contributions must include appropriate tests:

### Unit Tests (in-module)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let token = CapabilityToken::new();
        assert_eq!(token.bits(), 0);
        assert!(!token.has_permission(Permission::NetworkTcp));
    }
}
```

### Integration Tests
Place in `tests/` directory. These test cross-module interactions.

### Test Coverage Requirements
- New kernel features: minimum 80% coverage
- Security features: 100% of public API covered
- Bug fixes: must include regression test

### Running Tests

```bash
# Run all unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_test

# Run with output
cargo test -- --nocapture
```

---

## Submitting a Pull Request

### PR Title Format
Keep it under 70 characters: `feat(scope): brief description`

### PR Description Template

```markdown
## Summary
Brief description of what this PR does and why.

## Changes
- Added X to kernel/memory.rs
- Fixed Y in security/capability.rs
- Improved Z performance by N%

## Testing Done
- [ ] Unit tests pass: `cargo test --lib`
- [ ] Integration tests pass: `cargo test --test integration_test`
- [ ] Manually tested on QEMU
- [ ] No new security warnings introduced

## Security Considerations
Does this PR affect security? If yes, describe the threat model changes.

## References
- Closes #<issue number>
- Inspired by: [Arch Linux AUR](https://aur.archlinux.org/) 
```

### Review Process

1. At least one maintainer review required
2. All CI checks must pass
3. No new `severity=error` CodeQL alerts
4. Documentation updated if API changed

---

## Security Vulnerability Reporting

**Do not open public issues for security vulnerabilities.**

Instead, please:
1. Email: security@sigmaos.dev (or open a private GitHub Security Advisory)
2. Include: affected version, reproduction steps, impact assessment
3. Response time: we aim to respond within 48 hours

See [docs/SECURITY.md](docs/SECURITY.md) for the full security policy.

---

## Issue Guidelines

### Bug Reports
- Include SigmaOS version or commit hash
- Describe expected vs. actual behavior
- Include minimal reproduction steps
- Attach relevant logs or error messages

### Feature Requests
- Describe the use case and motivation
- Reference relevant Linux distro or OS that has this feature
- Consider the security implications

### Good First Issues

Look for issues labeled `good first issue` — these are suitable for newcomers:
- Documentation improvements
- Adding `#[allow(dead_code)]` suppressions
- Adding missing trait implementations
- Writing tests for existing code

---

## Documentation Standards

### Markdown Files
- Use ATX-style headers (`#`, `##`, `###`)
- Code blocks must specify language: ` ```rust `, ` ```bash `
- Table of Contents for files > 100 lines
- Update the wiki when adding new docs

### Rustdoc
- All `pub` items must have doc comments (`///`)
- Include `# Examples` section where relevant
- Use `# Safety` section for unsafe functions
- Use `# Errors` section for fallible functions

---

## Contact

- **GitHub Issues**: [github.com/AaryanSinghChauhan09/SigmaOS/issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **GitHub Wiki**: [github.com/AaryanSinghChauhan09/SigmaOS/wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- **Security**: Open a [GitHub Security Advisory](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories/new)

---

*"Sovereignty is the ultimate efficiency."* — SigmaOS Team

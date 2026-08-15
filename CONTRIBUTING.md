# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document outlines guidelines for contributing.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Development Setup

### Prerequisites

- Rust nightly toolchain: `rustup toolchain install nightly`
- QEMU for testing: `sudo apt install qemu-system-x86`
- Cross-compilation target: `rustup target add x86_64-unknown-none`
- Optional: `cargo install cargo-fuzz` for fuzzing

### Building

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
cargo build --release
# Or for kernel-only:
cargo build -p kernel --target x86_64-unknown-none
```

### Running Tests

```bash
cargo test                          # All unit tests
cargo test -p kernel                # Kernel tests only
cargo test --doc                    # Doctests
cargo test -p klib                  # klib tests
```

## Contribution Guidelines

### Code Style

1. **No std in kernel code**: Use `#![no_std]` + `extern crate alloc`
2. **Prefer klib**: Use `crate::klib` over `alloc` for kernel collections
3. **No `unwrap()`**: Use `?` or explicit error handling in kernel code
4. **Document with `///`**: All public APIs must have doc comments
5. **Rustfmt**: Run `cargo fmt` before submitting
6. **Clippy**: Run `cargo clippy -- -D warnings`

### Security Guidelines

1. **No hard-coded secrets**: Never commit keys, passwords, or secrets
2. **Validate all inputs**: Especially in security-sensitive modules
3. **Limit unsafe**: Document every `unsafe` block with a SAFETY comment
4. **Test security properties**: Write tests that verify security invariants

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `security`: Security improvement
- `perf`: Performance improvement
- `docs`: Documentation changes
- `test`: Test additions/improvements
- `refactor`: Code refactoring
- `chore`: Build/tooling changes

Examples:
```
feat(security): add Capsicum capability mode enforcement
fix(klib): correct race condition in buddy allocator
security(unveil): prevent directory traversal in path normalization
perf(scheduler): replace linear scan with BTreeMap in BORE scheduler
```

### Pull Request Process

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feat/my-feature`
3. **Make changes** following the guidelines above
4. **Add tests** for new functionality
5. **Run CI locally**: `cargo test && cargo clippy && cargo fmt --check`
6. **Push** and open a PR against `main`
7. **Fill out the PR template**
8. **Address review comments**

### PR Requirements

- [ ] All CI checks pass (CodeQL, tests, clippy, fmt)
- [ ] New public APIs have documentation
- [ ] Security-sensitive changes reviewed by maintainer
- [ ] No new `unwrap()` calls in kernel code
- [ ] CHANGELOG.md updated for user-visible changes

## Areas for Contribution

### High Priority
- **Security**: Fixes for [CodeQL alerts](https://github.com/AaryanSinghChauhan09/SigmaOS/security/code-scanning)
- **klib**: Extending the zero-dependency standard library
- **Drivers**: New hardware driver support
- **Documentation**: Wiki pages and examples

### Good First Issues
- Adding doc comments to undocumented public APIs
- Converting `std` imports to `alloc` imports in kernel code
- Writing tests for existing modules
- Fixing clippy warnings

### Advanced Topics
- SigmaBus IPC message routing improvements
- BORE scheduler enhancements
- Post-quantum cryptography integration
- New Linux/BSD parity features

## Architecture Overview

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full system architecture.

Key modules:
- `src/klib/` - Zero-dependency kernel library (start here!)
- `src/security/` - Security subsystems (pledge, unveil, capsicum)
- `src/kernel/` - Core kernel
- `src/ipc/` - Inter-process communication
- `src/scheduler/` - Process scheduling

## Getting Help

- GitHub Issues: For bug reports and feature requests
- GitHub Discussions: For questions and design discussions
- Wiki: For architecture and implementation details

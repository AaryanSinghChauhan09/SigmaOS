# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines for contributing.

## Code of Conduct

All contributors must follow our [Code of Conduct](CODE_OF_CONDUCT.md). Be respectful, inclusive, and constructive.

## Ways to Contribute

### 1. Code Contributions
- Bug fixes
- New features
- Performance improvements
- Security hardening
- Driver support

### 2. Documentation
- Wiki pages
- API documentation
- Tutorial writing
- Translation/localization

### 3. Testing
- Bug reports with reproduction steps
- Hardware compatibility testing
- Performance benchmarking
- Security auditing

### 4. Community
- Answer questions in issues
- Review pull requests
- Write blog posts
- Speak at conferences

## Development Setup

```bash
# Clone repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rust-src clippy rustfmt
rustup target add x86_64-unknown-none aarch64-unknown-none

# Build
cargo build --workspace

# Run tests
cargo test --workspace

# Lint
cargo clippy -- -D warnings
cargo fmt --check
```

## Pull Request Process

1. **Fork** the repository
2. **Branch** from `main` using naming convention:
   - `feat/<description>` for features
   - `fix/<description>` for bug fixes
   - `perf/<description>` for performance
   - `docs/<description>` for documentation
   - `sec/<description>` for security
3. **Commit** with conventional commit format:
   - `feat(scope): description`
   - `fix(scope): description`
   - `perf(scope): description`
   - `docs(scope): description`
4. **Test** your changes thoroughly
5. **Submit** PR with description of changes
6. **Respond** to review feedback

## Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `perf`, `docs`, `style`, `refactor`, `test`, `chore`, `sec`
Scopes: `kernel`, `ai`, `security`, `desktop`, `networking`, `fs`, `pkg`, `docs`

## Review Process

- All PRs require at least 1 review from a maintainer
- Security-related PRs require 2 reviews
- Architectural changes require TSC approval
- CI must pass before merge

## Security Reporting

Do NOT open public issues for security vulnerabilities.
Use GitHub Security Advisories instead.
See [SECURITY.md](SECURITY.md) for details.

## Recognition

All contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Annual contributor report
- SigmaOS project website

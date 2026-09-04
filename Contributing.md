# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS!

## How to Contribute

### Reporting Bugs

Open an issue at https://github.com/AaryanSinghChauhan09/SigmaOS/issues  
Use the Bug Report template and include:
- SigmaOS version
- Architecture
- Steps to reproduce
- Expected vs actual behavior

### Suggesting Features

Open a Feature Request issue with:
- Problem description
- Proposed solution
- Affected components

### Code Contributions

1. **Fork** the repository
2. Create a **feature branch**: `git checkout -b feat/my-feature`
3. Make your changes following the coding guidelines
4. **Test**: `cargo +nightly test --all`
5. **Lint**: `cargo +nightly clippy --all`
6. **Format**: `cargo +nightly fmt --all`
7. Open a **Pull Request**

## Coding Guidelines

- Follow Rust idioms and best practices
- Document all public APIs with `///` doc comments
- Justify every `unsafe` block with a `// SAFETY:` comment
- Use `checked_*` arithmetic in security-sensitive code
- No hardcoded credentials or secrets
- Test coverage for new features

## Branch Naming

| Type | Format | Example |
|------|--------|---------|
| Feature | `feat/description` | `feat/nvme-driver` |
| Bug fix | `fix/description` | `fix/ipv4-validation` |
| Documentation | `docs/description` | `docs/kernel-guide` |
| Performance | `perf/description` | `perf/scheduler-opt` |
| CI/CD | `ci/description` | `ci/add-audit` |

## Commit Messages

Use Conventional Commits format:
```
feat(kernel): add EEVDF scheduler
fix(security): reject IPv4 octets with leading zeros
docs(readme): fix badge URLs
chore: untrack node_modules from git
```

## Code of Conduct

Please read and follow our [[https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CODE_OF_CONDUCT.md|Code of Conduct]].

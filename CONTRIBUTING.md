# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust toolchain (stable)
- Python 3.x
- GCC (for kernel compilation)
- Docker (optional, for containerization)
- QEMU (optional, for testing)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the project
cargo build --release

# Run tests
cargo test

# Build ISO
make iso-stub
```

## Development Workflow

### Branching Strategy

- `main`: Stable releases
- `develop`: Development branch
- `feature/*`: Feature branches
- `bugfix/*`: Bug fix branches
- `hotfix/*`: Critical fixes for stable releases

### Creating a Feature Branch

```bash
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name
```

### Making Changes

1. Write code following the [Style Guide](#style-guide)
2. Add tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Run linter: `cargo clippy`
5. Format code: `cargo fmt`
6. Update documentation if needed

### Committing Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Build process or auxiliary tool changes
- `ci`: CI/CD changes

Example:
```
feat(kernel): add zero-copy network buffer implementation

Implements zero-copy buffer for network stack to improve
performance by avoiding unnecessary memory copies.

Closes #123
```

### Pull Request Process

1. Update your branch: `git pull origin develop`
2. Push to GitHub: `git push origin feature/your-feature-name`
3. Create Pull Request on GitHub
4. Fill PR template with required information
5. Wait for code review
6. Address review feedback
7. Merge when approved

## Style Guide

### Rust Code Style

- Use `cargo fmt` for formatting
- Follow Rust naming conventions:
  - Types: `PascalCase`
  - Functions/variables: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
- Avoid `#![allow(warnings, unused)]` unless necessary
- Document public APIs with `///` doc comments
- Write tests alongside code

### Shell Script Style

- Use `#!/usr/bin/env bash` shebang
- Use `set -euo pipefail` for error handling
- Quote variables: `"$VAR"`
- Use functions for reusable code
- Add comments for complex logic

### Documentation Style

- Use clear, concise language
- Include examples
- Use proper Markdown formatting
- Update relevant sections when code changes

## Testing

### Unit Tests

Write unit tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Add integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
#[test]
fn test_integration() {
    // Integration test code
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

## Component Development

### Creating a New Component

Use the automated script:

```bash
./scripts/create_component_repo.sh \
  SigmaOS-component-name \
  "Component description" \
  "Category" \
  "rust"
```

### Component Structure

```
SigmaOS-component-name/
├── src/
│   ├── lib.rs
│   └── core.rs
├── tests/
│   └── integration_test.rs
├── docs/
│   └── component-docs.md
├── .github/
│   └── workflows/
│       └── test.yml
├── Cargo.toml
├── README.md
├── LICENSE
└── CONTRIBUTING.md
```

### Component Dependencies

Add to COMPONENTS.toml:

```toml
[[component]]
name = "SigmaOS-component-name"
description = "Component description"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-component-name"
engine = "component"
category = "Category"
language = "rust"
inspirations = ["native"]
visibility = "public"
depends_on = ["SigmaOS-kernel", "SigmaOS-filesystem"]
```

## Documentation

### Documentation Standards

- Keep README.md concise (link to Wiki for details)
- Migrate detailed docs to GitHub Wiki
- Follow the Wiki structure in docs/wiki/WIKI_STRUCTURE.md
- Update CHANGELOG.md for significant changes

### Wiki Pages

- Create Wiki pages for new features
- Update existing pages when code changes
- Add examples and usage instructions
- Keep navigation structure consistent

## Principles

SigmaOS follows 100+ software engineering principles. Key principles to follow:

- **KISS**: Keep It Simple, Stupid
- **DRY**: Don't Repeat Yourself
- **SOLID**: Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion
- **YAGNI**: You Aren't Gonna Need It
- **Gall's Law**: Start simple, work up to complex systems
- **Zero-Copy**: Avoid unnecessary memory copies
- **Fail-Fast**: Fail early and explicitly
- **Defensive Programming**: Assume errors will happen

See [Principles Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Principles) for full list.

## Issue Reporting

### Before Creating an Issue

1. Check existing issues for duplicates
2. Check if the issue is already fixed in `develop` branch
3. Gather necessary information:
   - SigmaOS version
   - Rust version
   - Operating system
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Error messages/logs

### Issue Template

```markdown
## Description
Brief description of the issue

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- SigmaOS version: x.y.z
- Rust version: x.y.z
- OS: Linux x.y.z

## Additional Context
Any other relevant information
```

## Security Issues

For security vulnerabilities, please:
1. Do not create a public issue
2. Send email to security@example.com
3. Include detailed description of the vulnerability
4. Wait for response before disclosing publicly

## License

By contributing to SigmaOS, you agree that your contributions will be licensed under the MIT OR GPL-2.0 license.

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes for significant contributions
- Project documentation

## Getting Help

- **Documentation**: [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- **Issues**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)

## Release Process

See [VERSIONING_POLICY.md](VERSIONING_POLICY.md) for detailed release process.

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Semantic Versioning](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)

Thank you for contributing to SigmaOS!

# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Submitting Changes](#submitting-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Getting Started

### Prerequisites

- Rust toolchain (stable or nightly, depending on requirements)
- Git for version control
- Basic familiarity with Rust and operating system concepts
- Understanding of the project structure and goals

### First Steps

1. **Read the Documentation**: Start by reading the [About SigmaOS](About-SigmaOS) and [Architecture](ARCHITECTURE_DECISIONS) documents to understand the project's vision and technical approach.

2. **Set Up Development Environment**: Follow the [Build Instructions](BUILD) to set up your development environment.

3. **Explore the Codebase**: Spend time exploring the codebase structure, particularly the areas you're interested in contributing to.

4. **Join the Community**: Introduce yourself in our [community channels](Getting-Involved#communication-channels) and let us know what you're working on.

## Development Setup

### Forking and Cloning

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# Add the upstream repository
git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git
```

### Building the Project

```bash
# Build the project
cargo build --release

# Run tests
./run_sigma_tests.sh

# Check for compilation errors
cargo check
```

### Creating a Branch

```bash
# Create a new branch for your changes
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/your-bug-fix
```

## Coding Standards

### Rust Code Style

- Follow the official Rust style guidelines
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write clear, self-documenting code with meaningful variable and function names

### Documentation

- Document all public functions and types with doc comments
- Include usage examples in documentation
- Keep documentation up to date with code changes
- Use proper markdown formatting in comments

### Security Considerations

- Follow the security guidelines in [AGENTS.md](AGENTS.md)
- Ensure memory safety (no unsafe code without proper justification)
- Validate all user inputs
- Follow principle of least privilege
- Use post-quantum cryptography where applicable

### Architecture Principles

- Maintain compatibility across Linux and BSD distribution modes
- Support multi-architecture platforms (x86_64, AArch64, etc.)
- Keep components modular and loosely coupled
- Follow SOLID principles and clean code practices
- Prioritize stability, security, and performance

## Submitting Changes

### Commit Messages

Write clear, descriptive commit messages:

```
Add feature: Brief description

Detailed description of what the change does and why.
Include any relevant context or references to issues.

- Bullet points for specific changes
- Another bullet point

Closes #123
```

### Pull Request Process

1. **Update Your Branch**: Ensure your branch is up to date with the main branch

```bash
git fetch upstream
git rebase upstream/main
```

2. **Push Your Changes**: Push your branch to your fork

```bash
git push origin feature/your-feature-name
```

3. **Create Pull Request**: Create a pull request on GitHub with:
   - Clear title and description
   - Reference to related issues
   - Description of testing performed
   - Screenshots for UI changes (if applicable)

4. **Code Review**: Respond to review comments and make requested changes

5. **CI Checks**: Ensure all CI checks pass before requesting review

### Pull Request Template

Use the following template for your pull requests:

```markdown
## Description
Brief description of what this PR does

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested this change

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review of code completed
- [ ] Commented on complex code sections
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests added/updated
- [ ] All tests passing
```

## Testing

### Running Tests

```bash
# Run all tests
./run_sigma_tests.sh

# Run specific test suite
cargo test <test_name>

# Run with specific features
cargo test --features <feature_name>
```

### Writing Tests

- Write unit tests for all new functions
- Add integration tests for new features
- Test edge cases and error conditions
- Ensure tests are deterministic and repeatable

### Code Coverage

- Aim for high code coverage on new code
- Use coverage tools to identify untested code
- Prioritize testing critical security and stability code

## Documentation

### Wiki Contributions

- Keep documentation clear and concise
- Use consistent formatting and structure
- Include code examples where helpful
- Update related documentation when making changes
- Follow the [Wiki Contributing](Wiki-Contributing) guidelines

### Code Documentation

- Document all public APIs
- Include examples in doc comments
- Explain non-obvious implementation details
- Keep documentation in sync with code changes

## Release Process

### Version Bumping

Follow semantic versioning:
- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Changelog

Update the changelog with:
- Added features
- Changed behavior
- Deprecated features
- Removed features
- Fixed bugs
- Security updates

### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Release notes prepared
- [ ] Security review completed
- [ ] Performance testing completed

## Getting Help

If you need help with contributing:

- Check existing [documentation](Table-of-contents)
- Search [existing issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- Ask questions in [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- Contact maintainers via email for private matters

## Recognition

Contributors are recognized in:
- Contributor list in the repository
- Release notes for significant contributions
- Potential invitation to become a maintainer

Thank you for contributing to SigmaOS!

---

**[Getting Involved](Getting-Involved)** | **[Code of Conduct](Code-of-Conduct)** | **[Help:Editing](Help-Editing)**

# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines and information for contributing to the project.

## Code of Conduct

### Our Pledge

We are committed to making participation in SigmaOS a harassment-free experience for everyone, regardless of level of experience, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

### Our Standards

Examples of behavior that contributes to a positive environment:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Latest stable version
- **Git**: For version control
- **QEMU**: For testing (optional but recommended)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust toolchain
rustup install stable
rustup default stable
```

### Building

```bash
# Build the kernel
cargo build --release

# Build userland
cd userland
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run integration tests
./run_sigma_tests.sh

# Run regression tests
./scripts/regression_check.sh
```

## Contribution Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# Add upstream remote
git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git
```

### 2. Create a Branch

```bash
# Update from upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Make your changes
- Write tests for your changes
- Ensure all tests pass
- Format your code

### 4. Commit Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "Add feature: description of your changes"
```

### 5. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create pull request on GitHub
```

## Coding Standards

### Rust Code Style

**Formatting:**
```bash
# Format code
cargo fmt
```

**Linting:**
```bash
# Run clippy
cargo clippy
```

### Documentation

Add documentation comments for public items:
```rust
/// Brief description of what this does.
///
/// # Arguments
///
/// * `arg1` - Description of argument
///
/// # Returns
///
/// Description of return value
pub fn function(arg1: Type) -> ReturnType {
    // Implementation
}
```

## Pull Request Guidelines

### Pull Request Description

Your pull request should include:

- **Title**: Clear and descriptive title
- **Description**: Detailed description of changes
- **Motivation**: Why this change is needed
- **Testing**: How you tested the changes
- **Documentation**: Any documentation updates

### Pull Request Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted with `cargo fmt`

## Areas of Contribution

### Kernel Development
- Memory management optimization
- Scheduler improvements
- File system enhancements
- Network stack improvements
- Security features

### Userland Development
- System services
- Package manager
- Shell and utilities
- Desktop environment
- Applications

### Documentation
- API documentation
- User guides
- Developer guides
- Architecture documentation

### Testing
- Unit tests
- Integration tests
- Fuzzing
- Performance tests
- Security tests

## Recognition

Contributors are recognized in:
- Contributors file
- Release notes
- GitHub contributors list
- Project website

## Getting Help

If you need help:
- **Documentation**: Check existing documentation
- **Issues**: Search existing issues
- **Discussions**: Ask in GitHub Discussions
- **Email**: support@sigmaos.org

## License

By contributing to SigmaOS, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to SigmaOS!

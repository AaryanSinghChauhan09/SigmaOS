# SigmaOS Contributor Onboarding Guide

**Last Updated:** July 6, 2026  
**Version:** v16.3.0 Foundation

---

## Welcome to SigmaOS

Thank you for your interest in contributing to SigmaOS! This guide will help you get started with contributing to the project. SigmaOS is a native operating system built with Rust, focusing on reducing dependencies on predefined functions, libraries, and high-level programming languages.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Environment](#development-environment)
3. [Project Structure](#project-structure)
4. [Contribution Guidelines](#contribution-guidelines)
5. [Code Standards](#code-standards)
6. [Testing](#testing)
7. [Submitting Changes](#submitting-changes)
8. [Communication](#communication)
9. [Resources](#resources)

---

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust**: Latest stable version (1.80+)
- **Git**: Version 2.30+
- **QEMU**: For testing (optional but recommended)
- **Linux host**: For cross-compilation (optional)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/SigmaOS.git
   cd SigmaOS
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git
   ```

### Branch Strategy

SigmaOS uses a single branch strategy - only the `main` branch exists. All contributions should be submitted via pull requests to `main`.

---

## Development Environment

### Building SigmaOS

```bash
# Build the kernel
cargo build --release

# Build specific module
cargo build --release -p sigma_kernel

# Build all components
cargo build --release --workspace
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test -p sigma_kernel

# Run tests with output
cargo test -- --nocapture
```

### Development Tools

- **Rust Analyzer**: IDE support for VS Code, IntelliJ IDEA
- **Clippy**: Linting tool
- **Rustfmt**: Code formatting

Install development tools:
```bash
rustup component add rust-analyzer clippy rustfmt
```

---

## Project Structure

```
SigmaOS/
├── drivers/           # Hardware drivers
│   ├── gpu/          # GPU drivers (i915, amdgpu)
│   ├── net/          # Network drivers (wifi, ethernet)
│   └── usb/          # USB controller drivers
├── kernel/           # Core kernel implementation
├── lib/              # Native libraries
│   └── sigma_libc/   # Custom libc implementation
├── system/           # System services
│   ├── coredump/     # Core dump management
│   └── workflow/     # Workflow automation
├── education/        # Educational applications
│   ├── sigma_math.rs
│   └── sigma_classroom.rs
├── business/         # Business applications
│   └── sigma_erp.rs
├── gis/              # Geographic Information System
│   └── sigma_gis.rs
├── healthcare/       # Healthcare applications
│   └── sigma_health.rs
├── engineering/      # Engineering applications
│   └── sigma_cad.rs
├── wiki/             # Documentation
└── .github/          # GitHub configuration
    └── workflows/   # CI/CD pipelines
```

### Key Components

#### Drivers (`drivers/`)
- **GPU**: Intel i915, AMD amdgpu, NVIDIA support
- **Network**: Wi-Fi, Ethernet (r8169, igb, ixgbe)
- **USB**: EHCI, XHCI, UHCI, OHCI controllers

#### Kernel (`kernel/`)
- Custom kernel with latest Linux integration
- Native scheduler, memory management
- No dependency on external kernel implementations

#### Libraries (`lib/`)
- **sigma_libc**: Custom libc reducing dependency on musl
- Native implementations of common C library functions

#### System Services (`system/`)
- **sigma_coredump**: Core dump management
- **sigma_workflow**: Workflow automation replacing n8n

#### Application Suites
- **Education**: Mathematics engine, classroom management
- **Business**: ERP, accounting, library management
- **GIS**: Geographic information system
- **Healthcare**: Electronic health records
- **Engineering**: CAD system

---

## Contribution Guidelines

### What to Work On

Check the [Implementation Progress](Implementation-Progress.md) and GitHub Issues for areas needing work. Priority areas:

1. **Driver Support**: Additional hardware drivers
2. **Filesystem**: Enhanced filesystem layer
3. **Performance**: Optimization improvements
4. **Documentation**: Wiki updates and guides

### Types of Contributions

- **Bug Fixes**: Address reported issues
- **New Features**: Implement planned features from roadmap
- **Documentation**: Improve documentation
- **Testing**: Add test coverage
- **Refactoring**: Improve code quality

### Before Starting

1. Check existing Issues to avoid duplication
2. Comment on the Issue you plan to work on
3. Create a branch for your work (even though we only have main, use a local branch)
4. Follow the code standards below

---

## Code Standards

### Rust Conventions

SigmaOS follows standard Rust conventions with additional requirements:

#### no_std Compatibility

Most components use `#![no_std]` to reduce dependencies:

```rust
#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
// ... custom type aliases
```

#### C ABI Compatibility

Public functions use C-compatible FFI:

```rust
#[no_mangle]
pub unsafe extern "C" fn function_name() -> SigmaI32 {
    // Implementation
    0
}
```

#### Naming Conventions

- **Functions**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

#### Documentation

Add doc comments to public APIs:

```rust
/// Initialize the GPU driver
/// 
/// # Arguments
/// 
/// * `device_id` - PCI device ID
/// * `mmio_base` - Memory-mapped I/O base address
/// 
/// # Returns
/// 
/// * `SigmaI32` - 0 on success, negative on error
#[no_mangle]
pub unsafe extern "C" fn gpu_init(device_id: SigmaU32, mmio_base: SigmaU64) -> SigmaI32 {
    // Implementation
}
```

### Code Formatting

Use `rustfmt` before committing:

```bash
cargo fmt
```

### Linting

Run `clippy` to catch common issues:

```bash
cargo clippy -- -D warnings
```

---

## Testing

### Unit Tests

Write unit tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(function(), expected_value);
    }
}
```

### Integration Tests

Add integration tests in `tests/` directory for cross-component testing.

### Testing Drivers

Use QEMU for hardware driver testing:

```bash
qemu-system-x86_64 -kernel target/x86_64-sigmaos/release/sigma-kernel
```

### CI/CD

All changes must pass CI/CD pipeline. The pipeline runs:
- Build verification
- Unit tests
- Integration tests
- Linting (clippy)
- Formatting checks (rustfmt)

---

## Submitting Changes

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Maintenance tasks

Examples:
```
feat(gpu): Add NVIDIA driver support

Implement basic NVIDIA GPU driver with initialization
and framebuffer management.

Closes #123
```

### Pull Request Process

1. Update your branch with latest main:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push to your fork:
   ```bash
   git push origin feature-branch
   ```

3. Create Pull Request on GitHub

4. Fill PR template:
   - Description of changes
   - Related issues
   - Testing performed
   - Breaking changes (if any)

5. Wait for review and address feedback

### Review Process

- Maintainers will review your PR
- Address all review comments
- Update PR as needed
- Once approved, maintainers will merge to main

---

## Communication

### GitHub Issues

Use GitHub Issues for:
- Bug reports
- Feature requests
- Questions
- Discussions

Include:
- Clear title
- Detailed description
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Environment information

### Discussions

Use GitHub Discussions for:
- General questions
- Architecture discussions
- Feature brainstorming
- Community building

### Code of Conduct

SigmaOS maintains a respectful and inclusive community:
- Be respectful to all contributors
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Assume good intentions

---

## Resources

### Documentation

- [Implementation Progress](Implementation-Progress.md) - Track project progress
- [Migration Guides](Migration-Guides.md) - User migration documentation
- [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) - Additional documentation

### External Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust guide
- [Linux Kernel Documentation](https://www.kernel.org/doc/html/latest/)

### Project Goals

SigmaOS aims to:
- Reduce dependency on predefined functions and libraries
- Implement native, low-level replacements
- Improve performance, speed, and capabilities
- Provide comprehensive hardware support
- Replace industry-standard tools with native implementations

---

## Getting Help

If you need help:

1. Check existing documentation
2. Search GitHub Issues and Discussions
3. Create a new Issue with your question
4. Join community discussions

---

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project acknowledgments

All contributions are valued, regardless of size!

---

**Thank you for contributing to SigmaOS! Together we're building a more independent and capable operating system.**

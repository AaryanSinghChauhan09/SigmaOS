# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git
- QEMU (for testing)
- Make

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the project
cargo build

# Run tests
cargo test

# Run the project
cargo run
```

## Development Workflow

### Branching Strategy

- `main` - The main development branch
- All changes should be made through pull requests
- Feature branches should be named `feature/description`
- Bugfix branches should be named `fix/description`

### Commit Guidelines

- Use clear, descriptive commit messages
- Follow conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write tests for new functionality
- Document public APIs with rustdoc

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

- Aim for high test coverage
- Write unit tests for individual functions
- Write integration tests for component interactions
- Use property-based testing where appropriate

## Documentation

### Code Documentation

- Document all public functions and structs
- Use `///` for item documentation
- Use `//!` for module documentation
- Include examples where helpful

### Wiki Documentation

- Update the wiki for major features
- Add tutorials and guides
- Keep architecture diagrams up to date
- Document API changes

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Update documentation
6. Submit a pull request
7. Address review feedback
8. Get approval and merge

## Project Structure

```
SigmaOS/
├── src/              # Source code
├── tests/            # Integration tests
├── docs/             # Documentation
├── scripts/          # Utility scripts
├── .github/          # GitHub configuration
├── Cargo.toml        # Rust dependencies
└── README.md         # Project overview
```

## Module Guidelines

### Security Module

- Capability-based security model
- No unsafe code without justification
- Audit all security-sensitive operations

### Kernel Module

- No_std compatible where possible
- Minimal dependencies
- Clear error handling

### Package Manager

- Zero-dependency where possible
- Cryptographic verification
- Atomic transactions

## Issue Reporting

- Use GitHub Issues for bug reports
- Provide reproduction steps
- Include environment details
- Tag relevant maintainers

## Feature Requests

- Use GitHub Issues for feature requests
- Describe the use case
- Propose a solution
- Consider implementation complexity

## License

By contributing to SigmaOS, you agree that your contributions will be licensed under the same license as the project.

## Questions?

- Open an issue for questions
- Contact maintainers via GitHub
- Check existing documentation

Thank you for contributing to SigmaOS!

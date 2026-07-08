# SigmaOS Development Guide

This document provides guidelines for contributing to SigmaOS development.

## Getting Started

### Development Environment Setup

1. **Install Required Tools**
   ```bash
   # Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add x86_64-unknown-none

   # Nim
   curl https://nim-lang.org/choosenim/init.sh -sSf | sh

   # Other tools
   sudo apt-get install nasm qemu grub-pc-bin
   ```

2. **Clone Repository**
   ```bash
   git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
   cd SigmaOS
   ```

3. **Build Project**
   ```bash
   cargo build --release
   ```

## Code Style

### Rust Code Style

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Use `#![no_std]` for kernel code
- Avoid `unwrap()` in production code
- Use `Result` types for error handling

### Example Rust Code

```rust
#![no_std]

use core::result::Result;

pub fn example_function(input: u32) -> Result<u32, Error> {
    if input == 0 {
        return Err(Error::InvalidInput);
    }
    Ok(input * 2)
}
```

### Nim Code Style

- Follow Nim style guide
- Use snake_case for variables
- Use PascalCase for types
- Add doc comments

### Example Nim Code

```nim
## Example function
proc exampleFunction(input: int): Result[int] =
  if input == 0:
    return err(InvalidInput)
  ok(input * 2)
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example_function(5), Ok(10));
    }
}
```

### Integration Tests

Create tests in `tests/` directory:

```rust
// tests/integration_test.rs
use sigma_kernel::example_function;

#[test]
fn test_integration() {
    assert!(example_function(10).is_ok());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_example

# Run with output
cargo test -- --nocapture
```

## Documentation

### Inline Documentation

```rust
/// Example function that doubles the input
///
/// # Arguments
/// * `input` - The value to double
///
/// # Returns
/// * `Ok(u32)` - The doubled value
/// * `Err(Error)` - If input is invalid
///
/// # Examples
/// ```
/// let result = example_function(5);
/// assert_eq!(result, Ok(10));
/// ```
pub fn example_function(input: u32) -> Result<u32, Error> {
    // implementation
}
```

### Building Documentation

```bash
# Build documentation
cargo doc

# Open documentation in browser
cargo doc --open

# Build documentation for all packages
cargo doc --document-private-items
```

## Debugging

### QEMU Debugging

```bash
# Start QEMU with GDB server
qemu-system-x86_64 -cdrom sigmaos.iso -m 2G -s -S

# Connect GDB
gdb target/kernel
(gdb) target remote :1234
(gdb) break main
(gdb) continue
```

### Logging

Use the logging framework:

```rust
use log::{info, warn, error};

info!("Starting system");
warn!("Warning message");
error!("Error occurred");
```

### Kernel Debugging

Add debug prints:

```rust
// kernel/debug.rs
pub fn kprint(s: &str) {
    unsafe {
        // Write to serial port or framebuffer
    }
}
```

## Performance Profiling

### Benchmarking

```rust
#[bench]
fn bench_example(b: &mut Bencher) {
    b.iter(|| {
        example_function(42)
    });
}
```

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph
```

## Code Review Process

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make Changes**
   - Write code
   - Add tests
   - Update documentation

3. **Run Checks**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "Add my feature"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/my-feature
   ```

## Release Process

1. **Update Version**
   - Update `Cargo.toml` version
   - Update `CHANGELOG.md`

2. **Tag Release**
   ```bash
   git tag -a v1.0.0 -m "Release 1.0.0"
   git push origin v1.0.0
   ```

3. **Build Release**
   ```bash
   cargo build --release
   ```

4. **Create ISO**
   ```bash
   ./scripts/build-iso.sh
   ```

## Contributing Guidelines

### Before Contributing

1. Read the [Architecture Documentation](./ARCHITECTURE.md)
2. Read the [Security Documentation](./SECURITY.md)
3. Check existing issues and PRs

### Making Changes

1. Keep changes focused and minimal
2. Add tests for new functionality
3. Update documentation
4. Follow code style guidelines

### Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and documentation
5. Submit a pull request

## Project Structure

```
SigmaOS/
├── bootloader/          # UEFI bootloader
├── kernel/              # Kernel code
│   ├── core/           # Core kernel components
│   ├── mm/             # Memory management
│   ├── scheduler/      # Process scheduling
│   ├── security/       # Security subsystem
│   └── gfx/            # Graphics subsystem
├── userland/            # Userland applications
│   ├── coreutils/      # Core utilities
│   ├── system_api/     # System APIs
│   └── agent/          # AI agent
├── applications/       # Office applications
├── tools/              # Development tools
├── docs/               # Documentation
└── scripts/            # Build scripts
```

## Common Tasks

### Adding a New System Call

1. Define syscall number in `kernel/core/syscall.rs`
2. Implement syscall handler
3. Add to dispatch table
4. Update documentation

### Adding a New Driver

1. Create driver file in `kernel/drivers/`
2. Implement driver interface
3. Register driver in init
4. Add tests

### Adding a New Userland Application

1. Create application directory
2. Add `Cargo.toml`
3. Implement application
4. Add to workspace

## Troubleshooting

### Build Errors

- Check Rust version: `rustc --version`
- Update dependencies: `cargo update`
- Clean build: `cargo clean`

### Test Failures

- Run tests individually: `cargo test test_name`
- Check test output: `cargo test -- --nocapture`
- Enable backtrace: `RUST_BACKTRACE=1 cargo test`

### Runtime Errors

- Check QEMU output
- Enable serial logging
- Use GDB for debugging

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Nim Documentation](https://nim-lang.org/docs.html)
- [UEFI Specification](https://uefi.org/specifications)
- [x86_64 Architecture](https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html)

## Contact

- GitHub Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- GitHub Discussions: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

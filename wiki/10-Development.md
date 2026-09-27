# Development

SigmaOS provides development tools for building and contributing to the operating system.

## Development Tools

### Build System

SigmaOS uses Rust, Zig, and Nim for development:

```bash
# Build kernel
cargo build --release

# Build userland
cargo build --release --bin userland

# Build all
cargo build --release
```

### Zig Development

Build Zig components:

```bash
# Build Zig module
zig build-exe src/zig/module.zig

# Run Zig tests
zig test src/zig/
```

### Nim Development

Build Nim components:

```bash
# Build Nim module
nim c src/nim/module.nim

# Run Nim tests
nim test src/nim/
```

## Kernel Development

### Module Development

Create kernel modules:

```bash
# Create module skeleton
sigkmod create module-name

# Build module
sigkmod build module-name

# Load module
sigmod load module-name.ko
```

### Kernel Debugging

Debug kernel issues:

```bash
# Enable kernel debugging
echo 1 > /proc/sys/kernel/sysrq

# Show kernel messages
dmesg | tail

# Trigger kernel panic (for testing)
echo c > /proc/sysrq-trigger
```

## Testing

### Unit Tests

Run unit tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests

Run integration tests:

```bash
# Run integration tests
./run_sigma_tests.sh

# Run pytest tests
pytest tests/
```

## Cross-Compilation

### Cross-Compile for Other Architectures

Cross-compile for ARM64:

```bash
# Set target
export TARGET=aarch64-unknown-linux-gnu

# Build for target
cargo build --target=$TARGET
```

## Documentation

### Generate Documentation

Generate API documentation:

```bash
# Generate Rust documentation
cargo doc --open

# Generate documentation for specific crate
cargo doc -p crate-name --open
```

## Development Environment

### IDE Configuration

Configure development environment:

- **VS Code**: Install Rust, Zig, Nim extensions
- **IntelliJ IDEA**: Install Rust plugin
- **Vim/Neovim**: Configure with rust-analyzer, zls, nimlsp

### Code Style

Format code:

```bash
# Format Rust code
cargo fmt

# Format Zig code
zig fmt src/zig/

# Format Nim code
nimpretty src/nim/
```

## Contributing

### Git Workflow

Clone repository:

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

Create feature branch:

```bash
git checkout -b feature/your-feature
```

Commit changes:

```bash
git add .
git commit -m "feat: add your feature"
```

Push changes:

```bash
git push origin feature/your-feature
```

### Pull Request

Create pull request:

1. Fork repository on GitHub
2. Create feature branch
3. Make changes
4. Push to fork
5. Create pull request

## Next Steps

- [Kernel](04-Kernel.md) - Kernel development
- [Filesystems](05-Filesystems.md) - Filesystem development
- [Security](07-Security.md) - Security development

# Cross-Compilation Support

SigmaOS supports cross-compilation for multiple architectures.

## Supported Targets

- `x86_64-unknown-linux-gnu` (native)
- `aarch64-unknown-linux-gnu` (ARM 64-bit)
- `riscv64gc-unknown-linux-gnu` (RISC-V)

## Building for Different Architectures

```bash
# Build for aarch64
cargo check --lib --target aarch64-unknown-linux-gnu

# Build for riscv64
cargo check --lib --target riscv64gc-unknown-linux-gnu
```

## CI Testing

GitHub Actions automatically builds and tests all targets on every push to main.

## Adding New Targets

1. Add target to `.cargo/config.toml`
2. Add target to GitHub Actions workflow matrix
3. Update documentation

## Principles Followed

- **CROSS_COMPILATION**: Multi-architecture build support
- **PORTABILITY**: Support for different hardware architectures
- **POWER_OF_TEN**: "Premature optimization is the root of all evil" - verify on multiple targets
- **STABLE_APIS**: Consistent API across architectures
- **DEUTSCH_LIMIT**: One codebase, multiple targets

# Contributing to SigmaOS

Thank you for considering contributing to SigmaOS!

## Quick Start

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo build
cargo test
```

## Ways to Contribute

- 🐛 **Bug Reports**: Open a GitHub Issue
- ✨ **Feature Requests**: Open a GitHub Issue with `[RFC]` prefix  
- 📝 **Documentation**: Fix typos, add examples, improve clarity
- 🔧 **Code**: See open issues labeled `good first issue`
- 🔒 **Security**: See [SECURITY.md](SECURITY.md)

## Cross-Compilation & Target Architecture Setups (ACT-08)

SigmaOS is zero-dependency and `#![no_std]` compliant at its core. It supports cross-compilation across multiple hardware target triples:

### Target Architectures
- **x86_64 Bare-Metal**: `x86_64-unknown-none`
- **ARM64 / AArch64**: `aarch64-unknown-none`
- **RISC-V 64-Bit**: `riscv64gc-unknown-none-elf`

### Setting up Cross-Compilation Toolchains
```bash
# Add rustup target triples
rustup target add x86_64-unknown-none aarch64-unknown-none riscv64gc-unknown-none-elf

# Build for bare-metal ARM64 target
cargo build --target aarch64-unknown-none

# Build for bare-metal RISC-V target
cargo build --target riscv64gc-unknown-none-elf
```

## Development Process

1. Fork the repository
2. Create a feature branch: `feat/your-feature`
3. Make changes with clear commits
4. Run `cargo test` and `cargo clippy`
5. Submit a Pull Request to `main`

## Code Style

- Follow Rust idioms and use `rustfmt`
- Document all public APIs with `///` doc comments
- Write tests for new functionality
- Prefer `safe` Rust; document all `unsafe` blocks

## Commit Format

```
type(scope): description

Body explaining WHY

Fixes #issue
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `security`

See [Contributing Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing) for full guide.

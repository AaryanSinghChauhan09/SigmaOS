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

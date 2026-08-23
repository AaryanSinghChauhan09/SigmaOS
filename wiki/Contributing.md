# 🤝 Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS!

## Getting Started

### 1. Fork and Clone
```bash
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS
git remote add upstream https://github.com/AaryanSinghChauhan09/SigmaOS.git
```

### 2. Setup Development Environment
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Install build dependencies
sudo apt install gcc make nasm qemu-system-x86
```

### 3. Build the Project
```bash
cargo build
cargo test
```

## Branch Naming Convention

| Type | Format | Example |
|------|--------|---------|
| Feature | `feat/<description>` | `feat/zfs-support` |
| Bug fix | `fix/<description>` | `fix/memory-leak` |
| Documentation | `docs/<description>` | `docs/update-scheduler` |
| Refactor | `refactor/<description>` | `refactor/ipc-bus` |
| Security | `security/<description>` | `security/patch-cve-2026` |

## Development Workflow

1. **Create a branch** from latest `main`
2. **Make your changes** with small, focused commits
3. **Run tests**: `cargo test`
4. **Lint code**: `cargo clippy` and `cargo fmt`
5. **Submit a PR** targeting `main`
6. **Address review feedback**

## Code Style

### Rust Style
```rust
// Use descriptive names
pub struct EevdfScheduler {
    processes: Vec<Process>,
    system_vtime: u64,
}

// Document public APIs
/// Selects the next process to run using EEVDF algorithm.
/// Returns None if no eligible process exists.
pub fn schedule(&mut self) -> Option<&Process> {
    // implementation
}
```

### Testing
- Write unit tests for all new functions
- Place tests in `#[cfg(test)] mod tests {}` at bottom of file
- Integration tests go in `tests/` directory
- Aim for >80% code coverage

## Commit Message Format

```
type(scope): brief description

Detailed explanation of WHY the change was made,
not just WHAT was changed.

Fixes #123
```

**Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `security`, `perf`

## Areas Needing Contribution

### High Priority 🔴
- [ ] ZFS filesystem driver
- [ ] AUR build system completion
- [ ] AppArmor profile engine
- [ ] sigma-sdk documentation

### Medium Priority 🟡
- [ ] RISC-V architecture port
- [ ] Nix expression evaluator
- [ ] Predictive prefetcher
- [ ] sigma-profile profiler

### Documentation 🔵
- [ ] API reference pages
- [ ] Architecture deep-dives
- [ ] Tutorial articles
- [ ] Video walkthroughs

## Code of Conduct

- Be respectful and inclusive
- Focus on technical merit in reviews
- Help newcomers get started
- No harassment or discrimination

## Getting Help

- **GitHub Discussions**: Architecture questions
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Wiki**: Documentation and guides

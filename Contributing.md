# 🤝 Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This guide covers how to contribute code, documentation, bug reports, and feature requests.

---

## Code of Conduct

By participating, you agree to follow our [Code of Conduct](Code-of-Conduct). Be respectful, constructive, and collaborative.

---

## Ways to Contribute

### 🐛 Report Bugs

1. Check [existing issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
2. If not found, open a new issue with:
   - Environment details (OS, Rust version, CPU arch)
   - Steps to reproduce
   - Expected vs actual behavior
   - Error output / backtrace

### 💡 Suggest Features

1. Open a [Feature Request](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new?template=feature_request.md)
2. Describe the use case and motivation
3. Reference similar implementations in Linux distros if applicable

### 📝 Improve Documentation

- Fix typos or clarify existing wiki pages
- Add missing examples or diagrams
- Translate documentation

### 🔧 Submit Code

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feat/my-feature`
3. **Make changes** with tests
4. **Run tests**: `cargo test --workspace`
5. **Open a Pull Request** to `main`

---

## Development Setup

```bash
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview
cargo build
cargo test --workspace
```

---

## Pull Request Guidelines

| Rule | Detail |
|------|--------|
| Target branch | Always `main` |
| Tests required | `cargo test --workspace` must pass |
| Commit style | Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`) |
| PR description | Describe what changed and why |
| One feature per PR | Keep PRs focused and reviewable |

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]
[optional footer]
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `perf`, `ci`

Example:
```
feat(security): add pledge syscall restriction for userspace processes

Implements OpenBSD-style pledge() and unveil() for SigmaOS processes.
Adds runtime syscall whitelisting and path-access restriction.

Closes #123
```

---

## Component Priority

Pick up any 🔄 in-progress component from the [Components Master Table](Components-Master-Table). High-priority areas:

1. **SigmaNet** — TCP/IP stack completion
2. **SigmaGPU** — AMD/Intel GPU driver
3. **SigmaHAL** — AArch64 and RISC-V HAL
4. **SigmaAI** — AI orchestration subsystem
5. **SigmaWASM** — WebAssembly runtime

---

*Thank you for contributing to SigmaOS!*  
*[Back to Home](Home)*

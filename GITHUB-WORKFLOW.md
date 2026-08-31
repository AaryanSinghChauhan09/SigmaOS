# SigmaOS GitHub Workflow Guide

> Complete guide for contributing to, reviewing, and maintaining the SigmaOS GitHub repository.

---

## 🚀 Getting Started

### Prerequisites
```bash
# Install Rust (stable + nightly)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview

# Install dependencies
apt install -y qemu-system nasm grub-pc-bin xorriso
```

### Clone & Build
```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
cargo build
cargo test
```

---

## 🌳 Branch Strategy

> **SigmaOS uses a single-branch strategy. Only `main` exists.**

### Why Single Branch?
- Simpler history
- No merge conflicts between long-lived branches
- Continuous integration and delivery
- All features merged via PR + squash

### For Contributors
```bash
# WRONG: Creating persistent branches
git checkout -b feature/my-feature  # Don't leave this around!

# RIGHT: Create branch, work, PR, DELETE after merge
git checkout -b feat/my-feature-YYYYMMDD
# ... make changes ...
gh pr create --base main --title "feat: My Feature"
# After PR merge: branch is deleted automatically
```

---

## 📝 Pull Request Template

### Title Format
```
<type>(<scope>): <short description>

Types: feat | fix | docs | refactor | perf | test | chore | security
Scopes: kernel | security | network | desktop | fs | pkg | ai | virt | driver | compat
```

### PR Body Template
```markdown
## Summary
<!-- What does this PR do? -->

## Changes
<!-- List of files changed and why -->

## Testing
<!-- How was this tested? -->
- [ ] cargo test passes
- [ ] New tests added for new functionality
- [ ] Documentation updated

## Inspired By
<!-- Which distro/project inspired this? -->
```

---

## ✅ Code Review Checklist

- [ ] **Safety**: No unsafe Rust without justification
- [ ] **Security**: No hardcoded secrets or keys
- [ ] **Tests**: Tests pass with `cargo test`
- [ ] **Docs**: Public APIs documented
- [ ] **Format**: `cargo fmt` applied
- [ ] **Lint**: `cargo clippy` warnings addressed
- [ ] **Performance**: No unnecessary allocations in hot paths

---

## 🔖 Commit Message Standards

```
feat(kernel): add RISC-V support to HAL abstraction layer

- Implement HartID-based CPU initialization
- Add RISC-V fence.i instruction wrapper
- Extend architecture.rs with rv64gc profile

Closes #123
Inspired by: RISC-V Linux port, OpenSBI
```

---

## 🧪 CI/CD Pipelines

| Workflow | Trigger | Purpose |
|----------|---------|-------|
| `rust.yml` | Every push/PR | Build + test |
| `arch-gentoo-alpine-ci.yml` | Every push | Cross-distro compat |
| `bsd-linux-parity-ci.yml` | Every push | BSD parity tests |
| `security-audit.yml` | Weekly | `cargo audit` |
| `clippy.yml` | Every PR | Lint checking |

---

## 📖 Wiki Maintenance

### Updating Wiki
```bash
# Wiki is updated via the GitHub API
# All major .md files are mirrored to wiki
cargo run --bin wiki-sync  # (planned)
```

### Wiki Structure
- **Home**: Project overview
- **Architecture**: System architecture
- **Components Table**: Component registry
- **API Reference**: Public APIs
- **Contributing**: This guide
- **Security**: Security model

---

*SigmaOS GitHub Workflow Guide | Updated: 2026-08-23*
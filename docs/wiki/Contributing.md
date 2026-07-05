# Contributing to SigmaOS

Thank you for contributing to SigmaOS — a sovereign, self-sufficient operating system. Every line of code helps reduce dependency on external tools and brings us closer to a fully independent computing environment.

---

## 🚀 Quick Start

```bash

# 1. Fork and clone

git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# 2. Install toolchain

rustup toolchain install nightly
rustup target add x86_64-unknown-none
cargo install just

# 3. Build everything

just build

# 4. Run tests

just test

# 5. Run in QEMU

just qemu
```

---

## 📋 PR Workflow (main-only)

SigmaOS follows a **trunk-based development** model. All code merges directly to `main` via PR.

```
fork → feature branch → PR → review → CI green → squash merge to main
```

### Rules

- ✅ **PRs only** — no direct pushes to `main`

- ✅ **Squash merge** — one commit per PR on main

- ✅ **CI must be green** — no exceptions

- ✅ **At least 1 reviewer** for non-trivial changes

- ✅ **CODEOWNERS review** for subsystem changes (see `.github/CODEOWNERS`)

- ❌ No WIP PRs — use Draft PR instead

### Branch Naming

| Type | Pattern | Example |
|---|---|---|
| Feature | `feat/description` | `feat/sigma-sh-scripting` |
| Bug fix | `fix/description` | `fix/sigpkg-semver-compare` |
| Docs | `docs/description` | `docs/absorption-matrix` |
| Security | `security/description` | `security/hardened-allocator` |
| Refactor | `refactor/description` | `refactor/kernel-memory-api` |

---

## 🛠️ Development Setup

### Required Tools

| Tool | Version | Purpose |
|---|---|---|
| Rust | nightly (see `rust-toolchain.toml`) | Kernel + userland |
| Zig | 0.13+ | Userland build + some tools |
| GNAT/SPARK | Community 2024 | Ada/SPARK security modules |
| just | latest | Task runner |
| QEMU | 8.x+ | Testing |
| git | 2.40+ | Version control |

### Dev Container

The easiest setup — all tools pre-installed:
```bash

# Open in VSCode with Dev Containers extension

code .

# → Click "Reopen in Container"

```

---

## 🧪 Testing Requirements

Before submitting a PR:

```bash

# Run all tests

just test

# Lint (must pass cleanly)

cargo clippy --all -- -D warnings
cargo fmt --all --check

# QEMU smoke test

just qemu-test

# SPARK proofs (if modifying security/)

gnatprove -P security/security.gpr
```

All checks run automatically in CI (`ci.yml`). PRs cannot merge with failing checks.

---

## 🗂️ What to Work On

Check the issue tracker for tagged issues:

| Label | Meaning |
|---|---|
| `good first issue` | Great for newcomers |
| `help wanted` | Any contributor welcome |
| `phase:0.2` | Current milestone priority |
| `component:kernel` | Kernel subsystem work |
| `component:sigma-sh` | Shell work |
| `component:sigpkg` | Package manager work |
| `component:security` | Security-critical (SPARK required) |
| `absorption` | Implementing a sovereign tool replacement |

### High-Priority Now (v0.2)

- 🟡 QEMU CI smoke test (green CI on main)

- 🟡 sigma-sh: scripting improvements

- 🟡 sigpkg: real registry fetch implementation

- 🟡 sigma-core-utils: `ls`, `cat`, `cp`, `mv` in Rust

---

## 📝 Commit Message Format

```
type(scope): short description (50 chars max)

Optional longer body explaining the why, not the what.
Reference issues: Fixes #123, Closes #456
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `security`, `perf`

---

## 🔒 Security Contributions

For security-critical components (`security/`, `kernel/security/`, `sigma-crypto`):

- Ada/SPARK is **required** — no plain C/Rust without approval

- `gnatprove` must pass with 0 violations

- Two-maintainer review required

- Report vulnerabilities privately via [SECURITY.md](../SECURITY.md)

---

## 📚 Resources

- [Architecture Overview](Architecture.md)

- [Coding Standards](Coding-Standards.md)

- [Security Model](Security-Model.md)

- [Absorption Matrix](Absorption-Matrix.md) — pick a tool to absorb!

- [Roadmap](Roadmap.md)

- [Developer Guide](../DEVELOPER_GUIDE.md)

---

### SigmaOS is a community project. Be kind, be sovereign. 🛡️

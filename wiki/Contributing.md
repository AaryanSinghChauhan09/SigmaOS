# Contributing to SigmaOS

Welcome to SigmaOS! We're building a sovereign, AI-native operating system and we need help across every layer — from kernel drivers to desktop apps to documentation and governance.

This document covers everything you need to contribute effectively.

---

## Table of Contents

1. [Before You Start](#before-you-start)
2. [Development Environment Setup](#development-environment-setup)
3. [Repository Structure](#repository-structure)
4. [Branch Model](#branch-model)
5. [Commit Format](#commit-format)
6. [PR Process](#pr-process)
7. [Code Style](#code-style)
8. [Review Process](#review-process)
9. [Testing Requirements](#testing-requirements)
10. [Maintainer Model](#maintainer-model)
11. [Governance](#governance)
12. [Code of Conduct](#code-of-conduct)

---

## Before You Start

1. **Read the principles.** [Principles.md](Principles.md) governs all design decisions. If your change violates a principle, explain why in the PR.
2. **Read the architecture.** [Architecture.md](Architecture.md) explains which layer your component belongs to and what its constraints are.
3. **Check the component map.** [Component-Map.md](Component-Map.md) tells you which repo owns which functionality. Don't duplicate existing components.
4. **Talk first for big changes.** For significant features or architecture changes, open a GitHub Discussion or RFC issue before writing code. Small bug fixes can go directly to a PR.
5. **Check branch policy.** [Branch-Policy.md](Branch-Policy.md) — `main` contains only code, no `.md` or `.txt` files.

---

## Development Environment Setup

### Requirements

- Linux (SigmaOS development is Linux-native)
- Rust toolchain (stable + nightly for kernel modules)
- `git` 2.35+
- Docker or Podman (for reproducible builds)
- QEMU (for testing bootable images)

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add clippy rustfmt
rustup toolchain install nightly
```

### Step 2: Install system dependencies

```bash
# Debian/Ubuntu
sudo apt install build-essential git pkg-config libssl-dev \
  qemu-system-x86 qemu-utils docker.io python3 python3-pip \
  meson ninja-build libdbus-1-dev

# Arch Linux
sudo pacman -S base-devel git rust qemu docker python \
  meson ninja dbus
```

### Step 3: Clone the sigma-build meta-repo

```bash
git clone https://github.com/AaryanSinghChauhan09/sigma-build
cd sigma-build
./scripts/setup-dev-env.sh
```

This script:
- Clones all 149 component repos into `components/`
- Sets up a Cargo workspace linking them all
- Installs pre-commit hooks
- Configures git with the required settings

### Step 4: Clone a specific component

If you're only working on one component:

```bash
git clone https://github.com/AaryanSinghChauhan09/sigma-pkg
cd sigma-pkg
cargo build
cargo test
```

### Step 5: Set up the development container (optional but recommended)

```bash
docker pull ghcr.io/aaryansinghchauhan09/sigma-dev:latest
# or build locally:
docker build -f sigma-ci/dev.Dockerfile -t sigma-dev .
docker run -it -v $(pwd):/workspace sigma-dev
```

### Step 6: Test your environment

```bash
cargo check --all-targets   # Should produce no errors
cargo test                  # Should pass
cargo clippy -- -D warnings # Should produce no warnings
```

---

## Repository Structure

Each component repository follows this structure:

```
sigma-<component>/
├── src/               # Source code
│   ├── lib.rs         # Library root (if library crate)
│   ├── main.rs        # Binary root (if binary crate)
│   └── ...
├── tests/             # Integration tests
│   └── *.rs
├── benches/           # Benchmarks
│   └── *.rs
├── docs/              # Spec documents (NOT .md in main — see Branch Policy)
│   └── spec.md        # Only synced to docs branch
├── Cargo.toml         # Project metadata and dependencies
├── Makefile           # Build shortcuts
└── .github/
    └── workflows/
        └── ci.yml     # CI pipeline
```

**Important:** `.md` and `.txt` files belong on the `docs` branch, not `main`. See [Branch-Policy.md](Branch-Policy.md).

---

## Branch Model

```
main          — Production-ready code only. No .md or .txt files.
docs          — All documentation (.md, .txt, specs, changelogs)
feature/*     — New features (e.g., feature/add-delta-updates)
fix/*         — Bug fixes (e.g., fix/crash-on-empty-package-list)
refactor/*    — Refactoring (e.g., refactor/extract-transaction-engine)
security/*    — Security fixes (e.g., security/cve-2027-12345)
release/*     — Release preparation (e.g., release/v1.0-rc1)
```

### Rules

- Branch from `main` for all feature/fix/refactor branches
- Branch from `release/*` for hotfixes to a release
- Delete branches after merge (no stale branches)
- Never commit directly to `main` — always use a PR
- `docs` branch is synced automatically from `main`'s `docs/` directory via CI

See [Branch-Policy.md](Branch-Policy.md) for the complete policy.

---

## Commit Format

SigmaOS uses **Conventional Commits**:

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code refactoring (no behavior change) |
| `test` | Adding or fixing tests |
| `docs` | Documentation changes (on docs branch only) |
| `chore` | Build scripts, CI, dependencies |
| `perf` | Performance improvements |
| `security` | Security fixes |
| `revert` | Reverting a previous commit |

### Scope

The scope is the component name: `pkg`, `update`, `shell`, `kernel`, etc.

### Examples

```
feat(pkg): add support for delta package updates

Implements binary delta updates using the casync algorithm.
Delta updates reduce download size by ~85% for typical minor version bumps.

Closes #142
```

```
fix(shell): prevent compositor crash on monitor hot-unplug

When a monitor was unplugged while a window was maximized on it,
sigma-shell would dereference a null output pointer.

Fix: check output validity before accessing CRTC properties.

Fixes #301
```

```
security(update): validate package signature before extraction

Previously, packages were extracted before signature verification.
This allowed a TOCTOU attack. Now extraction only happens after
successful signature verification.

CVE: CVE-2027-XXXX
```

### Rules

- Subject line: imperative mood, no period, ≤ 72 characters
- Body: explain the *why*, not the *what* (the diff shows the what)
- Reference issues: `Closes #N`, `Fixes #N`, `Relates to #N`
- Commits must be signed (`git commit -S`) — GPG signing is required for maintainers

---

## PR Process

### 1. Create a branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Write code and tests

- Write code that passes all tests
- Write new tests for new behavior
- Ensure `cargo clippy -- -D warnings` passes
- Ensure `cargo fmt` is applied

### 3. Self-review

Before opening a PR, ask yourself:
- Does this respect the principles in [Principles.md](Principles.md)?
- Does this stay within the correct architecture layer?
- Is there a test for every new code path?
- Would a new contributor understand this code in 5 minutes?
- Is the commit history clean and logical?

### 4. Open the PR

Use the PR template. Fill in:
- **Summary**: What does this change do?
- **Motivation**: Why is this change needed?
- **Testing**: How was this tested?
- **Breaking changes**: Any API or behavior changes?
- **Principles**: Which principles does this apply or exemplify?

### 5. CI must pass

All PRs must pass:
- `cargo build --all-targets`
- `cargo test --all`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- `cargo audit` (no known vulnerabilities)
- Spell check on any docs changes

### 6. Review

- Minimum 1 review required (maintainer or Contributor+)
- All review comments must be addressed or replied to
- Approved PRs are merged by a maintainer

### 7. Merge strategy

- **Squash merge** for single-commit features (keeps history clean)
- **Rebase merge** for multi-commit features with a clean logical history
- **No merge commits** on main

---

## Code Style

### Rust

- Follow `rustfmt` formatting — run `cargo fmt` before committing
- Use `clippy` and treat all warnings as errors
- Use `thiserror` for error types, `anyhow` only in binaries (not libraries)
- All `unsafe` blocks require a `// SAFETY:` comment
- All `pub` items require doc comments (`///`)
- `unwrap()` is banned — use `expect("descriptive message")` or `?`
- Use `tracing` for logging, not `println!`

```rust
// Good
pub fn resolve_deps(pkg: &PackageName) -> Result<Vec<Package>, ResolveError> {
    // SAFETY: We hold the lock for the entire duration of this function.
    let db = unsafe { DB_PTR.as_ref() }.expect("database must be initialized before use");
    ...
}

// Bad
pub fn resolve_deps(pkg: &PackageName) -> Vec<Package> {
    let db = unsafe { DB_PTR.as_ref() }.unwrap();
    ...
}
```

### C (kernel modules)

- Follow Linux kernel coding style (4-space indent, `linux/coding-style.rst`)
- All functions < 80 lines
- All error paths handled
- No `goto` except for cleanup labels

### Python (tooling)

- PEP 8, Black formatting
- Type annotations required for all function signatures
- `subprocess.run()` with `check=True`, never `os.system()`
- f-strings preferred over `%` formatting

### Shell

- `#!/usr/bin/env bash`
- `set -euo pipefail` at the top of all scripts
- All variables quoted: `"$VAR"` not `$VAR`
- `shellcheck` must pass

---

## Review Process

### Reviewer Expectations

- Review within 5 business days of being assigned
- Be specific: "This could be simplified by using `Iterator::fold`" not "simplify this"
- Be constructive: suggest alternatives, don't just criticize
- Use GitHub's suggestion feature for simple one-line changes
- Distinguish between blocking feedback (🔴 must fix) and suggestions (💡 consider)

### Author Expectations

- Respond to all comments within 5 business days
- Don't push back on principle violations — they are non-negotiable
- If you disagree with a suggestion, explain why in a comment
- Mark resolved comments as resolved

### Approval Requirements

| Change Type | Approvals Required |
|-------------|-------------------|
| Documentation fix | 1 (any contributor) |
| Bug fix (low risk) | 1 (maintainer or contributor) |
| Feature (medium complexity) | 1 (maintainer) |
| Architecture change | 2 (maintainers) + RFC |
| Security-sensitive | 2 (maintainers) + security review |
| Kernel/driver change | 2 (maintainers, one must be kernel maintainer) |

---

## Testing Requirements

### Unit Tests

- All new functions must have at least one unit test
- Follow Arrange-Act-Assert pattern
- No flaky tests — use deterministic seeds and mock clocks
- Tests must run in < 100ms each (use mocks for slow dependencies)

### Integration Tests

- All new D-Bus interfaces must have integration tests
- Integration tests run in CI using a minimal VM

### Regression Tests

- Every bug fix must include a test that would have caught the bug

### Coverage

- Line coverage must not decrease in a PR
- Target: > 80% line coverage for all system layer components

---

## Maintainer Model

SigmaOS uses a **meritocratic maintainer model** with three tiers:

### Tier 1: Contributor

- Has had at least 1 merged PR
- Can comment on and review PRs (reviews count as one approval for Tier 1+ PRs)
- Listed in `CONTRIBUTORS.md`

**How to become one:** Merge your first PR.

### Tier 2: Committer

- Has had 5+ merged PRs of meaningful complexity
- Can approve PRs (counts toward the required approval count)
- Has repository write access to their domain repos
- Listed in `CODEOWNERS`

**How to become one:** Nominated by an existing Maintainer after 5 merged PRs.

### Tier 3: Maintainer

- Has had 20+ merged PRs
- Can merge PRs and cut releases
- Has write access to all repositories in their layer
- Participates in RFC voting
- Responsible for release engineering

**How to become one:** Nominated by two existing Maintainers, confirmed by majority vote.

### Tier 4: Core Maintainer

- Long-term technical stewards of the project
- Final authority on architecture and governance disputes
- Responsible for the overall project health

---

## Governance

### Decision Making

**Small decisions** (bug fixes, minor features): PR author + 1 reviewer
**Medium decisions** (significant features, API changes): RFC issue + 7-day comment period + maintainer vote
**Large decisions** (architecture changes, new components, governance changes): RFC issue + 14-day comment period + majority maintainer vote

### RFC Process

1. Open an issue titled `RFC: <title>`
2. Add label `rfc`
3. Post the RFC document (problem, proposal, alternatives, drawbacks)
4. Community comment period (7 or 14 days)
5. Maintainer vote (simple majority)
6. If approved: create a tracking issue and implementation PR

### Resolving Disputes

1. Discuss in the PR or issue thread
2. If unresolved: escalate to GitHub Discussions with `dispute` label
3. If still unresolved: Core Maintainer makes the final call

### Code of Conduct

SigmaOS follows the **Contributor Covenant v2.1**. Key points:
- Be respectful and constructive
- No harassment, discrimination, or personal attacks
- Disagreements are about code, not people
- Violations are reported to maintainers (or core@sigmaos.dev for sensitive reports)

Full text: [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md)

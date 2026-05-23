# Contributing to SigmaOS Zenith

Thank you for contributing to SigmaOS! This guide covers everything you need to know to get from zero to a merged pull request.

---

## 📋 Table of Contents

1. [Architecture Principles](#architecture-principles)
2. [Branch Strategy](#branch-strategy)
3. [Setting Up Your Environment](#setting-up-your-environment)
4. [Commit Conventions](#commit-conventions)
5. [Pull Request Process](#pull-request-process)
6. [Code Style](#code-style)
7. [Running Tests](#running-tests)
8. [Documentation Standards](#documentation-standards)

---

## 🏛️ Architecture Principles

SigmaOS enforces **absolute sovereignty**. Before writing a single line of code, internalize these rules:

- **Zero-Dependency in Ring-0**: Kernel shards (`/kernel/`) must never include standard C library headers. Use `include/sigma_kernel_types.h` for all freestanding type declarations.
- **Bounded String Operations**: Always use `strncpy`, `snprintf`, and length-bounded variants. Raw `strcpy` or `sprintf` calls will be rejected.
- **Shard Isolation**: Each subsystem must operate as a discrete unit. Cross-shard communication goes through the SPSC IPC queue — never via global mutable state.
- **600-Shard Lattice Integrity**: New modules must be registered in `SHARDS.manifest` with their correct shard ID and dependency map.

---

## 🌿 Branch Strategy

SigmaOS uses a **12-branch taxonomy** maintained by the S-BUSE (Branch Uniformity & Synchronization Engine).

| Branch | Purpose | Who Pushes Here |
| :--- | :--- | :--- |
| `main` | Stable production, source of truth | Maintainers only (via PR) |
| `release/standalone` | Bare-metal desktop target | Feature branches via PR |
| `release/rtos` | Real-time embedded systems | Feature branches via PR |
| `release/mobile` | Energy-aware mobile platforms | Feature branches via PR |
| `release/microkernel` | Ultra-minimal 120-shard config | Feature branches via PR |
| `release/dual-boot` | Co-operative dual-boot | Feature branches via PR |
| `release/distributed` | Cluster-native computing | Feature branches via PR |
| `release/cloud` | Headless virtualization | Feature branches via PR |
| `release/browser` | WebAssembly runtime | Feature branches via PR |
| `release/app` | App-store sandbox containers | Feature branches via PR |
| `performance-optimized` | SIMD-tuned builds | Maintainers only |
| `gh-pages` | Static documentation site | CI auto-deploy only |

**Workflow:**

1. Fork the repository and create a feature branch from `main`:
   ```bash
   git checkout -b feat/my-feature main
   ```
2. Make your changes. All changes flow through `main` first.
3. The S-BUSE engine (`tools/sync_all_branches.js`) propagates from `main` to all `release/*` branches automatically.
4. Never commit directly to `release/*` branches unless you are a core maintainer resolving a branch-specific build issue.

---

## 🛠️ Setting Up Your Environment

See the [Getting Started Wiki Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Getting-Started) for full instructions.

**Quick setup:**
```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
chmod +x scripts/setup.sh
./scripts/setup.sh      # Installs gcc-x86_64-elf, nasm, qemu, xorriso
npm install             # Install JS toolchain for UI and tests
```

---

## 📝 Commit Conventions

We follow the **Conventional Commits** specification:

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Types:**

| Type | When to Use |
| :--- | :--- |
| `feat` | New shard, subsystem, or feature |
| `fix` | Bug fix in existing code |
| `docs` | Documentation only changes |
| `refactor` | Code restructure with no functional change |
| `test` | Adding or fixing test cases |
| `ci` | Changes to GitHub Actions workflows |
| `chore` | Dependency updates, build tooling |

**Examples:**
```
feat(scheduler): add NUMA shard pinning to CFS
fix(allocator): correct slab bucket boundary check
docs(wiki): update Architecture-Overview for v15.2
test(ipc): add SPSC ring buffer overflow test
```

---

## 🔀 Pull Request Process

1. **Open an issue first** for any significant change (new subsystem, refactor, API change).
2. **Fork and branch** from `main` using the naming convention:
   - `feat/description` — new features
   - `fix/description` — bug fixes
   - `docs/description` — documentation only
3. **Pass all tests** before requesting review:
   ```bash
   npm run test
   ```
4. **Fill out the PR template** completely. Partial PRs will be held until the template is complete.
5. **Link the issue** in your PR description using `Closes #<issue-number>`.
6. **Request review** from a CODEOWNER (see `.github/CODEOWNERS`).
7. PRs require **1 approving review** from a maintainer and **all CI checks green**.

---

## 🖊️ Code Style

### C / C++ (Kernel Shards)
- Follow `.clang-format` (run `clang-format -i <file>` before committing)
- Naming: `snake_case` for functions and variables, `SCREAMING_SNAKE_CASE` for constants
- All kernel functions must be prefixed with `sigma_` or `sovereign_`
- No global mutable state outside of explicitly documented singleton shards
- Use `SIGMA_ASSERT` macros — never `assert()` from `<assert.h>`

### JavaScript / TypeScript (UI & Tests)
- Follow `.eslintrc` rules (run `npm run lint` to check)
- Prefer `const` over `let`, avoid `var`
- All test files live in `/tests/` and must end in `.test.js`

### Markdown (Documentation)
- Use ATX-style headings (`##`, not underline style)
- Every new Wiki page must be added to `wiki_repo/_Sidebar.md`
- Run `markdownlint` before committing doc changes

---

## 🧪 Running Tests

```bash
# Run the full test suite (82 tests must pass)
npm run test

# Run lint checks
npm run lint

# Run a specific test file
npx vitest run tests/subsystem_features.test.js
```

All 82 tests in `/tests` must return green before a PR can be merged. CI will block the merge automatically if any test fails.

---

## 📚 Documentation Standards

- All new subsystems must have a corresponding **Wiki page** added to `wiki_repo/`
- The page must be linked from `wiki_repo/_Sidebar.md` under the relevant section
- Use the existing `Architecture-Overview.md` format as a template
- API changes must update `wiki_repo/API-Documentation.md`

For the full documentation guide, see [WIKI-Contributing.md](wiki_repo/WIKI-Contributing.md).

---

## 💬 Getting Help

- 🐛 **Bug Reports**: [Open an Issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new?template=bug_report.md)
- 💡 **Feature Requests**: [Open an Issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new?template=feature_request.md)
- 📖 **Wiki**: [https://github.com/AaryanSinghChauhan09/SigmaOS/wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

---

*© 2026 SigmaOS Sovereign Project — Sovereignty is the ultimate efficiency.*

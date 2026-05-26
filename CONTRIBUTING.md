# Contributing to SigmaOS

SigmaOS is a sovereign project. We value industrial-grade code, zero-dependency engineering, and transparency.

## ⚖️ General Guidelines

- **Zero-Dependency**: Do not include external monolithic libraries (glibc, boost, etc.).

- **Silicon-Up**: Write code that interacts directly with the lattice/hal when possible.

- **PQC-First**: All networked shards must use Dilithium-5/Kyber-1024 signing.

## 🛠 Branching Strategy

- `main`: The Industrial Gold build. Stable and certified. Direct commits to `main` are strictly forbidden. All code must merge via a PR.
- `rolling`: Continuous updates and experimental shards.
- `feature/*`: For all new functionality or exploratory work. Branches must branch off `rolling`.
- `release/*`: Format-specific production builds. Only critical bug fixes may be pushed here directly.

## 📝 Commit Standards

We strictly follow Conventional Commits to maintain a clean history.
- **Format**: `type(scope): [SHARD-ID] description`
- **Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`
- **Example**: `feat(net): [S-NET] Implement Sovereign IPv6 Mesh Networking`
- Use imperative mood ("feat: add shard" not "added shard").

## 🎨 Coding Style

- **Formatting**: All C/C++ code must be formatted using `clang-format` based on the `.clang-format` rules located in the repository root.
- **Naming Conventions**: 
  - Syscalls and kernel APIs must use `snake_case` prefixed with `sigma_` (e.g., `sigma_spawn_shard`).
  - Classes (if C++ is used) must use `PascalCase`.
- **Static Analysis**: Code must pass `clang-tidy` checks with zero warnings.

## 🖇 Pull Request Process

1. Synchronize your local lattice with the current `rolling` branch.

2. Run the Matrix Testing Algorithm (`tests/matrix_test.py`).

3. Ensure 100% documentation parity in the Wiki.

## 🛡 Security Reports

Report vulnerabilities directly to the Sovereign Security Shard (`security@sigmaos.org`) using PQC-GPG encryption.

---

### Your contributions define the future of sovereignty

# Contributing to SigmaOS 🚀

Welcome to the **SigmaOS Sovereign Operating System** open-source project! Inspired by the engineering rigor of Arch Linux, FreeBSD, Gentoo, and Debian, SigmaOS is a next-generation bare-metal operating system built with Rust, zero external dependencies, and post-quantum cryptographic security.

---

## 🏛️ Sovereign Governance & Developer Philosophy

SigmaOS follows the **SigmaOS Supreme Court Governance Framework** and Developer Certificate of Origin (DCO). All contributions undergo automated verification, peer review, and statutory compliance checks.

### Developer Certificate of Origin (DCO) & Signoff

Every commit submitted to SigmaOS must contain a `Signed-off-by` line certifying compliance with the Developer Certificate of Origin:

```text
Signed-off-by: Developer Name <developer@example.com>
```

You can automatically add this line to your commits using `git commit -s`.

---

## 🛠️ Contribution Workflow & Architectural Rules

### 1. Bare-Metal OOP Rules
- All kernel and userland core modules reside under `src/` as `#![no_std]` Rust code.
- Dependencies outside core Rust components are forbidden. Do not add external crates unless approved by the Supreme Court Chamber.

### 2. Conventional Commit Guidelines
Commit messages are strictly linted via `@commitlint/cli`. Commit subject lines must follow the Conventional Commits format with allowed types:

- `feat`: New user-facing or system capability
- `fix`: Bug fix
- `docs`: Documentation update
- `style`: Formatting or visual adjustment
- `refactor`: Code reorganization without behavioral change
- `perf`: Performance optimization
- `test`: Addition or modification of unit/integration tests
- `chore`: Maintenance or repository maintenance task
- `revert`: Revert of a previous commit
- `impl`: Deep internal module implementation
- `driver`: Device driver framework modifications
- `security`: Security patch or sandboxing update
- `kernel`: Core kernel scheduler, memory allocator, or IPC change
- `arch`: Architecture bring-up or assembly optimization
- `ci`: GitHub Actions CI/CD workflow updates
- `pkg`: Universal package manager (`SigmaPkg`, `pacman-contrib`, AUR) updates
- `ai`: Autonomous AI agent or intelligence engine updates
- `ux`: Zenith visual desktop compositor updates
- `sdk`: Userland API or developer SDK update
- `boot`: UEFI, firmware, or initramfs bootloader update

**Example:**
```bash
git commit -s -m "feat(pkg): implement pacman-contrib paccache and pacdiff utilities"
```

---

## 🧪 Testing & Verification

Before submitting a pull request, ensure all system test suites pass:

```bash
# Run all system inspection and component tests
./run_sigma_tests.sh

# Run SPDX license compliance check
./scripts/check-spdx.sh
```

For standalone module unit testing:
```bash
mkdir -p build
rustc --edition 2021 --test src/sigpkg/verifier.rs -o build/test_verifier && ./build/test_verifier
```

---

## 📦 Packaging & Distro Parity Guidelines

SigmaOS natively absorbs and translates packages across foreign Linux & BSD ecosystems (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.flatpak`, `.appimage`, `.ebuild`, `.hpkg`).

When contributing package definitions or `SigmaPkg` adapters:
- Maintain source isolation within `src/sigpkg/` and `src/package/`.
- Run static security audits on build recipes (`MakepkgSandbox::audit_pkgbuild_security` & `AurClient::audit_pkgbuild_safety`).

---

## 📖 Wiki & Documentation

Detailed technical specifications, architecture diagrams, and roadmap milestones are maintained under `WIKI/` and synchronized across all wiki targets (`wiki/`, `wiki_repo/`) via:

```bash
./scripts/sync_wiki.sh
```

For more details, visit the [SigmaOS Sovereign Contribution Landing Page](WIKI/SOVEREIGN_CONTRIBUTION_LANDING_PAGE.md).

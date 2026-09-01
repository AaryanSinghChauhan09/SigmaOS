# 🤝 Contributing to SigmaOS

Thank you for contributing to SigmaOS! This guide outlines the development standards and pre-commit protocols required for all contributions.

---

## 1. Development Principles

* **Zero-Dependency Core:** Kernel and `klib` crates must not depend on external third-party crates unless explicitly configured for userland simulation.
* **Strict Shell Execution:** Shell scripts must enforce POSIX-compliant, strict execution modes (`set -euo pipefail`).
* **pnpm Mandate:** Always use `pnpm` exclusively (never `npm` or `yarn`) for package management and UI script execution in `zenith_desktop`.

---

## 2. Mandatory Verification & Quality Gate

Before submitting a Pull Request, run the following verification checks:

```bash
# 1. Run full test runner
./run_sigma_tests.sh

# 2. Run Quality Gate check
./scripts/sigma_quality_check.sh

# 3. Synchronize Wiki documentation
./scripts/sync_wiki.sh
./scripts/sigma_automation.sh wiki-sync
```

All Pull Requests must achieve:
- **0** open TODO/stub markers
- **0** critical security warnings
- **100%** test pass rate
- **1:1** synchronization between `WIKI/`, `wiki/`, and `wiki_repo/`

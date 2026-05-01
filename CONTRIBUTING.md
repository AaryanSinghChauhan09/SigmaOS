# Contributing to SigmaOS

Thank you for considering a contribution to the **SigmaOS Sovereign Lattice**!

## Getting Started

1. **Fork** the repository and create a branch from `main`.
2. **Build** the project using the deterministic build pipeline:

   ```bash
   python3 tools/sigma-build.py
   ```

3. **Run static analysis** before submitting:

   ```bash
   cppcheck --enable=warning,style,performance kernel/core/
   ```

## Code Standards

All contributions must adhere to the **Sovereign Shard Standard**:

- ✅ Every new kernel component must be a **C++ OOP Singleton** with `extern "C"` wrappers.
- ✅ Zero external library dependencies — strictly `sigma_types.h`, `sigma_hal.h`, `SovereignLibC.h`.
- ✅ All functions must emit structured log output via `sigma_log()` / `sigma_printf()`.
- ✅ No raw pointers without bounds checks via `sigma_hardened_strcpy()`.
- ✅ New shards must be registered in `SovereignUSR` at init.

## Pull Request Checklist

- [ ] Code compiles with `-Wall -Wextra -Werror`
- [ ] `cppcheck` reports zero warnings
- [ ] New shard has a `_init()`, at least one primary function, and C wrappers
- [ ] Relevant wiki page created or updated in `SigmaOS.wiki`
- [ ] `MISSING_COMPONENTS.md` updated if a gap is closed

## Security Issues

Please **do not** open public issues for security vulnerabilities. Read [SECURITY.md](../SECURITY.md) for responsible disclosure guidelines.

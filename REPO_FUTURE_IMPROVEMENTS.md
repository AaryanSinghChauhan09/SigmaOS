# Repository Future Improvements & Quality Gates

This document outlines structural recommendations for code maintenance, automated quality gates, and repository operations.

---

## 🚦 Automated Quality Gates

We propose integrating `scripts/sigma_quality_check.sh` as a mandatory pre-commit hook to maintain zero-stub and zero-credential properties.

```
                  [git commit triggered]
                            │
                            ▼
               [Run pre-commit quality hook]
                            │
         ┌──────────────────┴──────────────────┐
         ▼                                     ▼
 [SPDX Header check]                    [TODO/Stub check]
(Ensure all files have ID)            (Fail if open stubs > 100)
         │                                     │
         └──────────────────┬──────────────────┘
                            ▼
             [Credential leak scanner (regex)]
                            │
                            ▼
              [Allow commit / Block commit]
```

---

## 🧪 Unified Test Harness Architecture

Expand the `test_runner.rs` to validate all five compile-time profiles dynamically during the CI pipeline.

```toml
# .github/workflows/ci.yml (proposed matrix)
strategy:
  matrix:
    profile: [desktop, microkernel, cloud, mobile, rtos]
    target: [x86_64-unknown-none, aarch64-unknown-none]
```

---

## 📚 Automated Documentation Portals

1. **RustDoc**: Build cargo docs for all kernel submodules, publishing them to a private Github Pages endpoint on merge to `main`.
2. **Doxygen**: Generate class diagrams for legacy C++ libraries and host them dynamically under `docs.sigmaos.org`.
3. **Wiki-Sync**: Automate markdown migration loops using the NIM sync script to keep the Wiki parity in absolute synchronization.

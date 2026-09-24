# SigmaOS Next Steps Guidelines & Operational Execution Handbook

## Executive Mandate
This handbook outlines the mandatory operational standards, testing protocols, security guidelines, design principles, and synchronization rules for developers and autonomous agents working on **SigmaOS**.

All updates, bug fixes, features, and documentation enhancements must be committed **directly to the `main` branch**. Under no circumstances should pull requests (PRs) be created for codebase updates.

---

## 🛠️ Developer & Agent Operational Rules

### 1. Direct Commit Policy (Zero-PR Mandate)
- **Rule:** Never create a pull request (PR) for any changes in this repository.
- **Action:** Push changes directly to `main` after verifying that all tests pass and code quality checks are satisfied.

### 2. Tri-Agent Governance & Collaboration
Work in harmony with the three autonomous specialized agent personas:
- **⚡ Bolt (Performance & Optimization Agent):** Focuses on microsecond-level speedups, zero-allocation data structures, lockless ring buffers, and profiling.
- **🎨 Palette (UX & Accessibility Agent):** Ensures WCAG 2.2 AAA accessibility compliance, screen-reader AT-SPI2 bus integration, smooth animations, and high-contrast focus rings.
- **🛡️ Sentinel (Security & Compliance Agent):** Mandates ASLR guard page enforcement, TPM 2.0 PCR sealing, PQC post-quantum cryptography, and secret-free cleanroom execution.

### 3. Comprehensive Domain Requirements

#### A. Code Quality & Testing
- Run test suites before finalizing changes (`pytest tests/` and `./run_sigma_tests.sh`).
- Ensure all standalone unit tests pass (`rustc --test`).
- Zero syntax errors, runtime panics, or unhandled exceptions.

#### B. Performance & Optimization
- Prefer stack allocations, 32-byte slab descriptors, and 4KB page frame buffers over dynamic heap reallocations.
- Utilize zero-copy eBPF XDP rings for networking and DMA ring buffers for drivers.

#### C. Security & Compliance
- Never hardcode secrets, private keys, or API tokens.
- Maintain least-privilege `permissions:` blocks and 40-character SHA hash pinning in GitHub Actions workflows.
- Enforce ASLR guard gaps and Landlock v5 / Capsicum sandboxing.

#### D. Multi-Format Package System & PR Integration
- Support PR package transpilation (`PKGBUILD`, `ebuild`, `xbps-src`, FreeBSD Ports, Nix Flakes) into native `UnifiedPackage` structs via `PackagePullRequestParser` in `src/package/universal.rs`.

#### E. Object-Oriented Programming (OOP) Excellence
- Apply OOP design patterns (Strategy, Observer, Decorator, Command, Template Method, Composite) when structuring complex systems.

---

## 🔄 Mirror Directory Synchronization Standard

Whenever `ImprovementPlan.md` or `NEXT_STEPS_GUIDELINES.md` (or any core architecture documentation) is updated, it **MUST** be synchronously copied across all mirror locations in the repository:

1. `./` (Root directory)
2. `docs/`
3. `wiki/`
4. `WIKI/`
5. `wiki_content/`
6. `wiki_repo/`

### Mirror Sync Command Example:
```bash
cp ImprovementPlan.md NEXT_STEPS_GUIDELINES.md docs/
cp ImprovementPlan.md NEXT_STEPS_GUIDELINES.md wiki/
cp ImprovementPlan.md NEXT_STEPS_GUIDELINES.md WIKI/
cp ImprovementPlan.md NEXT_STEPS_GUIDELINES.md wiki_content/
cp ImprovementPlan.md NEXT_STEPS_GUIDELINES.md wiki_repo/
```

---

## 🎯 Immediate Tactical Execution Checklist
- [x] Fix module name collisions and mutability warnings in `src/distro/linux_bsd_distro_gaps.rs`.
- [x] Verify Python (`pytest tests/`) and Rust unit test suites (`./run_sigma_tests.sh`).
- [x] Audit all 8 core domains and update `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md`.
- [x] Synchronize master plans across all six mirror directories.
- [x] Execute pre-commit verification steps.
- [x] Commit and push directly to `main`.

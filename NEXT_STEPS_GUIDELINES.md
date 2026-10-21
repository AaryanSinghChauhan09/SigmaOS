# SigmaOS Next Steps Guidelines & Developer Operational Standards

This document specifies the operational guidelines, verification steps, and developer workflows for executing technical improvements on the **SigmaOS** codebase.

---

## 1. Direct Commit Workflow (No-PR Standard)

When implementing technical improvements, bug fixes, or documentation updates for this repository, follow the direct commit workflow:

1. **No Pull Requests**: Do not create external pull requests for internal agent tasks or core repository updates unless explicitly requested by the maintainer.
2. **Main Branch Consolidation**: Apply verified code changes directly onto the `main` branch.
3. **Atomic Commits**: Ensure each commit contains a focused set of related changes accompanied by a clear, git-agnostic commit message.

---

## 2. Pre-Commit Verification Gate

Before staging and committing any code changes to `main`, every developer and AI agent MUST execute the following verification steps:

```bash
# 1. Execute the comprehensive native test suite
./run_sigma_tests.sh

# 2. Check for rustfmt style compliance
cargo fmt -- --check

# 3. Check for library compilation warnings and errors
cargo check --lib
```

---

## 3. Tri-Agent Framework Integration & Operational Rules

SigmaOS development relies on three specialized agent personas working in harmony:

### ⚡ Bolt Agent (Performance & Optimization)
- **Focus**: Algorithmic speedups, memory allocation minimization, zero-copy data processing.
- **Rules**:
  - Profile before optimizing.
  - Prioritize `no_std`-friendly zero-allocation fastpaths (e.g. ASCII window searching over heap-allocated string transforms).
  - Document learnings in `.jules/bolt.md`.

### 🎨 Palette Agent (UX & Accessibility)
- **Focus**: User interface usability, keyboard navigation, high contrast visual styling, ARIA compliance.
- **Rules**:
  - Enforce WCAG 2.1 AA contrast ratios (>= 4.5:1) in CSS and desktop components.
  - Include accessible labels and focus-visible indicators.
  - Document learnings in `.jules/palette.md`.

### 🛡️ Sentinel Agent (Security & Hardening)
- **Focus**: Defense-in-depth, input sanitization, vulnerability remediation, memory safety.
- **Rules**:
  - Never commit hardcoded secrets, keys, or passwords.
  - Normalize and sanitize all external input vectors against injection (URL double encoding, path traversal, shell parameter expansion).
  - Document learnings in `.jules/sentinel.md`.

---

## 4. Prioritized Execution Checklist for Developers

- [ ] **Fix `no_std` `ToString` Import**: Update `src/creative/anicraft_missing_components_parity_pr.rs` to include `use alloc::string::ToString;` so that standalone compilation succeeds across all modules.
- [ ] **Fix Test Borrow Moves**: Add `.clone()` to string field accesses (`backup.id`, `container.id`, `vm.id`) in `src/system/sigmabackupmanager.rs`, `src/system/sigmacontainermanager.rs`, and `src/system/sigmavirtualizationmanager.rs`.
- [ ] **Clean Up Unused Imports**: Remove unused imports across `src/memory/huge_pages.rs`, `src/memory/kswapd.rs`, and `src/distro/sovereign_netbsd_parity_engine.rs`.
- [ ] **Expand Zero-Allocation Searching**: Replace `.to_lowercase()` calls in hot search loops with window-based ASCII case-insensitive comparisons.
- [ ] **Maintain Documentation Sync**: Keep `docs/` and `wiki/` documentation mirrors synchronized whenever adding new subsystems or architecture updates.

# SigmaOS Immediate Next Steps & Guidelines

> **Target Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS
> **Target Execution Branch:** `main`
> **Framework:** Tri-Agent Framework (Bolt ⚡, Palette 🎨, Sentinel 🛡️)

---

## Operating Guidelines & Directives

### 1. Direct Main Branch Commit Policy
- **No Pull Requests:** All verified specifications, technical improvement plans (`ImprovementPlan.md`), next steps guidelines (`NEXT_STEPS_GUIDELINES.md`), and codebase enhancements must be committed directly to the `main` branch without creating pull requests.
- **Atomic & Verified Commit History:** Every commit on `main` must represent a fully verified, self-contained milestone that compiles cleanly and passes all test suites.

### 2. Tri-Agent Framework Integration
- **Bolt ⚡ (Performance):** Continuously hunt for performance bottlenecks, zero-copy buffer fastpaths, SIMD optimizations, and algorithm speedups without sacrificing readability. Log critical performance insights in `.jules/bolt.md`.
- **Palette 🎨 (UX & Accessibility):** Enhance userland CLI and desktop interface interactions with WCAG 2.1 AA accessibility, keyboard navigation focus indicators, helpful error messages, ARIA labels, and intuitive feedback. Log critical UX learnings in `.jules/palette.md`.
- **Sentinel 🛡️ (Security & Hardening):** Guard against vulnerabilities, enforce Landlock v5/v6 sandboxing, Post-Quantum Cryptography (PQC Dilithium5/Kyber) signature validation, zeroize secret buffers, and ensure ISO 27001 audit logging. Log critical security discoveries in `.jules/sentinel.md`.

### 3. Continuous Verification Checklist
Before submitting any change to the repository, perform the following verification steps:
1. **Compilation Check:** Run `cargo check --lib` to confirm there are no syntax or type errors.
2. **Comprehensive Test Suite:** Execute `./run_sigma_tests.sh` to run all native Rust unit tests, standalone integration tests, and shell lifecycle checks. Confirm 100% pass rate.
3. **Format & Linting:** Run `cargo fmt -- --check` to ensure strict code style compliance.
4. **Documentation Mirroring:** Ensure master plans and roadmap specifications in `docs/` are reflected in `wiki/` and `WIKI/` mirrors.

---

## Actionable Next Steps

### Phase 1: High-Priority Engineering Improvements
1. **Clean Compiler Warnings:** Resolve non-fatal unused import/variable warnings in package advancement PR modules (`src/package/sovereign_distro_package_advancements_v*.rs`).
2. **Unify PR Gateway Generators:** Refactor repetitive versioned PR manifest structs (`PackagePrProposalManifestV68`, `V70`, `V78`) into a generic, trait-bounded manifest structure.
3. **Expand Zero-Copy Magic Byte Inspection:** Extend the Bolt ⚡ zero-copy payload header classifier in `UniversalPackageFormatInspectorV78` to cover all 240+ supported package formats.

### Phase 2: Security & Compliance Hardening
1. **Memory Zeroization:** Implement auto-zeroing memory wrappers using the `zeroize` trait for cryptographic signing keys and session tokens in `src/package/signing.rs` and `src/security/`.
2. **Automated CVE Feed Integration:** Connect `FreeBsdPkgAuditEngine` and `ArchAuditEngine` to local advisories and OSV/NVD database schemas for automated real-time vulnerability scanning during DPLL constraint solving.

### Phase 3: Tri-Agent Absorption Catalog Progress
1. Continue systematic absorption of the 500+ open-source GitHub OS repositories across the 20+ architectural system domains outlined in `SIGMAOS_MASTER_PLAN_TRI_AGENT_500_REPOS_ABSORPTION.md`.
2. Record all critical technical learnings and architectural patterns in persistent agent journals in `.jules/`.

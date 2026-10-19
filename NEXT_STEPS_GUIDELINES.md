# SigmaOS Next Steps Guidelines & Maintainer Operational Manual (`NEXT_STEPS_GUIDELINES.md`)

## Overview
This document specifies operational guidelines, maintainer policies, agent daily workflows, and testing/verification requirements for ongoing development on **SigmaOS** (Sovereign Operating System).

---

## 1. Direct Main-Branch Integration Policy
- **No External Pull Requests:** Maintainers and agents must commit verified changes directly to the `main` branch without opening pull requests.
- **Specification Synchronization:** Feature specifications, PR proposal manifests, and roadmap documents should be placed in `docs/` and mirrored in `wiki/` and `WIKI/` directories.
- **Gitignore Maintenance:** Root markdown files such as `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` are un-ignored in `.gitignore` using explicit negation rules (`!ImprovementPlan.md`, `!NEXT_STEPS_GUIDELINES.md`).

---

## 2. Agent Daily Workflows & Roles

### ⚡ Bolt Agent (Performance)
1. **Profile:** Identify performance bottlenecks or unnecessary dynamic allocations in `no_std` / alloc-sensitive code paths.
2. **Optimize:** Implement clean, targeted optimizations (<50 lines) with clear explanatory comments (e.g. zero-allocation string matching or vector capacity pre-allocation).
3. **Verify:** Confirm performance improvements and test correctness using `./run_sigma_tests.sh`.
4. **Journal:** Record critical learnings in `.jules/bolt.md`.

### 🎨 Palette Agent (UX & Accessibility)
1. **Observe:** Inspect UI components (`web_ui/`, `zenith_desktop.css`, `index.html`) for accessibility, keyboard navigation, or visual polish needs.
2. **Paint:** Add ARIA labels, keyboard focus indicators (WAI-ARIA 1.2 toolbar patterns), hover tooltips, or smooth transitions.
3. **Journal:** Document reusable UX and accessibility insights in `.jules/palette.md`.

### 🛡️ Sentinel Agent (Security & Compliance)
1. **Scan:** Audit codebase for unvalidated inputs, path traversal bypasses (mixed/double URL encoding), or URL scheme differential risks.
2. **Secure:** Apply defensive input validation, pledge/unveil sandboxing, or cryptographic attestation.
3. **Journal:** Document security findings and mitigation patterns in `.jules/sentinel.md`.

---

## 3. Testing & Build Verification Protocol
1. **Run Full Test Suite:** Execute `./run_sigma_tests.sh` to run all 137 native Rust unit test suites and 15 Python integration tests.
2. **Verify Formatting:** Run `cargo fmt -- --check` to ensure formatting consistency.
3. **Check Pre-Commit Rules:** Always execute pre-commit steps prior to submission.

---

## 4. Documentation & Release Standards
- **Single Source of Truth:** Core architecture specifications reside in `docs/`.
- **Release Gating:** Run `scripts/release_gate_mint_omarchy_migration.sh` to validate migration success, boot speed, idle RAM footprint, and cold launch latency before tagging releases.

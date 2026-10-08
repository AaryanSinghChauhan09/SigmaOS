# SigmaOS Next Steps & Operational Execution Guidelines

This document outlines operational guidelines for developers, AI agents, and maintainers working on **SigmaOS**.

---

## 1. Development Guidelines & Direct Commit Policy

1. **Direct Main Branch Policy**:
   - In accordance with repository conventions, changes are committed directly to `main` branch.
   - Pull Requests (PRs) are drafted and documented as Markdown proposals inside `docs/roadmap/` and synchronized across `wiki/` and `WIKI/` mirrors rather than creating external GitHub PR branches.

2. **Tri-Agent Collaboration Framework**:
   - **Bolt ⚡**: Focus on microsecond performance optimizations, zero-copy memory buffers, and benchmark verification. Log critical learnings in `.jules/bolt.md`.
   - **Palette 🎨**: Focus on visual accessibility, ARIA labels, responsive UI widgets, and user delight. Log critical UX insights in `.jules/palette.md`.
   - **Sentinel 🛡️**: Focus on security hardening, Landlock V4 / OpenBSD pledge sandboxing, and vulnerability scanning. Log security insights in `.jules/sentinel.md`.

---

## 2. Testing & Verification Workflow

Before finalizing any code modification:

1. **Native Subsystem Testing**:
   Run the comprehensive test suite:
   ```bash
   ./run_sigma_tests.sh
   ```

2. **Compilation & Warning Verification**:
   Verify clean library checking without errors:
   ```bash
   cargo check --lib
   ```

3. **Release Gate Validation**:
   Validate system performance gates:
   ```bash
   ./scripts/release_gate_mint_omarchy_migration.sh
   ```

---

## 3. Roadmaps & Feature Synchronization

- Keep `ImprovementPlan.md` updated with daily findings across Code Quality, Performance, Security, Documentation, Governance, Community, Tools, and OOP Principles.
- Synchronize all feature specifications between `docs/` and documentation mirrors in `wiki/` and `WIKI/`.

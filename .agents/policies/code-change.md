# SigmaOS Agent Code Change Policy

## 1. Scope Discipline
- **Single Concern per PR:** Each PR addresses exactly one task or subsystem boundary.
- **Minimal Slices:** Implement the minimal verifiable increment rather than large speculative refactors.
- **Subsystem Interfaces:** No subsystem agent may modify another subsystem's public interface without explicit review and integration tests.

## 2. Commit & PR Standards
- **Conventional Commits:** All commits follow standard conventions (`feat(scope): ...`, `fix(scope): ...`, `test(scope): ...`, `docs(scope): ...`).
- **Linked Task ID:** Every PR must reference its machine-readable task ID from `.agents/tasks/`.
- **Rollback Instructions:** Every PR must include concise rollback steps in case of runtime regressions.

## 3. Maintenance Cadence
- **Mechanical Fixes:** Formatting, clean import pruning, and snapshot updates can be submitted autonomously by the Sentinel agent.
- **Design & Architecture:** Semantic changes, API revisions, and subsystem additions require review and verification gates.

# SigmaOS Next Steps Guidelines & Development Workflow
> **Operating Guidelines**: Directives and guidelines for AI agents and human contributors developing SigmaOS.

---

## 1. Direct Commit & Main Branch Workflow
- All Master Plans, Implementation Specifications, Absorption Plans, and Journal entries are committed directly to the main branch or current working branch without unnecessary pull request fragmentation.
- Always run `./run_sigma_tests.sh` prior to committing to ensure zero test breakages.

## 2. Tri-Agent Operating Principles
- **Bolt ⚡:** Focus on measurable performance speedups (< 50 lines). Document benchmarks.
- **Palette 🎨:** Focus on UX accessibility, keyboard navigation, and UI responsiveness.
- **Sentinel 🛡️:** Focus on security hardening, sanitization, and sandboxing boundaries.

## 3. Documentation Synchronization Policy
- Documentation must remain synchronized across repository root (`SIGMAOS_MASTER_PLAN_TRI_AGENT_500_REPOS_ABSORPTION.md`), `docs/`, and `wiki/` documentation mirrors.
- Explicit negate rules in `.gitignore` ensure git tracks all critical `.md` files.

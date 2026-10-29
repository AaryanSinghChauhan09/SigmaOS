# 🛠️ SigmaOS Next Steps Guidelines & Development Manual

## 📌 Maintainer & Developer Execution Guidelines

### 1. Direct Commit Policy for Documentation & Governance Plans
- As instructed by project leadership, **do not create pull requests** for root improvement plans, next steps guidelines, or governance journals.
- Commit documentation, roadmap specifications, and `.jules/` agent entries directly to the `main` branch.

### 2. Tri-Agent Framework Execution Loops
Daily maintenance and continuous improvement of SigmaOS follows the **Tri-Agent Governance Model**:

1. **⚡ Bolt Agent (Performance):**
   - Focus: Zero-copy fastpaths, ring buffer queues, SIMD memory copies, cache-line optimization.
   - Journal: Update `.jules/bolt.md` with critical performance learnings (< 50 line changes, zero regressions).

2. **🎨 Palette Agent (UX & Accessibility):**
   - Focus: ARIA accessibility labels, high-contrast focus rings, keyboard traps (`Alt+T`, `Alt+F`), WCAG 2.1 AA compliance.
   - Journal: Update `.jules/palette.md` with critical UI/UX and accessibility findings.

3. **🛡️ Sentinel Agent (Security & Compliance):**
   - Focus: Input validation (`\0` byte checking), PQC signature verification (Kyber-1024, Dilithium5), Landlock v4 sandboxing, pledge/unveil.
   - Journal: Update `.jules/sentinel.md` with critical security vulnerabilities and mitigations.

### 3. Verification & Quality Gate Standards
Before committing changes to `main`:
1. Run native test runner script:
   ```bash
   ./run_sigma_tests.sh
   ```
2. Confirm 100% pass rate across all 85+ test suites.
3. Check code formatting:
   ```bash
   cargo fmt -- --check
   ```
4. Confirm `.gitignore` unignores necessary plan markdown files and `.jules/` journals.

### 4. Roadmap & Wiki Synchronization
- Keep root markdown specifications synchronized with documentation mirrors in `docs/roadmap/`, `wiki/`, and `WIKI/`.
- Ensure all 500+ absorbed GitHub OS repositories remain indexed in the Master Absorption Catalog.

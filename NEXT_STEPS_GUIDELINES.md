# SigmaOS Next Steps Guidelines for Maintainers & Agents

Target Repository: `https://github.com/AaryanSinghChauhan09/SigmaOS`
Guideline Version: 10.0-SOVEREIGN

---

## Operating Protocol for Agents & Developers

When contributing to `SigmaOS`, all human maintainers and AI agents must adhere strictly to the guidelines set forth below.

---

## 1. Tri-Agent Execution Protocol

1. **Agent "Bolt" ⚡ (Performance):**
   - Focus on one concise, measurable performance improvement (< 50 lines).
   - Profile before changing code; measure latency or allocation impact.
   - Record critical insights in `.jules/bolt.md`.

2. **Agent "Palette" 🎨 (UX & Accessibility):**
   - Focus on one micro-UX/a11y improvement (< 50 lines).
   - Verify keyboard navigation, visual focus states, and ARIA labels.
   - Record critical insights in `.jules/palette.md`.

3. **Agent "Sentinel" 🛡️ (Security & Hardening):**
   - Focus on one security fix or hardening enhancement (< 50 lines).
   - Validate input boundaries, SECCOMP/Landlock sandbox profiles, and cryptographic attestations.
   - Record critical insights in `.jules/sentinel.md`.

---

## 2. Daily Workflow & Code Verification Checklist

Before submitting any code change or Pull Request:

1. **Run Full Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```
   Ensure 100% of test suites pass without any failures.

2. **Check Code Formatting:**
   ```bash
   cargo fmt -- --check
   ```
   Resolve any formatting diffs.

3. **Run Pre-Commit Instructions:**
   Execute `pre_commit_instructions` tool to confirm test, review, and reflection checks are complete.

4. **Synchronize Documentation:**
   Ensure plan documents and roadmap specs in `docs/` and `docs/roadmap/` are kept up to date.

---

## 3. Maintenance Guidelines

- **Zero-Regression Guarantee:** Never break existing public APIs, syscall interfaces, or unit test expectations.
- **Git Tracking:** All master plan documents, journals (`.jules/`), and roadmap specifications must remain tracked in git via explicit negation rules in `.gitignore`.
- **Descriptive Commits:** Use standard commit formats (e.g., `feat: ...`, `fix: ...`, `docs: ...`, `perf: ...`).

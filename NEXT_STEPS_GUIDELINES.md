# SigmaOS Technical Next Steps & Execution Guidelines 📋

This document outlines the operational roadmap, engineering guidelines, and actionable next steps for future SigmaOS developments on the `main` branch.

---

## 1. Architectural Development Roadmap 🗺️

### Phase 1: Core Kernel & Syscall Consolidation (High Priority)
1. **Error Handling Hardening**: Audit all 10,295 `.unwrap()` instances in `src/`. Convert critical path unwraps to explicit error handling (`SovereignResult<T>`).
2. **Unsafe Block Validation**: Audit 2,144 `unsafe` code regions with `# Safety` contracts and Miri verification.
3. **`sched_ext` & Memory Optimization**: Expand BPF kernel scheduling policies and THP khugepaged defrag daemon tuning.

### Phase 2: Package Engine & Distro Adapter Parity (Medium Priority)
1. **Unified SAT DPLL Solver Engine**: Consolidate repetitive SAT solver code across historical package advancement suites (`v30` through `v70`) into a single canonical engine in `src/package/universal.rs`.
2. **Zero-Copy Memory Mapping**: Expand Content-Addressed Storage (CAS) zero-copy slice usage to all foreign format transpilers (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`).

### Phase 3: Desktop UX, Web UI & Accessibility (Medium Priority)
1. **Zenith Desktop Accessibility**: Enforce WCAG 2.1 AA contrast compliance, keyboard tab order, and screen reader ARIA labels across `web_ui/` and `zenith_desktop/`.
2. **Interactive CLI UX**: Standardize terminal spinner feedback, progress bars, and colored diagnostics across package management CLI tools.

---

## 2. Developer Workflow & Quality Assurance 🛠️

### CI / CD Pipeline Enhancement
- Enable `sccache` in GitHub Actions workflows to cache compiled Rust artifacts.
- Execute `./run_sigma_tests.sh` on all commits to ensure complete regression prevention.
- Enforce strict `cargo fmt -- --check` and `cargo clippy` lint passes.

### Tri-Agent Operating Guidelines
- **Bolt ⚡**: Pursue daily measurable performance gains (<50 LOC per optimization) and log findings in `.jules/bolt.md`.
- **Palette 🎨**: Implement micro-UX/accessibility improvements (<50 LOC) and record design insights in `.jules/palette.md`.
- **Sentinel 🛡️**: Execute continuous vulnerability scans, harden security boundaries, and record learnings in `.jules/sentinel.md`.

---

## 3. Recommended Immediate Next Steps Checklist 🚀

- [x] Fix bash syntax delimiter issues in `run_sigma_tests.sh`.
- [x] Establish agent journal files `.jules/bolt.md`, `.jules/palette.md`, `.jules/sentinel.md`.
- [x] Document comprehensive repository improvement plan in `ImprovementPlan.md`.
- [x] Publish next steps guidelines in `NEXT_STEPS_GUIDELINES.md`.
- [ ] Begin systematic refactoring of `.unwrap()` calls in `src/kernel/` to `SovereignError`.
- [ ] Benchmark `ahash` dependency lookup performance across 100k package graphs.

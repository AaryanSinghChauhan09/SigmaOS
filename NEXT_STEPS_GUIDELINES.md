# SigmaOS Operational Next Steps Guidelines

**Maintained by:** Tri-Agent Framework (Bolt ⚡, Palette 🎨, Sentinel 🛡️)
**Branch:** `main`
**Policy:** Direct Main Branch Commit & Roadmap Synchronization (No External Pull Requests Created)

---

## 1. Daily Development Process
1. **Pre-Execution Check:** Always run `./run_sigma_tests.sh` to confirm zero test failures before editing source code.
2. **Direct Branch Commit Policy:** Do not create external GitHub pull requests. Instead, prepare comprehensive PR proposals in Markdown format under `docs/roadmap/` and mirror them in `wiki/` and `WIKI/`.
3. **Tri-Agent Journaling:** Maintain entries in `.jules/bolt.md`, `.jules/palette.md`, and `.jules/sentinel.md` only for critical, actionable learnings.

---

## 2. Engineering Guidelines Across 8 System Domains

### Domain 1: Code Quality & Testing
* Preserve `#![no_std]` core kernel compatibility (`alloc` / `core` imports).
* Maintain feature-gated standalone testing (`#[cfg(feature = "standalone_test")]`).
* Run `cargo check --lib` and `./run_sigma_tests.sh` to ensure 0 compiler warnings/errors.

### Domain 2: Performance & Optimization (⚡ Bolt)
* Maintain sub-200MB baseline idle memory footprint (~164MB achieved).
* Avoid external subshell invocations (such as `bc`) in hotspot release scripts; use native `awk` or POSIX arithmetic.
* Target sub-1.8s cold boot to desktop and sub-0.4ms cold application launch times.

### Domain 3: Security & Compliance (🛡️ Sentinel)
* Enforce OpenBSD `pledge`/`unveil` sandbox capability restricts on package transpilers and scriptlet engines.
* Maintain zero hardcoded secrets and 0 critical CVE vulnerabilities in dependencies.
* Ensure GDPR amnesic memory scrubbing and WCAG 2.1 AA accessibility compliance across all UI components.

### Domain 4: Documentation & Workflow
* Synchronize all technical architecture specs and roadmap updates across `./`, `docs/`, `wiki/`, `WIKI/`, and `wiki_repo/`.
* Maintain the 9-chapter user manual, SBOM transparency guides, and community mentorship guides.

### Domain 5: Repository Governance
* Keep issue categories structured (Bug, Feature, Enhancement).
* Run `scripts/release_gate_mint_omarchy_migration.sh` to ensure all 5 release validation gates remain GREEN.

### Domain 6: Community & Collaboration
* Tag candidate beginner issues with `good-first-issue` as detailed in `docs/COMMUNITY_MENTORSHIP_GUIDE.md`.
* Promote pair programming and open architectural reviews.

### Domain 7: Tools & Utilities
* Validate CLI error handling for `sigpkg`, `makepkg`, `pacman-key`, `arch-audit`, and `warpinator`.
* Keep shell scripts idempotent and portable across Linux distros and BSD variants.

### Domain 8: Object-Oriented Programming (OOP) Principles
* Apply Encapsulation (hide internal DB / state representation).
* Apply Inheritance/Traits (shared behavior for packaging adapters and device drivers).
* Apply Polymorphism & Design Patterns (Strategy, Factory, Command, Memento, Observer) across package management and IPC subsystems.

---

## 3. Immediate Action Items
* Keep native unit tests green across all 137 Rust test binaries.
* Execute periodic release validation gates before tagging releases.
* Continue expanding multi-distro package format support V33+ without regressions.

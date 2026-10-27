# SigmaOS Next Steps & Development Guidelines

## Purpose
This document provides clear, actionable step-by-step developer guidelines for contributing to, testing, and expanding **SigmaOS**. It complements `ImprovementPlan.md` and serves as the operational standard on the `main` branch.

---

## 1. Development Workflow & Core Rules

### A. Direct Main Branch Governance
* **No Pull Request Requirement:** As per repository governance policy, technical improvements, plan specifications, and guidelines are committed directly to the `main` branch.
* **Documentation & Journaling:** All Markdown specifications, agent journals (`.jules/`), and root documentation files are unignored in `.gitignore` using explicit negate rules to guarantee persistent Git tracking.

### B. Pre-Commit Verification Checklist
Before finalizing any code change or committing directly to `main`, developers and automated agents MUST complete the following verification steps:
1. **Clean Compilation Check:**
   ```bash
   cargo check --lib
   ```
2. **Formatting Compliance:**
   ```bash
   cargo fmt -- --check
   ```
3. **Comprehensive Test Suite Execution:**
   ```bash
   ./run_sigma_tests.sh
   ```
   *Verify that all 70+ test suites (including distro adapters, creative parity modules, QEMU boot engines, and security governors) complete with `test result: ok`.*

---

## 2. Technical Execution Guidelines by Domain

### ⚡ Performance & Optimization Guidelines (Bolt ⚡)
* **Zero-Allocation Priority:** When writing high-frequency kernel loops, packet filters, or VFS lookup paths, prefer stack-allocated buffers (`[u8; N]`) or slice references over heap allocations (`Vec`/`String`).
* **Profile-Guided Compilation:** Always test performance using standard release fast profiles:
  ```bash
  cargo build --profile release-fast
  ```
* **Journaling Requirement:** Record unexpected performance bottlenecks and optimization benchmark numbers in `.jules/bolt.md`.

### 🎨 User Experience & Accessibility Guidelines (Palette 🎨)
* **Accessibility First:** Ensure all zenith desktop HTML/CSS components include keyboard focus states (`:focus-visible`), standard color contrast (>4.5:1), and proper ARIA labels (`aria-label`, `aria-expanded`).
* **Informative Status & Feedback:** Never launch asynchronous operations (such as package transactions or system snapshots) without displaying visual progress indicators or CLI status updates.
* **Journaling Requirement:** Record critical UX findings and accessibility design constraints in `.jules/palette.md`.

### 🛡️ Security & Hardening Guidelines (Sentinel 🛡️)
* **Defense in Depth:** Enforce multi-sandbox isolation (Landlock v4, Pledge/Unveil, Capsicum) across all execution boundaries where external or maintainer scriptlets are executed.
* **Post-Quantum Attestation:** Ensure package verification pipelines validate Dilithium5 post-quantum signature fields alongside OpenBSD Signify.
* **Journaling Requirement:** Record critical security vulnerability patterns and sandbox containment insights in `.jules/sentinel.md`.

---

## 3. Object-Oriented Refactoring & Architecture Guidelines

When adding or refactoring subsystem modules, adhere to the following OOP design patterns:
1. **Encapsulation:** Hide internal mutable state behind struct fields with restricted visibility (`pub(crate)` or private) and expose explicitAccessor methods.
2. **Strategy Pattern:** Use traits (`pub trait ExecutionStrategy`) to encapsulate swappable algorithms for scheduling, compression, and defragmentation.
3. **Command Pattern:** Implement `execute()` and `undo()` methods on transaction objects to support zero-downtime rollback for system updates and package installations.
4. **Factory Pattern:** Utilize factory constructors (`PackageAdapterFactory::create(format)`) to instantiate concrete implementations dynamically.

---

## 4. Maintenance & Support Roadmap

| Phase | Milestone | Key Deliverable | Target Date |
| :--- | :--- | :--- | :--- |
| **Phase 1** | Static Mut Warnings Cleanup | Refactor creative singletons to thread-safe atomic pointers / locks. | Q2 2025 |
| **Phase 2** | Bare-Metal Hardware Qualification | Verify NVMe, GOP display, and UEFI ESP bootloader on physical hardware. | Q2 2025 |
| **Phase 3** | PQC Package Signatures | Default all package repositories to Dilithium5 + Signify dual signatures. | Q3 2025 |
| **Phase 4** | Zenith Desktop Shell Polish | Finalize Wayland compositor animation frame pacing and high-DPI scaling. | Q3 2025 |

---

## 5. Quick Reference Commands

* **Run All Tests:** `./run_sigma_tests.sh`
* **Check Library Build:** `cargo check --lib`
* **Run Cargo Format:** `cargo fmt`
* **Build Sovereign Builder:** `cargo build --bin sovereign_edition_builder`
* **Inspect Git Status:** `git status`

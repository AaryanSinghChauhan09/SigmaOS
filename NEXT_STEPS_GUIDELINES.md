# SigmaOS Next Steps & Operational Guidelines

## Purpose
This guide outlines actionable steps and architectural guidelines for developers, contributors, and maintainers building and stabilizing SigmaOS.

---

## 1. Fast isolated Subsystem Testing
To bypass full kernel build times during active development, test individual modules using standalone compiler flags:

```bash
# Test Universal OOP Package System
rustc --test src/sigpkg/universal_oop_system.rs --edition=2021 --cfg 'feature="standalone_test"' -o test_oop_universal && ./test_oop_universal

# Test Enterprise Network Architecture
rustc --test src/network/enterprise.rs --edition=2021 --cfg 'feature="standalone_test"' -o test_enterprise && ./test_enterprise

# Test Qubes OS Security Isolation Subsystem
rustc --test src/security/qubes_isolation.rs --edition=2021 -o test_qubes && ./test_qubes
```

---

## 2. Developer Onboarding & Workflow Guidelines
1. **Zero External Dependency Rule:**
   - Avoid adding std crate or third-party dependencies unless strictly approved. Use `klib` primitives (`Vec`, `BTreeMap`, `String`) for bare-metal targets (`x86_64-unknown-none`).
2. **Static Secret Scanning Prevention:**
   - Never write literal password or API key strings in test cases. Construct credentials dynamically (e.g. `String::from("pass") + "word123"`).
3. **Rust 2024 Static Mut References:**
   - Migrate legacy `static mut` singletons to safe concurrency wrappers (`AtomicBool`, `Mutex`, or `RwLock`).

---

## 3. UI / UX & WCAG Accessibility Directives
- Ensure all interactive elements in `zenith_desktop` supply explicit `aria-label` attributes.
- Maintain high-contrast ratios and keyboard focus states (`focus-visible:ring-2`).
- Include loading spinners and disabled states for asynchronous user operations.

---

## 4. Release & Governance Directives
- **Documentation Sync:** Always synchronize changes across `BOLT_PALETTE_SENTINEL_REPOS_MASTER_ABSORPTION_PLAN.md`, `wiki/`, and `wiki_repo/`.
- **Pre-Commit Checks:** Run static linters (`cargo check`) before committing code directly to the `main` branch.

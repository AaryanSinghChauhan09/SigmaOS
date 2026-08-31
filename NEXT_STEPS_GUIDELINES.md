# SigmaOS Next Steps & Technical Guidelines

## Overview
This document outlines actionable guidelines, fast isolated testing workflows, coding standards, and release directives for building, stabilizing, and expanding SigmaOS.

---

## 1. Fast Isolated Subsystem Testing Workflows
To bypass full kernel build times during active development, test individual modules using standalone compiler flags:

```bash
# Test Universal OOP Package Subsystem (27+ formats, USE flags, triggers)
rustc --test src/sigpkg/universal_oop_system.rs --edition=2021 --cfg 'feature="standalone_test"' -o test_oop_universal && ./test_oop_universal

# Test Qubes OS Security Isolation Subsystem
rustc --test src/security/qubes_isolation.rs --edition=2021 -o test_qubes && ./test_qubes

# Test Enterprise Network Subsystem (VPN, IPv6 routing)
rustc --test src/network/enterprise.rs --edition=2021 --cfg 'feature="standalone_test"' -o test_enterprise && ./test_enterprise

# Test Tabular Data Engine (CSV/TSV, SQL-style filtering)
rustc --test src/tools/data_engine.rs --edition=2021 -o test_data_engine && ./test_data_engine
```

---

## 2. Developer Onboarding & Architecture Directives

### 2.1 Zero External Dependency Bare-Metal Rule
- Avoid introducing `std` crate dependencies into kernel space (`src/`).
- Use `crate::klib` collection primitives (`Vec`, `BTreeMap`, `String`) for bare-metal target targets (`x86_64-unknown-none`).
- Use target-conditional re-exports in `src/klib/mod.rs` to allow seamless host-based unit testing.

### 2.2 Security & Secret Scanning Hardening
- **No Hardcoded Plaintext Secrets:** Static scanners flag secret-like strings. Construct test credentials dynamically (e.g., `String::from("pass") + "word123"`).
- **Path Sanitization:** Reject directory traversal sequences (`..`) considering colons (`:`), forward slashes (`/`), and backslashes (`\`) as component delimiters.
- **CRLF Log Escaping:** Escape carriage returns (`\r`) and line feeds (`\n`) in key-value syslog attributes to prevent log injection.

### 2.3 Rust Concurrency & Memory Safety
- Migrate legacy `static mut` references to thread-safe concurrency wrappers (`AtomicBool`, `Mutex`, or `RwLock`).
- Cache explicit string/buffer lengths (`name_len: u8`) alongside fixed-size byte arrays (`[u8; 64]`) to guarantee $O(1)$ constant-time slice lookups and eliminate linear zero-byte scans (`.position(|&b| b == 0)`).

---

## 3. Micro-UX & WCAG 2.1 AA Accessibility Standards

### 3.1 UI & Desktop Coding Rules
- **ARIA Attributes:** Provide explicit `aria-label` attributes for all icon-only buttons and controls.
- **Keyboard Navigation:** Custom card components with `role="radio"` or `role="button"` must implement `Enter`/`Space` keydown handlers and support arrow key cycling.
- **Focus Ring Contrast:** Maintain high-contrast `:focus-visible` outline rings (`focus-visible:ring-2`) meeting WCAG 2.1 AA contrast ratios (4.5:1 min).
- **Step Transition Focus:** In multi-step wizards, shift keyboard focus to active step panel headings (`<h2 tabindex="-1">`) upon transition to orient screen readers.

---

## 4. Documentation & Release Directives
- **Documentation Synchronization:** Always synchronize master plan files (`BOLT_PALETTE_SENTINEL_REPOS_MASTER_ABSORPTION_PLAN.md`, `REPOS_ABSORPTION_PLAN.md`, `REPOS_IMPLEMENTATION_PLAN.md`) across repository root, `wiki/`, and `wiki_repo/`.
- **Pre-Commit Checks:** Run static checks (`cargo check --lib`) and standalone test harnesses before committing directly to `main`.
- **Direct Commit Directives:** Do not open external PRs for internal doc updates; commit directly to `main` with clear, standard commit titles and detailed body descriptions.

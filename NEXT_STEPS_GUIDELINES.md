# SigmaOS Next Steps Guidelines & Development Roadmap

## Purpose & Scope
This guideline document serves as the operational manual for contributors, maintainers, and autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) contributing to the SigmaOS codebase on the `main` branch.

---

## 1. Daily Development Cadence & Principles

### 1.1 Direct Contribution Model
- When instructed to work directly on the repository without pull requests, all updates, plans, and technical improvements are verified and committed directly to the `main` branch.
- Ensure every commit message follows conventional commit specifications:
  - Format: `<type>(<scope>): <short descriptive summary>`
  - Examples: `perf(routing): implement zero-alloc search`, `docs(governance): update next steps guidelines`.

### 1.2 Agent Boundaries & Philosophies
- **⚡ Bolt (Performance Agent):**
  - Always profile before optimizing. Keep daily optimizations targeted (< 50 lines), low-risk, and measurably impactful.
  - Document learnings in `.jules/bolt.md`.
- **🎨 Palette (UX & Accessibility Agent):**
  - Ensure all interfaces, terminals, and web shell components adhere to WCAG 2.1 AA contrast and screen-reader accessibility rules.
  - Document learnings in `.jules/palette.md`.
- **🛡️ Sentinel (Security Agent):**
  - Enforce defense-in-depth, least privilege, zero-trust memory sanitization, and strict input validation.
  - Document critical security insights in `.jules/sentinel.md`.

---

## 2. Immediate Next Steps & Implementation Milestones

### Phase 1: Performance & Allocation Hardening (Immediate)
1. **Zero-Allocation String Scanning:** Audit remaining string transforms across `src/package/` and `src/userland/` to replace `.to_lowercase()` and `format!()` with zero-allocation byte slice operations.
2. **Stack Allocation for Bounded Arguments:** Replace dynamically allocated `Vec<String>` in process execution arguments with fixed-size arrays where argument counts are known to be bounded (<= 16 args).
3. **Linker & Build Time Tuning:** Verify dev compile times remain under 4 minutes with `profile.dev` settings (`codegen-units = 16`, `opt-level = 0`).

### Phase 2: Security & Regulatory Compliance (Near-Term)
1. **Cryptographic Memory Zeroing:** Add explicit memory sanitization (`zeroize`) to all kernel crypto buffers, private key storage, and authentication tokens before memory deallocation.
2. **GDPR Cryptographic Data Shredding:** Implement native user profile data shredding in the VFS layer (`src/vfs/crypto_shred.rs`) to guarantee permanent, unrecoverable erasure upon user request.
3. **Audit Trail Hash Chaining:** Integrate tamper-evident SHA-256 / Blake3 hash chaining in `/var/log/sigma/` security logs for HIPAA and ISO 27001 auditability.

### Phase 3: Desktop UX Polish & Accessibility (Mid-Term)
1. **Zenith Shell WCAG 2.1 AA Compliance:** Update `zenith_desktop.css` and web UI templates to enforce a minimum 4.5:1 text contrast ratio and 3:1 focus ring boundaries.
2. **Accessible Live Regions:** Add `role="status"` and `aria-live="polite"` tags to asynchronous notifications in Zenith Desktop.
3. **Keyboard Shortcut Discoverability:** Provide intuitive tooltips displaying keyboard shortcuts for all system dock and window management buttons.

### Phase 4: OOP Refactoring & Modular Architecture (Ongoing)
1. **Command Pattern Decoupling:** Convert procedural CLI routers in `tools/` into Command objects implementing a common `SigmaCommand` trait:
   ```rust
   pub trait SigmaCommand {
       fn name(&self) -> &'static str;
       fn execute(&self, args: &[&str]) -> Result<i32, CommandError>;
       fn help(&self) -> &'static str;
   }
   ```
2. **Strategy Pattern for Schedulers:** Ensure all scheduler implementations (`sched_ext`, BORE, EEVDF) adhere to a shared, extensible trait interface.
3. **Encapsulation Audit:** Review all public fields in kernel and package structs; transition mutable state to private fields with safe accessor methods.

---

## 3. Verification & Quality Gates

Before finalizing any changes on `main`:
1. **Type Checking:** Run `cargo check --lib` to ensure zero compilation errors across core modules.
2. **Test Suite Execution:** Run `./run_sigma_tests.sh` or targeted test runners to verify unit test assertions pass cleanly.
3. **Git Tracking Check:** Run `git status -u` to confirm documentation and journals are properly tracked and unignored.

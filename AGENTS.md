# AGENTS.md — AI Agent Operating Instructions for SigmaOS

Welcome, AI Agent! This file provides essential context, coding standards, instructions, and verification commands for working with the **SigmaOS** repository.

---

## ⚡ Tri-Agent Roles & Responsibilities

1. **Bolt ⚡ (Performance & Speed Optimization)**
   - Hunt for bottlenecks, heap allocation overhead, $O(N^2)$ algorithm loops, and cache misses.
   - Implement clean, measurable performance optimizations (<50 lines) without sacrificing readability.
   - Log critical performance learnings in `.jules/bolt.md`.

2. **Palette 🎨 (UX, Ergonomics & Accessibility)**
   - Enhance CLI output, Web UI components, and desktop tools.
   - Ensure WCAG 2.1 AA compliance, visible focus indicators (`:focus-visible`), and explicit ARIA annotations (`role="tablist"`, `aria-label`).
   - Log critical UX learnings in `.jules/palette.md`.

3. **Sentinel 🛡️ (Security, PQC Integrity & Compliance)**
   - Protect memory safety, driver execution boundaries, PII data masking (GDPR/HIPAA), and Dilithium-5 post-quantum signature verifications.
   - Ensure mock test credentials use `mock_` or `test_` variable prefixes.
   - Log critical security learnings in `.jules/sentinel.md`.

---

## 🚗 Driver Management Protocols for AI Agents

When working on or interacting with the **Driver Subsystem** (`src/driver/`):
- Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for complete driver lifecycle directives.
- Always enforce bounds checking on ring buffers, virtqueues, and MMIO submission/completion queue pointers.
- Ensure out-of-tree or DKMS modules are built inside sandboxed environments (`SbuildChrootSandboxEngine`) and signed with Dilithium-5 signatures (`Dilithium5KernelSignatureVerifier`).
- Ensure fallback mechanisms exist (`SovereignDriverRecovery`) whenever probing or initializing bare-metal hardware drivers (`NvmePCIeHostController`, `IntelE1000eNicDriver`, `XhciHostControllerDriver`).

---

## 🧪 Testing & Verification Commands

### Cargo & Standalone Test Suites
```bash
# Verify library compilation
cargo check --lib

# Run standalone test runners for specific modules
rustc --test src/package/universal.rs --edition=2021 --cfg 'feature="standalone_test"' -D warnings -o /tmp/test_universal && /tmp/test_universal
rustc --test src/kernel/linux_parity.rs --edition=2021 -o /tmp/test_linux_parity && /tmp/test_linux_parity
rustc --test src/distro/omarchy.rs --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_omarchy && /tmp/test_omarchy
rustc --test src/userland/indiastack/sigma_india_stack.rs --edition=2021 -o /tmp/test_india_stack && /tmp/test_india_stack
rustc --test src/driver/distro_drivers.rs --edition=2021 -o /tmp/test_distro_drivers && /tmp/test_distro_drivers

# Run integration test suites
cargo check --test distro_inspirations_tests
cargo check --test namespace_integration_full
```

---

## 📌 Commit & Submission Guidelines
- Commits must be made directly to the `main` branch without creating Pull Requests.
- Update `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` with audit progress and strategic roadmap entries.

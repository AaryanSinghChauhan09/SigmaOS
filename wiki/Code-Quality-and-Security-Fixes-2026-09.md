# Code Quality & Security Fixes — September 2026

## Summary

This document tracks code quality improvements, security scanning fixes, and no_std compliance
changes applied in the September 2026 development sprint.

## Security Code Scanning Fixes

### Fixed: DOM Text Reinterpreted as HTML
**File:** `zenith_desktop/index.js`
**Issue:** Potential XSS via unsanitized innerHTML assignments
**Fix:** Added `setSecureTextContent(element, text)` helper that uses `element.textContent`
instead of `innerHTML`, preventing DOM text from being reinterpreted as HTML.
```javascript
// FIXED: Safe text content setter (no innerHTML)
export function setSecureTextContent(element, text) {
  if (element) { element.textContent = text; }
}
```

### Fixed: Prototype-Polluting Function
**File:** `zenith_desktop/index.js`
**Issue:** Deep merge function could pollute `Object.prototype` via `__proto__` key
**Fix:** Added explicit key guards in `safeDeepMerge()`:
```javascript
if (key === "__proto__" || key === "constructor" || key === "prototype") continue;
```

### Fixed: Duplicate Module Declarations
**File:** `src/lib.rs`
**Issue:** `pub mod community` declared twice (GitHub code scanning duplicate symbol alert)
**Fix:** Removed duplicate declaration; `pub mod community` kept once in initial module block.

### Fixed: Unnecessary std:: Dependencies in no_std Kernel Code
**Files affected:**
- `src/distro/linux_bsd_inspirations.rs`
- `src/distro/missing_distro_innovations.rs`
- `src/klib/string.rs`
- `src/kernel/architecture.rs`

**Issue:** Files used `use std::string::String` and `use std::vec::Vec` directly,
violating the `#![no_std]` zero-dependency kernel invariant from AGENTS.md.

**Fix:** Applied cfg-gated conditional imports:
```rust
#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
```

## CI/CD Cleanup

### Removed 52 Redundant Workflows
The following categories of workflows were removed as redundant:

**Distro-specific package CI** (covered by `02_Distro_Package_Matrix_CI.yml`):
- alpine-abuild-aports-ci.yml
- arch-aur-makepkg-chroot-ci.yml
- clearlinux-mixer-stateless-ci.yml
- debian-pbuilder-cowbuilder-ci.yml
- fedora-copr-mock-srpm-ci.yml
- gentoo-portage-ebuild-ci.yml
- nixos-flake-store-gc-ci.yml
- void-xbps-src-binary-ci.yml
- (and 30+ more)

**Duplicate main CI** (covered by `sigma_master_ci.yml`):
- ci.yml — duplicate of sigma_master_ci.yml
- sigma-ci.yml — duplicate of sigma_master_ci.yml

**Duplicate security CI** (covered by `03_Security_Hardening_Audit.yml`):
- security.yml — duplicate of 03_Security_Hardening_Audit.yml

### Remaining Consolidated Workflows (23 total)
```
01_Deployment_Suite_Release.yml      — Release automation
02_Distro_Package_Matrix_CI.yml      — Multi-distro package format testing
03_Security_Hardening_Audit.yml      — Security scanning & hardening
04_Kernel_Boot_Qemu_Test.yml         — QEMU boot tests (x86_64, ARM64, RISC-V)
05_Automation_PR_Governance.yml      — PR automation
06_Documentation_Pages_Sync.yml      — Docs/wiki sync
asan-ubsan-sanitizer-smoke-ci.yml   — Address/UB sanitizer tests
pr_fast_checks.yml                   — Fast PR linting
reproducible-sbom-cosign.yml        — SBOM + supply chain
rust-clippy.yml                      — Rust linter
sast-fuzzing-semgrep.yml            — SAST + fuzzing
security-audit.yml                   — Dependency audit
security-mac-lsm-audit.yml         — MAC/LSM kernel security
security-network-discovery-audit.yml — Network security audit
security-pledge-unveil-sandbox.yml  — OpenBSD pledge/unveil CI
security-root-doas-pam-audit.yml    — Privilege escalation audit
sigma_master_ci.yml                  — Master Rust test suite
sovereign-ai-agent-governance-ci.yml — AI agent rule compliance
sovereign-distro-wiki-pages-deployment.yml — Wiki deployment
sovereign-github-pages-publishing.yml — GitHub Pages
sovereign-nightly-release-automation.yml — Nightly builds
sovereign-universal-package-formats-ci.yml — Universal pkg formats
wiki-sync.yml                        — Wiki synchronization
```

## Code Quality Guidelines Enforced

Per AGENTS.md zero-dependency invariant:
1. Kernel core code (`src/kernel/`, `src/klib/`, `src/distro/`) MUST use `alloc::` not `std::`
2. Test and standalone_test feature gates use `std::` for hosted test environment
3. No external crates in `Cargo.toml` kernel dependencies
4. All ring buffers must have power-of-two capacity

## See Also
- [AGENTS.md](../AGENTS.md) — AI agent coding guidelines
- [SECURITY.md](../SECURITY.md) — Security policy
- [DEVELOPER_RULES.md](../DEVELOPER_RULES.md) — Developer rules

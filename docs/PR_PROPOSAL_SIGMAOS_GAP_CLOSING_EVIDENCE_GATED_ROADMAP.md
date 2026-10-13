# PR Proposal: Distro-Style Evidence-Gated Gap-Closing Roadmap and Release Quality Framework

**PR Title:** `feat(roadmap): Implement Distro-Style Evidence-Gated Gap-Closing Roadmap and Quality Gates`
**PR Branch:** `feature/evidence-gated-gap-closing-roadmap`
**PR Category:** Distro Engineering, Release Quality, Project Governance
**Status:** Open / Under Review

---

## Pull Request Summary

This PR establishes a **distro-style, evidence-gated gap-closing model** for SigmaOS to bridge the distance between documented ambition and reproducible, runtime-verified OS behavior.

SigmaOS's central challenge is closing the gap between aspirational designs and verified runtime execution. Under this framework, component claims must be backed by reproducible test evidence in QEMU x86_64 or named physical hardware.

---

## Key Changes Included in PR

### 1. Distro Practice Matrix Application

| Distro/BSD practice | SigmaOS gap-closing application |
|---|---|
| **Debian** installer first | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| **Arch** reproducible build discipline | Every artifact must be rebuildable from a locked toolchain and commit |
| **FreeBSD** release engineering | Maintain a release checklist, known issues, and rollback path |
| **OpenBSD** least privilege | Security boundaries are reviewed separately and never bypassed for speed |
| **Fedora** CI/gating | CI must block regressions, not merely record them |
| **NixOS** declarative configuration | Make system configuration, tests, and build inputs explicit |
| **Linux** hardware enablement | Maintain a support matrix with logs, dates, and tested device models |

---

### 2. Gap Inventory & Priority Gap Classes

| Gap class | Current signal | Closure outcome |
|---|---|---|
| **Boot and installation** | QEMU x86_64 boot success is 0%; UEFI/BIOS marked "not started" | Reproducible boot-to-shell and clean-install path |
| **Runtime integration** | Source modules are not proof of working runtime features | End-to-end subsystem tests connected to boot |
| **Capability truth** | Many plans, broad status vocabulary, historical documents | Evidence-backed component status |
| **Hardware enablement** | No real hardware tested; no graphics, Wi-Fi, audio, USB, or suspend support | Tiered device support with test logs |
| **Packaging and updates** | Packaging/update recovery remains a stated priority | Safe install, update, rollback, and recovery workflow |
| **Security** | Security is a core component but requires separate assurance | Threat model, capability tests, and review gates |
| **Documentation sprawl** | 165 wiki pages and many historical planning documents | Canonical component pages and archived plans |

---

### 3. Gap-Closing Operating Loop

```text
IDENTIFY GAP
  → CLASSIFY severity and user impact
  → DEFINE MINIMUM PROOF
  → CREATE VERTICAL SLICE
  → IMPLEMENT
  → TEST UNIT + INTEGRATION + RUNTIME
  → UPDATE CANONICAL COMPONENT PAGE
  → RELEASE GATE
  → MONITOR REGRESSIONS
```

A gap is closed only when its implementation, tests, documentation, runtime evidence, and release status agree.

---

### 4. Gap Severity Model (S0–S3)

| Severity | Definition | Examples | Closure target |
|---|---|---|---|
| **S0 — Blocking** | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| **S1 — Core parity** | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| **S2 — Distro usability** | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| **S3 — Enhancement** | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

---

### 5. Minimum Proof Standard (10-Point Definition of Done)

A gap may be marked **closed** only when all 10 of the following criteria are met:

1. **Implementation exists** in the canonical source path.
2. **Unit tests pass** for isolated logic.
3. **Integration tests pass** across module boundaries.
4. **Runtime evidence exists** from QEMU or named hardware.
5. **Documentation matches reality**, including limitations.
6. **Regression test exists** to prevent reversal.
7. **Rollback or recovery path is defined** for installer, update, and boot changes.
8. **Security review is complete** for privilege, memory, crypto, sandbox, package, or network changes.
9. **Status is updated** on the component's canonical wiki page, not in an archived plan.
10. **Evidence is linked**, dated, and reproducible.

---

### 6. Canonical Status Vocabulary

| Status | Allowed evidence | Prohibited claim |
|---|---|---|
| `Proposed` | Design document or issue | "Works" or "supported" |
| `Prototype` | Unit test or isolated demo | "Boot-ready" or "production-ready" |
| `Integrated` | Integration test plus runtime path | "Hardware supported" without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | "Universal" compatibility without test corpus |

---

### 7. 7 Core Workstreams

1. **Boot and installation:** Pinned ISO build, UEFI/BIOS boot in QEMU x86_64, serial logging, boot-to-shell, clean install on virtual disk, update rollback test.
2. **Kernel and core runtime:** Boot-to-shell path tracing memory allocation, scheduling, syscalls, VFS, and process execution.
3. **Security and isolation:** Threat model, capability boundaries, security denial tests, parser fuzzing, sandboxed package execution.
4. **Packaging, updates, and recovery:** Package metadata validation, dependency resolution dry-runs, sandboxed installation, atomic update rollback.
5. **Hardware enablement:** QEMU x86_64 Tier 1 stability, virtio devices (`virtio-blk`, `virtio-net`, VGA), device matrix with test logs.
6. **Desktop and userspace:** Display output, input, window creation, application launch, and graceful shutdown contract.
7. **Documentation and project governance:** Canonical component pages, archived plans namespace, verification front-matter blocks, contributor guide.

---

### 8. 12 Industry-Standard Implementation Rules

- **Rule 1: Single source of truth** - Each component has exactly one canonical status page.
- **Rule 2: Evidence-gated promotion** - Promotion requires linked, dated, reproducible evidence.
- **Rule 3: Vertical slices over horizontal promises** - Prefer completing one narrow end-to-end path over partial horizontal implementations.
- **Rule 4: CI is a gate, not a dashboard** - Failing checks block merge.
- **Rule 5: Reproducibility** - Record commit hash, toolchain, commands, exit codes, and artifact hashes.
- **Rule 6: Least privilege** - Agents and CI receive minimal necessary permissions.
- **Rule 7: Security changes require separate review** - Security-sensitive code requires separate security review.
- **Rule 8: Fail safely** - Storage and update operations require rollback paths.
- **Rule 9: No unsupported claims** - Reference-project capabilities cannot be claimed as SigmaOS capabilities.
- **Rule 10: Regression protection** - Every closed gap adds a regression test.
- **Rule 11: Scope discipline** - One PR closes one gap or narrow sub-gap.
- **Rule 12: Human accountability** - Humans remain accountable for approving security, release, and destructive operations.

---

### 9. Gap-Closing Backlog (P0–P3)

- **P0 — Restore credibility:** Machine-readable gap register, claim audit/downgrade, CI gates, QEMU boot validator, "Verified Today" report.
- **P1 — Complete M1:** QEMU UEFI & BIOS boot, boot-to-shell, VGA framebuffer, `virtio-net` networking, clean install on disposable virtual disk, update rollback.
- **P2 — Make OS usable:** End-to-end VFS/process runtime trace, package dependency resolver, sandboxed installer, minimal desktop launch.
- **P3 — Expand support:** QEMU ARM64 target, physical hardware validation program, package compatibility expansion.

---

### 10. Metrics and Release Gates

| Metric | M1 target | Current baseline | Gate |
|---|---:|---:|---|
| QEMU x86_64 boot success | 100% | 0% | Must pass before nightly |
| QEMU ARM64 boot success | 95% | 0% | Future milestone |
| Clean installation success | 95%+ | 0% | Required for validated release |
| Boot-to-shell duration | Defined and stable | Unknown | Regression threshold |
| Capability claims with evidence | 100% | Unknown | Required for status promotion |
| Critical security regression response | Within 1 week for Tier 1 | Not established | Release policy |
| Package update rollback success | 100% | Unknown | Required before update channel |

---

### 11. First 30 Days Timeline

- **Day 1–3:** Create gap register, severity model, evidence policy, and canonical component page template.
- **Day 4–7:** Run repository claim audit and downgrade unsupported claims.
- **Day 8–14:** Implement QEMU x86_64 boot-to-shell validator and capture serial logs.
- **Day 15–21:** Fix primary boot blocker until boot-to-shell is reproducibly solved.
- **Day 22–27:** Implement clean-install and update rollback tests.
- **Day 28–30:** Publish evidence-backed M1 status report and public known-issues list.

---

## Verification and Testing

This PR is verified by unit and integration tests in `src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs` and executed via `./run_sigma_tests.sh`.

- [x] `#![no_std]` Rust implementation compiles cleanly.
- [x] All 10 Definition of Done criteria verified by gateway engine.
- [x] Patch digest verification active (`fnv1a_gap_closing_digest`).
- [x] Full test suite executed and passing.

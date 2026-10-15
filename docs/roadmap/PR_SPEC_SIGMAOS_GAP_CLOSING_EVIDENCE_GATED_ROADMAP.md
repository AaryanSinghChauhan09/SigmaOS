# PR Specification: SigmaOS Gap-Closing Evidence-Gated Roadmap

## Metadata
- **PR Title**: `feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap`
- **PR Branch**: `feature/gap-closing-evidence-gated-roadmap`
- **Status**: Specification Ready / Approved
- **Severity Classification**: S0 to S3
- **Canonical Status Vocabulary**: `Proposed`, `Prototype`, `Integrated`, `Supported`

---

## Executive Summary

SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. The wiki itself states that a listed source module is not proof of a working runtime feature, and its support matrix currently records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

This roadmap establishes a **distro-style, evidence-gated closure model**: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how practical Linux/BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad feature parity.

---

## Gap-Closing Strategy: Distro & BSD Parity Application

| Distro/BSD Practice | SigmaOS Gap-Closing Application |
|---|---|
| **Debian installer first** | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| **Arch reproducible build discipline** | Every artifact must be rebuildable from a locked toolchain and commit |
| **FreeBSD release engineering** | Maintain a release checklist, known issues, and rollback path |
| **OpenBSD least privilege** | Security boundaries are reviewed separately and never bypassed for speed |
| **Fedora CI/gating** | CI must block regressions, not merely record them |
| **NixOS declarative configuration** | Make system configuration, tests, and build inputs explicit |
| **Linux hardware enablement** | Maintain a support matrix with logs, dates, and tested device models |

---

## Current Gap Inventory & Priority Gap Classes

The repository contains extensive component documentation, agent files, roadmaps, and reference-project analyses, but the project’s own wiki warns that imported plans and reference-project capabilities must not be presented as SigmaOS capabilities. The support matrix confirms that the primary QEMU x86_64 target has no boot evidence, while graphics, networking, audio, USB, suspend/resume, and real hardware support are all unstarted.

| Gap Class | Current Signal | Closure Outcome |
|---|---|---|
| **Boot and installation** | QEMU x86_64 boot success is 0%; UEFI/BIOS marked “not started” | Reproducible boot-to-shell and clean-install path |
| **Runtime integration** | Source modules are not proof of working runtime features | End-to-end subsystem tests connected to boot |
| **Capability truth** | Many plans, broad status vocabulary, historical documents | Evidence-backed component status |
| **Hardware enablement** | No real hardware tested; no graphics, Wi-Fi, audio, USB, or suspend support | Tiered device support with test logs |
| **Packaging and updates** | Packaging/update recovery remains a stated priority | Safe install, update, rollback, and recovery workflow |
| **Security** | Security is a core component but requires separate assurance | Threat model, capability tests, and review gates |
| **Documentation sprawl** | 165 wiki pages and many historical planning documents | Canonical component pages and archived plans |

---

## Gap-Closing Operating Loop

Every gap moves through a strict 9-stage controlled lifecycle:

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

## Gap Severity Model

A four-level classification ensures agents and contributors prioritize critical paths:

| Severity | Definition | Examples | Closure Target |
|---|---|---|---|
| **S0 — Blocking** | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| **S1 — Core parity** | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| **S2 — Distro usability** | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| **S3 — Enhancement** | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

---

## Minimum Proof Standard (10-Point Definition of Done)

Borrowing industry-standard definitions of done from release engineering and safety-critical development, a gap may be marked **closed** only when all 10 criteria are met:

1. **Implementation exists** in the canonical source path.
2. **Unit tests pass** for isolated logic.
3. **Integration tests pass** across module boundaries.
4. **Runtime evidence exists** from QEMU or named hardware.
5. **Documentation matches reality**, including limitations.
6. **Regression test exists** to prevent reversal.
7. **Rollback or recovery path is defined** for installer, update, and boot changes.
8. **Security review is complete** for privilege, memory, crypto, sandbox, package, or network changes.
9. **Status is updated** on the component’s canonical wiki page, not in an archived plan.
10. **Evidence is linked**, dated, and reproducible.

---

## Canonical Status Vocabulary

| Status | Allowed Evidence | Prohibited Claim |
|---|---|---|
| `Proposed` | Design document or issue | “Works” or “supported” |
| `Prototype` | Unit test or isolated demo | “Boot-ready” or “production-ready” |
| `Integrated` | Integration test plus runtime path | “Hardware supported” without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | “Universal” compatibility without test corpus |

---

## Workstreams

### 1. Boot and Installation
- **Gap**: QEMU x86_64 target has 0% boot evidence.
- **Closure Plan**: Build clean ISO from pinned commit and toolchain; test UEFI and BIOS boot separately; capture serial console & kernel log; define minimum boot contract; add boot-to-shell smoke suite & clean-install test.
- **Exit Criterion**: 100% boot success across supported QEMU x86_64 CI images.

### 2. Kernel and Core Runtime
- **Gap**: Kernel, memory, scheduler, filesystem/VFS, IPC, syscalls, init, drivers lack runtime integration proof.
- **Closure Plan**: Select minimum viable kernel path; write integration test per subsystem; create runtime dependency map; treat un-reachable subsystems as `Prototype`; add fault-injection tests.
- **Exit Criterion**: Boot-to-shell run exercises every claimed core subsystem and produces execution trace.

### 3. Security and Isolation
- **Gap**: Security components require explicit threat model and capability enforcement proof.
- **Closure Plan**: Threat model for untrusted packages/binaries/network/inputs; define capability boundaries; add negative denial tests; conduct separate security reviews; parser fuzzing.
- **Exit Criterion**: Documented threat model, capability matrix, and passing denial tests for every boundary.

### 4. Packaging, Updates, and Recovery
- **Gap**: Package installation and update recovery need atomic transaction and rollback verification.
- **Closure Plan**: Phase 1 metadata parse -> Phase 2 dep resolve/dry-run -> Phase 3 isolated test image -> Phase 4 sandboxed script execution -> Phase 5 atomic update/rollback tests -> Phase 6 compatibility matrix.
- **Exit Criterion**: Failed update must complete atomically or restore prior known-good state proven by test.

### 5. Hardware Enablement
- **Gap**: No real hardware tested; missing GPU acceleration, Wi-Fi, audio, USB, suspend/resume.
- **Closure Plan**: Tier 1 QEMU x86_64 -> Tier 2 QEMU ARM64 -> device test matrix with exact models and logs -> start with virtio -> physical hardware after QEMU parity.
- **Exit Criterion**: Every support claim has linked test logs; unsupported features remain explicitly listed as limitations.

### 6. Desktop and Userspace
- **Gap**: Desktop compositor and apps must not precede reliable boot and core runtime.
- **Closure Plan**: Define minimal desktop contract (display, input, window, launch, shutdown); automated UI smoke tests; crash reporting & diagnostics.
- **Exit Criterion**: QEMU boot reaches desktop, launches app, accepts input, and shuts down cleanly.

### 7. Documentation and Project Governance
- **Gap**: 165 wiki pages and historical planning docs create confusion over current status.
- **Closure Plan**: Declare component wiki pages canonical; move historical plans to archive namespace; add front-matter verification blocks; contributor guide.
- **Exit Criterion**: Every active component page has current status and evidence links; no feature claim exists outside canonical system.

---

## 12 Industry-Standard Rules

1. **Rule 1: Single source of truth** - Component canonical wiki page is the sole status authority.
2. **Rule 2: Evidence-gated promotion** - No status bump without linked, dated, reproducible evidence.
3. **Rule 3: Vertical slices over horizontal promises** - Complete end-to-end paths before broadening scope.
4. **Rule 4: CI is a gate, not a dashboard** - Failing required checks blocks merge.
5. **Rule 5: Reproducibility** - Record commit, toolchain, container image, commands, timestamps, and hashes.
6. **Rule 6: Least privilege** - CI and agents receive minimal required permissions.
7. **Rule 7: Security changes require separate review** - Dedicated security review for privileges/crypto/sandboxing.
8. **Rule 8: Fail safely** - Installer, update, and storage changes must define rollback path before merge.
9. **Rule 9: No unsupported claims** - Do not present reference-project capabilities as SigmaOS capabilities.
10. **Rule 10: Regression protection** - Every closed gap must add an automated regression test.
11. **Rule 11: Scope discipline** - One PR closes one gap or sub-gap; large changes require ADR.
12. **Rule 12: Human accountability** - Humans approve security, release, hardware, and destructive operations.

---

## Gap-Closing Backlog (P0–P3)

### P0 — Restore Credibility
1. Create machine-readable gap register.
2. Audit all component pages and downgrade unevidenced claims.
3. Add CI gates for clean build, unit tests, formatting, and test scripts.
4. Create QEMU x86_64 boot-to-shell validator.
5. Publish "Verified Today" report generated from actual test evidence.

### P1 — Complete M1
6. Prove UEFI boot in QEMU.
7. Prove BIOS boot in QEMU.
8. Prove boot-to-shell with serial logs.
9. Add VGA framebuffer smoke test.
10. Add basic `virtio-net` networking test.
11. Add clean-install test on disposable virtual disk.
12. Add update rollback and recovery test.

### P2 — Make the OS Usable
13. Connect filesystem/VFS, process creation, syscalls, and shell into runtime trace.
14. Add package metadata parsing and dependency resolution.
15. Add sandboxed package installation.
16. Add minimal desktop launch and input smoke tests.
17. Add diagnostics, crash logs, and known-issue report.

### P3 — Expand Support
18. Add ARM64 QEMU after x86_64 meets target.
19. Add real-hardware validation program with named devices and logs.
20. Expand package compatibility after metadata, dependency, sandbox, and rollback tests are stable.

---

## Metrics and Release Gates

| Metric | M1 Target | Current Baseline | Gate |
|---|---:|---:|---|
| **QEMU x86_64 boot success** | 100% | 0% | Must pass before nightly |
| **QEMU ARM64 boot success** | 95% | 0% | Future milestone |
| **Clean installation success** | 95%+ | 0% | Required for validated release |
| **Boot-to-shell duration** | Defined and stable | Unknown | Regression threshold |
| **Capability claims with evidence** | 100% | Unknown | Required for status promotion |
| **Critical security regression response** | Within 1 week | Not established | Release policy |
| **Package update rollback success** | 100% | Unknown | Required before update channel |

---

## First 30 Days Execution Calendar

- **Days 1–3**: Create gap register, severity model, evidence policy, and canonical component page template.
- **Days 4–7**: Run full audit and downgrade unsupported claims.
- **Days 8–14**: Implement QEMU x86_64 boot validator and retain serial logs.
- **Days 15–21**: Fix first boot blocker; do not expand scope until boot-to-shell is reproducible.
- **Days 22–27**: Add clean-install and rollback tests.
- **Days 28–30**: Publish first evidence-backed M1 status report and public known-issues list.

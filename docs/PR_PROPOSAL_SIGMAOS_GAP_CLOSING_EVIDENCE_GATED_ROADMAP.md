# Pull Request Proposal: Distro-Style Evidence-Gated Gap-Closing Roadmap for SigmaOS

**PR Title:** `docs(roadmap): Adopt Distro-Style Evidence-Gated Gap-Closing Roadmap`
**PR Branch:** `feature/evidence-gated-gap-closing-roadmap`
**Target Branch:** `main`
**Status:** Proposal / Specification Ready
**Component Scope:** Kernel, Drivers, Userspace, Shell, Security, Packaging, Desktop, Installer, Documentation, Governance

---

## 1. Executive Summary & Problem Statement

SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. As explicitly recorded in the project wiki, a listed source module is not proof of a working runtime feature, and the current support matrix records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

This proposal establishes a **distro-style, evidence-gated closure model**: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how mature Linux and BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad desktop features.

---

## 2. Distro-Style Gap-Closing Application

| Distro/BSD Practice | SigmaOS Gap-Closing Application |
|---|---|
| **Debian** installer first | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| **Arch** reproducible build discipline | Every artifact must be rebuildable from a locked toolchain and commit |
| **FreeBSD** release engineering | Maintain a release checklist, known issues, and rollback path |
| **OpenBSD** least privilege | Security boundaries are reviewed separately and never bypassed for speed |
| **Fedora** CI/gating | CI must block regressions, not merely record them |
| **NixOS** declarative configuration | Make system configuration, tests, and build inputs explicit |
| **Linux** hardware enablement | Maintain a support matrix with logs, dates, and tested device models |

---

## 3. Current Gap Inventory & Priority Gap Classes

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

## 4. Gap-Closing Operating Loop

Every gap moves through a strict 9-stage lifecycle:

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

## 5. Severity Model

| Severity | Definition | Examples | Closure Target |
|---|---|---|---|
| **S0 — Blocking** | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| **S1 — Core parity** | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| **S2 — Distro usability** | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| **S3 — Enhancement** | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

---

## 6. 10-Point Minimum Proof Standard

A gap may be marked **closed** only when all of the following 10 conditions are met:

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

## 7. Canonical Status Vocabulary

| Status | Allowed Evidence | Prohibited Claim |
|---|---|---|
| `Proposed` | Design document or issue | “Works” or “supported” |
| `Prototype` | Unit test or isolated demo | “Boot-ready” or “production-ready” |
| `Integrated` | Integration test plus runtime path | “Hardware supported” without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | “Universal” compatibility without test corpus |

---

## 8. Detailed Workstreams

### Workstream 1: Boot and Installation
- **Gap:** The M1 target—QEMU x86_64 boot, framebuffer, basic networking, and boot-to-login—has no evidence yet.
- **Closure Plan:**
  - Build a clean ISO artifact from a pinned commit and toolchain.
  - Test UEFI and BIOS boot separately in QEMU.
  - Capture serial console, kernel log, boot duration, and failure stage.
  - Define minimum boot contract: bootloader → kernel init → root filesystem → init/services → shell/login.
  - Add boot-to-shell smoke suite, clean-install test on a disposable virtual disk, and update/rollback tests.
- **Exit Criterion:** 100% boot success across supported QEMU x86_64 CI images, with retained logs and artifact hashes.

### Workstream 2: Kernel and Core Runtime
- **Gap:** Source presence alone does not establish runtime integration across kernel, memory, scheduler, VFS, IPC, syscalls, init, and drivers.
- **Closure Plan:**
  - Select minimum viable kernel path: boot, memory allocation, scheduling, syscalls, VFS, process creation, shell execution.
  - Write integration tests exercising real runtime paths.
  - Add runtime dependency map showing required initializations prior to shell.
  - Treat non-reachable subsystems as `Prototype` regardless of unit tests.
  - Add fault-injection tests (allocation failure, invalid syscalls, malformed paths, driver errors).
- **Exit Criterion:** Boot-to-shell run exercises every claimed core subsystem with initialization traces.

### Workstream 3: Security and Isolation
- **Gap:** Security claims require stronger assurance than ordinary features.
- **Closure Plan:**
  - Create threat model covering untrusted packages, downloaded binaries, network input, local users, compromised processes, and update channels.
  - Define capability boundaries for VFS, network, process, IPC, device, and package scripts.
  - Add negative denial tests: protected operations must fail without required capability.
  - Separate security review from feature review and add fuzzing for parsers and syscalls.
- **Exit Criterion:** Documented threat model, capability matrix, and passing denial tests for every security boundary.

### Workstream 4: Packaging, Updates, and Recovery
- **Gap:** Update/installer recovery remains an unverified near-term priority.
- **Closure Plan:**
  - Phase 1: parse and validate package metadata.
  - Phase 2: resolve dependencies and simulate installation.
  - Phase 3: install into isolated test image.
  - Phase 4: execute package scripts only in sandboxed environment.
  - Phase 5: add atomic update, rollback, and recovery tests.
  - Phase 6: publish package format compatibility matrix.
- **Exit Criterion:** Failed updates complete atomically or restore prior known-good state, proven by automated tests.

### Workstream 5: Hardware Enablement
- **Gap:** No real hardware tested; missing graphics acceleration, Wi-Fi, audio, USB, suspend/resume.
- **Closure Plan:**
  - Maintain QEMU x86_64 as Tier 1 until stable before adding ARM64.
  - Maintain device matrix with model, driver version, test date, result, and log link.
  - Focus first on virtio devices (`virtio-blk`, `virtio-net`, VGA framebuffer, `virtio-gpu`).
  - Require reproducible hardware test logs before claiming physical hardware support.
- **Exit Criterion:** Every support-tier claim backed by current test logs; unsupported features listed as limitations.

### Workstream 6: Desktop and Userspace
- **Gap:** Desktop and GUI components must not precede reliable boot and runtime.
- **Closure Plan:**
  - Define minimal desktop contract: display output, input, window creation, app launch, graceful shutdown.
  - Add automated UI smoke tests after runtime stabilization.
  - Treat software store as packaging frontend; defer advanced desktop features until minimal desktop passes repeated tests.
- **Exit Criterion:** QEMU boot reaches desktop, launches basic application, accepts input, and shuts down cleanly.

### Workstream 7: Documentation and Governance
- **Gap:** 165 wiki pages and historical planning documents create status ambiguity.
- **Closure Plan:**
  - Declare component wiki pages canonical.
  - Move historical plans into archive namespace.
  - Add front-matter block to component pages: status, owner, last verified date, evidence links, limitations, next milestone.
  - Automatically flag stale unverified pages.
- **Exit Criterion:** Every active component page has current status, evidence, and next action; no feature claim exists outside canonical status system.

---

## 9. 12 Industry-Standard Rules

1. **Rule 1: Single source of truth:** Canonical component pages override archived plans or README aspirations.
2. **Rule 2: Evidence-gated promotion:** Capability promotion requires linked, dated, reproducible evidence.
3. **Rule 3: Vertical slices over horizontal promises:** Complete narrow end-to-end paths before adding subsystem breadth.
4. **Rule 4: CI is a gate, not a dashboard:** Failing required checks must block PR merge.
5. **Rule 5: Reproducibility:** Record commit hash, toolchain versions, container image, commands, timestamps, exit codes, and hashes.
6. **Rule 6: Least privilege:** Minimal agent and CI permissions; no broad credentials or unrestricted execution.
7. **Rule 7: Security changes require separate review:** Security-sensitive code requires dedicated security approval.
8. **Rule 8: Fail safely:** Installer, update, boot, and storage changes must define rollback or recovery paths before merge.
9. **Rule 9: No unsupported claims:** Reference-project capabilities cannot be claimed as SigmaOS capabilities without evidence.
10. **Rule 10: Regression protection:** Every closed gap requires an automated regression test or manual procedure.
11. **Rule 11: Scope discipline:** One PR closes one gap or one narrow sub-gap; cross-cutting changes require ADRs.
12. **Rule 12: Human accountability:** Humans approve security, release, hardware-support, and destructive operations.

---

## 10. Gap-Closing Backlog

### P0 — Restore Credibility
1. Create machine-readable gap register (ID, severity, owner, proof required, milestone).
2. Audit component pages and downgrade unevidenced claims.
3. Add CI gates for clean build, unit tests, formatting, and test scripts.
4. Create QEMU x86_64 boot-to-shell validator.
5. Publish "Verified Today" report generated from actual test and boot evidence.

### P1 — Complete M1
6. Prove UEFI boot in QEMU.
7. Prove BIOS boot in QEMU.
8. Prove boot-to-shell with serial logs.
9. Add VGA framebuffer smoke test.
10. Add basic `virtio-net` networking test.
11. Add clean-install test on a disposable virtual disk.
12. Add update rollback and recovery test.

### P2 — Make the OS Usable
13. Connect VFS, process creation, syscalls, and shell into one runtime trace.
14. Add package metadata parsing and dependency resolution.
15. Add sandboxed package installation.
16. Add minimal desktop launch and input smoke tests.
17. Add diagnostics, crash logs, and known-issue report.

### P3 — Expand Support
18. Add ARM64 QEMU after x86_64 target is met.
19. Add real-hardware validation program with named devices and logs.
20. Expand package compatibility after metadata, dependency, sandbox, and rollback tests are stable.

---

## 11. Metrics and Release Gates

| Metric | M1 Target | Current Baseline | Gate |
|---|---:|---:|---|
| **QEMU x86_64 boot success** | 100% | 0% | Must pass before nightly |
| **QEMU ARM64 boot success** | 95% | 0% | Future milestone |
| **Clean installation success** | 95%+ | 0% | Required for validated release |
| **Boot-to-shell duration** | Defined and stable | Unknown | Regression threshold |
| **Capability claims with evidence** | 100% | Unknown | Required for status promotion |
| **Critical security regression response** | Within 1 week for Tier 1 | Not established | Release policy |
| **Package update rollback success** | 100% | Unknown | Required before update channel |

---

## 12. First 30 Days Execution Schedule

- **Days 1–3:** Create gap register, severity model, evidence policy, and canonical component page template.
- **Days 4–7:** Run full audit and downgrade unsupported claims.
- **Days 8–14:** Implement QEMU x86_64 boot validator and retain serial logs.
- **Days 15–21:** Fix first boot blocker; hold scope until boot-to-shell is reproducible.
- **Days 22–27:** Add clean-install and rollback tests.
- **Days 28–30:** Publish first evidence-backed M1 status report and public known-issues list.

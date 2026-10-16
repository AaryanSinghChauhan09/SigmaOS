# PR Proposal: SigmaOS Distro-Style Evidence-Gated Gap-Closing System

## Metadata
- **PR Title**: `feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap`
- **PR Branch**: `feature/gap-closing-evidence-gated-roadmap`
- **Status**: `Proposed / Specification Ready`
- **Severity Classification**: S0 to S3
- **Canonical Status Vocabulary**: `Proposed`, `Prototype`, `Integrated`, `Supported`

---

## Executive Summary

SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. The wiki itself states that a listed source module is not proof of a working runtime feature, and its support matrix currently records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

This proposal introduces a **distro-style, evidence-gated closure model**: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how practical Linux/BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad feature parity.

---

## Gap-Closing Strategy

| Distro/BSD practice | SigmaOS gap-closing application |
|---|---|
| Debian installer first | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| Arch reproducible build discipline | Every artifact must be rebuildable from a locked toolchain and commit |
| FreeBSD release engineering | Maintain a release checklist, known issues, and rollback path |
| OpenBSD least privilege | Security boundaries are reviewed separately and never bypassed for speed |
| Fedora CI/gating | CI must block regressions, not merely record them |
| NixOS declarative configuration | Make system configuration, tests, and build inputs explicit |
| Linux hardware enablement | Maintain a support matrix with logs, dates, and tested device models |

---

## Current Gap Inventory

The repository contains extensive component documentation, agent files, roadmaps, and reference-project analyses, but the project’s own wiki warns that imported plans and reference-project capabilities must not be presented as SigmaOS capabilities. The support matrix confirms that the primary QEMU x86_64 target has no boot evidence, while graphics, networking, audio, USB, suspend/resume, and real hardware support are all unstarted.

### Priority Gap Classes

| Gap class | Current signal | Closure outcome |
|---|---|---|
| Boot and installation | QEMU x86_64 boot success is 0%; UEFI/BIOS marked “not started” | Reproducible boot-to-shell and clean-install path |
| Runtime integration | Source modules are not proof of working runtime features | End-to-end subsystem tests connected to boot |
| Capability truth | Many plans, broad status vocabulary, historical documents | Evidence-backed component status |
| Hardware enablement | No real hardware tested; no graphics, Wi-Fi, audio, USB, or suspend support | Tiered device support with test logs |
| Packaging and updates | Packaging/update recovery remains a stated priority | Safe install, update, rollback, and recovery workflow |
| Security | Security is a core component but requires separate assurance | Threat model, capability tests, and review gates |
| Documentation sprawl | 165 wiki pages and many historical planning documents | Canonical component pages and archived plans |

---

## Gap-Closing Operating Loop

Every gap should move through the same controlled lifecycle:

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

Use a four-level classification so agents and contributors work on the right things.

| Severity | Definition | Examples | Closure target |
|---|---|---|---|
| S0 — Blocking | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| S1 — Core parity | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| S2 — Distro usability | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| S3 — Enhancement | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

The support matrix already identifies the correct S0/S1 sequence: QEMU x86_64 boot, framebuffer graphics, basic networking, and boot-to-login are the first M1 objectives.

---

## Minimum Proof Standard

Borrow an industry-standard “definition of done” from release engineering and safety-critical development.

A gap may be marked **closed** only when all of the following are true:

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

The wiki’s own rule is important: component pages—not older imported plans—are the canonical source for implementation status and validation.

---

## Canonical Status Vocabulary

SigmaOS already proposes `Proposed`, `Prototype`, `Integrated`, and `Supported`; retain those terms but bind each to objective evidence.

| Status | Allowed evidence | Prohibited claim |
|---|---|---|
| `Proposed` | Design document or issue | “Works” or “supported” |
| `Prototype` | Unit test or isolated demo | “Boot-ready” or “production-ready” |
| `Integrated` | Integration test plus runtime path | “Hardware supported” without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | “Universal” compatibility without test corpus |

---

## Workstreams

### 1. Boot and Installation

**Gap:** The M1 target—QEMU x86_64 boot, framebuffer, basic networking, and boot-to-login—has no evidence yet.

**Closure plan**
- Build a clean ISO artifact from a pinned commit and toolchain.
- Test UEFI and BIOS boot separately in QEMU.
- Capture serial console, kernel log, boot duration, and failure stage.
- Define the minimum boot contract: bootloader → kernel init → root filesystem → init/services → shell/login.
- Add a boot-to-shell smoke suite.
- Add a clean-install test using a disposable virtual disk.
- Add update and rollback tests before calling installation complete.

**Exit criterion:** 100% boot success across the supported QEMU x86_64 CI images, with retained logs and artifact hashes.

### 2. Kernel and Core Runtime

**Gap:** Kernel, memory, scheduler, filesystem/VFS, IPC, syscalls, init, and drivers are listed as core components, but source presence alone does not establish runtime integration.

**Closure plan**
- Select the minimum viable kernel path: boot, memory allocation, scheduling, basic syscalls, VFS, process creation, and shell execution.
- For each subsystem, write one integration test that exercises the real runtime path.
- Add a “runtime dependency map” showing which components must initialize before the shell.
- Treat any subsystem not reachable from boot as `Prototype`, regardless of unit-test success.
- Add fault-injection tests for allocation failure, invalid syscalls, malformed paths, and driver initialization errors.

**Exit criterion:** A boot-to-shell run exercises every claimed core subsystem and produces a trace showing its initialization and use.

### 3. Security and Isolation

**Gap:** Security is a first-class component, but security claims require stronger assurance than ordinary features.

**Closure plan**
- Create a threat model covering untrusted packages, downloaded binaries, network input, local users, compromised processes, and update channels.
- Define capability boundaries for filesystem, network, process, IPC, device, and package-script execution.
- Add negative tests: every protected operation must fail without the required capability.
- Separate security review from feature review.
- Use fuzzing for parsers, syscall handlers, IPC, filesystem paths, and package metadata.
- Never allow package scripts or third-party binaries to execute with broad host access.

**Exit criterion:** A documented threat model, capability matrix, and passing denial tests for every security boundary.

### 4. Packaging, Updates, and Recovery

**Gap:** Packaging and updates are core components, and update/installer recovery is an explicit near-term priority.

**Closure plan**
- Phase 1: parse and validate package metadata.
- Phase 2: resolve dependencies and simulate installation.
- Phase 3: install into an isolated test image.
- Phase 4: execute package scripts only in a sandboxed environment.
- Phase 5: add atomic update, rollback, and recovery tests.
- Phase 6: publish a compatibility matrix for package formats and features.

**Exit criterion:** A failed update must either complete atomically or restore the prior known-good state; the result must be proven by an automated test.

### 5. Hardware Enablement

**Gap:** The support matrix records no real hardware testing and identifies missing graphics acceleration, Wi-Fi, audio, USB, and suspend/resume support.

**Closure plan**
- Keep QEMU x86_64 as Tier 1 until it is stable.
- Add ARM64 QEMU only after x86_64 reaches its boot-success target.
- Maintain a device matrix with exact model, kernel/driver version, test date, pass/fail result, and log link.
- Start with virtio devices: `virtio-blk`, `virtio-net`, VGA framebuffer, and optionally `virtio-gpu`.
- Add real hardware only after QEMU parity is demonstrated.
- Do not claim Wi-Fi, audio, USB, or suspend support until each has a reproducible hardware test.

**Exit criterion:** Every support-tier claim has a current test result; unsupported features remain explicitly listed as limitations.

### 6. Desktop and Userspace

**Gap:** Desktop, compositor, software store, command palette, diagnostics, and crash reporting are listed as core components, but they should not precede a reliable boot and core runtime.

**Closure plan**
- Define a minimal desktop contract: display output, input, window creation, application launch, and graceful shutdown.
- Add automated UI smoke tests only after the underlying runtime is stable.
- Treat the software store as a packaging frontend, not an independent feature stream.
- Add crash reporting only after a reliable logging and diagnostics path exists.
- Defer advanced desktop features until the minimal desktop passes repeated QEMU tests.

**Exit criterion:** A QEMU boot reaches the desktop, launches a basic application, accepts input, and shuts down cleanly.

### 7. Documentation and Project Governance

**Gap:** The wiki has 165 pages and many historical or imported planning documents, creating a risk that outdated plans are mistaken for current status.

**Closure plan**
- Declare each component wiki page canonical.
- Move historical plans into an archive namespace.
- Add a front-matter block to every component page: status, owner, last verified date, evidence links, known limitations, and next milestone.
- Automatically flag pages with no verification update in the current release cycle.
- Replace duplicated roadmap documents with links to canonical component pages.
- Add a contributor guide that distinguishes “implement,” “integrate,” “validate,” and “support.”

**Exit criterion:** Every active component page has current status, evidence, and a next action; no feature claim exists outside the canonical status system.

---

## Industry-Standard Rules

### Rule 1: Single source of truth
Each component has exactly one canonical status page. Archived plans, README aspirations, and imported analyses may inform work but cannot override measured status.

### Rule 2: Evidence-gated promotion
A component cannot move from `Prototype` to `Integrated`, or from `Integrated` to `Supported`, without linked, dated, reproducible evidence.

### Rule 3: Vertical slices over horizontal promises
Prefer completing one narrow end-to-end path—such as QEMU UEFI boot to shell—over partially implementing many unrelated subsystems.

### Rule 4: CI is a gate, not a dashboard
Failing required checks must block merge. CI should run build, tests, lint/format, boot smoke tests, and security-sensitive checks.

### Rule 5: Reproducibility
Every build and test must record commit hash, toolchain versions, container image, commands, timestamps, exit codes, and artifact hashes.

### Rule 6: Least privilege
Agents and CI jobs receive only the permissions needed for their task. No agent receives broad credentials, force-push rights, or unrestricted package-execution privileges.

### Rule 7: Security changes require separate review
Capability enforcement, sandboxing, memory safety, cryptography, syscall validation, package execution, networking, and update mechanisms require a dedicated security review.

### Rule 8: Fail safely
Installer, update, boot, and storage changes must define a rollback or recovery path before merge. A failed operation must not leave the system in an unbootable or unrecoverable state.

### Rule 9: No unsupported claims
Do not describe reference-project capabilities as SigmaOS capabilities. The wiki explicitly prohibits presenting reference-project features as SigmaOS features.

### Rule 10: Regression protection
Every closed gap must add a regression test. If a bug cannot be reproduced by an automated test, document why and add a manual validation procedure.

### Rule 11: Scope discipline
One PR closes one gap or one narrowly defined sub-gap. Large cross-cutting changes require an architecture decision record and staged rollout.

### Rule 12: Human accountability
Agents may propose, implement, test, and report; humans remain accountable for approving security, release, hardware-support, and destructive operations.

---

## Gap-Closing Backlog

### P0 — Restore Credibility
1. Create a machine-readable gap register with ID, severity, owner, proof required, and target milestone.
2. Audit all component pages and downgrade unevidenced claims.
3. Add CI gates for clean build, unit tests, formatting, and test scripts.
4. Create a QEMU x86_64 boot-to-shell validator.
5. Publish a “Verified Today” report generated from actual test and boot evidence.

### P1 — Complete M1
6. Prove UEFI boot in QEMU.
7. Prove BIOS boot in QEMU.
8. Prove boot-to-shell with serial logs.
9. Add VGA framebuffer smoke test.
10. Add basic `virtio-net` networking test.
11. Add clean-install test on a disposable virtual disk.
12. Add update rollback and recovery test.

### P2 — Make the OS Usable
13. Connect filesystem/VFS, process creation, syscalls, and shell into one runtime trace.
14. Add package metadata parsing and dependency resolution.
15. Add sandboxed package installation.
16. Add minimal desktop launch and input smoke tests.
17. Add diagnostics, crash logs, and a known-issue report.

### P3 — Expand Support
18. Add ARM64 QEMU after x86_64 meets its target.
19. Add a small real-hardware validation program with named devices and logs.
20. Expand package compatibility only after metadata, dependency, sandbox, and rollback tests are stable.

---

## Metrics and Release Gates

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

## First 30 Days

1. **Day 1–3:** Create the gap register, severity model, evidence policy, and canonical component-page template.
2. **Day 4–7:** Run a full audit and downgrade unsupported claims.
3. **Day 8–14:** Implement the QEMU x86_64 boot validator and retain every serial log.
4. **Day 15–21:** Fix the first boot blocker; do not expand scope until boot-to-shell is reproducible.
5. **Day 22–27:** Add clean-install and rollback tests.
6. **Day 28–30:** Publish the first evidence-backed M1 status report and a public known-issues list.

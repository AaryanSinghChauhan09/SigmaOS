# PR Proposal: SigmaOS Distro-Style Evidence-Gated Gap-Closing Roadmap Strategy

## Executive Summary

SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. The wiki itself states that a listed source module is not proof of a working runtime feature, and its support matrix currently records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

This proposal establishes a **distro-style, evidence-gated closure model**: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how practical Linux/BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad feature parity.

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

## Gap-Closing Operating Loop

Every gap moves through a controlled lifecycle:

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

## Gap Severity Model

| Severity | Definition | Examples | Closure target |
|---|---|---|---|
| S0 — Blocking | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| S1 — Core parity | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| S2 — Distro usability | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| S3 — Enhancement | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

## Minimum Proof Standard

A gap may be marked **closed** only when all 10 standard criteria are satisfied:

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

## Canonical Status Vocabulary

| Status | Allowed evidence | Prohibited claim |
|---|---|---|
| `Proposed` | Design document or issue | “Works” or “supported” |
| `Prototype` | Unit test or isolated demo | “Boot-ready” or “production-ready” |
| `Integrated` | Integration test plus runtime path | “Hardware supported” without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | “Universal” compatibility without test corpus |

## Workstreams

### 1. Boot and Installation
- Build clean ISO artifact from pinned commit and toolchain.
- Test UEFI and BIOS boot separately in QEMU.
- Capture serial console, kernel log, boot duration, and failure stage.
- Define minimum boot contract: bootloader → kernel init → root filesystem → init/services → shell/login.
- Exit criterion: 100% boot success across supported QEMU x86_64 CI images.

### 2. Kernel and Core Runtime
- Select minimum viable kernel path.
- Write one integration test exercising real runtime path for each subsystem.
- Add runtime dependency map.
- Exit criterion: Boot-to-shell run exercises every claimed core subsystem with trace.

### 3. Security and Isolation
- Threat model covering untrusted packages, binaries, network input, local users.
- Define capability boundaries.
- Add negative denial tests.
- Fuzzing for parsers, syscall handlers, IPC, package metadata.
- Exit criterion: Documented threat model, capability matrix, and passing denial tests.

### 4. Packaging, Updates, and Recovery
- Phase 1: parse/validate package metadata.
- Phase 2: resolve dependencies and dry-run.
- Phase 3: install into isolated test image.
- Phase 4: sandboxed script execution.
- Phase 5: atomic update, rollback, recovery.
- Exit criterion: Failed update completes atomically or restores prior known-good state.

### 5. Hardware Enablement
- Keep QEMU x86_64 as Tier 1.
- ARM64 QEMU after x86_64 target met.
- Maintain device matrix with models, test date, pass/fail, log link.
- Virtio devices first (`virtio-blk`, `virtio-net`, VGA framebuffer).
- Exit criterion: Every support-tier claim has linked test logs.

### 6. Desktop and Userspace
- Minimal desktop contract: display output, input, window creation, app launch, clean shutdown.
- Automated UI smoke tests after runtime stability.
- Exit criterion: QEMU boot reaches desktop, launches app, accepts input, shuts down cleanly.

### 7. Documentation and Governance
- Component wiki pages canonical.
- Historical plans moved to archive namespace.
- Front-matter verification blocks on every page.
- Exit criterion: Every active component page has current status, evidence, and next action.

## 12 Industry-Standard Rules

1. **Single source of truth:** Canonical component pages override aspirations.
2. **Evidence-gated promotion:** Status changes require linked, dated, reproducible evidence.
3. **Vertical slices over horizontal promises:** Complete one end-to-end path over many partial ones.
4. **CI is a gate, not a dashboard:** Block merge on failure.
5. **Reproducibility:** Record commit, toolchain, container, timestamps, hashes.
6. **Least privilege:** Restricted agent/CI permissions.
7. **Security changes require separate review:** Dedicated security review required.
8. **Fail safely:** Define rollback or recovery paths before merge.
9. **No unsupported claims:** Reference project features are not OS features until proven.
10. **Regression protection:** Every closed gap adds a regression test.
11. **Scope discipline:** One PR closes one gap or narrow sub-gap.
12. **Human accountability:** Humans approve security, releases, and destructive operations.

## Backlog Priority (P0-P3)

- **P0:** Gap register, audit component pages, CI gates, QEMU boot validator, "Verified Today" report.
- **P1:** Prove UEFI/BIOS boot, serial boot-to-shell, VGA smoke test, virtio-net, clean install, rollback recovery.
- **P2:** Runtime trace, package metadata parsing, sandboxed installation, minimal desktop, crash logs.
- **P3:** ARM64 QEMU, real hardware validation, package compatibility expansion.

## First 30 Days Execution Plan

- **Day 1–3:** Create gap register, severity model, evidence policy, template.
- **Day 4–7:** Run audit and downgrade unsupported claims.
- **Day 8–14:** Implement QEMU x86_64 boot validator and retain serial logs.
- **Day 15–21:** Fix first boot blocker; achieve reproducible boot-to-shell.
- **Day 22–27:** Add clean-install and rollback tests.
- **Day 28–30:** Publish first evidence-backed M1 status report and public known-issues list.

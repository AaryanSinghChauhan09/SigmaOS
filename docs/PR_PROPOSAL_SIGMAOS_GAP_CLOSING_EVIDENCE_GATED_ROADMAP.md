# PR Proposal: Distro-Style Evidence-Gated Gap-Closing Roadmap

- **PR Title**: `feat(roadmap): Distro-Style Evidence-Gated Gap-Closing Roadmap`
- **PR Branch**: `feature/gap-closing-evidence-gated-roadmap`
- **Status**: Approved / Specification Ready

## Executive Summary

SigmaOS’s central gap is not a shortage of plans—it is the distance between documented ambition and reproducible, runtime-verified OS behavior. The wiki itself states that a listed source module is not proof of a working runtime feature, and its support matrix currently records 0% QEMU boot success, no tested hardware, and no evidence for the M1 x86_64 boot milestone.

## Gap-Closing Strategy

SigmaOS adopts a **distro-style, evidence-gated closure model**: identify gaps, rank them by user-visible risk, close the smallest complete vertical slice, prove it in QEMU, then expand outward. This mirrors how practical Linux/BSD distributions prioritize a bootable installer, core userspace, package management, and recovery before broad feature parity.

| Distro/BSD Practice | SigmaOS Gap-Closing Application |
|---|---|
| **Debian installer first** | Prove QEMU x86_64 boot-to-shell before desktop ambitions |
| **Arch reproducible build discipline** | Every artifact must be rebuildable from a locked toolchain and commit |
| **FreeBSD release engineering** | Maintain a release checklist, known issues, and rollback path |
| **OpenBSD least privilege** | Security boundaries are reviewed separately and never bypassed for speed |
| **Fedora CI/gating** | CI must block regressions, not merely record them |
| **NixOS declarative configuration** | Make system configuration, tests, and build inputs explicit |
| **Linux hardware enablement** | Maintain a support matrix with logs, dates, and tested device models |

## Current Gap Inventory & Priority Gap Classes

The repository contains extensive component documentation, agent files, roadmaps, and reference-project analyses, but the project’s own wiki warns that imported plans and reference-project capabilities must not be presented as SigmaOS capabilities. The support matrix confirms that the primary QEMU x86_64 target has no boot evidence, while graphics, networking, audio, USB, suspend/resume, and real hardware support are all unstarted.

| Priority Gap Class | Current Signal | Closure Outcome |
|---|---|---|
| **Boot and installation** | QEMU x86_64 boot success is 0%; UEFI/BIOS marked "not started" | Reproducible boot-to-shell and clean-install path |
| **Runtime integration** | Source modules are not proof of working runtime features | End-to-end subsystem tests connected to boot |
| **Capability truth** | Many plans, broad status vocabulary, historical documents | Evidence-backed component status |
| **Hardware enablement** | No real hardware tested; missing graphics, Wi-Fi, audio, USB, suspend | Tiered device support with test logs |
| **Packaging and updates** | Packaging/update recovery remains a stated priority | Safe install, update, rollback, and recovery workflow |
| **Security** | Security is a core component but requires separate assurance | Threat model, capability tests, and review gates |
| **Documentation sprawl** | 165 wiki pages and many historical planning documents | Canonical component pages and archived plans |

## Gap-Closing Operating Loop

Every gap moves through the same controlled lifecycle:

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

| Severity | Definition | Examples | Closure Target |
|---|---|---|---|
| **S0 — Blocking** | Prevents boot, install, update, recovery, or basic use | Kernel panic, broken ISO, failed boot-to-shell | Fix immediately |
| **S1 — Core Parity** | Required for a minimal usable OS | Shell, filesystem, process execution, basic networking | Current milestone |
| **S2 — Distro Usability** | Expected by desktop Linux/BSD users | Installer recovery, package updates, desktop basics | Next release |
| **S3 — Enhancement** | Valuable but not required for core use | Gaming suite, streaming apps, advanced GPU acceleration | Backlog |

## Minimum Proof Standard (10-Point Checklist)

A gap may be marked **closed** only when all of the following 10 criteria are met:

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

| Status | Allowed Evidence | Prohibited Claim |
|---|---|---|
| `Proposed` | Design document or issue | "Works" or "supported" |
| `Prototype` | Unit test or isolated demo | "Boot-ready" or "production-ready" |
| `Integrated` | Integration test plus runtime path | "Hardware supported" without hardware log |
| `Supported` | Reproducible QEMU/hardware test, release checklist, regression coverage | "Universal" compatibility without test corpus |

## Workstreams

1. **Boot and Installation**: Clean ISO build from pinned toolchain, separate UEFI/BIOS QEMU tests, serial console log capture, boot contract definition, boot-to-shell smoke suite, clean-install test, update/rollback test.
2. **Kernel and Core Runtime**: Minimum viable kernel path, subsystem integration tests, runtime dependency map, reachable runtime checks, fault-injection tests.
3. **Security and Isolation**: Comprehensive threat model, capability boundaries, negative denial tests, independent security review, parser & syscall fuzzing.
4. **Packaging, Updates, and Recovery**: Package metadata parsing, dry-run dependency resolution, isolated image installation, sandboxed script execution, atomic updates and rollback verification.
5. **Hardware Enablement**: Tier 1 QEMU x86_64, QEMU ARM64, device support matrix, virtio driver validation, hardware test logs.
6. **Desktop and Userspace**: Minimal desktop contract (display, input, window, launch, shutdown), automated UI smoke tests, crash reporting and diagnostics.
7. **Documentation and Project Governance**: Canonical component pages, archived historical plans, front-matter verification blocks, contributor guide.

## 12 Industry-Standard Rules

1. **Single source of truth**: Each component has exactly one canonical status page.
2. **Evidence-gated promotion**: Status promotion requires linked, dated, reproducible evidence.
3. **Vertical slices over horizontal promises**: Complete narrow end-to-end paths first.
4. **CI is a gate, not a dashboard**: Failing required checks blocks merge.
5. **Reproducibility**: Record commit, toolchain, container image, commands, timestamps, exit codes, and artifact hashes.
6. **Least privilege**: Agents and CI jobs receive minimal necessary permissions.
7. **Security changes require separate review**: Capability enforcement, sandboxing, memory safety, and crypto need dedicated security review.
8. **Fail safely**: Installer, update, boot, and storage changes must define rollback/recovery before merge.
9. **No unsupported claims**: Do not present reference-project capabilities as SigmaOS capabilities.
10. **Regression protection**: Every closed gap adds an automated regression test.
11. **Scope discipline**: One PR closes one gap or one narrowly defined sub-gap.
12. **Human accountability**: Humans remain accountable for security, release, hardware, and destructive operations.

## Gap-Closing Backlog (P0–P3)

- **P0**: Machine-readable gap register, audit component pages, CI gates for clean build/tests/formatting, QEMU x86_64 boot-to-shell validator, "Verified Today" report.
- **P1**: Prove UEFI boot in QEMU, prove BIOS boot in QEMU, prove boot-to-shell with serial logs, VGA framebuffer test, basic virtio-net test, clean-install test, update rollback test.
- **P2**: Connect VFS/process/syscalls/shell runtime trace, package metadata & dependency resolution, sandboxed package installation, minimal desktop smoke test, diagnostics & crash logs.
- **P3**: ARM64 QEMU target, real-hardware validation program, expand package format compatibility.

## Metrics and Release Gates

- **QEMU x86_64 boot success**: 100% target before nightly release.
- **Clean installation success**: 95%+ target required for validated release.
- **Package update rollback success**: 100% required before update channel enablement.
- **Capability claims with evidence**: 100% required for status promotion.

## First 30 Days Execution Calendar

- **Days 1–3**: Create gap register, severity model, evidence policy, and canonical component page template.
- **Days 4–7**: Run full audit and downgrade unevidenced claims.
- **Days 8–14**: Implement QEMU x86_64 boot validator and retain serial logs.
- **Days 15–21**: Fix first boot blocker; maintain boot-to-shell reproducibility.
- **Days 22–27**: Add clean-install and rollback tests.
- **Days 28–30**: Publish evidence-backed M1 status report and public known-issues list.

## Native Rust Engine Implementation

The roadmap specifications, 10-point checklist evaluator, severity classifications, workstreams, backlog, and PR proposal diff generator are natively implemented in Rust in `src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs` and verified in `tests/test_sigmaos_gap_closing_evidence_gated_roadmap.rs`.

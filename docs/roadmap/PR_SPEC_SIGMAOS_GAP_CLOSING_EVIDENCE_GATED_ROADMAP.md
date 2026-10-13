# Specification: SigmaOS Evidence-Gated Gap-Closing Roadmap (PR Specification)

**Specification Title:** `PR_SPEC_SIGMAOS_GAP_CLOSING_EVIDENCE_GATED_ROADMAP`
**Associated PR Branch:** `feature/evidence-gated-gap-closing-roadmap`
**Target Milestone:** M1 (Bootable Core & Release Quality Gates)

---

## 1. Objective

Define the specification, data contracts, status vocabulary, severity taxonomy, 10-point evidence criteria, and release gates for SigmaOS's evidence-gated gap-closing roadmap.

---

## 2. Status Taxonomy

Components in SigmaOS must hold one of four explicit status levels, bound to verified evidence:

1. `Proposed`: Design document or open issue exists. No claims of runtime execution permitted.
2. `Prototype`: Unit tests or isolated bench execution pass. Cannot be claimed as boot-ready or supported.
3. `Integrated`: Integration tests pass and subsystem is connected to the runtime boot path. Requires QEMU trace.
4. `Supported`: Reproducible QEMU/hardware test logs exist, release checklist passed, zero known S0 regressions.

---

## 3. Severity Taxonomy

- `S0 (Blocking)`: Prevents boot, installation, filesystem mount, kernel execution, or system recovery. Target: Fix immediately.
- `S1 (Core Parity)`: Prevents core OS usability (shell, VFS, basic networking, process spawning). Target: Current milestone (M1).
- `S2 (Distro Usability)`: Prevents standard distro user experience (installer GUI, package update, desktop basic app). Target: Next milestone (M2/M3).
- `S3 (Enhancement)`: Non-essential desktop effects, gaming meta tools, or optional device drivers. Target: Backlog.

---

## 4. 10-Point Definition of Done Checklist

Every gap closure pull request must satisfy all 10 points:

1. Canonical source code implementation.
2. Isolated unit tests pass.
3. Cross-module integration tests pass.
4. QEMU or physical hardware runtime log linked.
5. Canonical wiki component page updated with limitations.
6. Automated regression test added.
7. Rollback / recovery mechanism implemented and tested.
8. Security boundary review completed.
9. Verification date and commit hash recorded.
10. Evidence artifact hash stored in `reports/capability/verified_today.md`.

---

## 5. Automated Verification & Patch Integrity

The Rust gateway engine `SigmaOsGapClosingPrGatewayEngine` in `src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs` validates payload integrity via FNV-1a checksums and asserts DoD compliance before allowing status promotion.

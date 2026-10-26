# PR Specification: SigmaOS Distro-Style Evidence-Gated Gap-Closing Roadmap

## Document Overview
This specification details the technical requirements, evidence rules, engine contracts, and implementation structure for the **Distro-Style Evidence-Gated Gap-Closing Roadmap** implemented in `src/distro/sigmaos_gap_closing_evidence_gated_roadmap_pr.rs`.

---

## Engine Data Structures & Contracts

### 1. Gap Severity (`GapSeverity`)
```rust
pub enum GapSeverity {
    S0Blocking,      // "S0 — Blocking", "Fix immediately"
    S1CoreParity,    // "S1 — Core Parity", "Current milestone"
    S2DistroUsability,// "S2 — Distro Usability", "Next release"
    S3Enhancement,   // "S3 — Enhancement", "Backlog"
}
```

### 2. Canonical Status (`CanonicalStatus`)
```rust
pub enum CanonicalStatus {
    Proposed,   // Allowed: "Design document or issue", Prohibited: "Works or supported"
    Prototype,  // Allowed: "Unit test or isolated demo", Prohibited: "Boot-ready or production-ready"
    Integrated, // Allowed: "Integration test plus runtime path", Prohibited: "Hardware supported without log"
    Supported,  // Allowed: "Reproducible QEMU/hardware test", Prohibited: "Universal compatibility"
}
```

### 3. Minimum Proof Checklist (`EvidenceProofChecklist`)
To transition a gap to `Closed`, all 10 fields must evaluate to `true`:
1. `implementation_exists`
2. `unit_tests_pass`
3. `integration_tests_pass`
4. `runtime_evidence_exists`
5. `documentation_matches_reality`
6. `regression_test_exists`
7. `rollback_recovery_path_defined`
8. `security_review_complete`
9. `status_updated_on_canonical_page`
10. `evidence_linked_and_reproducible`

---

## 7 Core Workstreams
1. **Boot and installation** — Target: 100% QEMU x86_64 boot success, serial log retention.
2. **Kernel and core runtime** — Target: Boot-to-shell trace exercising claimed core subsystems.
3. **Security and isolation** — Target: Threat model, capability matrix, passing negative denial tests.
4. **Packaging, updates, and recovery** — Target: Atomic updates with guaranteed rollback on failure.
5. **Hardware enablement** — Target: Linked test logs for every device support claim.
6. **Desktop and userspace** — Target: QEMU boot reaching desktop, launching app, and clean shutdown.
7. **Documentation and project governance** — Target: Canonical component pages with front-matter status.

---

## 12 Industry Rules
1. Single source of truth
2. Evidence-gated promotion
3. Vertical slices over horizontal promises
4. CI is a gate, not a dashboard
5. Reproducibility
6. Least privilege
7. Security changes require separate review
8. Fail safely
9. No unsupported claims
10. Regression protection
11. Scope discipline
12. Human accountability

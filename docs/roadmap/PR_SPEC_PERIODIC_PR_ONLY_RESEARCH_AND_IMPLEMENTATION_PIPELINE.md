# PR Specification: Periodic PR-Only Research-and-Implementation Pipeline

## 1. Governance Specification Overview
This specification defines the execution constraints, state transitions, validation hooks, and automation rules for SigmaOS's Periodic PR-Only Research-and-Implementation Pipeline.

---

## 2. Pipeline Lifecycle & State Machine

```text
[ Study Issue ]
       │
       ▼
[ Research / ADR PR ] ──► (research/upstream/, research/decisions/)
       │
       ▼
[ Implementation PR ] ──► (src/<subsystem>/)
       │
       ▼
[ Test & Evidence PR ] ──► (tests/, reports/)
       │
       ▼
[ Docs & Status PR ] ──► (docs/, CAPABILITY_MATRIX.toml, FEATURE_STATUS.toml)
       │
       ▼
[ Issue Closed with Linked PRs ]
```

---

## 3. Automation Engine Data Structures

The pipeline state is tracked programmatically in `src/governance/periodic_pr_only_research_pipeline_engine.rs`:

```rust
pub enum CadenceLevel { Daily, Weekly, Biweekly, Monthly, Quarterly }
pub enum ComponentPriority { P0, P1, P2, P3 }
pub enum PrType { Research, Implementation, TestAndEvidence, DocumentationAndStatus, ChoreCi, Security }
```

### Required Enforcement Rules
1. **Rule 1 (Strict PR Gate)**: Direct commits to `main` are rejected by CI and repository protection.
2. **Rule 2 (Atomic Outcome)**: Each PR modifies at most one subsystem boundary per implementation cycle.
3. **Rule 3 (Traceability)**: Every feature PR must reference an accepted ADR in `research/decisions/`.
4. **Rule 4 (Evidence Gate)**: Status progression from `Proposed` -> `Prototype` -> `Integrated` -> `Supported` requires linked QEMU / runtime log evidence.
5. **Rule 5 (Validation Compliance)**: All 6 standard checks must pass (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`).

---

## 4. Verification Standard

Every component PR must execute the full verification sequence without errors or warnings:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

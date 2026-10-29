# Specification: Role-Based Operating Plan & Strategic Execution Model

- **Specification Title**: `PR_SPEC_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN`
- **Target File**: `docs/PR_PROPOSAL_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md`
- **Engine File**: `src/governance/role_based_operating_plan_pr.rs`
- **Test Suite**: `tests/test_role_based_operating_plan.rs`

## Overview

This specification formalizes the engine architecture, role models, roadmap phases, step progression, and guardrails required to enforce SigmaOS's role-based operating model and strategic wedge execution plan.

## Engine Interface Specification

The Rust engine in `src/governance/role_based_operating_plan_pr.rs` must provide:

1. **Role Matrix Evaluator**:
   - `AgentRole`: Enum representing 12 maintenance roles (`Maintainer`, `Updater`, `Reviewer`, `Consolidator`, `Researcher`, `Triager`, `TesterFuzzer`, `SecurityOfficer`, `ReleaseManager`, `DocsSteward`, `Benchmarker`, `CommunityManager`).
   - `RoleDefinition`: Struct containing role name, mission, main cadence, and leadership classification (human vs. agent).

2. **Phased Roadmap Progression**:
   - `RoadmapPhase`: Enum representing `Phase0GroundTruth`, `Phase1RealHardwareBoot`, `Phase2UsabilityAndDogfooding`, and `Phase3WinTheWedge`.
   - `RoadmapPhaseProgress`: Tracking completion percentage, blockers, and evidence links.

3. **8-Step Execution Guide**:
   - `ExecutionStep`: Struct tracking step number, name, timeframe, action items, and mandatory output artifact.

4. **Weekly Operational Calendar**:
   - `CalendarDay`: Enum representing days Monday through Friday, with assigned operational focus and role dispatches.

5. **Governance Guardrails Validator**:
   - `GovernanceGuardrail`: Enum representing the 7 mandatory guardrails.
   - `GuardrailValidationVerdict`: Validating whether proposed changes adhere to signoff requirements, evidence standards, sandboxing, and regression ratchets.

## Acceptance Criteria

1. All 12 maintenance roles are enumerated and validated.
2. Roadmap phases track evidence links and completion status.
3. 8 execution steps produce their defined output artifacts (`VISION.md`, `FEATURE_STATUS.toml`, HCL, etc.).
4. Weekly operational calendar dispatches tasks according to scheduled role assignments.
5. 7 governance guardrails enforce human signoffs and evidence gates.
6. Unit tests pass with 100% coverage across all governance engine features.

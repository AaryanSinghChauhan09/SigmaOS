# Specification: Study-to-Implementation Pipeline Engine

## Overview

This specification details the engine architecture and runtime requirements for SigmaOS's **Study → Extract Pattern → Implement Minimal Slice → Validate at Runtime** methodology.

---

## Data Model & Pipeline Schema

### 1. Upstream Study Item
- `project_name`: String (e.g. `"xv6"`, `"linux"`, `"freebsd"`, `"openbsd"`, `"arch"`, `"debian"`, `"fedora"`, `"nixos"`)
- `concept_name`: String (e.g. `"boot"`, `"scheduler"`, `"vfs"`, `"pledge"`, `"packaging"`)
- `upstream_problem`: String
- `data_structure_control_flow`: String
- `security_failure_assumptions`: String
- `adopted_directly`: String
- `simplified_for_rust`: String
- `exit_test_criteria`: String
- `licensing_attribution`: String

### 2. Architecture Decision Record (ADR)
- `adr_id`: String (e.g. `"ADR-001"`, `"ADR-002"`, `"ADR-003"`, `"ADR-004"`, `"ADR-005"`)
- `title`: String
- `status`: AdrStatus (`Proposed`, `Accepted`, `Deprecated`, `Superseded`)
- `context`: String
- `upstream_study_ref`: String
- `decision`: String
- `consequences`: String
- `validation_method`: String

### 3. Comparison Matrix
- `domain`: String (e.g. `"scheduler"`, `"filesystem"`, `"security"`, `"packaging"`, `"hardware-support"`)
- `comparisons`: Vector of OS mechanism records
- `tradeoff_analysis`: String

### 4. Curriculum Phase
- `phase_id`: usize (1 to 5)
- `name`: String
- `duration_weeks`: usize
- `study_focus`: String
- `deliverables`: String
- `exit_test_status`: ExitTestStatus (`Pending`, `InProgress`, `Passed`)

---

## Pipeline Execution Rules Engine

The pipeline engine validates adherence to the 12 core rules:

1. `Rule 1`: Single Concept Scope per Study Cycle.
2. `Rule 2`: Invariant Extraction (no raw code copy-paste).
3. `Rule 3`: License Compatibility Verification (MIT/GPL-2.0).
4. `Rule 4`: Minimal Connected Vertical Slice (boot → shell).
5. `Rule 5`: Mandatory Runtime Boot / QEMU Validation.
6. `Rule 6`: Mandatory CI Gating Gates (`run_sigma_tests.sh`).
7. `Rule 7`: Dedicated Security Review for Isolation Changes.
8. `Rule 8`: Experiment Reproducibility Tracking.
9. `Rule 9`: Least-Privilege Execution Guarantees.
10. `Rule 10`: Explicit Adoption / Rejection Boundaries.
11. `Rule 11`: Mandatory Behavioral Regression Tests.
12. `Rule 12`: Human Sign-Off for High-Risk Subsystems.

---

## Gate Verification Methods

The engine provides runtime verification methods:
- `validate_10_step_cycle()`: Ensures all 10 steps are satisfied.
- `evaluate_curriculum_progress()`: Computes completion percentage across 5 curriculum phases.
- `generate_pr_gateway_manifest()`: Formats PR proposal report for pull request submission.

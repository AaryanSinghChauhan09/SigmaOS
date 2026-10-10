# Weekly Upstream Study Runbook

This runbook outlines the weekly cycle for studying upstream OS mechanisms and converting research into reviewable, test-backed SigmaOS pull requests.

## Weekly Cadence Steps

1. **Step 1: Open Study Issue**
   - Create a study issue using `.github/ISSUE_TEMPLATE/study_issue.yml`.
   - Specify the target upstream project, source commit/version, and narrow mechanism to study (e.g., Linux VFS path resolution, xv6 page frame allocation).

2. **Step 2: Create Research PR**
   - Create branch `study/<component>-<topic>-<id>`.
   - Add research note in `research/upstream/<project>/<topic>.md`.
   - Add Architecture Decision Record (ADR) in `research/decisions/ADR-<id>-<topic>.md`.
   - Add comparison analysis in `research/comparisons/<component>.md`.

3. **Step 3: Create Implementation PR**
   - Create branch `feat/<component>-<capability>-<id>`.
   - Implement minimum connected component slice under module boundaries.
   - Include unit and integration tests.

4. **Step 4: Create Test and Evidence PR**
   - Create branch `test/<component>-<scenario>-<id>`.
   - Execute QEMU / runtime tests and output structured evidence to `reports/`.

5. **Step 5: Documentation & Capability Status Update**
   - Update `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` only when supported by passing test and runtime evidence.

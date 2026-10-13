# Runbook: Weekly Component Improvement Cycle

## Purpose
Execute 7-day loop to select one component, study an upstream reference, write an ADR, implement a minimal safe change, and prove improvement via runtime evidence.

## Daily Workflow
- **Day 1:** Select component and narrow gap.
- **Day 2:** Study upstream in Linux, BSD, or reference OS.
- **Day 3:** Write ADR in `research/decisions/`.
- **Day 4-5:** Implement smallest safe change in Rust.
- **Day 6:** Run CI checks and runtime test.
- **Day 7:** Publish report in `research/reports/weekly/` and promote maturity level if evidence passes.

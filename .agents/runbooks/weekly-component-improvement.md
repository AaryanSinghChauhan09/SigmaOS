# Weekly Component Improvement Runbook

## Objective
Execute the 7-day component study, improvement, and evidence recording cycle for a single targeted SigmaOS component.

## Frequency
Weekly

## Steps
1. **Day 1: Select Target Component & Gap**
   Choose one component and a narrow feature/fix gap from `CAPABILITY_MATRIX.toml`.

2. **Day 2: Upstream Reference Study**
   Study upstream implementation in Linux, FreeBSD, OpenBSD, Arch, or Redox. Record architecture notes in `research/upstream/`.

3. **Day 3: Write Architecture Decision Record (ADR)**
   Create ADR in `research/decisions/ADR-XXX-<name>.md` detailing scope, non-goals, and validation gates.

4. **Days 4–5: Safe Implementation**
   Implement minimal Rust changes adhering to safe Rust and `#![no_std]` bounds where appropriate.

5. **Day 6: Validation**
   Run unit tests, integration tests, and QEMU/runtime validation:
   ```bash
   cargo check --lib
   cargo test --lib
   ./run_sigma_tests.sh
   ```

6. **Day 7: Report & Promotion**
   Write report in `reports/weekly/`, record test evidence log, and update `CAPABILITY_MATRIX.toml` component status if runtime criteria pass.

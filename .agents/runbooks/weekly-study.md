# Weekly Study Runbook

## Weekly Execution Flow

1. **Day 1: Topic Selection**
   - Select one narrow mechanism from the curriculum or gap list.
   - Instantiate `.agents/tasks/open/weekly-template.toml`.

2. **Day 2: Upstream Research**
   - Read upstream source code and documentation.
   - Record findings in `research/upstream/<project>/<topic>.md`.

3. **Day 3: Architecture Decision Record**
   - Draft `research/decisions/ADR-XXX-<topic>.md`.
   - Update comparison table in `research/comparisons/<component>.md`.

4. **Days 4–5: Minimum Connected Slice**
   - Implement the smallest slice under module boundaries.
   - Add unit tests and integration tests.

5. **Days 6–7: Verification & Publishing**
   - Execute verification: `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`.
   - Run QEMU runtime test.
   - Publish weekly report under `research/reports/weekly/`.

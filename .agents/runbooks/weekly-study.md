# Weekly Study Runbook — SigmaOS

## Objective
Run the 5-stage weekly study cycle to study one narrow OS mechanism, convert it into an ADR under `research/decisions/`, implement the smallest connected vertical slice, and validate it in QEMU/runtime tests.

## Workflow
1. **Day 1: Select mechanism:** Choose one narrowly bounded topic (e.g. Linux scheduler run queues, VFS path resolution, pledge/unveil). Create task `.agents/tasks/open/STUDY-YYYY-Wxx.toml`.
2. **Day 2: Study upstream:** Analyze upstream code and docs. Save note to `research/upstream/<system>/<topic>.md`.
3. **Day 3: Write ADR:** Write decision record `research/decisions/ADR-xxx-<topic>.md`.
4. **Days 4–5: Implement slice:** Implement minimal vertical slice in codebase.
5. **Day 6: Validate:**
   ```bash
   cargo check --lib
   cargo test --lib
   ./run_sigma_tests.sh
   make check
   make test
   make format
   ```
6. **Day 7: Publish:** Save weekly study report to `research/reports/weekly/`. Update task status to done.

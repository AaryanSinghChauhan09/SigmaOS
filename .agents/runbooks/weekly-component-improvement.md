# Weekly Component Improvement Runbook

## Objective
Study one upstream reference mechanism and implement a single, narrowly scoped component improvement with runtime test evidence.

## Steps
1. **Day 1**: Select component domain and narrow gap from `CAPABILITY_MATRIX.toml`.
2. **Day 2**: Study Linux/BSD reference implementation; record data structures, flow, adaptations, and rejections in `research/upstream/`.
3. **Day 3**: Write an Architecture Decision Record (ADR) in `research/decisions/`.
4. **Day 4-5**: Implement smallest safe change in Rust; write unit/integration tests.
5. **Day 6**: Validate with `./run_sigma_tests.sh` and QEMU/runtime test harness.
6. **Day 7**: Publish Weekly Improvement Report in `research/reports/weekly/` and update capability status.

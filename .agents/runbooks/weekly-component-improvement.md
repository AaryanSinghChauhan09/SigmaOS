# Weekly Component Improvement Runbook — SigmaOS

## Objective
Select exactly one SigmaOS component, study upstream reference implementation, write ADR, implement smallest safe change, validate with runtime tests, and update capability status.

## Workflow Steps
1. **Day 1**: Choose target component and single gap.
2. **Day 2**: Study upstream mechanism (Linux, xv6, FreeBSD, OpenBSD, Arch, Debian, Fedora, Redox). Record in `research/upstream/`.
3. **Day 3**: Write ADR in `research/decisions/ADR-xxx.md`.
4. **Day 4-5**: Implement smallest safe change in Rust.
5. **Day 6**: Validate with `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and QEMU/runtime test.
6. **Day 7**: Publish evidence report in `research/reports/weekly/` and update `CAPABILITY_MATRIX.toml`.

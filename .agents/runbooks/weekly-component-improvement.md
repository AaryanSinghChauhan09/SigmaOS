# Weekly Component Improvement Runbook

## Objective
Study one upstream mechanism and improve exactly one SigmaOS component.

## Cadence
Weekly cycle (7-day cadence).

## Procedure
1. **Day 1**: Select one component and narrow gap.
2. **Day 2**: Study upstream reference (Linux, xv6, FreeBSD, OpenBSD, Arch, Mint, Fedora, Redox).
3. **Day 3**: Write an improvement ADR in `research/decisions/`.
4. **Day 4–5**: Implement smallest safe change in Rust.
5. **Day 6**: Validate with `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and runtime QEMU test.
6. **Day 7**: Publish weekly improvement report in `research/reports/weekly/`.

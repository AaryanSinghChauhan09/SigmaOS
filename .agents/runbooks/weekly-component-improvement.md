# Weekly Component Improvement Runbook

## Purpose
Study one upstream mechanism and implement one narrowly scoped component improvement in SigmaOS.

## Weekly Execution Rhythm
- **Day 1:** Select one component and narrow gap.
- **Day 2:** Study upstream implementation (Linux, FreeBSD, OpenBSD, xv6, Arch, Debian, Fedora, or Redox).
- **Day 3:** Write an Architecture Decision Record (ADR) under `research/decisions/`.
- **Day 4-5:** Implement the smallest safe change.
- **Day 6:** Run unit, integration, and runtime/QEMU tests (`./run_sigma_tests.sh`, `cargo test --lib`, `make check`).
- **Day 7:** Publish weekly improvement report under `research/reports/weekly/` with test and boot evidence.

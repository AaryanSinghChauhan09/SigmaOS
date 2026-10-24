# SigmaOS Kernel Release Policy

## Overview
SigmaOS kernel release management enforces an 8-gate progression (G0 through G7). No kernel release candidate is tagged without passing all release readiness gates.

## Release Readiness Gates (G0 - G7)

| Gate Code | Gate Name | Requirement | Enforcement Mechanism |
|---|---|---|---|
| `G0` | Code Quality | `cargo check --lib`, formatting, and unit tests pass cleanly | CI Pipeline |
| `G1` | Unit & Integration | All standalone integration test suites pass | `./run_sigma_tests.sh` |
| `G2` | Memory Safety | No undocumented `unsafe` blocks, bounded queues, explicit OOM policy | Review & Audit |
| `G3` | Boot Smoke | QEMU boot-to-shell smoke test passes within timeout | `scripts/ci/qemu_boot_smoke.sh` |
| `G4` | Recoverability | Installer dry-run and failure injection recovery rehearsals pass | `installer/tests/` |
| `G5` | Security Baseline | Threat model audit, capability policy checks, and seccomp review complete | Security Audit |
| `G6` | Hardware Evidence | All driver claims backed by test evidence in `HARDWARE_SUPPORT.md` | Hardware Matrix Audit |
| `G7` | Release Checklist | Release checklist verified and known limitations documented | Final Governance Signoff |

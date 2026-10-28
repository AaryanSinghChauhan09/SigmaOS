# SigmaOS Kernel Release Policy & Quality Gates

This document defines the release readiness criteria and release engineering policy for the SigmaOS kernel.

## Kernel Release Readiness Gates

To qualify for a release candidate or production tag, a kernel release candidate must pass all 8 release readiness gates (G0 through G7):

| Gate ID | Gate Name | Requirement | Verification Tool |
|---|---|---|---|
| **G0** | Code Quality & Hygiene | `cargo check --lib`, formatting, zero clippy warnings | `cargo check`, `cargo fmt` |
| **G1** | Unit & Integration | All unit and integration test suites pass with 0 failures | `./run_sigma_tests.sh` |
| **G2** | Memory & Concurrency | No undocumented `unsafe` blocks, bounded queues, lock order defined | Concurrency audit |
| **G3** | Boot Smoke | QEMU boot-to-shell smoke test completes within 5.0s | `scripts/ci/qemu_boot_smoke.sh` |
| **G4** | Recoverability | Installer dry-run and failure-injection recovery tests pass | `installer/tests/` |
| **G5** | Security Baseline | Threat model audit, privilege separation & capability review complete | Security audit |
| **G6** | Hardware Evidence | Driver claims backed by emulator or physical hardware evidence | `docs/platform/HARDWARE_SUPPORT.md` |
| **G7** | Release Notes & Errata | Complete release checklist, changelog, and known errata documented | Release review |

## Gate Evaluation Protocol

Evaluating release readiness is executed via `KernelLifecycleProgramEngine::evaluate_release_gates(&gates)`. A release candidate is marked READY if and only if `evaluate_release_gates` returns `true` (all 8 gates passed).

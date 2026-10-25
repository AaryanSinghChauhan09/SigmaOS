# SigmaOS Kernel Release Policy & Quality Gates

## Release Quality Gates G0 through G7

| Gate | Code | Gate Name | Requirement Description |
|---|---|---|---|
| **G0** | Code Quality | `cargo check --lib`, formatting, unit tests pass cleanly |
| **G1** | Integration | Standalone integration test suites pass |
| **G2** | Memory Safety | No undocumented `unsafe` blocks, bounded queues, explicit OOM policy |
| **G3** | Boot Smoke | QEMU boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`) |
| **G4** | Recoverability | Installer and failure recovery rehearsal passes |
| **G5** | Security Baseline | Privilege, capability, seccomp, and PQC signature review complete |
| **G6** | Hardware Evidence | Hardware driver claims supported by evidence log |
| **G7** | Release Notes | Release checklist completed and known errata documented |

## Enforcement

Release gates are evaluated programmatically via `KernelLifecycleProgramEngine::evaluate_release_gates()`.

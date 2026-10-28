# SigmaOS Kernel Release Policy

## Overview
This policy governs kernel release readiness through 8 sequential release gates (G0 to G7). No release build is published unless all 8 gates pass with 100% compliance.

---

## Release Gates (G0 - G7)

| Gate | Name | Requirement |
|---|---|---|
| **G0** | Code Quality | `cargo check --lib`, `cargo fmt --check`, unit tests pass |
| **G1** | Integration Suites | Standalone integration test suites pass |
| **G2** | Memory & Safety | Zero undocumented unsafe blocks or unbounded queues |
| **G3** | Boot Smoke | QEMU boot-to-shell smoke test passes |
| **G4** | Recoverability | Installer dry-run & failure recovery rehearsal pass |
| **G5** | Security Baseline | Privilege, capability, and seccomp audit complete |
| **G6** | Hardware Evidence | Hardware driver claims backed by evidence |
| **G7** | Release Audit | Release checklist complete & errata documented |

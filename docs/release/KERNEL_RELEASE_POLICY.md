# SigmaOS Kernel Release Policy

## Overview

The Kernel Release Policy defines 8 strict release readiness gates (G0 to G7) that must pass before any version release tag is issued.

## Release Readiness Gates

| Gate | Code | Requirement |
|---|---|---|
| **G0** | Code Quality | `cargo check --lib`, formatting, unit tests pass cleanly without errors |
| **G1** | Integration | Standalone integration test suites pass |
| **G2** | Memory Safety | No undocumented `unsafe` blocks or unbounded queues |
| **G3** | Boot Smoke | QEMU boot-to-shell smoke test passes |
| **G4** | Recoverability | Installer and failure recovery rehearsal passes |
| **G5** | Security Baseline | Privilege, capability, and seccomp review complete |
| **G6** | Hardware Evidence | Hardware driver claims evidence-backed |
| **G7** | Release Audit | Release checklist complete and errata documented |

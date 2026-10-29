# SigmaOS Kernel Release Policy

This document defines the 8 mandatory release gates (G0 through G7) required before any kernel milestone or release candidate can be tagged or deployed.

## Release Gates Overview

| Gate | Name | Requirement | Enforcement |
|---|---|---|---|
| **G0** | Code Quality Gate | `cargo check --lib`, formatting, and zero library warnings | CI |
| **G1** | Unit & Integration Gate | Standalone unit and integration tests pass cleanly | CI (`./run_sigma_tests.sh`) |
| **G2** | Memory Safety Gate | No new `unsafe` code without documented invariants and approval | Review |
| **G3** | Boot Smoke Gate | QEMU x86_64 boot-to-shell test reaches `shell_ready` marker | Automated QEMU CI |
| **G4** | Recoverability Gate | Installer and failure recovery rehearsal passes without data loss | Test Suite |
| **G5** | Security Baseline Gate | Privileged syscall, capability, and seccomp policy audit complete | Security Review |
| **G6** | Hardware Evidence Gate | All driver hardware support claims backed by logs or emulator traces | Audit (`HARDWARE_SUPPORT.md`) |
| **G7** | Release Notes & Audit | Known issues, errata, and tech-debt backlog documented | Review |

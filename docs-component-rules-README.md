# SigmaOS Component Rules Framework

## Overview

SigmaOS adopts a **periodic, PR-only component-rule system**. Every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

## Periodic Governance Cadence

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

## Component Rule Lifecycle

1. **SELECT COMPONENT**
2. **MEASURE CURRENT CAPABILITY**
3. **SELECT UPSTREAM REFERENCE**
4. **STUDY ONE MECHANISM**
5. **WRITE COMPONENT RULES**
6. **OPEN RESEARCH/ADR PR**
7. **IMPLEMENT SMALLEST SAFE CHANGE**
8. **ADD TESTS AND RUNTIME VALIDATION**
9. **UPDATE DOCS AND CAPABILITY STATUS**
10. **OPEN REVIEW PR**
11. **MERGE ONLY AFTER GATES PASS**
12. **MONITOR FOR REGRESSIONS**

## Standard Component Rules Directory

- [`kernel.md`](./kernel.md) - Kernel, Bootloader, Interrupts, Syscalls & Scheduler Rules
- [`memory.md`](./memory.md) - Physical Memory, Frame Allocator, Paging & Virtual Memory Rules
- [`vfs.md`](./vfs.md) - Virtual File System, Mounts & Filesystem Conformance Rules
- [`ipc.md`](./ipc.md) - Inter-Process Communication, Pipes, Signals & Shared Memory Rules
- [`security.md`](./security.md) - Capability Tokens, Sandboxing & Privilege Separation Rules
- [`drivers.md`](./drivers.md) - Device Drivers, WDM Architecture & VirtIO Probe Rules
- [`packaging.md`](./packaging.md) - Package Metadata, Dependency Resolver & Dry-Run Rules
- [`desktop.md`](./desktop.md) - Zenith Desktop, Compositor & Graphical Smoke Test Rules
- [`installer_update.md`](./installer_update.md) - Installation, Atomic Updates & Rollback Recovery Rules

## Promotion Criteria Levels

| Level | Requirement |
|---|---|
| **Proposed** | ADR and gap analysis |
| **Prototype** | Unit tests |
| **Integrated** | Runtime path plus integration tests |
| **Hardened** | Fuzzing, negative tests, security review |
| **Supported** | Reproducible boot/install/update evidence |

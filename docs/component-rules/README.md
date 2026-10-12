# SigmaOS Component Rules Directory

This directory contains component-specific rule files governing behavior, safety invariants, required evidence, and promotion criteria across all SigmaOS subsystems.

## Index of Component Rules

- [Kernel Rules](kernel.md)
- [Memory Rules](memory.md)
- [VFS Rules](vfs.md)
- [IPC Rules](ipc.md)
- [Security Rules](security.md)
- [Driver Rules](driver.md)
- [Packaging Rules](packaging.md)
- [Desktop Rules](desktop.md)
- [Installer & Update Rules](installer.md)

## Component Rule Template

```markdown
# Component Rules: <COMPONENT>

## Component owner
<agent-owner>

## Upstream references
- Reference 1
- Reference 2

## Purpose
Define required behavior, safety boundaries, and validation.

## Mandatory rules
1. Rule 1
2. Rule 2

## Prohibited
- Prohibited item 1
- Prohibited item 2

## Required evidence
- Evidence item 1

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
```

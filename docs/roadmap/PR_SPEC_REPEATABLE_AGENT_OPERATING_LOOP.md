# Specification: Repeatable Agent Operating Loop

## Specification Details

- **PR Title**: `feat(distro): Turn Repository into Repeatable Agent Operating Loop`
- **PR Branch**: `feat/agent-operating-loop`
- **PR Engine Module**: `src/distro/repeatable_agent_operating_loop_pr.rs`
- **Status**: `Approved / Implementation Ready`

## Architecture Specification

The Repeatable Agent Operating Loop architecture establishes a distro-maintainer governance model for SigmaOS, structuring agent workflows into narrow ownership boundaries, machine-readable runbooks, and evidence-gated capability promotion.

### 1. Agent Roles & Permissions Matrix

| Role Code | Role Name | Owner Path | FS Permission | Network | Package Exec | Git Write |
|---|---|---|---|---|---|---|
| `steward` | Repository Steward | `.github/` | Read repo, write `reports/steward/` | No | No | PR only |
| `sentinel` | Build and Test Sentinel | `reports/test/` | Read repo, write `reports/` and temp | No | No | PR only |
| `boot-validator` | Boot and Runtime Validator | `reports/boot/` | Read repo, write `reports/boot/` | No | No | PR only |
| `capability-auditor` | Capability Auditor | `reports/capability/` | Read-only plus `reports/` | No | No | Issue / PR only |
| `kernel-subsystem` | Kernel & Subsystem Agent | `src/kernel/` | Scoped source paths | No | No | PR only |
| `package-compat` | Package & Distro Compat Agent | `src/package/` | Sandbox only | No default | Sandboxed dry-run | PR only |
| `docs-onboarding` | Documentation & Onboarding | `docs/` | Read repo, write `docs/` and `reports/` | Optional allowlist | No | PR only |

### 2. Machine-Readable Task Schema (TOML)

All recurring agent jobs follow a standard schema stored in `.agents/tasks/`:

```toml
id = "TASK-ID"
title = "Task Title"
agent = "agent-role-code"
priority = "P0 | P1 | P2 | P3"
scope = ["path/to/scope/"]
repeat = "daily | weekly | per-pr"

steps = [
  "step 1 description",
  "step 2 description"
]

evidence = [
  "path/to/evidence/artifact.log",
  "path/to/evidence/artifact.json"
]

success = [
  "success criteria 1",
  "success criteria 2"
]

on_failure = "failure handling rule"
```

### 3. Report Output Standard

Agents write structured verification reports under `reports/`:
- `reports/build/`: Clean build manifests and compilation logs.
- `reports/test/`: Unit/integration test execution manifests.
- `reports/boot/`: QEMU boot logs, serial output, and screenshots.
- `reports/capability/`: Public truth tables and capability audit reports.
- `reports/package/`: Package schema parsing and dependency resolution dry-run traces.
- `reports/security/`: Threat model and unsafe-code audit reports.
- `reports/steward/`: Daily health status and stale-work reports.

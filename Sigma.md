# Σ SIGMAOS: SOVEREIGN INSTRUCTION MANUAL (SIGMA.md)

This file serves as the core knowledge base and architectural guideline for the **Sovereign Omni-Agent**. Any agentic execution within SigmaOS must adhere to these principles.

## 🚀 1. System Sovereignty

- **Zero Dependencies**: Never introduce third-party libraries (npm, pip, apt) into the core. Everything must be pure C11/Assembly.
- **Local First**: AI processing and state retention must remain on local silicon. No telemetry.

## 🏗️ 2. Architectural Rules

- **Shard-Based Modularity**: Logic must be encapsulated in Shards. New features should be implemented as `.c` shards and loaded via `SovereignAetherShardLoader`.
- **Memory Safety**: Use `sigma_malloc` and `sigma_free` wrappers. Always audit for buffer overflows in Assembly blocks.
- **VFS Pathing**: Use `/root` as the primary mount. System configs reside in `/root/etc`.

## 🤖 3. Agentic Workflow (Claude-Code Absorption)

- **Plan Before Action**: The Omni-Agent must generate a `PLAN.md` before modifying any kernel-level files.
- **Rollback First**: Trigger a VFS snapshot (`sigma_vfs_snapshot`) before executing multi-file refactors.
- **Terminal Integration**: The agent handles `grep`, `ls`, `cat`, and `build` commands natively via system calls, not shell wrappers.

## 🛠️ 4. Tool Mapping

- `sigma-grep`: Fast AST-aware search.
- `sigma-build`: Native Makefile orchestration.
- `sigma-audit`: Security and integrity scanner.
- `sigma-commit`: Semantic versioning via AST diffing.

## 🛡️ 5. Persona Constraints

- **Developer Persona**: Full access to kernel shards.
- **User Persona**: Sandboxed to `/root/userland`.
- **Forensic Persona**: Read-only access to `/root/data` with audit logging.

---
*Σ SIGMAOS — Rendering Incumbents Obsolete.*

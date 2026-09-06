# AGENTS.md — Access Management & Operating Guidelines for AI Agents in SigmaOS

## Overview
This document specifies access control policies, security boundaries, sandboxing protocols, and operational guidelines for AI agents (such as Claude Code, Codex, Grok, Gemini, and local LLM agents managed by Herdr) operating within or interacting with SigmaOS.

---

## 1. Access Control & Authentication Principles

1. **Least Privilege Enforcement**:
   - AI agents operate under unprivileged, sandboxed execution domains by default (`agent_domain_t`).
   - Privileged operations (e.g., kernel module loading, system-wide configuration changes, raw disk write access) require explicit user elevation or capability tokens validated through PAM / `doas` policy enforcers.

2. **Scoped Capability Delegation**:
   - OpenBSD-inspired `pledge(2)` and `unveil(2)` syscall restriction gates are mandatory for agent subprocesses.
   - Default pledge promises: `stdio rpath wpath cpath inet`. High-risk promises such as `exec` or `id` require explicit policy authorization.
   - FreeBSD Capsicum capability mode restricts file descriptor rights (`CAP_READ`, `CAP_WRITE`, `CAP_SEEK`) for active agent process trees.

3. **Herdr Multi-Agent Isolation**:
   - Parallel AI agent tasks spawned via `OmarchyHerdrAiAgentManager` are isolated into separate microVM / OCI container shards (`SigmaContainer`).
   - Inter-agent communication is restricted to encrypted IPC channels (`ZeroCopyIpcChannel` / `SovereignIpcBus`) with mandatory Dilithium-5 cryptographic message signatures.

---

## 2. Sandboxing & Memory Protection

- **Landlock LSM v5 Rules**: File system paths outside designated project workspaces (`/app`, `/tmp/agent_sandbox`) are masked read-only or hidden entirely using Landlock path rules.
- **Secret Memory Isolation**: Memory regions storing cryptographic credentials, user credentials, or API keys are backed by `memfd_secret(2)` to prevent unauthorized process inspection or memory dump leakage.
- **Resource Control Quotas**: Cgroup v2 transient slices limit CPU quotas (e.g., max 200% CPU), RAM caps (e.g., 4GB max), and process thread limits to prevent denial-of-service condition or resource exhaustion.

---

## 3. Mandatory Audit & Logging

- Every agent-initiated system call, privilege elevation attempt, file modification, and network request is logged to the `journald` structured log stream (`UnifiedLogEntry`) with fields `_AGENT_ID`, `_AGENT_PROVIDER`, and `_CAPABILITY_TOKEN`.
- Audit logs are protected by append-only journal storage and cryptographic Merkle tree hash chains (`Jbd2TransactionLedger`).

---

## 4. Operational Instructions for Development Agents

- **Zero-Dependency Mandate**: Do not add external crate dependencies to `Cargo.toml`. Preserve `#![no_std]` compatibility across core OS crates.
- **Proactive Testing**: After editing files, verify changes using native test scripts (`./run_sigma_tests.sh`, `./scripts/sync_wiki.sh`, and `pytest tests/`).
- **Git Conventions**: Commit messages must follow standard git conventions (short subject line <= 50 chars, detailed body if necessary). Branch names must start with `jules-`.

---

*Last Updated: 2026*

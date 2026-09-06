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

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling (`src/tools/bootloader.rs`), Zenith compositor render frame-rate profiling (`zenith_desktop/`), zero-allocation hot paths.
- **Rules**:
  - Maintain 60+ FPS compositor rendering and eliminate window layout recalculation bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes (`TokyoNight`, `Catppuccin`, `Nord`), boot splash graphics, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and web console interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 module signatures, desktop process sandbox isolation (`DistrictSandbox`).
- **Rules**:
  - Enforce process isolation for desktop applets and web2app launchers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. DESKTOP ENVIRONMENT & COMPOSITOR POLICIES (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

- **Wayland Ozone Launchers**: Third-party web applications must be launched with Wayland Ozone isolation flags (`--ozone-platform=wayland`).
- **Accessibility Invariants**: All interactive UI elements must render high-contrast focus rings on keyboard TAB focus.

---

## 3. CANARY VALUE MANAGEMENT & SECURITY HARDENING (`docs/AGENTS_CANARY_VALUE_MANAGEMENT.md`)

- **Thread-Local SSP Canaries**: All thread guard values generated by `BinaryProtectionManager` in `src/security/binary_protection.rs` must enforce LSB NUL-byte formatting (`canary & 0xFF == 0x00`) to terminate string buffer overflow attacks.
- **OpenBSD Context Switch Guards**: CPU context switches in `src/kernel/roundrobin.rs` must validate context canary values (`stack_canary`) before restoring execution frames, triggering controlled `__stack_chk_fail` fault handling on mismatch.

---

## 4. CLOUD COMPUTING OPERATIONS MANAGEMENT (`docs/AGENTS_CLOUD_COMPUTING_OPERATIONS_MANAGEMENT.md`)

- **Headless Cloud Targets**: Booting under `SystemTarget::Cloud` (`cloud.target`) in `src/init/sigmainit.rs` must bypass GUI compositors and optimize zero-copy E1000/xHCI network queues (< 16MB RAM footprint).
- **Capability-Gated Cloud-Init**: User-data `#cloud-config` scripts executed by `CloudInitBootstrapEngine` (`src/distro/linux_bsd_parity_extended.rs`) must run inside Ring 3 sandboxes governed by `PledgeManager`.

---

## 5. STATE MANAGEMENT ARCHITECTURE (`docs/AGENTS_STATE_MANAGEMENT.md`)

- **Declarative System State Graph**: State mutations in `src/system/state.rs` must generate immutable generation snapshots supporting $O(1)$ atomic rollback (`rollback()`).
- **Process Lifecycle Machine**: Kernel process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) must adhere strictly to valid lifecycle paths (`New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting`/`BlockedSuspended` $\to$ `Zombie` $\to$ `Terminated`).

---

## 6. TOP-LEVEL COMPONENT MANAGEMENT (`docs/AGENTS_TOP_LEVEL_COMPONENT_MANAGEMENT.md`)

- **Subsystem Isolation**: Top-level components (Microkernel Core, HAL/Drivers, VFS Storage, Network, Security, Package System, Zenith Compositor, Universal Distro Bridge) must not share mutable raw global state across boundaries.
- **Cross-Subsystem Distro Bridge**: Cross-component interactions route through `SovereignUniversalDistroBridge` (`src/distro/linux_bsd_inspirations.rs`) using capability-gated IPC ring buffers and explicit trait interfaces.

---

## 7. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```

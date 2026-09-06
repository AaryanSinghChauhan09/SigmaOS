# AI Agents Access Management Specification for SigmaOS

## Abstract
SigmaOS provides native support for autonomous and user-assisted AI coding agents (including Claude Code, Codex, Grok, Gemini, and local LLM models orchestrated via `OmarchyHerdrAiAgentManager`). This specification defines the architecture, access control models, security policies, token management, and sandboxing infrastructure governing AI agents in SigmaOS.

---

## 1. Architectural Access Model

AI agent interactions are structured around a 4-tier security architecture:

```
[ AI Agent Task / Provider ]
           │
           ▼ (Token Authentication & Scoped Identity)
[ Herdr Agent Manager / Policy Engine ]
           │
           ▼ (Syscall Sandboxing: Pledge / Unveil / Landlock)
[ Sovereign MicroVM / Container Shard ]
           │
           ▼ (Hardware & VFS Enforcement)
[ SigmaOS Kernel Core / Security Hooks ]
```

### 1.1 Identity & Authentication
- Every active agent task is issued a cryptographically signed `AgentToken` containing:
  - `agent_id`: Unique 64-bit task identifier
  - `provider`: Provider enum (`ClaudeCode`, `Codex`, `Grok`, `Gemini`, `LocalLlama`)
  - `capabilities`: Bitmask of granted capability tokens (`CAP_FS_READ`, `CAP_FS_WRITE`, `CAP_NET_OUT`, `CAP_EXEC_SANDBOXED`)
  - `expiration_timestamp`: POSIX expiration timestamp
  - `signature`: Ed25519 / Dilithium-5 post-quantum signature

---

## 2. Security & Sandboxing Infrastructure

### 2.1 Syscall Sandboxing (OpenBSD Pledge & Unveil Parity)
- **Pledge Sandbox Promises**:
  - `stdio`: Basic I/O streams
  - `rpath`: Read-only access to unveiled project workspace files
  - `wpath`: Write access restricted to declared build output directories (`/app/target`, `/tmp`)
  - `inet`: Outbound network socket communication (gated by firewall domain policy)
- **Unveil Path Masking**:
  - Unveiled paths are strictly scoped to the active workspace (`/app`).
  - Sensitive system paths (`/etc/shadow`, `/etc/sudoers`, `/dev/kmem`, `/proc/kcore`) are masked and invisible to agent processes.

### 2.2 Landlock LSM v5 Path & Port Rules
- Landlock LSM rules enforce path-based access control at the kernel layer.
- Outbound network sockets are constrained to approved destination domains (e.g. `api.anthropic.com`, `api.openai.com`, `registry.sigmaos.dev`).

### 2.3 Resource Control (Cgroups v2 & RACCT)
- AI agent container shards operate inside isolated cgroup slices:
  - `cpu.max`: Capped to user-configured percentage (e.g. 200% = 2 cores)
  - `memory.max`: Hard ceiling (e.g. 4096 MB)
  - `pids.max`: Hard process thread limit (e.g. 64 processes)

---

## 3. Privilege Elevation & Policy Enforcement

1. **Direct Root Access Prevention**:
   - AI agents cannot execute `sudo` or `doas` commands directly without interactive user approval or pre-signed elevation rules in `/etc/doas.conf`.
2. **Kernel Module Gating**:
   - Loading unsigned kernel modules or modifying core kernel drivers requires hardware TPM2 / Dilithium-5 key validation.
3. **Audit Trail**:
   - All agent file system modifications, network requests, and privilege escalation attempts are logged to `journald` with `_AGENT_ID` and `_CAPABILITY_TOKEN`.

---

## 4. Wiki Sync & Interoperability

This specification is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_ACCESS_MANAGEMENT_SPEC.md`
- `wiki/AI_AGENTS_ACCESS_MANAGEMENT_SPEC.md`
- `wiki_repo/AI_AGENTS_ACCESS_MANAGEMENT_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Security Architecture*

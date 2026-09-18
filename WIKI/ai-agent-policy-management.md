# AI Agent Security & System Policy Management in SigmaOS

## Overview

SigmaOS policy management infrastructure (`src/security/`, `src/policy_mechanism.rs`, `src/expanded_wiki_innovations.rs`, `src/access/`) enforces system-wide security, cryptographic, resource allocation, and capability policies.

AI agents (such as Jules, Herdr agentic tasks, Copilot code assistants, and automated system maintenance daemons) must operate under strict compliance with system policy profiles.

---

## Policy Management Architecture

```
AI Agent Request → System Crypto Policy Check (`FedoraCryptoPoliciesEngine`)
                           │
                           ▼
              Mandatory Access Control (`sigma_agent_t` / AppArmor)
                           │
                           ▼
              Syscall Sandbox Promises (`OpenBsdPledgeEngine`)
                           │
                           ▼
              Cgroup v2 Resource Policy (`sigma-agent.service`)
```

---

## 1. System Cryptographic Policies (`FedoraCryptoPoliciesEngine`)

System-wide cryptographic policy levels govern the encryption algorithms and key sizes allowed for AI agent network and storage operations:

| Crypto Policy Level | Description | PQC Signature Algorithm | Ciphers Allowed |
|---------------------|-------------|-------------------------|-----------------|
| **`DEFAULT`** | Standard balanced policy for modern systems | Dilithium5 / Kyber1024 | AES-256-GCM, CHACHA20-POLY1305, TLS 1.3 |
| **`LEGACY`** | Backward-compatibility mode | Dilithium3 / RSA-3090 | SHA-256, AES-128, TLS 1.2+ |
| **`FUTURE`** | Strict post-quantum security enforcement | Dilithium5 Only | AES-256-GCM, PQC hybrid, No TLS 1.2 |
| **`FIPS`** | FIPS 140-3 validated cryptographic modules | FIPS Dilithium5 | FIPS-approved AES & SHA-3 |

```rust
use sigmaos::expanded_wiki_innovations::{FedoraCryptoPoliciesEngine, CryptoPolicyLevel};

let mut crypto_policy = FedoraCryptoPoliciesEngine::new();

// Verify current active cryptographic policy before establishing network TLS session
if crypto_policy.get_active_level() == CryptoPolicyLevel::Future {
    // Enforce strict post-quantum TLS 1.3 session
    agent_session.enable_strict_pqc(true)?;
}
```

---

## 2. Mandatory Access Control (MAC) Policies

AI agent processes execute under strict SELinux type enforcement (`sigma_agent_t`) and AppArmor confinement:

```
# SELinux Policy Directive for AI Agents
type sigma_agent_t;
domain_type(sigma_agent_t)

# Allow reading workspace & writing temporary outputs
allow sigma_agent_t userland_workspace_t:file { read write getattr open };
allow sigma_agent_t tmp_t:file { create read write unlink };

# Deny raw disk access & kernel memory inspection
neverallow sigma_agent_t kmem_t:chr_file { read write };
```

---

## 3. Resource & Execution Policies

AI agents must comply with cgroup resource policies (`sigma-agent.service`):

- **CPU Quota**: Max 80% CPU usage limit (`cpu.max = 80000 100000`).
- **Memory Ceiling**: 2048 MB hard RAM limit (`memory.max = 2G`).
- **Thread Count Limit**: Max 64 child threads (`pids.max = 64`).

---

## Directives for AI Agents

1. **Verify Crypto Policy Compliance**: Ensure TLS, SSH, and storage encryption schemes comply with active `FedoraCryptoPoliciesEngine` level.
2. **Respect Access Denials**: Never attempt to bypass MAC or pledge denials; log permission exceptions in `ForensicReadinessAuditor`.
3. **Query System Policy Status**: Use `sysctl` or `PolicyManager` API to inspect active policy parameters before initiating heavy tasks.

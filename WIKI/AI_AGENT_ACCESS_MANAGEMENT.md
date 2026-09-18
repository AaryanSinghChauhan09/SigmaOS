# AI Agent Access Management in SigmaOS

## Overview
SigmaOS incorporates a multi-layered, zero-trust Access Management Subsystem governed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational protocols, APIs, and security boundaries for AI agents managing authentication, authorization, capability tokens, Mandatory Access Control (MAC), Discretionary Access Control (DAC), Pluggable Authentication Modules (PAM), and Post-Quantum Cryptography (PQC) Enclaves.

AI agents interact directly with `src/access/` (`control.rs`), `src/auth/` (`identity.rs`, `pam.rs`, `systemd_homed.rs`), and `src/security/` (`capability.rs`, `capsicum.rs`, `pledge.rs`, `unveil.rs`, `pqc_enclave.rs`, `selinux.rs`).

---

## 1. Access Control Subsystems & Agent Boundaries

### 1.1 Access Control Frameworks
SigmaOS combines POSIX DAC with multi-model Security Enforcement:
* **Role-Based & Attribute-Based Access Control (RBAC/ABAC)**: Implemented in `src/access/control.rs`, providing dynamic role resolution, permission inheritance, and contextual resource evaluation.
* **Linux Capabilities & Capability Tokens**: Implemented in `src/security/capability.rs` and `capability_token.rs`, mapping POSIX capabilities (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_NET_BIND_SERVICE`) to scoped, cryptographic capability tokens.
* **FreeBSD Capsicum Capability Rights**: Implemented in `src/security/capsicum.rs` and `src/security/rules.rs` (`SovereignCapsicumRightsRules`), delegating file descriptor rights (`CAP_READ`, `CAP_WRITE`, `CAP_FSTAT`, `CAP_SEEK`).
* **OpenBSD Pledge & Unveil Sandbox Gates**: Implemented in `src/security/pledge.rs` and `sigma_unveil.rs`, restricting system call promises (`stdio rpath wpath cpath inet`) and filesystem path visibilities.
* **Mandatory Access Control (MAC)**: Implemented in `src/security/mandatory_access_control.rs`, `selinux.rs`, and `lsm.rs` (`SovereignLsmTypeEnforcementRules`), providing SELinux MLS/MCS labels and AppArmor profile type enforcement.
* **Pluggable Authentication Modules (PAM)**: Implemented in `src/security/pam.rs` and `src/auth/authentication_pipeline.rs`, handling multi-factor authentication, biometric auth, and `systemd-homed` encrypted user directories.

---

## 2. AI Agent Operational Directives & Workflows

### 2.1 Identity Verification & Token Generation
1. **Contextual Token Request**:
   When an AI agent requests administrative or elevated execution privileges, it must generate a time-bound, cryptographically signed Capability Token (`CapabilityToken`).
2. **Post-Quantum Enclave Authentication**:
   High-privilege security actions require authentication via `PqcEnclave` (`src/security/pqc_enclave.rs`), utilizing Dilithium-5 signatures and SPHINCS+ hash-based post-quantum key exchange.

### 2.2 Dynamic Sandbox Constraint Enforcement
AI agents automatically construct process sandboxes prior to executing untrusted userland applications or foreign packages:
* **Step 1 (Path Unveil)**: Invoke `SigmaUnveil::unveil(path, permissions)` to restrict filesystem view to required directories.
* **Step 2 (Syscall Pledge)**: Invoke `SigmaPledge::pledge("stdio rpath inet")` to drop non-essential syscall categories.
* **Step 3 (Capsicum Descriptor Delegation)**: Delegate restricted file descriptor rights via `CapsicumRights::limit_fd(fd, rights)`.

### 2.3 Real-Time Anomaly Detection & Access Revocation
1. **AI Anomaly Monitoring**:
   Security agents query `AiAnomalyDetectionEngine` (`src/security/ai_anomaly_detection.rs`) to track token usage velocity, anomalous syscall patterns, or capability abuse.
2. **Automated Token Revocation**:
   Upon detecting compromised or abnormal behavior, the agent invokes `AccessControlManager::revoke_token(token_id)` and triggers an emergency security lockdown (`SovereignIsolationGuard`).

---

## 3. Compliance & Security Rules for Access Management

1. **Least Privilege Default**:
   All AI agent processes execute in unprivileged user namespaces by default (`UserNamespaceManager` under `src/security/user_namespace.rs`).
2. **No Cleartext Credentials**:
   Credentials, API keys, and session tokens must never be logged or stored in plain text. Test credentials must strictly use `mock_` or `test_` variable prefixes.
3. **Audit Ledger Logging**:
   Every privilege escalation, token grant, and access denial event is logged to `DefensiveAuditLogger` (`src/security/defensive_audit.rs`) and `ChainedAuditTrailLedger`.

---

## 4. Sample Agent Commands & CLI Interactions

```bash
# Inspect process capability tokens and active privileges
sigma-access token-info --pid 1024

# Verify PAM & biometric authentication status
sigma-access pam-verify --user root --service sudo

# Test OpenBSD Pledge & Unveil sandbox enforcement on binary
sigma-access sandbox-test --exec /usr/bin/sigpkg --pledge "stdio rpath" --unveil "/tmp:r"

# Revoke dynamic access token across AI agent pool
sigma-access revoke-token --token-id tok_98234_pqc
```

# SigmaOS AI Agent Access Control Management Guidelines

## 1. Overview
SigmaOS implements multi-layered access control frameworks governing autonomous AI agents (such as `AccessControlAgent`, `CapabilityGateValidator`, `PamAuthBridge`, and `ZeroTrustSecurityPolicy`). These guidelines define access control policies, Capability Tokens, PAM/Shadow authentication integration, Role-Based Access Control (RBAC), and Mandatory Access Control (MAC) sandboxing for AI agents in SigmaOS.

## 2. Core Access Control Principles

### 2.1 Identity Verification & Token Delegation
- **Agent Identity**: Every AI agent process possesses an unforgeable identity token (`AgentIdentityToken`).
- **Capability Tokens**: Capabilities are represented as cryptographically signed tokens (`CapabilityToken`). Capability tokens cannot be forged or escalated without system administrator approval.
- **Short-Lived Delegation**: Agents issue short-lived capability sub-tokens to ephemeral worker subprocesses. Sub-tokens automatically expire upon task completion.

### 2.2 Discretionary & Mandatory Access Control (DAC & MAC)
- **POSIX DAC Integration**: AI agents observe standard POSIX user/group permissions (`mode_t`, `uid_t`, `gid_t`).
- **AppArmor & SELinux MAC Integration**: AI agents enforce path-based AppArmor rules (`AppArmorPathRuleEngine`) and SELinux security context flow labeling (`SecmarkPacketLabel`) on all network and file I/O operations.

### 2.3 Pluggable Authentication Modules (PAM) Integration
- **PAM Authentication Bridge**: AI agents authenticating user commands or administrative requests interface with `PamAuthBridge` (`src/auth/access.rs`).
- **Biometric & FIDO2 WebAuthn Integration**: Agents support multi-factor authentication (MFA) triggers for elevated privilege requests (`sudo` / `doas` equivalents).

### 2.4 OpenBSD Pledge & Unveil Restrictions
- **Mandatory Policy Drops**: Access control management agents enforce `pledge()` promises (`stdio rpath wpath cpath inet id`) and `unveil()` restrictions on all managed agent sandboxes prior to processing untrusted data.
- **Immutable Policy Enforcement**: Once an agent issues `pledge()` or `unveil(NULL, NULL)`, the kernel prevents subsequent policy expansion.

---
*Maintained by the SigmaOS Security & SIG-Security Steering Committee.*

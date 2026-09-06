# AI Agent Access Control Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                         AI Agent Access Control Engine                          |
|         (AgentIdentityToken, CapabilityGateValidator, PamAuthBridge)          |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                             Policy Evaluation Engine                            |
|             (RBAC / ABAC Roles, CapabilityToken Matrix, MFA Triggers)           |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
|  OpenBSD Pledge/Unveil|   | AppArmor MAC Rules    |   | SELinux SECMARK Flow  |
| (Syscall & Path Drops)|   | (Path Enforce/Complain|   | (Net Packet Labeling) |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       SigmaOS Security Enforcement Gate                         |
|            (Kernel Syscall Dispatcher, VFS Permission Check, IPTables)          |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Capability Gate & Identity Validation**:
   - `CapabilityGateValidator` verifies `CapabilityToken` signatures and scope bounds before allowing file, IPC, or network access.
   - Disallows unauthorized capability escalation attempts and logs access violations.

2. **Authentication & Role Mapping**:
   - `PamAuthBridge` handles user authentication via PAM modules, shadow password validation, and FIDO2/WebAuthn MFA.
   - `RoleBasedAccessControlEngine` maps system roles (Admin, Developer, Guest, Service) to agent permission scopes.

3. **Mandatory Access Control (MAC) Sandboxing**:
   - Enforces path-based AppArmor rules (`AppArmorPathRuleEngine`) in enforce or complain mode.
   - Enforces SELinux SECMARK context labeling (`SecmarkPacketLabel`) for network packet isolation.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.

# SigmaOS Security Architecture

## Overview

SigmaOS implements a defense-in-depth security model drawing from the best security features of major OS projects:

| Inspiration | Feature | SigmaOS Implementation |
|------------|---------|------------------------|
| OpenBSD | pledge() | `security::pledge_impl` |
| OpenBSD | unveil() | `security::sigma_unveil` |
| FreeBSD | Capsicum | `security::capsicum` |
| Qubes OS | Domain isolation | `security::qubes_isolation` |
| Sandboxie | Process sandboxing | `security::sandbox` |
| Linux | SELinux/AppArmor MAC | `security::mac` |
| Linux | Audit subsystem | `security::audit` |
| Linux | PKI/Certificate mgmt | `security::pki` |
| Linux | Vulnerability scanning | `security::vulnerability` |
| BSD | Integrity checking | `security::integrity` |

## Security Layers

```
┌────────────────────────────────────────┐
│         User Applications              │
├────────────────────────────────────────┤
│  Pledge + Capsicum (Process Caps)       │
├────────────────────────────────────────┤
│  MAC (Mandatory Access Control)         │
├────────────────────────────────────────┤
│  Audit + PKI (Identity & Logging)       │
├────────────────────────────────────────┤
│  Qubes Isolation (VM Domains)           │
├────────────────────────────────────────┤
│  Process Sandbox (Firejail/Sandboxie)   │
├────────────────────────────────────────┤
│  Kernel (Sigma Core)                    │
└────────────────────────────────────────┘
```

## Pledge (OpenBSD-inspired)

After initialization, processes can restrict their own capabilities using pledge:
```rust
enforcer.pledge(pid, &[PledgePromise::Stdio, PledgePromise::Rpath])
```

## Capsicum (FreeBSD-inspired)

Fine-grained file descriptor capabilities:
```rust
caps.grant(fd, CapabilityRights::read_only());
caps.enter(); // No new capabilities after this point
```

## Sigma Unveil (OpenBSD-inspired)

Restrict filesystem visibility to specific paths:
```rust
unveil("/usr/share", "r");  // Only see /usr/share, read-only
unveil_finalize();          // Lock unveil list
```

## Qubes Domain Isolation

Hard isolation between domains using hardware virtualization:
- `sys` - System services domain
- `net` - Untrusted network domain  
- `work` - Work domain
- `personal` - Personal domain
- `vault` - Offline secrets domain

## Mandatory Access Control

Based on SELinux/AppArmor concepts with Sigma-specific policy language.

## Audit Subsystem

All security-relevant events are logged to the audit trail:
- Authentication events
- Privilege changes
- File access violations
- Network connection attempts

## Scanning and Hardening

Run `sigma-security scan` to check for:
- Weak file permissions
- Suspicious setuid binaries
- Kernel module integrity
- Network exposure

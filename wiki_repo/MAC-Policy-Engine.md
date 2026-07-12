# MAC Policy Engine - SELinux-Style Security

## Overview

SigmaOS Mandatory Access Control (MAC) provides SELinux-style security with modular policies and capability tokens for zero-trust security.

## Key Components

### Security Context
```rust
pub struct SecurityContext {
    pub user: [SigmaU8; 64],
    pub role: [SigmaU8; 64],
    pub type_: [SigmaU8; 64],
    pub level: [SigmaU8; 64],
    pub category: [SigmaU8; 128],
}
```

### Security Classes
- Process, File, Directory, Socket, Device, IPC, System, Capability

### Permissions
- Read, Write, Execute, Create, Delete, Append, Ioctl, Mmap, Connect, Bind, Accept, Send, Receive

## Policy Engine

### Configuration
```rust
pub struct PolicyEngine {
    pub modules: *mut PolicyModule,
    pub module_count: SigmaU32,
    pub default_deny: SigmaBool,
    pub audit_enabled: SigmaBool,
}
```

### Decision Process
1. Check all enabled modules (by priority)
2. Find matching rules
3. If any rule denies → Deny
4. If any rule allows → Allow
5. If no rules match → default_deny ? Deny : Allow

## Capability System

Linux-style capability sets:
- **Effective** - Currently active
- **Permitted** - Maximum allowed
- **Inheritable** - Inheritable by children
- **Bounding** - Upper bound for all sets
- **Ambient** - Preserved across exec

## API

```rust
// Initialize
mac_policy_engine_init() -> SigmaI32

// Check permission
mac_check_permission(source, target, class, permission) -> PolicyDecision

// Capability operations
mac_has_capability(cap: SigmaU64) -> SigmaBool
mac_raise_capability(cap: SigmaU64) -> SigmaI32
mac_drop_capability(cap: SigmaU64) -> SigmaI32

// Audit logging
mac_log_audit(entry: AuditEntry) -> SigmaI32
```

## References

- [Security Policy](Security-Policy.md)
- [Sandbox Documentation](Sandbox.md)

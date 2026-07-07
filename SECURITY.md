# SigmaOS Security Architecture & Enforcement

## Overview
SigmaOS implements a Zero-Trust, capability-first security architecture that completely eliminates legacy sudo access or raw root accounts. To guarantee application isolation, the system couples a fine-grained capability-based token system with modern Mandatory Access Control (MAC) policies, running Landlock and seccomp filters at the kernel boundary.

## Security Architecture
The security layer operates at the system call dispatcher. Every requested system call passes through a capability validation check before execution.

```
 [Application Syscall Request]
               │
               ▼
   [Capabilities Validator]  ──► Invalid? ──► Terminate
               │
               ▼ Valid
      [MAC / Landlock Audit] ──► Blocked File? ──► Access Denied
               │
               ▼ Approved
      [Kernel Execution]
```

## Security Configuration
MAC profiles are defined declaratively in `/etc/sigma/security/profiles.d/`.

Example profile (`user-app.sigma`):
```toml
[profile]
name = "user-app"
inherit = "base-sandbox"

[capabilities]
allow_net_connect = false
allow_fs_write = ["/home/user/downloads", "/tmp"]
allow_fs_read = ["/home/user/documents", "/usr/share"]

[syscalls]
allow = ["read", "write", "exit", "futex", "epoll_wait"]
deny = ["ptrace", "sys_chroot", "reboot"]
```

## Technical Implementation
The token-based sandbox checks process capabilities stored in the task control block (TCB).

```rust
// kernel/security/capability.rs
pub const CAP_NET_CONNECT: u64 = 1 << 0;
pub const CAP_FS_WRITE: u64 = 1 << 1;

pub fn validate_capability(current_mask: u64, requested_cap: u64) -> Result<(), SecurityError> {
    if (current_mask & requested_cap) == 0 {
        return Err(SecurityError::PermissionDenied);
    }
    Ok(())
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Implementation of capability-token bitmasks in the task manager.
- **Phase 2 (Months 3-6)**: Integration of Landlock filesystem sandboxing hooks.
- **Phase 3 (Months 6-9)**: Automated profiling tool (`sigtrace`) that generates sandboxing manifests by tracing system calls.
- **Phase 4 (Months 9-12)**: System-wide Zero-Trust verification requiring cryptographically signed capability tokens for all IPC interactions.

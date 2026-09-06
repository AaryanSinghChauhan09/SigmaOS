# AI Agent Access Management & Security Governance in SigmaOS

## Overview

SigmaOS access control subsystem (`src/access/`, `src/access/control.rs`, `src/security/`, `src/security/sandbox.rs`) provides mandatory access control (MAC), role-based and attribute-based access control (RBAC/ABAC), capability token validation, and OpenBSD pledge/unveil restrictions.

AI agents (such as Jules, Herdr agentic tasks, Copilot code generators, and background automation daemons) must operate under principle-of-least-privilege access rules to safeguard kernel and userland resources.

---

## Access Control Framework

```
Agent Process → Capability Token Check (64-bit Hardware Tag)
                      │
                      ▼
            RBAC / ABAC Role Policy Verification
                      │
                      ▼
            OpenBSD Pledge / Unveil Syscall Gate
                      │
                      ▼
            SELinux / AppArmor MAC Enforcement
                      │
                      ▼
            Audit Log Entry Generation
```

---

## 1. Capability Tokens & RBAC Roles

Every AI agent process is assigned a 64-bit hardware-tagged capability token upon creation:

| Role Name | Scope | Permitted Operations |
|-----------|-------|----------------------|
| `AgentRole::CodeGenerator` | Workspace & Temp | Read/Write source files in `/userland/workspace`, compile, run unit tests |
| `AgentRole::SystemInspector` | System Telemetry | Read `/proc`, `sysctl`, cgroup statistics, no write access |
| `AgentRole::NetworkAssistant` | Socket Access | Resolve DNS, bind port > 1024, HTTP GET/POST queries |
| `AgentRole::RootAdmin` | System Wide | Requires explicit interactive GKSU graphical sudo prompt authorization |

---

## 2. Programmatic Pledge & Unveil Restrictions

Before executing userland scripts or subagent processes, AI agents MUST invoke pledge/unveil restrictions:

```rust
use sigmaos::access::{AccessController, CapabilityToken};

let mut controller = AccessController::new(CapabilityToken::for_agent("herdr-coder"));

// Restrict syscall promises
controller.pledge(&["stdio", "rpath", "wpath", "cpath", "inet", "dns"])?;

// Unveil workspace paths
controller.unveil("/userland/workspace", "rwc")?;
controller.unveil("/tmp", "rwc")?;
controller.unveil_finalize()?;
```

---

## 3. SELinux & AppArmor MAC Integration

AI agent processes run under dedicated SELinux domains (`sigma_agent_t`) and AppArmor profiles (`flags=(attach_disconnected,enforce)`):

```
# AppArmor Profile for AI Agent Subprocess
profile sigma_agent /usr/bin/herdr-agent {
  #include <abstractions/base>

  /userland/workspace/** rw,
  /tmp/** rw,
  /usr/bin/* rix,
  deny /etc/shadow r,
  deny /boot/** w,
}
```

---

## 4. Forensic Audit Trail Logging

All capability checks, pledge violations, and file access attempts are recorded in the immutable forensic audit ledger (`ForensicReadinessAuditor`):

```
[00:01:05] CAP_CHECK: agent 'herdr-coder' (PID 1001) requested write access to '/userland/workspace/src/lib.rs' - ALLOWED
[00:02:10] PLEDGE_VIOLATION: agent 'untrusted-subagent' (PID 1002) attempted 'sys_ptrace' - BLOCKED (pledge mask exceeded)
```

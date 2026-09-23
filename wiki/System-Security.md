# System Security

SigmaOS implements comprehensive system security with Linux and BSD-inspired features including access control, sandboxing, auditing, and security policies.

## Overview

System security provides:
- Access control with POSIX permissions, ACLs, and capabilities
- Sandbox isolation with Landlock, Capsicum, pledge, and unveil
- Security auditing and logging
- Mandatory access control (MAC) framework
- Security policies and profiles
- Cryptographic verification and integrity
- Vulnerability management and patching
- Secure boot and measured boot

## Implementation

### Access Control
```rust
// src/security/access_control.rs
pub struct AccessControlManager {
    pub capabilities: BTreeMap<u32, CapabilitySet>,
    pub selinux_policy: Option<SelinuxPolicy>,
    pub apparmor_policy: Option<AppArmorPolicy>,
}

#[derive(Debug, Clone)]
pub struct CapabilitySet {
    pub effective: u64,
    pub permitted: u64,
    pub inheritable: u64,
}

impl AccessControlManager {
    pub fn new() -> Self {
        AccessControlManager {
            capabilities: BTreeMap::new(),
            selinux_policy: None,
            apparmor_policy: None,
        }
    }

    pub fn check_capability(&self, uid: u32, cap: Capability) -> bool {
        if let Some(cap_set) = self.capabilities.get(&uid) {
            (cap_set.effective & (1 << cap as u64)) != 0
        } else {
            false
        }
    }

    pub fn grant_capability(&mut self, uid: u32, cap: Capability) {
        let cap_set = self.capabilities.entry(uid).or_insert_with(CapabilitySet::new);
        cap_set.effective |= 1 << cap as u64;
        cap_set.permitted |= 1 << cap as u64;
    }

    pub fn revoke_capability(&mut self, uid: u32, cap: Capability) {
        if let Some(cap_set) = self.capabilities.get_mut(&uid) {
            cap_set.effective &= !(1 << cap as u64);
        }
    }
}
```

### Security Auditing
```rust
// src/security/auditing.rs
pub struct SecurityAuditor {
    pub audit_log: RingBuffer<AuditEntry>,
    pub audit_rules: Vec<AuditRule>,
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: SystemTime,
    pub uid: u32,
    pub pid: Pid,
    pub event_type: AuditEventType,
    pub path: Option<String>,
    pub outcome: AuditOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventType {
    FileAccess,
    FileAccessDenied,
    PermissionChange,
    CapabilityUse,
    SandboxViolation,
    PolicyViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
}

impl SecurityAuditor {
    pub fn new() -> Self {
        SecurityAuditor {
            audit_log: RingBuffer::new(10000),
            audit_rules: Vec::new(),
        }
    }

    pub fn log_event(&mut self, entry: AuditEntry) {
        self.audit_log.push(entry);
    }

    pub fn check_rules(&self, entry: &AuditEntry) -> bool {
        for rule in &self.audit_rules {
            if rule.matches(entry) {
                return true;
            }
        }
        false
    }

    pub fn query_log(&self, uid: u32, event_type: AuditEventType) -> Vec<AuditEntry> {
        self.audit_log
            .iter()
            .filter(|entry| entry.uid == uid && entry.event_type == event_type)
            .cloned()
            .collect()
    }
}
```

### Mandatory Access Control
```rust
// src/security/mac.rs
pub struct MacFramework {
    pub policies: BTreeMap<String, MacPolicy>,
    pub default_policy: MacPolicy,
}

#[derive(Debug, Clone)]
pub struct MacPolicy {
    pub name: String,
    pub rules: Vec<MacRule>,
    pub enforced: bool,
}

#[derive(Debug, Clone)]
pub struct MacRule {
    pub subject: String,
    pub object: String,
    pub operation: MacOperation,
    pub decision: MacDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacOperation {
    Read,
    Write,
    Execute,
    Create,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacDecision {
    Allow,
    Deny,
    Audit,
}

impl MacFramework {
    pub fn new() -> Self {
        MacFramework {
            policies: BTreeMap::new(),
            default_policy: MacPolicy::default_deny(),
        }
    }

    pub fn add_policy(&mut self, policy: MacPolicy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    pub fn check_access(&self, subject: &str, object: &str, operation: MacOperation) -> MacDecision {
        // Check specific policies
        for policy in self.policies.values() {
            if !policy.enforced {
                continue;
            }

            for rule in &policy.rules {
                if rule.subject == subject && rule.object == object && rule.operation == operation {
                    return rule.decision;
                }
            }
        }

        // Apply default policy
        self.default_policy.check_access(subject, object, operation)
    }
}
```

## Configuration

### Security Configuration
```toml
# /etc/sigmaos/security.toml
[access_control]
# Access control settings
capabilities_enabled = true
acl_enabled = true
selinux_enabled = false
apparmor_enabled = false

[auditing]
# Auditing settings
enabled = true
log_path = "/var/log/audit.log"
max_log_size_mb = 100
rotation_days = 7

[mac]
# Mandatory access control settings
enabled = false
default_policy = "deny"
policy_path = "/etc/sigmaos/mac/policies"

[sandboxing]
# Sandboxing settings
landlock_enabled = true
capsicum_enabled = true
pledge_enabled = true
unveil_enabled = true

[integrity]
# Integrity settings
secure_boot_enabled = false
measured_boot_enabled = false
ima_enabled = false
```

### Runtime Control
```bash
# Show security status
sigsec status

# Enable auditing
sigsec enable-audit

# Show audit log
sigsec show-audit-log

# Add audit rule
sigsec add-audit-rule "type=file_access uid=1000"

# Enable MAC
sigsec enable-mac

# Load MAC policy
sigsec load-mac-policy /etc/sigmaos/mac/policy.toml

# Show capabilities
sigsec show-capabilities

# Grant capability
sigsec grant-capability 1000 CAP_NET_ADMIN

# Revoke capability
sigsec revoke-capability 1000 CAP_NET_ADMIN
```

## Performance Optimization

### Access Control Optimization
Optimize access control for performance:
```bash
# Enable capability caching
sigsec enable-capability-cache

# Enable ACL caching
sigsec enable-acl-cache

# Reduce audit log verbosity
sigsec set-audit-level critical

# Enable async auditing
sigsec enable-async-audit
```

### Sandboxing Optimization
Optimize sandboxing for performance:
```bash
# Use Landlock v5
sigsec enable-landlock-v5

# Use Capsicum with reduced rights
sigsec enable-capsicum-reduced

# Use pledge with limited promises
sigsec enable-pledge-limited
```

## Troubleshooting

### Access Denied
If access denied:
1. Check permissions: `sigsec check-permissions /path/to/file`
2. Check capabilities: `sigsec show-capabilities`
3. Check ACLs: `sigsec show-acl /path/to/file`
4. Check MAC policy: `sigsec check-mac /path/to/file`
5. Check audit log: `sigsec show-audit-log`

### Sandbox Violation
If sandbox violation:
1. Check violation logs: `sigsec show-violations`
2. Check sandbox rules
3. Adjust sandbox permissions
4. Check for necessary capabilities
5. Update sandbox configuration

### Audit Log Full
If audit log is full:
1. Check log size: `sigsec show-audit-log-size`
2. Rotate logs: `sigsec rotate-audit-log`
3. Increase log size: `sigsec set-audit-log-size 200`
4. Reduce audit level
5. Archive old logs

### MAC Policy Conflict
If MAC policy conflict:
1. Check policy conflicts: `sigsec check-mac-conflicts`
2. Review policy rules
3. Resolve conflicts
4. Reload policy: `sigsec reload-mac-policy`
5. Test with default policy

---

**[System Security](Category-Security)** | **[Access Control](Category-Access-Control)** | **[Auditing](Category-Auditing)**

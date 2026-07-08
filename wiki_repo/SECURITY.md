# SigmaOS Security Architecture & Enforcement

## Overview

SigmaOS implements a Zero-Trust, capability-first security architecture that completely eliminates legacy sudo access or raw root accounts. To guarantee application isolation, the system couples a fine-grained capability-based token system with modern Mandatory Access Control (MAC) policies, running Landlock and seccomp filters at the kernel boundary.

### Key Security Principles

- **Zero Trust**: Never trust, always verify
- **Capability-Based**: Fine-grained capability tokens instead of root
- **Mandatory Access Control**: MAC policies for all processes
- **Application Isolation**: Strong sandboxing for all applications
- **Secure Boot**: Chain of trust from firmware to kernel
- **Signed Packages**: GPG-signed packages and repositories
- **Audit Logging**: Comprehensive security audit trails

## Security Architecture

### System Call Flow

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
      [seccomp Filter Check] ──► Blocked Syscall? ──► Access Denied
               │
               ▼ Approved
      [Kernel Execution]
```

### Security Layers

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ User Apps│ System   │ Services     │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Capability System                  │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Token    │ Validator│ Auditor     │ │
│  │ Manager  │          │              │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Mandatory Access Control           │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ SELinux  │ Landlock │ AppArmor     │ │
│  │ Policies │ Filters  │ Profiles     │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      seccomp Filters                     │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Syscall  │ Filter   │ BPF          │ │
│  │ Allowlist│ Engine   │ Programs     │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Kernel Security                     │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Secure   │ Kernel   │ Memory       │ │
│  │ Boot     │ Signing  │ Protection   │ │
│  └──────────┴──────────┴──────────────┘ │
└─────────────────────────────────────────┘
```

## Capability System

### Capability Tokens

**Token Structure**:
```rust
// kernel/security/capability.rs
pub const CAP_NET_CONNECT: u64 = 1 << 0;
pub const CAP_FS_WRITE: u64 = 1 << 1;
pub const CAP_FS_READ: u64 = 1 << 2;
pub const CAP_PROCESS_SPAWN: u64 = 1 << 3;
pub const CAP_DEVICE_ACCESS: u64 = 1 << 4;
pub const CAP_IPC: u64 = 1 << 5;
pub const CAP_SYS_ADMIN: u64 = 1 << 6;

pub struct CapabilityToken {
    pub mask: u64,
    pub process_id: u64,
    pub expiry: Option<u64>,
    pub signature: [u8; 64],
}
```

### Capability Validation

```rust
// kernel/security/capability.rs
pub fn validate_capability(current_mask: u64, requested_cap: u64) -> Result<(), SecurityError> {
    if (current_mask & requested_cap) == 0 {
        return Err(SecurityError::PermissionDenied);
    }
    Ok(())
}

pub fn validate_token(token: &CapabilityToken, process_id: u64) -> Result<(), SecurityError> {
    // Verify process ID matches
    if token.process_id != process_id {
        return Err(SecurityError::InvalidToken);
    }
    
    // Check expiry
    if let Some(expiry) = token.expiry {
        if get_timestamp() > expiry {
            return Err(SecurityError::TokenExpired);
        }
    }
    
    // Verify signature
    if !verify_signature(token) {
        return Err(SecurityError::InvalidSignature);
    }
    
    Ok(())
}
```

### Capability Assignment

```rust
// kernel/security/capability_manager.rs
pub struct CapabilityManager {
    tokens: HashMap<u64, CapabilityToken>,
}

impl CapabilityManager {
    pub fn assign_capability(&mut self, process_id: u64, capability: u64) -> Result<(), SecurityError> {
        let token = self.tokens.get_mut(&process_id)
            .ok_or(SecurityError::ProcessNotFound)?;
        
        token.mask |= capability;
        Ok(())
    }
    
    pub fn revoke_capability(&mut self, process_id: u64, capability: u64) -> Result<(), SecurityError> {
        let token = self.tokens.get_mut(&process_id)
            .ok_or(SecurityError::ProcessNotFound)?;
        
        token.mask &= !capability;
        Ok(())
    }
}
```

## Mandatory Access Control

### SELinux Integration

**Policy Structure**:
```
/etc/sigma/selinux/policy/
├── types/
│   ├── user_app.te
│   ├── system_service.te
│   └── network_service.te
├── roles/
│   ├── user_r.te
│   └── system_r.te
└── rules/
    ├── allow_rules.te
    └── deny_rules.te
```

**Policy Example**:
```te
# user_app.te
type user_app_t;
type user_app_exec_t;

# Allow reading user documents
allow user_app_t user_home_t:file { read getattr };

# Deny network access
neverallow user_app_t port_t:tcp_socket { connect };

# Allow writing to downloads
allow user_app_t downloads_t:file { write create };
```

### Landlock Integration

**Filesystem Sandbox**:
```rust
// kernel/security/landlock.rs
use landlock::{Ruleset, Access, PathBeneath};

pub fn create_sandbox(rules: &[SandboxRule]) -> Result<Ruleset, LandlockError> {
    let mut ruleset = Ruleset::new()
        .handle_access(Access::FS_READ_FILE)
        .handle_access(Access::FS_WRITE_FILE);
    
    for rule in rules {
        let path_beneath = PathBeneath::new(rule.path.clone(), rule.access);
        ruleset.add_rule(path_beneath)?;
    }
    
    Ok(ruleset)
}

pub struct SandboxRule {
    pub path: PathBuf,
    pub access: Access,
}
```

### AppArmor Integration

**Profile Example**:
```
# /etc/apparmor.d/user-app
#include <tunables/global>

profile user-app /usr/bin/user-app {
  #include <abstractions/base>
  
  # Allow reading user documents
  owner /home/user/documents/** r,
  
  # Allow writing to downloads
  owner /home/user/downloads/** rw,
  
  # Deny network access
  deny network,
  
  # Deny raw devices
  deny /dev/** rw,
}
```

## seccomp Filters

### BPF Filter Programs

```rust
// kernel/security/seccomp.rs
use seccomp::{SeccompFilter, SeccompAction, SeccompCondition};

pub fn create_seccomp_filter() -> Result<SeccompFilter, SeccompError> {
    let mut filter = SeccompFilter::new(SeccompAction::Allow)?;
    
    // Allow basic syscalls
    filter.add_rule(SeccompCondition::new(libc::SYS_read, SeccompAction::Allow))?;
    filter.add_rule(SeccompCondition::new(libc::SYS_write, SeccompAction::Allow))?;
    filter.add_rule(SeccompCondition::new(libc::SYS_exit, SeccompAction::Allow))?;
    
    // Deny dangerous syscalls
    filter.add_rule(SeccompCondition::new(libc::SYS_ptrace, SeccompAction::KillProcess))?;
    filter.add_rule(SeccompCondition::new(libc::SYS_chroot, SeccompAction::KillProcess))?;
    filter.add_rule(SeccompCondition::new(libc::SYS_reboot, SeccompAction::KillProcess))?;
    
    Ok(filter)
}
```

## Security Configuration

### MAC Profiles

**File**: `/etc/sigma/security/profiles.d/user-app.sigma`

```toml
[profile]
name = "user-app"
inherit = "base-sandbox"

[capabilities]
allow_net_connect = false
allow_fs_write = ["/home/user/downloads", "/tmp"]
allow_fs_read = ["/home/user/documents", "/usr/share"]
allow_process_spawn = false
allow_device_access = false

[syscalls]
allow = ["read", "write", "exit", "futex", "epoll_wait", "mmap"]
deny = ["ptrace", "sys_chroot", "reboot", "mount", "umount"]

[network]
allow_tcp = false
allow_udp = false
allowed_ports = []

[filesystem]
read_paths = ["/home/user/documents", "/usr/share"]
write_paths = ["/home/user/downloads", "/tmp"]
deny_paths = ["/etc", "/var", "/root"]
```

### System Security Policy

**File**: `/etc/sigma/security/system.sigma`

```toml
[system]
secure_boot = true
kernel_signing = true
module_signing = true
ima_policy = "appraise_tcb"

[audit]
enabled = true
log_file = "/var/log/sigma/audit.log"
log_level = "info"
remote_logging = false

[zero_trust]
require_signed_tokens = true
token_expiry = 3600
revocation_check = true
```

## Secure Boot

### Chain of Trust

```
UEFI Firmware
    │
    ▼ (Verify)
Bootloader (SigmaOS Boot)
    │
    ▼ (Verify)
Kernel (SigmaOS Kernel)
    │
    ▼ (Verify)
Initramfs
    │
    ▼ (Verify)
Kernel Modules
    │
    ▼ (Verify)
System Files
```

### Implementation

```rust
// kernel/security/secure_boot.rs
pub fn verify_chain_of_trust() -> Result<(), SecurityError> {
    // Verify bootloader signature
    verify_bootloader_signature()?;
    
    // Verify kernel signature
    verify_kernel_signature()?;
    
    // Verify initramfs signature
    verify_initramfs_signature()?;
    
    // Verify module signatures
    verify_module_signatures()?;
    
    Ok(())
}

pub fn verify_kernel_signature() -> Result<(), SecurityError> {
    let kernel_path = "/boot/vmlinuz-sigmaos";
    let signature_path = "/boot/vmlinuz-sigmaos.sig";
    
    let kernel_data = read_file(kernel_path)?;
    let signature_data = read_file(signature_path)?;
    
    if !verify_signature(&kernel_data, &signature_data, &SYSTEM_PUBLIC_KEY) {
        return Err(SecurityError::SignatureVerificationFailed);
    }
    
    Ok(())
}
```

## Package Security

### GPG Signing

**Repository Signing**:
```bash
# Generate signing key
gpg --full-generate-key --key-type RSA --key-length 4096

# Export public key
gpg --export --armor > sigmaos-keyring.asc

# Sign repository
sigma-repo sign --key sigmaos-keyring

# Verify repository
sigma-repo verify --key sigmaos-keyring.asc
```

### Package Verification

```rust
// userland/sigpkg/security.rs
pub fn verify_package(package_path: &Path, signature_path: &Path) -> Result<(), SecurityError> {
    let package_data = read_file(package_path)?;
    let signature_data = read_file(signature_path)?;
    
    // Verify GPG signature
    if !verify_gpg_signature(&package_data, &signature_data) {
        return Err(SecurityError::SignatureVerificationFailed);
    }
    
    // Verify checksum
    let expected_hash = get_package_hash(package_path)?;
    let actual_hash = compute_hash(&package_data)?;
    
    if expected_hash != actual_hash {
        return Err(SecurityError::ChecksumMismatch);
    }
    
    Ok(())
}
```

## Audit Logging

### Audit Events

```rust
// kernel/security/audit.rs
pub enum AuditEvent {
    SyscallRequest {
        process_id: u64,
        syscall: u64,
        arguments: Vec<u64>,
    },
    CapabilityCheck {
        process_id: u64,
        capability: u64,
        result: bool,
    },
    FileAccess {
        process_id: u64,
        path: PathBuf,
        operation: AccessOperation,
    },
    NetworkAccess {
        process_id: u64,
        address: IpAddr,
        port: u16,
    },
}

pub fn log_audit_event(event: AuditEvent) -> Result<(), AuditError> {
    let log_entry = format_audit_entry(event)?;
    write_audit_log(log_entry)?;
    Ok(())
}
```

### Audit Configuration

```toml
[audit]
enabled = true
log_file = "/var/log/sigma/audit.log"
log_level = "info"
max_size = "100MB"
rotation = "daily"
retention = "30d"

[remote]
enabled = false
server = "audit.sigmaos.org"
port = 514
protocol = "syslog"
```

## Incident Response

### Security Incident Response

**Detection**:
- Anomaly detection in audit logs
- Unusual capability requests
- Failed authentication attempts
- Suspicious file access patterns

**Response**:
1. Isolate affected system
2. Collect forensic evidence
3. Analyze audit logs
4. Identify root cause
5. Implement remediation
6. Update security policies

**Tools**:
- `sigma-audit`: Audit log analysis
- `sigma-forensics`: Forensic data collection
- `sigma-isolate`: System isolation

## Best Practices

### Development

1. **Security First**: Design security from the start
2. **Least Privilege**: Minimize capabilities
3. **Defense in Depth**: Multiple security layers
4. **Audit Everything**: Comprehensive logging

### Configuration

1. **Secure Defaults**: Enable security by default
2. **Regular Updates**: Keep security patches current
3. **Policy Review**: Regular policy reviews
4. **Testing**: Security testing before deployment

### Monitoring

1. **Real-time Monitoring**: Monitor security events
2. **Alerting**: Automated alerting for incidents
3. **Analysis**: Regular log analysis
4. **Reporting**: Regular security reports

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Implementation of capability-token bitmasks
- Basic capability validation
- seccomp filter integration
- Audit logging infrastructure

### Phase 2 (Months 3-6)
- Integration of Landlock filesystem sandboxing
- SELinux policy development
- AppArmor profile support
- Secure boot implementation

### Phase 3 (Months 6-9)
- Automated profiling tool (sigtrace)
- Policy generation from system call traces
- Advanced audit analysis
- Incident response tools

### Phase 4 (Months 9-12)
- System-wide Zero-Trust verification
- Cryptographically signed capability tokens
- IPC security enforcement
- Advanced threat detection

## References

- [SELinux Documentation](https://selinuxproject.org/)
- [Landlock Documentation](https://www.kernel.org/doc/html/latest/userspace-api/landlock.html)
- [seccomp Documentation](https://www.kernel.org/doc/html/latest/userspace-api/seccomp_filter.html)
- [AppArmor Documentation](https://gitlab.com/apparmor/apparmor/-/wikis/home)
- [Secure Boot Specification](https://uefi.org/specifications)

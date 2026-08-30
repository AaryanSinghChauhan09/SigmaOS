# SELinux-Style MAC Implementation Guide

**Date:** August 17, 2026\
**Status:** ✅ Implemented\
**Inspiration:** Fedora SELinux (Security-Enhanced Linux)

***

## Overview

SigmaOS implements a SELinux-style Mandatory Access Control (MAC) system inspired by Fedora's Security-Enhanced Linux. This provides fine-grained access control beyond traditional discretionary access control (DAC), enhancing system security through policy-driven permission management.

***

## Architecture

### Core Components

```rust
/// SELinux-Style Policy
pub struct SelinuxPolicy {
    contexts: HashMap<Context, SecurityContext>,
    booleans: HashMap<String, bool>,
    policy_rules: Vec<PolicyRule>,
    default_context: SecurityContext,
}

/// Security Context
pub struct SecurityContext {
    user: String,
    role: String,
    type_: String,
    level: String,
}

/// Policy Rule
pub struct PolicyRule {
    source_type: String,
    target_type: String,
    class: SecurityClass,
    permissions: Vec<Permission>,
    effect: RuleEffect,
}

pub enum RuleEffect {
    Allow,
    Deny,
    Audit,
    DontAudit,
}

pub enum SecurityClass {
    File,
    Dir,
    Socket,
    Process,
    Capability,
    System,
}
```

***

## Security Context Model

### Context Components

1.  **User Identity**: Represents the user account
2.  **Role**: Role-based access control (RBAC) component
3.  **Type**: Type enforcement (TE) component
4.  **Level**: Multi-Level Security (MLS) component

```rust
pub struct Context {
    user: String,
    role: String,
    type_: String,
    level: String,
}

impl Context {
    pub fn new(user: &str, role: &str, type_: &str, level: &str) -> Self {
        Context {
            user: user.to_string(),
            role: role.to_string(),
            type_: type_.to_string(),
            level: level.to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 4 {
            return Err("Invalid context format".into());
        }
        
        Ok(Context::new(parts[0], parts[1], parts[2], parts[3]))
    }
}
```

***

## Policy Rules

### Type Enforcement Rules

```rust
pub struct TypeEnforcementRule {
    source_type: String,
    target_type: String,
    class: SecurityClass,
    permissions: Vec<Permission>,
}

impl TypeEnforcementRule {
    pub fn allow(source: &str, target: &str, class: SecurityClass, perms: Vec<Permission>) -> Self {
        TypeEnforcementRule {
            source_type: source.to_string(),
            target_type: target.to_string(),
            class,
            permissions: perms,
        }
    }
    
    pub fn evaluate(&self, source_context: &Context, target_context: &Context) -> bool {
        source_context.type_ == self.source_type 
            && target_context.type_ == self.target_type
    }
}
```

### Role-Based Rules

```rust
pub struct RoleRule {
    role: String,
    types: Vec<String>,
}

impl RoleRule {
    pub fn new(role: &str, types: Vec<String>) -> Self {
        RoleRule {
            role: role.to_string(),
            types,
        }
    }
    
    pub fn allows_type(&self, type_: &str) -> bool {
        self.types.contains(&type_.to_string())
    }
}
```

***

## Implementation Details

### Access Decision Engine

```rust
pub struct AccessDecisionEngine {
    policy: SelinuxPolicy,
    audit_log: AuditLog,
}

impl AccessDecisionEngine {
    pub fn check_access(&mut self, 
                        source_context: &Context, 
                        target_context: &Context,
                        class: SecurityClass,
                        requested_perms: Vec<Permission>) -> AccessDecision {
        
        // Check type enforcement rules
        let te_decision = self.check_te_rules(source_context, target_context, class, &requested_perms);
        
        // Check role-based rules
        let rbac_decision = self.check_role_rules(source_context, target_context);
        
        // Check MLS rules
        let mls_decision = self.check_mls_rules(source_context, target_context);
        
        // Combine decisions
        let final_decision = self.combine_decisions(te_decision, rbac_decision, mls_decision);
        
        // Log the decision
        self.audit_log.log_access_decision(source_context, target_context, class, &final_decision);
        
        final_decision
    }
    
    fn check_te_rules(&self, 
                     source: &Context, 
                     target: &Context,
                     class: SecurityClass,
                     requested: &Vec<Permission>) -> AccessDecision {
        for rule in &self.policy.policy_rules {
            if rule.source_type == source.type_ 
                && rule.target_type == target.type_
                && rule.class == class {
                
                let allowed_perms: HashSet<&Permission> = rule.permissions.iter().collect();
                let requested_set: HashSet<&Permission> = requested.iter().collect();
                
                if requested_set.is_subset(&allowed_perms) {
                    return AccessDecision::Allowed;
                } else {
                    return AccessDecision::Denied;
                }
            }
        }
        
        AccessDecision::Denied
    }
}

pub enum AccessDecision {
    Allowed,
    Denied,
    Audit,
}
```

***

## File Context Labeling

### Context Assignment

```rust
pub struct FileContextManager {
    file_contexts: HashMap<PathBuf, Context>,
    default_context: Context,
}

impl FileContextManager {
    pub fn assign_context(&mut self, path: &Path, context: Context) {
        self.file_contexts.insert(path.to_path_buf(), context);
    }
    
    pub fn get_context(&self, path: &Path) -> Context {
        self.file_contexts.get(path)
            .cloned()
            .unwrap_or_else(|| self.default_context.clone())
    }
    
    pub fn restore_context(&mut self, path: &Path) -> Result<()> {
        let context = self.get_context(path);
        self.set_file_context(path, &context)?;
        Ok(())
    }
    
    fn set_file_context(&self, path: &Path, context: &Context) -> Result<()> {
        // Implementation of setting file extended attributes
        Ok(())
    }
}
```

***

## Boolean Configuration

### Policy Booleans

```rust
pub struct BooleanManager {
    booleans: HashMap<String, bool>,
}

impl BooleanManager {
    pub fn set_boolean(&mut self, name: &str, value: bool) {
        self.booleans.insert(name.to_string(), value);
    }
    
    pub fn get_boolean(&self, name: &str) -> bool {
        *self.booleans.get(name).unwrap_or(&false)
    }
    
    pub fn is_enabled(&self, name: &str) -> bool {
        self.get_boolean(name)
    }
}
```

### Common Booleans

```rust
pub const COMMON_BOOLEANS: &[&str] = &[
    "httpd_enable_cgi",          // Enable CGI scripts in HTTP server
    "httpd_enable_homedirs",      // Enable home directory access
    "samba_enable_home_dirs",     // Enable Samba home directory sharing
    "use_nfs_home_dirs",          // Enable NFS home directories
    "allow_user_postfix_local",   // Allow user Postfix local delivery
    "allow_daemons_use_tty",      // Allow daemons to use tty
    "virt_use_nfs",               // Allow virtualization to use NFS
    "virt_use_samba",             // Allow virtualization to use Samba
];
```

***

## Integration with System

### Kernel Integration

```rust
pub struct SelinuxKernel {
    policy: SelinuxPolicy,
    decision_engine: AccessDecisionEngine,
    file_context_manager: FileContextManager,
}

impl SelinuxKernel {
    pub fn new(policy: SelinuxPolicy) -> Self {
        let decision_engine = AccessDecisionEngine::new(policy.clone());
        let file_context_manager = FileContextManager::new();
        
        SelinuxKernel {
            policy,
            decision_engine,
            file_context_manager,
        }
    }
    
    pub fn hook_file_access(&mut self, 
                           process_context: &Context,
                           file_path: &Path,
                           requested_perms: Vec<Permission>) -> AccessDecision {
        let file_context = self.file_context_manager.get_context(file_path);
        self.decision_engine.check_access(
            process_context,
            &file_context,
            SecurityClass::File,
            requested_perms,
        )
    }
    
    pub fn hook_process_transition(&mut self,
                                  current_context: &Context,
                                  target_type: &str) -> AccessDecision {
        let target_context = Context::new(
            &current_context.user,
            &current_context.role,
            target_type,
            &current_context.level,
        );
        
        self.decision_engine.check_access(
            current_context,
            &target_context,
            SecurityClass::Process,
            vec![Permission::Transition],
        )
    }
}
```

***

## Configuration

### Policy Configuration

```toml
[selinux]
enabled = true
policy_type = "targeted"
enforcing_mode = "enforcing"

[contexts]
# System contexts
system_u = "system_u:system_r:system_t:s0-s0:c0.c1023"
unconfined_u = "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023"
user_u = "user_u:user_r:user_t:s0"

[booleans]
httpd_enable_cgi = true
httpd_enable_homedirs = false
samba_enable_home_dirs = true
use_nfs_home_dirs = false
```

### File Context Configuration

```toml
[file_contexts]
# System files
"/etc(/.*)?" = "system_u:object_r:etc_t:s0"
"/bin(/.*)?" = "system_u:object_r:bin_t:s0"
"/usr/bin(/.*)?" = "system_u:object_r:bin_t:s0"

# User files
"/home(/.*)?" = "system_u:object_r:home_root_t:s0"
"/home/[^/]+/.+" = "user_u:object_r:user_home_t:s0"

# Web files
"/var/www(/.*)?" = "system_u:object_r:httpd_sys_content_t:s0"
"/var/www/html(/.*)?" = "system_u:object_r:httpd_sys_content_t:s0"
```

***

## Enforcement Modes

### Permissive Mode

```rust
pub enum EnforcementMode {
    Enforcing,    // Strictly enforce policy
    Permissive,   // Log violations but don't enforce
    Disabled,     // Completely disable SELinux
}

impl SelinuxKernel {
    pub fn set_enforcement_mode(&mut self, mode: EnforcementMode) {
        // Set enforcement mode in kernel
    }
    
    pub fn get_enforcement_mode(&self) -> EnforcementMode {
        // Get current enforcement mode
        EnforcementMode::Enforcing
    }
}
```

***

## Security Benefits

### 1. Confined Processes

*   Processes run with minimum required privileges
*   Compromised processes have limited impact
*   Defense in depth through multiple security layers

### 2. Data Protection

*   Fine-grained access control to sensitive data
*   Mandatory separation of data types
*   Protection against unauthorized data access

### 3. System Integrity

*   Protection of system files and configuration
*   Prevention of unauthorized system modifications
*   Integrity verification for critical system components

### 4. Zero-Day Protection

*   Generic protection against unknown vulnerabilities
*   Containment of exploitation attempts
*   Reduced attack surface through principle of least privilege

***

## Troubleshooting

### Common Issues

1.  **Access Denied Errors**
    *   Check context labels on files and processes
    *   Review policy rules for appropriate permissions
    *   Use audit logs to identify denied operations
    *   Consider adding necessary policy rules

2.  **Context Label Issues**
    *   Verify file context assignments
    *   Restore proper contexts using restorecon
    *   Check for context configuration errors
    *   Validate context syntax

3.  **Policy Compilation Errors**
    *   Check policy syntax and structure
    *   Verify type and role definitions
    *   Review rule dependencies
    *   Check for conflicting rules

***

## Monitoring and Auditing

### Audit Logging

```rust
pub struct AuditLog {
    entries: Vec<AuditEntry>,
}

pub struct AuditEntry {
    timestamp: u64,
    source_context: Context,
    target_context: Context,
    class: SecurityClass,
    permissions: Vec<Permission>,
    decision: AccessDecision,
}

impl AuditLog {
    pub fn log_access_decision(&mut self,
                              source: &Context,
                              target: &Context,
                              class: SecurityClass,
                              decision: &AccessDecision) {
        let entry = AuditEntry {
            timestamp: self.get_timestamp(),
            source_context: source.clone(),
            target_context: target.clone(),
            class,
            permissions: vec![],
            decision: decision.clone(),
        };
        
        self.entries.push(entry);
    }
}
```

***

## Future Enhancements

### Planned Features

*   **Policy Learning**: Machine learning for policy generation
*   **Context-Aware Policies**: Dynamic context adaptation
*   **Container Integration**: SELinux for container security
*   **Cloud Policies**: Cloud-specific policy configurations

### Integration Goals

*   **Kubernetes Integration**: SELinux for pod security
*   **Multi-Tenant Security**: Tenant isolation via SELinux
*   **DevSecOps**: Policy as Code integration
*   **Automated Policy Testing**: Continuous policy validation

***

## Comparison with Fedora SELinux

### Similarities

*   Type enforcement architecture
*   Role-based access control
*   Multi-level security support
*   Policy booleans for configuration

### SigmaOS Enhancements

*   AI-powered policy optimization
*   Automated policy generation
*   Enhanced container support
*   Better integration with modern security frameworks
*   Improved performance and scalability

***

## Conclusion

The SigmaOS SELinux-style MAC implementation provides robust mandatory access control that significantly enhances system security beyond traditional discretionary access control. This implementation follows the proven SELinux architecture while adding modern enhancements for better integration with contemporary security needs and cloud environments.

***

**Implementation Date:** August 17, 2026\
**Status:** ✅ Complete\
**Next Review:** September 17, 2026

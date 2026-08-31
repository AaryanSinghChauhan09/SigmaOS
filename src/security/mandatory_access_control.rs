extern crate alloc;
// SigmaOS Mandatory Access Control (MAC) System
// Inspired by SELinux and AppArmor
// Provides fine-grained access control beyond traditional Unix permissions


use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// SELinux-style security context for processes and objects
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelinuxSecurityContext {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

impl SelinuxSecurityContext {
    /// Create a new security context
    pub fn new(user: &str, role: &str, type_: &str, level: &str) -> Self {
        Self {
            user: user.to_string(),
            role: role.to_string(),
            type_: type_.to_string(),
            level: level.to_string(),
        }
    }

    /// Parse security context from string format using sovereign byte-splitting
    pub fn from_str(s: &str) -> Result<Self, MacError> {
        let mut parts = [""; 4];
        let mut count = 0;
        let bytes = s.as_bytes();
        let mut start = 0;

        for (i, &b) in bytes.iter().enumerate() {
            if b == b':' {
                if count >= 4 {
                    return Err(MacError::InvalidContext);
                }
                parts[count] = core::str::from_utf8(&bytes[start..i]).map_err(|_| MacError::InvalidContext)?;
                count += 1;
                start = i + 1;
            }
        }

        if count < 4 && start <= bytes.len() {
            if count == 3 {
                parts[count] = core::str::from_utf8(&bytes[start..]).map_err(|_| MacError::InvalidContext)?;
                count += 1;
            }
        }

        if count != 4 {
            return Err(MacError::InvalidContext);
        }

        Ok(Self::new(parts[0], parts[1], parts[2], parts[3]))
    }

    /// Convert to string format
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self.type_, self.level)
    }
}

/// Access control rule
#[derive(Debug, Clone)]
pub struct SelinuxAccessRule {
    pub source: SelinuxSecurityContext,
    pub target: SelinuxSecurityContext,
    pub class: AccessClass,
    pub permissions: u32,
}

/// Access class (object types)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessClass {
    File,
    Dir,
    Socket,
    Process,
    Network,
    Device,
    IPC,
}

impl AccessClass {
    /// Parse from string
    pub fn from_str(s: &str) -> Result<Self, MacError> {
        match s {
            "file" => Ok(AccessClass::File),
            "dir" => Ok(AccessClass::Dir),
            "socket" => Ok(AccessClass::Socket),
            "process" => Ok(AccessClass::Process),
            "network" => Ok(AccessClass::Network),
            "device" => Ok(AccessClass::Device),
            "ipc" => Ok(AccessClass::IPC),
            _ => Err(MacError::InvalidClass),
        }
    }
}

/// MAC policy enforcement errors
#[derive(Debug, Clone)]
pub enum MacError {
    InvalidContext,
    InvalidClass,
    DeniedAccess,
    PolicyLoadFailed,
    ContextTransitionFailed,
}

/// SELinux MAC policy engine
pub struct SelinuxMacPolicyEngine {
    pub rules: Vec<SelinuxAccessRule>,
    pub default_deny: bool,
    pub enforce: bool,
}

impl SelinuxMacPolicyEngine {
    /// Create new MAC policy engine
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            default_deny: true,
            enforce: true,
        }
    }

    /// Add access rule
    pub fn add_rule(&mut self, rule: SelinuxAccessRule) {
        self.rules.push(rule);
    }

    /// Check if access is permitted
    pub fn check_access(
        &self,
        source: &SelinuxSecurityContext,
        target: &SelinuxSecurityContext,
        class: AccessClass,
        required_permissions: u32,
    ) -> Result<(), MacError> {
        if !self.enforce {
            return Ok(()); // Permissive mode
        }

        // Check for matching rule
        for rule in &self.rules {
            if rule.source == *source && rule.target == *target && rule.class == class {
                if (rule.permissions & required_permissions) == required_permissions {
                    return Ok(());
                }
            }
        }

        // Default deny if no matching rule
        if self.default_deny {
            Err(MacError::DeniedAccess)
        } else {
            Ok(())
        }
    }

    /// Load policy from string
    pub fn load_policy(&mut self, policy: &str) -> Result<(), MacError> {
        // Parse policy lines and add rules
        for line in policy.lines() {
            if line.starts_with("#") || line.trim().is_empty() {
                continue;
            }
            // Simplified parsing - real implementation would be more robust
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let source = SelinuxSecurityContext::from_str(parts[0])?;
                let target = SelinuxSecurityContext::from_str(parts[1])?;
                let class = AccessClass::from_str(parts[2])?;
                let permissions = u32::from_str_radix(parts[3], 16)
                    .map_err(|_| MacError::PolicyLoadFailed)?;
                
                self.add_rule(SelinuxAccessRule {
                    source,
                    target,
                    class,
                    permissions,
                });
            }
        }
        Ok(())
    }

    /// Set enforcement mode
    pub fn set_enforce(&mut self, enforce: bool) {
        self.enforce = enforce;
    }

    /// Get current enforcement mode
    pub fn is_enforcing(&self) -> bool {
        self.enforce
    }
}

impl Default for SelinuxMacPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Security context transition
pub struct SelinuxContextTransition {
    pub from: SelinuxSecurityContext,
    pub to: SelinuxSecurityContext,
    pub executable: String,
}

/// SELinux context manager
pub struct SelinuxContextManager {
    pub transitions: Vec<SelinuxContextTransition>,
    pub current_context: SelinuxSecurityContext,
}

impl SelinuxContextManager {
    /// Create new context manager
    pub fn new(initial_context: SelinuxSecurityContext) -> Self {
        Self {
            transitions: Vec::new(),
            current_context: initial_context,
        }
    }

    /// Add context transition rule
    pub fn add_transition(&mut self, transition: SelinuxContextTransition) {
        self.transitions.push(transition);
    }

    /// Perform context transition
    pub fn transition(&mut self, executable: &str) -> Result<SelinuxSecurityContext, MacError> {
        for transition in &self.transitions {
            if transition.executable == executable && transition.from == self.current_context {
                self.current_context = transition.to.clone();
                return Ok(self.current_context.clone());
            }
        }
        Err(MacError::ContextTransitionFailed)
    }

    /// Get current context
    pub fn get_current(&self) -> &SelinuxSecurityContext {
        &self.current_context
    }

    /// Set current context (with privilege check in real implementation)
    pub fn set_context(&mut self, context: SelinuxSecurityContext) {
        self.current_context = context;
    }
}

/// SELinux MAC-aware file operations
pub struct SelinuxMacFileOperations {
    pub policy: SelinuxMacPolicyEngine,
}

impl SelinuxMacFileOperations {
    /// Create new MAC file operations
    pub fn new(policy: SelinuxMacPolicyEngine) -> Self {
        Self { policy }
    }

    /// Check file read access
    pub fn check_read_access(
        &self,
        source: &SelinuxSecurityContext,
        file_context: &SelinuxSecurityContext,
    ) -> Result<(), MacError> {
        self.policy.check_access(
            source,
            file_context,
            AccessClass::File,
            0x1, // READ permission
        )
    }

    /// Check file write access
    pub fn check_write_access(
        &self,
        source: &SelinuxSecurityContext,
        file_context: &SelinuxSecurityContext,
    ) -> Result<(), MacError> {
        self.policy.check_access(
            source,
            file_context,
            AccessClass::File,
            0x2, // WRITE permission
        )
    }

    /// Check file execute access
    pub fn check_execute_access(
        &self,
        source: &SelinuxSecurityContext,
        file_context: &SelinuxSecurityContext,
    ) -> Result<(), MacError> {
        self.policy.check_access(
            source,
            file_context,
            AccessClass::File,
            0x4, // EXECUTE permission
        )
    }
}

/// Default SELinux MAC policy setup
pub fn setup_selinux_default_policy() -> SelinuxMacPolicyEngine {
    let mut policy = SelinuxMacPolicyEngine::new();
    
    // Add some default rules
    let system_context = SelinuxSecurityContext::new("system_u", "system_r", "system_t", "s0");
    let admin_context = SelinuxSecurityContext::new("user_u", "staff_r", "staff_t", "s0");
    let user_context = SelinuxSecurityContext::new("user_u", "user_r", "user_t", "s0");
    
    // System can access everything
    let system_file_context = SelinuxSecurityContext::new("system_u", "object_r", "system_file_t", "s0");
    policy.add_rule(SelinuxAccessRule {
        source: system_context.clone(),
        target: system_file_context.clone(),
        class: AccessClass::File,
        permissions: 0xFFFFFFFF, // All permissions
    });
    
    // Admin can read system files
    policy.add_rule(SelinuxAccessRule {
        source: admin_context.clone(),
        target: system_file_context,
        class: AccessClass::File,
        permissions: 0x1, // READ only
    });
    
    // Users can access their own files
    let user_file_context = SelinuxSecurityContext::new("user_u", "object_r", "user_file_t", "s0");
    policy.add_rule(SelinuxAccessRule {
        source: user_context.clone(),
        target: user_file_context,
        class: AccessClass::File,
        permissions: 0x7, // READ, WRITE, EXECUTE
    });
    
    policy
}

// ============================================================================
// LSM-Style Kernel Hook API for Inode, Ptrace, and Network Operations
// ============================================================================

/// Types of kernel operations guarded by MAC hooks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacHookType {
    // Inode operations
    InodeCreate,
    InodeOpen,
    InodeUnlink,
    // Process / Ptrace operations
    PtraceAccessCheck,
    PtraceTraced,
    // Network / Socket operations
    SocketCreate,
    SocketConnect,
    SocketBind,
}

/// Evaluation result from a MAC kernel hook
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacHookResult {
    Allow,
    Deny,
    AuditLog,
}

/// Parameters passed to an Inode operation hook
#[derive(Debug, Clone)]
pub struct InodeHookParams {
    pub process_context: SelinuxSecurityContext,
    pub inode_path: String,
    pub requested_mask: u32, // Read = 0x1, Write = 0x2, Exec = 0x4
}

/// Parameters passed to a Ptrace operation hook
#[derive(Debug, Clone)]
pub struct PtraceHookParams {
    pub tracer_context: SelinuxSecurityContext,
    pub tracee_context: SelinuxSecurityContext,
    pub tracer_pid: u64,
    pub tracee_pid: u64,
    pub is_attach: bool,
}

/// Parameters passed to a Socket operation hook
#[derive(Debug, Clone)]
pub struct SocketHookParams {
    pub process_context: SelinuxSecurityContext,
    pub domain: u32,  // AF_INET = 2, AF_INET6 = 10, AF_UNIX = 1
    pub type_: u32,   // SOCK_STREAM = 1, SOCK_DGRAM = 2
    pub protocol: u32,// IPPROTO_TCP = 6, IPPROTO_UDP = 17
    pub target_ip: [u8; 4],
    pub target_port: u16,
}

/// Sovereign MAC LSM Kernel Hook Registry
pub struct SovereignMacLsmHookRegistry {
    pub policy_engine: SelinuxMacPolicyEngine,
    pub blocked_ptrace_targets: Vec<String>, // Type strings blocked from ptrace
    pub audit_log: Vec<String>,
}

impl SovereignMacLsmHookRegistry {
    pub fn new(policy_engine: SelinuxMacPolicyEngine) -> Self {
        Self {
            policy_engine,
            blocked_ptrace_targets: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    /// Register a security type whose processes cannot be traced/attached via ptrace
    pub fn block_ptrace_target_type(&mut self, target_type: &str) {
        self.blocked_ptrace_targets.push(target_type.to_string());
    }

    /// Hook for Inode operations (open, create, unlink)
    pub fn hook_inode_access(&mut self, hook_type: MacHookType, params: &InodeHookParams) -> MacHookResult {
        let file_context = SelinuxSecurityContext::new("system_u", "object_r", "system_file_t", "s0");
        let res = self.policy_engine.check_access(
            &params.process_context,
            &file_context,
            AccessClass::File,
            params.requested_mask,
        );

        match res {
            Ok(()) => MacHookResult::Allow,
            Err(_) => {
                self.audit_log.push(format!(
                    "MAC_AUDIT: DENY inode {:?} on path '{}' for context {}",
                    hook_type,
                    params.inode_path,
                    params.process_context.to_string()
                ));
                MacHookResult::Deny
            }
        }
    }

    /// Hook for Ptrace operations (debugging, process inspection, memory injection)
    pub fn hook_ptrace_access(&mut self, params: &PtraceHookParams) -> MacHookResult {
        // 1. Check if tracee belongs to a restricted security type
        if self.blocked_ptrace_targets.contains(&params.tracee_context.type_) {
            self.audit_log.push(format!(
                "MAC_AUDIT: DENY ptrace attach from PID {} ({}) to protected PID {} ({})",
                params.tracer_pid,
                params.tracer_context.to_string(),
                params.tracee_pid,
                params.tracee_context.to_string()
            ));
            return MacHookResult::Deny;
        }

        // 2. Check policy engine permission
        let res = self.policy_engine.check_access(
            &params.tracer_context,
            &params.tracee_context,
            AccessClass::Process,
            0x8, // PTRACE permission flag
        );

        match res {
            Ok(()) => MacHookResult::Allow,
            Err(_) => MacHookResult::Deny,
        }
    }

    /// Hook for Socket / Network operations (socket creation, bind, connect)
    pub fn hook_socket_operation(&mut self, hook_type: MacHookType, params: &SocketHookParams) -> MacHookResult {
        let net_context = SelinuxSecurityContext::new("system_u", "object_r", "network_t", "s0");
        let mask = match hook_type {
            MacHookType::SocketCreate => 0x1,
            MacHookType::SocketBind => 0x2,
            MacHookType::SocketConnect => 0x4,
            _ => 0x1,
        };

        let res = self.policy_engine.check_access(
            &params.process_context,
            &net_context,
            AccessClass::Network,
            mask,
        );

        match res {
            Ok(()) => MacHookResult::Allow,
            Err(_) => {
                self.audit_log.push(format!(
                    "MAC_AUDIT: DENY socket {:?} to {}:{} for context {}",
                    hook_type,
                    format!("{}.{}.{}.{}", params.target_ip[0], params.target_ip[1], params.target_ip[2], params.target_ip[3]),
                    params.target_port,
                    params.process_context.to_string()
                ));
                MacHookResult::Deny
            }
        }
    }
}

impl Default for SovereignMacLsmHookRegistry {
    fn default() -> Self {
        Self::new(setup_selinux_default_policy())
    }
}

#[cfg(test)]
mod mac_hook_tests {
    use super::*;

    #[test]
    fn test_mac_lsm_inode_hooks() {
        let policy = setup_selinux_default_policy();
        let mut registry = SovereignMacLsmHookRegistry::new(policy);

        let system_ctx = SelinuxSecurityContext::new("system_u", "system_r", "system_t", "s0");
        let user_ctx = SelinuxSecurityContext::new("user_u", "user_r", "user_t", "s0");

        let inode_params_system = InodeHookParams {
            process_context: system_ctx,
            inode_path: "/etc/shadow".to_string(),
            requested_mask: 0x1, // Read
        };
        assert_eq!(registry.hook_inode_access(MacHookType::InodeOpen, &inode_params_system), MacHookResult::Allow);

        let inode_params_user = InodeHookParams {
            process_context: user_ctx,
            inode_path: "/etc/shadow".to_string(),
            requested_mask: 0x2, // Write
        };
        assert_eq!(registry.hook_inode_access(MacHookType::InodeOpen, &inode_params_user), MacHookResult::Deny);
        assert!(!registry.audit_log.is_empty());
    }

    #[test]
    fn test_mac_lsm_ptrace_hooks() {
        let policy = setup_selinux_default_policy();
        let mut registry = SovereignMacLsmHookRegistry::new(policy);
        registry.block_ptrace_target_type("system_t");

        let tracer_ctx = SelinuxSecurityContext::new("user_u", "user_r", "user_t", "s0");
        let tracee_ctx = SelinuxSecurityContext::new("system_u", "system_r", "system_t", "s0");

        let ptrace_params = PtraceHookParams {
            tracer_context: tracer_ctx,
            tracee_context: tracee_ctx,
            tracer_pid: 1001,
            tracee_pid: 1,
            is_attach: true,
        };

        assert_eq!(registry.hook_ptrace_access(&ptrace_params), MacHookResult::Deny);
        assert!(registry.audit_log[0].contains("DENY ptrace attach"));
    }

    #[test]
    fn test_mac_lsm_socket_hooks() {
        let mut policy = setup_selinux_default_policy();
        let system_ctx = SelinuxSecurityContext::new("system_u", "system_r", "system_t", "s0");
        let net_ctx = SelinuxSecurityContext::new("system_u", "object_r", "network_t", "s0");

        policy.add_rule(SelinuxAccessRule {
            source: system_ctx.clone(),
            target: net_ctx,
            class: AccessClass::Network,
            permissions: 0x7, // Create, Bind, Connect
        });

        let mut registry = SovereignMacLsmHookRegistry::new(policy);

        let sock_params = SocketHookParams {
            process_context: system_ctx,
            domain: 2,
            type_: 1,
            protocol: 6,
            target_ip: [127, 0, 0, 1],
            target_port: 8080,
        };

        assert_eq!(registry.hook_socket_operation(MacHookType::SocketBind, &sock_params), MacHookResult::Allow);
    }
}
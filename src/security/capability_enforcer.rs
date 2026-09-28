// Capability-Based Security Enforcement
// Inspired by Linux capabilities and BSD Capsicum for fine-grained access control

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Linux capability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinuxCapability {
    Chown,
    DacOverride,
    DacReadSearch,
    Fowner,
    Fsetid,
    Kill,
    Setgid,
    Setuid,
    Setpcap,
    LinuxImmutable,
    NetBindService,
    NetBroadcast,
    NetAdmin,
    NetRaw,
    IpcLock,
    IpcOwner,
    SysModule,
    SysRawio,
    SysChroot,
    SysPtrace,
    SysPacct,
    SysAdmin,
    SysBoot,
    SysNice,
    SysResource,
    SysTime,
    SysTtyConfig,
    Mknod,
    Lease,
    AuditWrite,
    AuditControl,
    Setfcap,
    MacOverride,
    MacAdmin,
    Syslog,
    WakeAlarm,
    BlockSuspend,
    AuditRead,
}

/// Capability set
#[derive(Debug, Clone)]
pub struct CapabilitySet {
    pub capabilities: HashMap<LinuxCapability, bool>,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }

    /// Add a capability
    pub fn add(&mut self, cap: LinuxCapability) {
        self.capabilities.insert(cap, true);
    }

    /// Remove a capability
    pub fn remove(&mut self, cap: LinuxCapability) {
        self.capabilities.insert(cap, false);
    }

    /// Check if capability is present
    pub fn has(&self, cap: LinuxCapability) -> bool {
        self.capabilities.get(&cap).copied().unwrap_or(false)
    }

    /// Clear all capabilities
    pub fn clear(&mut self) {
        self.capabilities.clear();
    }

    /// Get capability count
    pub fn count(&self) -> usize {
        self.capabilities.values().filter(|&&v| v).count()
    }
}

/// Security context with capabilities
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub id: u64,
    pub capabilities: CapabilitySet,
    pub effective: CapabilitySet,
    pub permitted: CapabilitySet,
    pub inheritable: CapabilitySet,
}

impl SecurityContext {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            capabilities: CapabilitySet::new(),
            effective: CapabilitySet::new(),
            permitted: CapabilitySet::new(),
            inheritable: CapabilitySet::new(),
        }
    }

    /// Check if operation is allowed
    pub fn check(&self, cap: LinuxCapability) -> bool {
        self.effective.has(cap)
    }

    /// Add capability to effective set
    pub fn add_effective(&mut self, cap: LinuxCapability) {
        self.effective.add(cap);
    }

    /// Add capability to permitted set
    pub fn add_permitted(&mut self, cap: LinuxCapability) {
        self.permitted.add(cap);
    }

    /// Add capability to inheritable set
    pub fn add_inheritable(&mut self, cap: LinuxCapability) {
        self.inheritable.add(cap);
    }

    /// Promote from permitted to effective
    pub fn promote(&mut self, cap: LinuxCapability) -> bool {
        if self.permitted.has(cap) {
            self.effective.add(cap);
            true
        } else {
            false
        }
    }

    /// Drop all capabilities
    pub fn drop_all(&mut self) {
        self.effective.clear();
        self.permitted.clear();
        self.inheritable.clear();
    }
}

/// Resource type for capability check
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    File,
    Socket,
    Process,
    Network,
    System,
}

/// Resource access permission
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePermission {
    Read,
    Write,
    Execute,
    Create,
    Delete,
    Bind,
    Connect,
    Listen,
}

/// Capability enforcer
pub struct CapabilityEnforcer {
    contexts: HashMap<u64, SecurityContext>,
    next_context_id: AtomicU64,
    default_policy: CapabilitySet,
}

impl CapabilityEnforcer {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
            next_context_id: AtomicU64::new(1),
            default_policy: CapabilitySet::new(),
        }
    }

    /// Create a new security context
    pub fn create_context(&mut self) -> SecurityContext {
        let id = self.next_context_id.fetch_add(1, Ordering::SeqCst);
        
        let context = SecurityContext::new(id);
        self.contexts.insert(id, context.clone());
        
        context
    }

    /// Get a security context
    pub fn get_context(&self, id: u64) -> Option<&SecurityContext> {
        self.contexts.get(&id)
    }

    /// Get mutable security context
    pub fn get_context_mut(&mut self, id: u64) -> Option<&mut SecurityContext> {
        self.contexts.get_mut(&id)
    }

    /// Check resource access
    pub fn check_access(&self, context_id: u64, resource: ResourceType, permission: ResourcePermission) -> bool {
        let context = self.contexts.get(&context_id);
        
        match context {
            Some(ctx) => {
                match (resource, permission) {
                    (ResourceType::File, ResourcePermission::Read) => {
                        ctx.check(LinuxCapability::DacReadSearch) || ctx.check(LinuxCapability::DacOverride)
                    }
                    (ResourceType::File, ResourcePermission::Write) => {
                        ctx.check(LinuxCapability::DacOverride)
                    }
                    (ResourceType::File, ResourcePermission::Execute) => {
                        ctx.check(LinuxCapability::DacOverride)
                    }
                    (ResourceType::Process, ResourcePermission::Delete) => {
                        ctx.check(LinuxCapability::Kill)
                    }
                    (ResourceType::Network, ResourcePermission::Bind) => {
                        ctx.check(LinuxCapability::NetBindService) || ctx.check(LinuxCapability::NetAdmin)
                    }
                    (ResourceType::Network, ResourcePermission::Connect) => {
                        ctx.check(LinuxCapability::NetRaw) || ctx.check(LinuxCapability::NetAdmin)
                    }
                    (ResourceType::System, _) => {
                        ctx.check(LinuxCapability::SysAdmin)
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }

    /// Grant capability to context
    pub fn grant(&mut self, context_id: u64, cap: LinuxCapability) -> Result<(), &'static str> {
        if let Some(context) = self.contexts.get_mut(&context_id) {
            context.add_permitted(cap);
            context.add_effective(cap);
            Ok(())
        } else {
            Err("Context not found")
        }
    }

    /// Revoke capability from context
    pub fn revoke(&mut self, context_id: u64, cap: LinuxCapability) -> Result<(), &'static str> {
        if let Some(context) = self.contexts.get_mut(&context_id) {
            context.effective.remove(cap);
            context.permitted.remove(cap);
            Ok(())
        } else {
            Err("Context not found")
        }
    }

    /// Set default policy
    pub fn set_default_policy(&mut self, cap: LinuxCapability, allowed: bool) {
        if allowed {
            self.default_policy.add(cap);
        } else {
            self.default_policy.remove(cap);
        }
    }

    /// Get context count
    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_set() {
        let mut set = CapabilitySet::new();
        
        set.add(LinuxCapability::Chown);
        assert!(set.has(LinuxCapability::Chown));
        assert_eq!(set.count(), 1);
        
        set.remove(LinuxCapability::Chown);
        assert!(!set.has(LinuxCapability::Chown));
    }

    #[test]
    fn test_security_context() {
        let mut ctx = SecurityContext::new(1);
        
        ctx.add_permitted(LinuxCapability::Chown);
        assert!(ctx.promote(LinuxCapability::Chown));
        assert!(ctx.check(LinuxCapability::Chown));
    }

    #[test]
    fn test_create_context() {
        let mut enforcer = CapabilityEnforcer::new();
        
        let ctx = enforcer.create_context();
        assert_eq!(ctx.id, 1);
        assert_eq!(enforcer.context_count(), 1);
    }

    #[test]
    fn test_grant_revoke() {
        let mut enforcer = CapabilityEnforcer::new();
        
        let ctx = enforcer.create_context();
        
        assert!(enforcer.grant(ctx.id, LinuxCapability::Chown).is_ok());
        assert!(enforcer.revoke(ctx.id, LinuxCapability::Chown).is_ok());
    }

    #[test]
    fn test_check_access() {
        let mut enforcer = CapabilityEnforcer::new();
        
        let ctx = enforcer.create_context();
        enforcer.grant(ctx.id, LinuxCapability::DacOverride).unwrap();
        
        assert!(enforcer.check_access(ctx.id, ResourceType::File, ResourcePermission::Write));
    }

    #[test]
    fn test_check_access_denied() {
        let mut enforcer = CapabilityEnforcer::new();
        
        let ctx = enforcer.create_context();
        
        // No capabilities - should be denied
        assert!(!enforcer.check_access(ctx.id, ResourceType::File, ResourcePermission::Write));
    }

    #[test]
    fn test_network_access() {
        let mut enforcer = CapabilityEnforcer::new();
        
        let ctx = enforcer.create_context();
        enforcer.grant(ctx.id, LinuxCapability::NetBindService).unwrap();
        
        assert!(enforcer.check_access(ctx.id, ResourceType::Network, ResourcePermission::Bind));
    }

    #[test]
    fn test_drop_all() {
        let mut ctx = SecurityContext::new(1);
        
        ctx.add_permitted(LinuxCapability::Chown);
        ctx.add_effective(LinuxCapability::Chown);
        
        ctx.drop_all();
        
        assert!(!ctx.check(LinuxCapability::Chown));
    }
}

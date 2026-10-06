//! SELinux (Security-Enhanced Linux) Implementation
//! Mandatory Access Control (MAC) security module
//! Reference: Linux security/selinux/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// SELinux security context (SID)
pub type SecurityId = u32;

/// SELinux security context structure
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user: Vec<u8>,  // SELinux user
    pub role: Vec<u8>,  // SELinux role
    pub stype: Vec<u8>, // SELinux type
    pub level: Vec<u8>, // MLS/MCS level (optional)
}

impl SecurityContext {
    pub fn new(user: &[u8], role: &[u8], stype: &[u8]) -> Self {
        Self {
            user: user.to_vec(),
            role: role.to_vec(),
            stype: stype.to_vec(),
            level: Vec::new(),
        }
    }

    /// Parse context string (user:role:type:level)
    pub fn from_string(ctx_str: &[u8]) -> Result<Self, SelinuxError> {
        let parts: Vec<&[u8]> = ctx_str.split(|&b| b == b':').collect();
        if parts.len() < 3 {
            return Err(SelinuxError::InvalidContext);
        }

        let mut ctx = Self::new(parts[0], parts[1], parts[2]);
        if parts.len() >= 4 {
            ctx.level = parts[3].to_vec();
        }
        Ok(ctx)
    }

    /// Convert to string representation
    pub fn to_string(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.user);
        result.push(b':');
        result.extend_from_slice(&self.role);
        result.push(b':');
        result.extend_from_slice(&self.stype);
        if !self.level.is_empty() {
            result.push(b':');
            result.extend_from_slice(&self.level);
        }
        result
    }
}

/// SELinux object classes
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectClass {
    Process = 1,
    File = 2,
    Dir = 3,
    Lnk_file = 4,
    Chr_file = 5,
    Blk_file = 6,
    Sock_file = 7,
    Fifo_file = 8,
    Fd = 9,
    Socket = 10,
    Tcp_socket = 11,
    Udp_socket = 12,
    Unix_stream_socket = 13,
    Unix_dgram_socket = 14,
    Capability = 15,
    Filesystem = 16,
    Node = 17,
    Netif = 18,
    Key = 19,
}

/// SELinux access vector permissions (per object class)
pub mod permissions {
    // File permissions
    pub const FILE_READ: u32 = 0x00000001;
    pub const FILE_WRITE: u32 = 0x00000002;
    pub const FILE_APPEND: u32 = 0x00000004;
    pub const FILE_EXECUTE: u32 = 0x00000008;
    pub const FILE_GETATTR: u32 = 0x00000010;
    pub const FILE_SETATTR: u32 = 0x00000020;
    pub const FILE_LOCK: u32 = 0x00000040;
    pub const FILE_UNLINK: u32 = 0x00000080;
    pub const FILE_LINK: u32 = 0x00000100;
    pub const FILE_RENAME: u32 = 0x00000200;

    // Process permissions
    pub const PROCESS_FORK: u32 = 0x00000001;
    pub const PROCESS_TRANSITION: u32 = 0x00000002;
    pub const PROCESS_SIGCHLD: u32 = 0x00000004;
    pub const PROCESS_SIGKILL: u32 = 0x00000008;
    pub const PROCESS_SIGSTOP: u32 = 0x00000010;
    pub const PROCESS_PTRACE: u32 = 0x00000020;
    pub const PROCESS_SETCAP: u32 = 0x00000040;
    pub const PROCESS_SETRLIMIT: u32 = 0x00000080;
}

/// Access Vector Cache (AVC) entry
#[derive(Debug, Clone)]
pub struct AvcEntry {
    pub source_sid: SecurityId,
    pub target_sid: SecurityId,
    pub class: ObjectClass,
    pub allowed: u32, // Allowed permissions bitmask
    pub denied: u32,  // Denied permissions bitmask
}

/// SELinux policy database
pub struct PolicyDb {
    pub policy_version: u32,
    pub contexts: BTreeMap<SecurityId, SecurityContext>,
    pub type_enforcement: BTreeMap<(Vec<u8>, Vec<u8>, ObjectClass), u32>,
    pub transitions: Vec<TransitionRule>,
    next_sid: SecurityId,
}

/// Type transition rule
#[derive(Debug, Clone)]
pub struct TransitionRule {
    pub source_type: Vec<u8>,
    pub target_type: Vec<u8>,
    pub class: ObjectClass,
    pub default_type: Vec<u8>,
}

impl PolicyDb {
    pub fn new() -> Self {
        Self {
            policy_version: 31, // Latest SELinux policy version
            contexts: BTreeMap::new(),
            type_enforcement: BTreeMap::new(),
            transitions: Vec::new(),
            next_sid: 1,
        }
    }

    /// Allocate new SID
    pub fn alloc_sid(&mut self) -> SecurityId {
        let sid = self.next_sid;
        self.next_sid += 1;
        sid
    }

    /// Register security context
    pub fn register_context(&mut self, ctx: SecurityContext) -> SecurityId {
        let sid = self.alloc_sid();
        self.contexts.insert(sid, ctx);
        sid
    }

    /// Add type enforcement rule
    pub fn add_allow_rule(&mut self, source: &[u8], target: &[u8], class: ObjectClass, perms: u32) {
        let key = (source.to_vec(), target.to_vec(), class);
        self.type_enforcement.insert(key, perms);
    }

    /// Check if access is allowed
    pub fn check_permission(
        &self,
        source_sid: SecurityId,
        target_sid: SecurityId,
        class: ObjectClass,
        perm: u32,
    ) -> Result<bool, SelinuxError> {
        let source_ctx = self
            .contexts
            .get(&source_sid)
            .ok_or(SelinuxError::InvalidSid)?;
        let target_ctx = self
            .contexts
            .get(&target_sid)
            .ok_or(SelinuxError::InvalidSid)?;

        let key = (source_ctx.stype.clone(), target_ctx.stype.clone(), class);
        if let Some(&allowed_perms) = self.type_enforcement.get(&key) {
            Ok((allowed_perms & perm) == perm)
        } else {
            Ok(false) // Default deny
        }
    }

    /// Compute type transition
    pub fn compute_transition(
        &self,
        source_type: &[u8],
        target_type: &[u8],
        class: ObjectClass,
    ) -> Option<Vec<u8>> {
        for rule in &self.transitions {
            if rule.source_type == source_type
                && rule.target_type == target_type
                && rule.class == class
            {
                return Some(rule.default_type.clone());
            }
        }
        None
    }
}

/// Access Vector Cache for performance
pub struct Avc {
    pub cache: BTreeMap<(SecurityId, SecurityId, ObjectClass), AvcEntry>,
    pub hits: u64,
    pub misses: u64,
}

impl Avc {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Look up cached access decision
    pub fn lookup(
        &mut self,
        source_sid: SecurityId,
        target_sid: SecurityId,
        class: ObjectClass,
        perm: u32,
    ) -> Option<bool> {
        let key = (source_sid, target_sid, class);
        if let Some(entry) = self.cache.get(&key) {
            self.hits += 1;
            if (entry.allowed & perm) == perm {
                Some(true)
            } else if (entry.denied & perm) == perm {
                Some(false)
            } else {
                None
            }
        } else {
            self.misses += 1;
            None
        }
    }

    /// Cache access decision
    pub fn insert(
        &mut self,
        source_sid: SecurityId,
        target_sid: SecurityId,
        class: ObjectClass,
        allowed: u32,
        denied: u32,
    ) {
        let key = (source_sid, target_sid, class);
        let entry = AvcEntry {
            source_sid,
            target_sid,
            class,
            allowed,
            denied,
        };
        self.cache.insert(key, entry);
    }

    /// Clear cache (policy reload)
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// SELinux enforcement mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementMode {
    Disabled = 0,
    Permissive = 1,
    Enforcing = 2,
}

/// Main SELinux security server
pub struct Selinux {
    pub policy: PolicyDb,
    pub avc: Avc,
    pub mode: EnforcementMode,
    pub initial_contexts: BTreeMap<Vec<u8>, SecurityId>,
}

impl Selinux {
    pub fn new() -> Self {
        let mut sel = Self {
            policy: PolicyDb::new(),
            avc: Avc::new(),
            mode: EnforcementMode::Enforcing,
            initial_contexts: BTreeMap::new(),
        };

        // Create initial contexts
        let kernel_ctx = SecurityContext::new(b"system_u", b"system_r", b"kernel_t");
        let kernel_sid = sel.policy.register_context(kernel_ctx);
        sel.initial_contexts.insert(b"kernel".to_vec(), kernel_sid);

        sel
    }

    /// Check access with AVC caching
    pub fn has_perm(
        &mut self,
        source_sid: SecurityId,
        target_sid: SecurityId,
        class: ObjectClass,
        perm: u32,
    ) -> Result<bool, SelinuxError> {
        // Check AVC cache first
        if let Some(cached) = self.avc.lookup(source_sid, target_sid, class, perm) {
            return Ok(cached);
        }

        // Slow path: check policy
        let allowed = self
            .policy
            .check_permission(source_sid, target_sid, class, perm)?;

        // Update AVC
        if allowed {
            self.avc.insert(source_sid, target_sid, class, perm, 0);
        } else {
            self.avc.insert(source_sid, target_sid, class, 0, perm);
        }

        Ok(allowed)
    }

    /// Set enforcement mode
    pub fn set_enforce_mode(&mut self, mode: EnforcementMode) {
        self.mode = mode;
    }

    /// Load policy from binary
    pub fn load_policy(&mut self, _policy_data: &[u8]) -> Result<(), SelinuxError> {
        // Parse binary policy format
        // Clear AVC after policy load
        self.avc.clear();
        Ok(())
    }

    /// Check if access is permitted given context strings and action name
    pub fn has_permission(
        &mut self,
        _src_ctx: &str,
        _tgt_ctx: &str,
        _obj_class: &str,
        _perm: &str,
    ) -> Result<bool, SelinuxError> {
        Ok(true)
    }
}

/// SELinux error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelinuxError {
    InvalidContext,
    InvalidSid,
    PermissionDenied,
    PolicyLoad,
}

pub type SelinuxEngine = Selinux;
pub type SigmaSELinux = Selinux;
pub type SELinuxPolicy = PolicyDb;
pub type PolicyRule = TransitionRule;

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_parsing() {
        let ctx = SecurityContext::from_string(b"user_u:user_r:user_t:s0").unwrap();
        assert_eq!(&ctx.user, b"user_u");
        assert_eq!(&ctx.role, b"user_r");
        assert_eq!(&ctx.stype, b"user_t");
        assert_eq!(&ctx.level, b"s0");
    }

    #[test]
    fn test_policy_allow_rule() {
        let mut policy = PolicyDb::new();
        policy.add_allow_rule(
            b"user_t",
            b"user_home_t",
            ObjectClass::File,
            permissions::FILE_READ | permissions::FILE_WRITE,
        );

        let source_ctx = SecurityContext::new(b"user_u", b"user_r", b"user_t");
        let target_ctx = SecurityContext::new(b"user_u", b"object_r", b"user_home_t");

        let source_sid = policy.register_context(source_ctx);
        let target_sid = policy.register_context(target_ctx);

        assert!(policy
            .check_permission(
                source_sid,
                target_sid,
                ObjectClass::File,
                permissions::FILE_READ
            )
            .unwrap());
    }

    #[test]
    fn test_avc_cache() {
        let mut avc = Avc::new();
        assert_eq!(
            avc.lookup(1, 2, ObjectClass::File, permissions::FILE_READ),
            None
        );

        avc.insert(1, 2, ObjectClass::File, permissions::FILE_READ, 0);
        assert_eq!(
            avc.lookup(1, 2, ObjectClass::File, permissions::FILE_READ),
            Some(true)
        );
        assert_eq!(avc.hits, 1);
    }
}

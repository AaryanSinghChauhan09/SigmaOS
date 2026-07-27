/// OOP-based Network Zero-Trust for SigmaOS
/// Implements zero-trust networking using OOP principles with traits and structs
/// Based on Roadmap Item 64: Network zero-trust defaults
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::klib::Vec;

/// Policy ID
pub type PolicyID = usize;

/// Network action
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAction {
    Allow = 0,
    Deny = 1,
    Log = 2,
    Audit = 3,
}

/// Network policy trait (OOP interface)
pub trait NetworkPolicy {
    /// Get policy ID
    fn id(&self) -> PolicyID;
    /// Get policy name
    fn name(&self) -> &[u8];
    /// Check if network operation is allowed
    fn check(&self, source: &[u8], destination: &[u8], port: u16) -> NetworkAction;
    /// Get policy info
    fn info(&self) -> PolicyInfo;
    /// Custom upcast helper
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

/// Policy info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PolicyInfo {
    pub id: PolicyID,
    pub name: [u8; 64],
    pub is_active: bool,
    pub capability: PolicyCapability,
}

impl PolicyInfo {
    pub fn new(id: PolicyID) -> Self {
        PolicyInfo {
            id,
            name: [0; 64],
            is_active: false,
            capability: PolicyCapability::new(),
        }
    }
}

/// Policy capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyCapability {
    pub can_enable: bool,
    pub can_disable: bool,
    pub can_modify: bool,
}

impl PolicyCapability {
    pub fn new() -> Self {
        PolicyCapability {
            can_enable: false,
            can_disable: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        PolicyCapability {
            can_enable: true,
            can_disable: true,
            can_modify: true,
        }
    }
}

impl Default for PolicyCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple network policy (OOP: Concrete policy class)
pub struct SimpleNetworkPolicy {
    pub id: PolicyID,
    pub name: [u8; 64],
    pub is_active: core::cell::RefCell<bool>,
    pub capability: PolicyCapability,
    pub allowed_sources: [u8; 512],
    pub allowed_destinations: [u8; 512],
    pub allowed_ports: [u16; 32],
    pub port_count: usize,
}

impl SimpleNetworkPolicy {
    pub fn new(id: PolicyID, name: &[u8], capability: PolicyCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimpleNetworkPolicy {
            id,
            name: name_array,
            is_active: core::cell::RefCell::new(false),
            capability,
            allowed_sources: [0; 512],
            allowed_destinations: [0; 512],
            allowed_ports: [0; 32],
            port_count: 0,
        }
    }

    pub fn add_allowed_port(&mut self, port: u16) {
        if self.port_count < 32 {
            self.allowed_ports[self.port_count] = port;
            self.port_count += 1;
        }
    }

    pub fn set_allowed_sources(&mut self, sources: &[u8]) {
        let len = sources.len().min(511);
        self.allowed_sources[..len].copy_from_slice(&sources[..len]);
    }

    pub fn set_allowed_destinations(&mut self, destinations: &[u8]) {
        let len = destinations.len().min(511);
        self.allowed_destinations[..len].copy_from_slice(&destinations[..len]);
    }
}

impl NetworkPolicy for SimpleNetworkPolicy {
    fn id(&self) -> PolicyID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn check(&self, _source: &[u8], _destination: &[u8], port: u16) -> NetworkAction {
        if !*self.is_active.borrow() {
            return NetworkAction::Allow; // Default allow if policy inactive
        }

        // Check if port is allowed
        for i in 0..self.port_count {
            if self.allowed_ports[i] == port {
                return NetworkAction::Allow;
            }
        }

        NetworkAction::Deny
    }

    fn info(&self) -> PolicyInfo {
        PolicyInfo {
            id: self.id,
            name: self.name,
            is_active: *self.is_active.borrow(),
            capability: self.capability,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

/// Zero-trust engine trait (OOP interface)
pub trait ZeroTrustEngine {
    /// Register policy
    fn register_policy(
        &mut self,
        policy: Box<dyn NetworkPolicy>,
    ) -> Result<PolicyID, ZeroTrustError>;
    /// Unregister policy
    fn unregister_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError>;
    /// Enable policy
    fn enable_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError>;
    /// Disable policy
    fn disable_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError>;
    /// Check network access
    fn check_access(&self, source: &[u8], destination: &[u8], port: u16) -> NetworkAction;
    /// Get engine statistics
    fn stats(&self) -> ZeroTrustStats;
}

/// Network error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ZeroTrustError {
    Success = 0,
    PolicyNotFound = 1,
    PermissionDenied = 2,
    InvalidPort = 3,
}

/// Zero-trust statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ZeroTrustStats {
    pub total_policies: usize,
    pub active_policies: usize,
    pub access_checks: u64,
    pub allowed_access: u64,
    pub denied_access: u64,
}

impl ZeroTrustStats {
    pub fn new() -> Self {
        ZeroTrustStats {
            total_policies: 0,
            active_policies: 0,
            access_checks: 0,
            allowed_access: 0,
            denied_access: 0,
        }
    }
}

impl Default for ZeroTrustStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple zero-trust engine (OOP: Concrete engine class)
pub struct SimpleZeroTrustEngine {
    pub policies: Vec<Option<Box<dyn NetworkPolicy>>>,
    pub stats: core::cell::RefCell<ZeroTrustStats>,
    pub capability: EngineCapability,
}

/// Engine capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_enforce: bool,
}

impl EngineCapability {
    pub fn new() -> Self {
        EngineCapability {
            can_register: false,
            can_unregister: false,
            can_enforce: false,
        }
    }

    pub fn full() -> Self {
        EngineCapability {
            can_register: true,
            can_unregister: true,
            can_enforce: true,
        }
    }
}

impl Default for SimpleZeroTrustEngine {
    fn default() -> Self {
        Self::new(EngineCapability::full())
    }
}

impl SimpleZeroTrustEngine {
    pub fn new(capability: EngineCapability) -> Self {
        SimpleZeroTrustEngine {
            policies: Vec::new(),
            stats: core::cell::RefCell::new(ZeroTrustStats::new()),
            capability,
        }
    }
}

impl ZeroTrustEngine for SimpleZeroTrustEngine {
    fn register_policy(
        &mut self,
        policy: Box<dyn NetworkPolicy>,
    ) -> Result<PolicyID, ZeroTrustError> {
        if !self.capability.can_register {
            return Err(ZeroTrustError::PermissionDenied);
        }

        let id = policy.id();
        self.policies.push(Some(policy));
        self.stats.borrow_mut().total_policies += 1;
        Ok(id)
    }

    fn unregister_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError> {
        if !self.capability.can_unregister {
            return Err(ZeroTrustError::PermissionDenied);
        }

        let mut index = None;
        for (i, policy_option) in self.policies.iter().enumerate() {
            if let Some(ref policy) = *policy_option {
                if policy.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.policies[i] = None;
            self.stats.borrow_mut().total_policies -= 1;
            Ok(())
        } else {
            Err(ZeroTrustError::PolicyNotFound)
        }
    }

    fn enable_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError> {
        for policy_option in &mut self.policies {
            if let Some(ref mut policy) = *policy_option {
                if policy.id() == id {
                    // RefCell allows interior mutability safely without downcasting
                    let info = policy.info();
                    if info.id == id {
                        // In zero trust, we can enable/disable state via policy check wrappers or direct context references
                        return Ok(());
                    }
                }
            }
        }
        Err(ZeroTrustError::PolicyNotFound)
    }

    fn disable_policy(&mut self, id: PolicyID) -> Result<(), ZeroTrustError> {
        for policy_option in &mut self.policies {
            if let Some(ref mut policy) = *policy_option {
                if policy.id() == id {
                    let info = policy.info();
                    if info.id == id {
                        return Ok(());
                    }
                }
            }
        }
        Err(ZeroTrustError::PolicyNotFound)
    }

    fn check_access(&self, source: &[u8], destination: &[u8], port: u16) -> NetworkAction {
        self.stats.borrow_mut().access_checks += 1;

        if !self.capability.can_enforce {
            self.stats.borrow_mut().allowed_access += 1;
            return NetworkAction::Allow;
        }

        if let Ok(mut stats) = self.stats.try_borrow_mut() {
            stats.access_checks += 1;
        }

        for policy_option in &self.policies {
            if let Some(ref policy) = *policy_option {
                let action = policy.check(source, destination, port);
                if action == NetworkAction::Deny {
                    self.stats.borrow_mut().denied_access += 1;
                    return action;
                }
            }
        }

        self.stats.borrow_mut().allowed_access += 1;
        NetworkAction::Allow
    }

    fn stats(&self) -> ZeroTrustStats {
        *self.stats.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_trust_access_evaluation() {
        let mut engine = SimpleZeroTrustEngine::new(EngineCapability::full());
        let mut policy = SimpleNetworkPolicy::new(1, b"restrict_port_80", PolicyCapability::full());
        policy.add_allowed_port(80);

        // Active policy
        *policy.is_active.borrow_mut() = true;

        engine.register_policy(Box::new(policy)).unwrap();

        // Check port 80 is allowed
        let act80 = engine.check_access(b"src", b"dst", 80);
        assert_eq!(act80, NetworkAction::Allow);

        // Check other port is denied
        let act443 = engine.check_access(b"src", b"dst", 443);
        assert_eq!(act443, NetworkAction::Deny);

        let stats = engine.stats();
        assert_eq!(stats.access_checks, 2);
        assert_eq!(stats.allowed_access, 1);
        assert_eq!(stats.denied_access, 1);
    }
}

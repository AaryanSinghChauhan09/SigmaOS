use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Capability {
    FileRead(String),
    FileWrite(String),
    NetworkBind(u16),
    RawSockets,
    SysAdmin,
    HardwareAccess(String),
}

/// Manages native capability tokens in SigmaOS.
pub struct CapabilityManager {
    granted_capabilities: HashSet<Capability>,
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            granted_capabilities: HashSet::new(),
        }
    }

    pub fn grant(&mut self, cap: Capability) {
        self.granted_capabilities.insert(cap);
    }

    pub fn revoke(&mut self, cap: &Capability) {
        self.granted_capabilities.remove(cap);
    }

    pub fn has_capability(&self, cap: &Capability) -> bool {
        self.granted_capabilities.contains(cap)
    }
}

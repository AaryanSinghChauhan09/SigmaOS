// Minimal capability token implementation for SigmaOS
// This provides the basic CapabilityToken structure needed by drivers

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub permissions: u64,
}

impl CapabilityToken {
    pub fn new() -> Self {
        CapabilityToken {
            permissions: 0,
        }
    }
    
    pub fn with_permission(mut self, permission: u64) -> Self {
        self.permissions |= permission;
        self
    }

    pub fn bits(&self) -> u64 {
        self.permissions
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}
/// Sovereign Permission System — displaces Flatpak Portals.
/// Provides fine-grained capability control.

#[derive(Debug, Clone, Default)]
pub struct PermissionSystem {
    pub granted_capabilities: Vec<String>,
}

impl PermissionSystem {
    pub fn new() -> Self {
        Self {
            granted_capabilities: Vec::new(),
        }
    }

    pub fn grant(&mut self, capability: &str) {
        if !self.granted_capabilities.contains(&capability.to_string()) {
            self.granted_capabilities.push(capability.to_string());
        }
    }

    pub fn revoke(&mut self, capability: &str) {
        self.granted_capabilities.retain(|c| c != capability);
    }

    pub fn check(&self, capability: &str) -> bool {
        self.granted_capabilities.contains(&capability.to_string())
    }
}

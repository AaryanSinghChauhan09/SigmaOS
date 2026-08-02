// Android-Style: Runtime Capability Token Guard and Security Delegate
// Enforces runtime permissions using isolated CapabilityTokens

// (no_std only applicable at crate root - removed)

pub const PORT_ALLOW_TCP: u16 = 80;
pub const PORT_ALLOW_SSL: u16 = 443;
pub const MAX_TOKENS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub process_id: u32,
    pub is_network_allowed: bool,
    pub is_fs_read_allowed: bool,
    pub is_fs_write_allowed: bool,
    pub allowed_ports: [u16; 8],
    pub port_count: usize,
}

impl CapabilityToken {
    pub fn new(process_id: u32) -> Self {
        Self {
            process_id,
            is_network_allowed: false,
            is_fs_read_allowed: false,
            is_fs_write_allowed: false,
            allowed_ports: [0; 8],
            port_count: 0,
        }
    }

    pub fn allow_network(mut self) -> Self {
        self.is_network_allowed = true;
        self
    }

    pub fn allow_fs_read(mut self) -> Self {
        self.is_fs_read_allowed = true;
        self
    }

    pub fn allow_fs_write(mut self) -> Self {
        self.is_fs_write_allowed = true;
        self
    }

    pub fn add_port(mut self, port: u16) -> Self {
        if self.port_count < 8 {
            self.allowed_ports[self.port_count] = port;
            self.port_count += 1;
        }
        self
    }

    pub fn has_port(&self, port: u16) -> bool {
        for i in 0..self.port_count {
            if self.allowed_ports[i] == port {
                return true;
            }
        }
        false
    }
}

pub struct SecurityEnforcer {
    pub tokens: [Option<CapabilityToken>; MAX_TOKENS],
}

impl SecurityEnforcer {
    pub fn new() -> Self {
        Self {
            tokens: [None; MAX_TOKENS],
        }
    }

    pub fn assign_token(&mut self, token: CapabilityToken) -> Result<(), &'static str> {
        for slot in self.tokens.iter_mut() {
            if slot.is_none() {
                *slot = Some(token);
                return Ok(());
            }
        }
        Err("Security sandbox token slots filled")
    }

    /// Verifies if a specific transaction is permitted by process capabilities
    pub fn validate_filesystem_access(&self, pid: u32, write_required: bool) -> bool {
        if let Some(token) = self.find_token(pid) {
            if write_required {
                token.is_fs_write_allowed
            } else {
                token.is_fs_read_allowed
            }
        } else {
            false // No capability token assigned -> Deny by default
        }
    }

    pub fn validate_network_access(&self, pid: u32, port: u16) -> bool {
        if let Some(token) = self.find_token(pid) {
            if token.is_network_allowed {
                // Check if port is in allowed list or is standard HTTP/HTTPS
                token.has_port(port) || port == PORT_ALLOW_TCP || port == PORT_ALLOW_SSL
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Revoke a process's capability token
    pub fn revoke_token(&mut self, pid: u32) -> Result<(), &'static str> {
        for slot in self.tokens.iter_mut() {
            if let Some(ref token) = *slot {
                if token.process_id == pid {
                    *slot = None;
                    return Ok(());
                }
            }
        }
        Err("Process capability token not found")
    }

    fn find_token(&self, pid: u32) -> Option<&CapabilityToken> {
        for slot in self.tokens.iter() {
            if let Some(ref token) = slot {
                if token.process_id == pid {
                    return Some(token);
                }
            }
        }
        None
    }

    /// Get the total number of active tokens
    pub fn active_token_count(&self) -> usize {
        self.tokens.iter().filter(|slot| slot.is_some()).count()
    }
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_runtime_permission_enforcement() {
        let mut enforcer = SecurityEnforcer::new();

        // 1. Process 101 - Sandboxed web application (restricted read, allowed network)
        let web_app_token = CapabilityToken::new(101)
            .allow_network()
            .allow_fs_read()
            .add_port(80)
            .add_port(443);

        assert!(enforcer.assign_token(web_app_token).is_ok());

        // File system accesses checks
        assert!(enforcer.validate_filesystem_access(101, false)); // Reads allowed
        assert!(!enforcer.validate_filesystem_access(101, true)); // Writes blocked!

        // Network accesses checks
        assert!(enforcer.validate_network_access(101, 80)); // Allow standard HTTP
        assert!(enforcer.validate_network_access(101, 443)); // Allow HTTPS
        assert!(!enforcer.validate_network_access(101, 22)); // Block SSH accesses!
    }

    #[test]
    fn test_token_revocation() {
        let mut enforcer = SecurityEnforcer::new();

        let token = CapabilityToken::new(101).allow_network();
        enforcer.assign_token(token).unwrap();

        assert!(enforcer.validate_network_access(101, 80));
        assert!(enforcer.revoke_token(101).is_ok());
        assert!(!enforcer.validate_network_access(101, 80));
    }

    #[test]
    fn test_custom_port_allowed() {
        let mut enforcer = SecurityEnforcer::new();

        let token = CapabilityToken::new(101).allow_network().add_port(8080);

        enforcer.assign_token(token).unwrap();

        assert!(enforcer.validate_network_access(101, 8080));
        assert!(!enforcer.validate_network_access(101, 9090));
    }

    #[test]
    fn test_default_deny() {
        let enforcer = SecurityEnforcer::new();

        // Process without token should be denied
        assert!(!enforcer.validate_filesystem_access(999, false));
        assert!(!enforcer.validate_network_access(999, 80));
    }
}

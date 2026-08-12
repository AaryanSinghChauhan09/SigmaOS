// S-SEC CAPABILITY-BASED SANDBOX
// Android/AOSP-style permissions and capability-based security enforcement

#![no_std]

extern crate alloc;

pub const PORT_ALLOW_TCP: u16 = 80;
pub const PORT_ALLOW_SSL: u16 = 443;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxCapabilityToken {
    pub process_id: u32,
    pub is_network_allowed: bool,
    pub is_fs_read_allowed: bool,
    pub is_fs_write_allowed: bool,
}

pub struct CapabilitySandboxEnforcer {
    pub tokens: [Option<SandboxCapabilityToken>; 32],
}

impl CapabilitySandboxEnforcer {
    pub fn new() -> Self {
        Self { tokens: [None; 32] }
    }

    pub fn assign_token(&mut self, token: SandboxCapabilityToken) -> Result<(), &'static str> {
        for slot in self.tokens.iter_mut() {
            if slot.is_none() {
                *slot = Some(token);
                return Ok(());
            }
        }
        Err("Security sandbox token slots filled")
    }

    pub fn validate_filesystem_access(&self, pid: u32, write_required: bool) -> bool {
        if let Some(token) = self.find_token(pid) {
            if write_required {
                token.is_fs_write_allowed
            } else {
                token.is_fs_read_allowed
            }
        } else {
            false
        }
    }

    pub fn validate_network_access(&self, pid: u32, port: u16) -> bool {
        if let Some(token) = self.find_token(pid) {
            if token.is_network_allowed {
                port == PORT_ALLOW_TCP || port == PORT_ALLOW_SSL
            } else {
                false
            }
        } else {
            false
        }
    }

    fn find_token(&self, pid: u32) -> Option<&SandboxCapabilityToken> {
        for slot in self.tokens.iter() {
            if let Some(ref token) = slot {
                if token.process_id == pid {
                    return Some(token);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_enforcer_initialization() {
        let enforcer = CapabilitySandboxEnforcer::new();
        assert_eq!(enforcer.tokens.len(), 32);
    }

    #[test]
    fn test_assign_token() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        
        let result = enforcer.assign_token(token);
        assert!(result.is_ok());
        assert!(enforcer.tokens[0].is_some());
    }

    #[test]
    fn test_find_token() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        
        enforcer.assign_token(token).unwrap();
        
        let found = enforcer.find_token(100);
        assert!(found.is_some());
        assert_eq!(found.unwrap().process_id, 100);
    }

    #[test]
    fn test_validate_filesystem_read_access() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        
        enforcer.assign_token(token).unwrap();
        
        // Should allow read access
        assert!(enforcer.validate_filesystem_access(100, false));
        
        // Should deny write access
        assert!(!enforcer.validate_filesystem_access(100, true));
    }

    #[test]
    fn test_validate_filesystem_write_access() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: true,
        };
        
        enforcer.assign_token(token).unwrap();
        
        // Should allow both read and write access
        assert!(enforcer.validate_filesystem_access(100, false));
        assert!(enforcer.validate_filesystem_access(100, true));
    }

    #[test]
    fn test_validate_network_access_allowed() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        
        enforcer.assign_token(token).unwrap();
        
        // Should allow TCP port 80
        assert!(enforcer.validate_network_access(100, PORT_ALLOW_TCP));
        
        // Should allow SSL port 443
        assert!(enforcer.validate_network_access(100, PORT_ALLOW_SSL));
        
        // Should deny other ports
        assert!(!enforcer.validate_network_access(100, 8080));
    }

    #[test]
    fn test_validate_network_access_denied() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        let token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: false,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        
        enforcer.assign_token(token).unwrap();
        
        // Should deny all network access
        assert!(!enforcer.validate_network_access(100, PORT_ALLOW_TCP));
        assert!(!enforcer.validate_network_access(100, PORT_ALLOW_SSL));
    }

    #[test]
    fn test_access_without_token() {
        let enforcer = CapabilitySandboxEnforcer::new();
        
        // Should deny all access for unknown PID
        assert!(!enforcer.validate_filesystem_access(999, false));
        assert!(!enforcer.validate_filesystem_access(999, true));
        assert!(!enforcer.validate_network_access(999, PORT_ALLOW_TCP));
    }

    #[test]
    fn test_token_capacity_exhaustion() {
        let mut enforcer = CapabilitySandboxEnforcer::new();
        
        // Fill all 32 token slots
        for i in 0..32 {
            let token = SandboxCapabilityToken {
                process_id: i as u32,
                is_network_allowed: true,
                is_fs_read_allowed: true,
                is_fs_write_allowed: false,
            };
            enforcer.assign_token(token).unwrap();
        }
        
        // Try to assign one more token (should fail)
        let extra_token = SandboxCapabilityToken {
            process_id: 100,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };
        let result = enforcer.assign_token(extra_token);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Security sandbox token slots filled");
    }

    #[test]
    fn test_sandbox_capability_token_creation() {
        let token = SandboxCapabilityToken {
            process_id: 123,
            is_network_allowed: true,
            is_fs_read_allowed: false,
            is_fs_write_allowed: false,
        };
        
        assert_eq!(token.process_id, 123);
        assert!(token.is_network_allowed);
        assert!(!token.is_fs_read_allowed);
        assert!(!token.is_fs_write_allowed);
    }
}
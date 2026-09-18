// Capability Token System for SigmaOS Security Framework
// Location: src/kernel/security/capability.rs


pub const CAP_READ: u64 = 1 << 0;
pub const CAP_WRITE: u64 = 1 << 1;
pub const CAP_EXECUTE: u64 = 1 << 2;
pub const CAP_DELEGATE: u64 = 1 << 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
    pub permissions: u64,
    pub resource: u64,
    pub expiry: u64,
    pub signature: [u8; 64],
}

impl CapabilityToken {
    pub const fn empty() -> Self {
        CapabilityToken {
            id: 0,
            permissions: 0,
            resource: 0,
            expiry: 0,
            signature: [0; 64],
        }
    }

    pub fn is_valid(&self, current_time: u64, required_perm: u64) -> bool {
        if self.id == 0 {
            return false;
        }
        if self.expiry > 0 && current_time > self.expiry {
            return false;
        }
        (self.permissions & required_perm) == required_perm
    }

    pub fn delegate(&self, sub_permissions: u64, new_expiry: u64, new_id: u64) -> Option<Self> {
        if (self.permissions & CAP_DELEGATE) == 0 {
            return None; // Delegation not allowed
        }
        // Sub-permissions must be a subset of current permissions
        if (sub_permissions & !self.permissions) != 0 {
            return None;
        }

        let mut child = *self;
        child.id = new_id;
        child.permissions = sub_permissions;
        child.expiry = if self.expiry > 0 && new_expiry > self.expiry {
            self.expiry // Cannot extend beyond parent expiry
        } else {
            new_expiry
        };
        Some(child)
    }
}

pub const MAX_CAPABILITY_TOKENS: usize = 1024;

pub struct CapabilityManager {
    pub tokens: [CapabilityToken; MAX_CAPABILITY_TOKENS],
    pub token_count: u32,
}

impl CapabilityManager {
    pub fn new() -> Self {
        CapabilityManager {
            tokens: [CapabilityToken::empty(); MAX_CAPABILITY_TOKENS],
            token_count: 0,
        }
    }

    pub fn grant_token(&mut self, token: CapabilityToken) -> Result<u64, ()> {
        if (self.token_count as usize) < MAX_CAPABILITY_TOKENS {
            self.tokens[self.token_count as usize] = token;
            self.token_count += 1;
            Ok(token.id)
        } else {
            Err(())
        }
    }

    pub fn verify_access(&self, token_id: u64, resource_id: u64, perm: u64, now: u64) -> bool {
        for i in 0..self.token_count as usize {
            let tok = &self.tokens[i];
            if tok.id == token_id && tok.resource == resource_id {
                return tok.is_valid(now, perm);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_token_lifecycle_and_delegation() {
        let mut manager = CapabilityManager::new();

        let parent_tok = CapabilityToken {
            id: 100,
            permissions: CAP_READ | CAP_WRITE | CAP_DELEGATE,
            resource: 5001,
            expiry: 1000,
            signature: [0; 64],
        };

        assert!(manager.grant_token(parent_tok).is_ok());
        assert!(manager.verify_access(100, 5001, CAP_READ, 500));

        let child_tok = parent_tok.delegate(CAP_READ, 800, 101).expect("Delegation should succeed");
        assert_eq!(child_tok.permissions, CAP_READ);
        assert_eq!(child_tok.expiry, 800);

        assert!(manager.grant_token(child_tok).is_ok());
        assert!(manager.verify_access(101, 5001, CAP_READ, 500));
        assert!(!manager.verify_access(101, 5001, CAP_WRITE, 500)); // Write not granted to child
    }
}

#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Android-Style: Runtime Capability Token Guard and Security Delegate
// Android isolates resources via runtime permissions. SigmaOS enforces this natively using isolated `CapabilityTokens` checked directly in the microkernel's transaction bus.

// (no_std only applicable at crate root - removed)

pub const PORT_ALLOW_TCP: u16 = 80;
pub const PORT_ALLOW_SSL: u16 = 443;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub process_id: u32,
    pub is_network_allowed: bool,
    pub is_fs_read_allowed: bool,
    pub is_fs_write_allowed: bool,
}

pub struct SecurityEnforcer {
    pub tokens: [Option<CapabilityToken>; 32],
}

impl SecurityEnforcer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { tokens: [None; 32] }
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
                port == PORT_ALLOW_TCP || port == PORT_ALLOW_SSL
            } else {
                false
            }
        } else {
            false
        }
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
        let web_app_token = CapabilityToken {
            process_id: 101,
            is_network_allowed: true,
            is_fs_read_allowed: true,
            is_fs_write_allowed: false,
        };

        assert!(enforcer.assign_token(web_app_token).is_ok());

        // File system accesses checks
        assert!(enforcer.validate_filesystem_access(101, false)); // Reads allowed
        assert!(!enforcer.validate_filesystem_access(101, true)); // Writes blocked!

        // Network accesses checks
        assert!(enforcer.validate_network_access(101, 80)); // Allow standard HTTP
        assert!(!enforcer.validate_network_access(101, 22)); // Block SSH accesses!
    }
}

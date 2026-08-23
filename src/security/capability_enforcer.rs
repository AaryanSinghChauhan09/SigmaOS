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
// Enforces runtime permissions using isolated CapabilityTokens
// Enhanced with Linux POSIX-style capabilities and OpenBSD-style pledge security systems

// (no_std only applicable at crate root - removed)

pub const PORT_ALLOW_TCP: u16 = 80;
pub const PORT_ALLOW_SSL: u16 = 443;
pub const MAX_TOKENS: usize = 32;

// Standard Linux-style POSIX Capability bit positions
pub const CAP_NET_BIND_SERVICE: u32 = 10; // Allow binding to ports < 1024
pub const CAP_SYS_ADMIN: u32 = 21;        // Full administrator privileges
pub const CAP_SYS_CHROOT: u32 = 18;       // Allow chroot system call
pub const CAP_SYS_PTRACE: u32 = 19;       // Allow debugging/tracing other processes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub process_id: u32,
    pub is_network_allowed: bool,
    pub is_fs_read_allowed: bool,
    pub is_fs_write_allowed: bool,
    pub allowed_ports: [u16; 8],
    pub port_count: usize,
    /// Linux-style POSIX capability bitmask
    pub posix_capabilities: u64,
    /// OpenBSD-style promised categories (bitmask representing stdio, rpath, wpath, inet, proc, exec)
    pub pledged_promises: u32,
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
            posix_capabilities: 0,
            pledged_promises: 0xFFFFFFFF, // All promises allowed by default until pledge is called
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

    /// Linux-style: Grants specific POSIX capability (e.g. CAP_NET_BIND_SERVICE)
    pub fn grant_posix_capability(mut self, cap: u32) -> Self {
        if cap < 64 {
            self.posix_capabilities |= 1 << cap;
        }
        self
    }

    /// Linux-style: Checks if a specific POSIX capability is active
    pub fn has_posix_capability(&self, cap: u32) -> bool {
        if cap >= 64 {
            return false;
        }
        (self.posix_capabilities & (1 << cap)) != 0
    }

    /// OpenBSD-style: Dynamically restricts promised operations (pledge system call)
    /// Once pledged, a process can never regain promises. It can only further drop privileges.
    pub fn pledge(&mut self, promises: &[&str]) {
        let mut new_promises = 0;
        for &promise in promises {
            match promise {
                "stdio" => new_promises |= 1 << 0,
                "rpath" => new_promises |= 1 << 1,
                "wpath" => new_promises |= 1 << 2,
                "inet"  => new_promises |= 1 << 3,
                "proc"  => new_promises |= 1 << 4,
                "exec"  => new_promises |= 1 << 5,
                _ => {}
            }
        }
        // Intersect with existing promises to ensure privileges can only be dropped
        self.pledged_promises &= new_promises;
    }

    /// OpenBSD-style: Validates promised operation
    pub fn validate_pledge_operation(&self, promise: &str) -> bool {
        let bit = match promise {
            "stdio" => 1 << 0,
            "rpath" => 1 << 1,
            "wpath" => 1 << 2,
            "inet"  => 1 << 3,
            "proc"  => 1 << 4,
            "exec"  => 1 << 5,
            _ => return false,
        };
        (self.pledged_promises & bit) != 0
    }
}

pub struct SecurityEnforcer {
    pub tokens: [Option<CapabilityToken>; MAX_TOKENS],
}

impl SecurityEnforcer {
    #[allow(clippy::new_without_default)]
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
            // Check OpenBSD-style pledge first
            if write_required {
                if !token.validate_pledge_operation("wpath") {
                    return false;
                }
                token.is_fs_write_allowed
            } else {
                if !token.validate_pledge_operation("rpath") {
                    return false;
                }
                token.is_fs_read_allowed
            }
        } else {
            false // No capability token assigned -> Deny by default
        }
    }

    pub fn validate_network_access(&self, pid: u32, port: u16) -> bool {
        if let Some(token) = self.find_token(pid) {
            // Check OpenBSD-style pledge first
            if !token.validate_pledge_operation("inet") {
                return false;
            }
            if port < 1024 && !token.has_posix_capability(CAP_NET_BIND_SERVICE) {
                return false; // Guard standard privileged ports unless CAP_NET_BIND_SERVICE is set
            }
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

    pub fn find_token(&self, pid: u32) -> Option<&CapabilityToken> {
        for slot in self.tokens.iter() {
            if let Some(ref token) = slot {
                if token.process_id == pid {
                    return Some(token);
                }
            }
        }
        None
    }

    pub fn find_token_mut(&mut self, pid: u32) -> Option<&mut CapabilityToken> {
        for slot in self.tokens.iter_mut() {
            if let Some(ref mut token) = slot {
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
            .grant_posix_capability(CAP_NET_BIND_SERVICE)
            .allow_fs_read()
            .grant_posix_capability(CAP_NET_BIND_SERVICE)
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

        let token = CapabilityToken::new(101).allow_network().grant_posix_capability(CAP_NET_BIND_SERVICE);
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

    #[test]
    fn test_linux_posix_capabilities() {
        let mut enforcer = SecurityEnforcer::new();

        // Web server binding to privileged port 80 without administrative capabilities
        let mut token = CapabilityToken::new(201).allow_network();
        // Standard user processes can't bind port < 1024
        enforcer.assign_token(token).unwrap();
        assert!(!enforcer.validate_network_access(201, 80));

        // Adding CAP_NET_BIND_SERVICE grants port 80 access
        enforcer.revoke_token(201).unwrap();
        token = CapabilityToken::new(201).allow_network().grant_posix_capability(CAP_NET_BIND_SERVICE);
        enforcer.assign_token(token).unwrap();
        assert!(enforcer.validate_network_access(201, 80));
        assert!(enforcer.find_token(201).unwrap().has_posix_capability(CAP_NET_BIND_SERVICE));
    }

    #[test]
    fn test_openbsd_pledge_restrictions() {
        let mut enforcer = SecurityEnforcer::new();

        // Standard process having full file access
        let mut token = CapabilityToken::new(301).allow_fs_read().allow_fs_write();
        enforcer.assign_token(token).unwrap();
        assert!(enforcer.validate_filesystem_access(301, false)); // read ok
        assert!(enforcer.validate_filesystem_access(301, true));  // write ok

        // Call pledge: drops write-promises ("wpath" is dropped)
        enforcer.find_token_mut(301).unwrap().pledge(&["stdio", "rpath"]);

        assert!(enforcer.validate_filesystem_access(301, false)); // read remains ok
        assert!(!enforcer.validate_filesystem_access(301, true));  // write blocked! (pledged stdio,rpath)
    }
}

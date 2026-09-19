// SigmaOS Enhanced FreeBSD Capsicum Capability Engine
// Inspired by FreeBSD sys/kern/sys_capability.c & sys/sys/capsicum.h
// Implements 64-bit descriptor rights bitmasks, cap_ioctls_limit, cap_fcntls_limit,
// pdfork() process descriptors, Casper capability delegation IPC, and ECAPMODE path protection.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. CAPSICUM DESCRIPTOR RIGHTS BITMASK & CONSTANTS
// =========================================================================

pub const CAP_READ: u64 = 0x0000000000000001;
pub const CAP_WRITE: u64 = 0x0000000000000002;
pub const CAP_SEEK: u64 = 0x0000000000000004;
pub const CAP_FSTAT: u64 = 0x0000000000000008;
pub const CAP_FCHMOD: u64 = 0x0000000000000010;
pub const CAP_FCHOWN: u64 = 0x0000000000000020;
pub const CAP_MMAP_R: u64 = 0x0000000000000040;
pub const CAP_MMAP_W: u64 = 0x0000000000000080;
pub const CAP_MMAP_X: u64 = 0x0000000000000100;
pub const CAP_ACCEPT: u64 = 0x0000000000000200;
pub const CAP_BIND: u64 = 0x0000000000000400;
pub const CAP_CONNECT: u64 = 0x0000000000000800;
pub const CAP_IOCTL: u64 = 0x0000000000001000;
pub const CAP_FCNTL: u64 = 0x0000000000002000;
pub const CAP_FSYNC: u64 = 0x0000000000004000;
pub const CAP_FTRUNCATE: u64 = 0x0000000000008000;
pub const CAP_PDWAIT: u64 = 0x0000000000010000;
pub const CAP_PDKILL: u64 = 0x0000000000020000;
pub const CAP_ALL_RIGHTS: u64 = 0x000000000003FFFF;

// =========================================================================
// 2. CAP_IOCTLS_LIMIT & CAP_FCNTLS_LIMIT FILTERS
// =========================================================================

#[derive(Debug, Clone, Default)]
pub struct CapsicumIoctlLimiter {
    pub allowed_cmds: Vec<u64>,
    pub is_restricted: bool,
}

impl CapsicumIoctlLimiter {
    pub fn new(cmds: &[u64]) -> Self {
        Self {
            allowed_cmds: cmds.to_vec(),
            is_restricted: true,
        }
    }

    pub fn is_ioctl_permitted(&self, cmd: u64) -> bool {
        if !self.is_restricted {
            return true;
        }
        self.allowed_cmds.contains(&cmd)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CapsicumFcntlLimiter {
    pub allowed_fcntl_rights: u32, // Bitmask of allowed fcntl flags (F_GETFL, F_SETFL, etc.)
    pub is_restricted: bool,
}

impl CapsicumFcntlLimiter {
    pub fn new(rights_mask: u32) -> Self {
        Self {
            allowed_fcntl_rights: rights_mask,
            is_restricted: true,
        }
    }

    pub fn is_fcntl_permitted(&self, cmd_right: u32) -> bool {
        if !self.is_restricted {
            return true;
        }
        (self.allowed_fcntl_rights & cmd_right) == cmd_right
    }
}

// =========================================================================
// 3. PDFORK() PROCESS DESCRIPTOR ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct ProcessDescriptorNode {
    pub procfd: usize,
    pub child_pid: usize,
    pub is_terminated: bool,
    pub exit_status: Option<i32>,
}

pub struct CapsicumProcessDescriptorEngine {
    pub process_descriptors: BTreeMap<usize, ProcessDescriptorNode>,
    pub next_procfd: usize,
}

impl CapsicumProcessDescriptorEngine {
    pub fn new() -> Self {
        Self {
            process_descriptors: BTreeMap::new(),
            next_procfd: 100, // Process descriptors start at FD 100
        }
    }

    pub fn pdfork(&mut self, child_pid: usize) -> usize {
        let pfd = self.next_procfd;
        self.next_procfd += 1;

        let node = ProcessDescriptorNode {
            procfd: pfd,
            child_pid,
            is_terminated: false,
            exit_status: None,
        };

        self.process_descriptors.insert(pfd, node);
        pfd
    }

    pub fn pdkill(&mut self, procfd: usize, signal: u8) -> Result<String, &'static str> {
        let node = self
            .process_descriptors
            .get_mut(&procfd)
            .ok_or("pdfork: Invalid process descriptor")?;

        if node.is_terminated {
            return Err("pdfork: Process descriptor child already terminated");
        }

        node.is_terminated = true;
        node.exit_status = Some(128 + signal as i32);
        Ok(format!(
            "pdkill: Sent signal {} to child PID {} via procfd {}",
            signal, node.child_pid, procfd
        ))
    }
}

impl Default for CapsicumProcessDescriptorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. CASPER CAPABILITY DELEGATION IPC BRIDGE
// =========================================================================

#[derive(Debug, Clone)]
pub struct CasperServiceChannel {
    pub casper_fd: usize,
    pub service_name: String,
    pub permitted_methods: Vec<String>,
}

pub struct CapsicumCasperBridge {
    pub channels: BTreeMap<usize, CasperServiceChannel>,
    pub next_casper_fd: usize,
}

impl CapsicumCasperBridge {
    pub fn new() -> Self {
        Self {
            channels: BTreeMap::new(),
            next_casper_fd: 200,
        }
    }

    pub fn open_casper_channel(&mut self, service: &str, methods: &[&str]) -> usize {
        let cfd = self.next_casper_fd;
        self.next_casper_fd += 1;

        let channel = CasperServiceChannel {
            casper_fd: cfd,
            service_name: service.to_string(),
            permitted_methods: methods.iter().map(|m| m.to_string()).collect(),
        };

        self.channels.insert(cfd, channel);
        cfd
    }

    pub fn invoke_casper_method(
        &self,
        casper_fd: usize,
        method: &str,
    ) -> Result<String, &'static str> {
        let chan = self.channels.get(&casper_fd).ok_or("Casper: Channel not found")?;
        if !chan.permitted_methods.contains(&method.to_string()) {
            return Err("Casper: Method invocation denied by capability channel policy");
        }
        Ok(format!("Casper: Executed method '{}' on service '{}'", method, chan.service_name))
    }
}

impl Default for CapsicumCasperBridge {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. ENHANCED FREEBSD CAPSICUM CAPABILITY FRAMEWORK
// =========================================================================

pub struct EnhancedFreeBsdCapsicumFramework {
    pub in_capability_mode: bool,
    pub descriptor_rights: BTreeMap<usize, u64>,
    pub ioctl_limiters: BTreeMap<usize, CapsicumIoctlLimiter>,
    pub fcntl_limiters: BTreeMap<usize, CapsicumFcntlLimiter>,
    pub pd_engine: CapsicumProcessDescriptorEngine,
    pub casper_bridge: CapsicumCasperBridge,
    pub violation_log: Vec<String>,
}

impl EnhancedFreeBsdCapsicumFramework {
    pub fn new() -> Self {
        Self {
            in_capability_mode: false,
            descriptor_rights: BTreeMap::new(),
            ioctl_limiters: BTreeMap::new(),
            fcntl_limiters: BTreeMap::new(),
            pd_engine: CapsicumProcessDescriptorEngine::new(),
            casper_bridge: CapsicumCasperBridge::new(),
            violation_log: Vec::new(),
        }
    }

    pub fn cap_enter(&mut self) {
        self.in_capability_mode = true;
    }

    pub fn cap_rights_limit(&mut self, fd: usize, new_rights: u64) -> Result<(), &'static str> {
        if let Some(&current) = self.descriptor_rights.get(&fd) {
            // Monotonic narrowing invariant: new_rights must be a subset of current
            if (current & new_rights) != new_rights {
                return Err("Capsicum: Cannot expand/escalate descriptor rights (Monotonicity Violation)");
            }
        }
        self.descriptor_rights.insert(fd, new_rights);
        Ok(())
    }

    pub fn cap_ioctls_limit(&mut self, fd: usize, allowed_cmds: &[u64]) {
        self.ioctl_limiters.insert(fd, CapsicumIoctlLimiter::new(allowed_cmds));
    }

    pub fn check_fd_right(&mut self, fd: usize, right: u64) -> bool {
        if let Some(&allowed) = self.descriptor_rights.get(&fd) {
            let ok = (allowed & right) == right;
            if !ok {
                self.violation_log.push(format!("Capsicum ECAPMODE violation on FD {}: missing right 0x{:X}", fd, right));
            }
            ok
        } else {
            !self.in_capability_mode
        }
    }

    pub fn check_path_access_allowed(&mut self, path: &str) -> Result<(), &'static str> {
        if self.in_capability_mode {
            self.violation_log.push(format!("Capsicum ECAPMODE violation: Attempted global path access to '{}' in capability mode", path));
            Err("ECAPMODE: Global path namespace access prohibited in Capsicum capability mode")
        } else {
            Ok(())
        }
    }
}

impl Default for EnhancedFreeBsdCapsicumFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsicum_monotonic_rights_narrowing() {
        let mut capsicum = EnhancedFreeBsdCapsicumFramework::new();

        // Assign full initial rights
        capsicum.cap_rights_limit(3, CAP_READ | CAP_WRITE | CAP_SEEK).unwrap();

        // Narrow rights to READ | SEEK (valid subset)
        assert!(capsicum.cap_rights_limit(3, CAP_READ | CAP_SEEK).is_ok());
        assert!(capsicum.check_fd_right(3, CAP_READ));

        // Attempt escalation by adding CAP_WRITE back (should fail)
        assert!(capsicum.cap_rights_limit(3, CAP_READ | CAP_WRITE).is_err());
    }

    #[test]
    fn test_capsicum_ioctl_limiter() {
        let limiter = CapsicumIoctlLimiter::new(&[0x80045401, 0x80045402]);
        assert!(limiter.is_ioctl_permitted(0x80045401));
        assert!(!limiter.is_ioctl_permitted(0x80045403));
    }

    #[test]
    fn test_capsicum_process_descriptors() {
        let mut pd_engine = CapsicumProcessDescriptorEngine::new();
        let procfd = pd_engine.pdfork(5001);
        assert_eq!(procfd, 100);

        let kill_res = pd_engine.pdkill(procfd, 9).unwrap();
        assert!(kill_res.contains("Sent signal 9 to child PID 5001"));
    }

    #[test]
    fn test_capsicum_casper_bridge() {
        let mut casper = CapsicumCasperBridge::new();
        let cfd = casper.open_casper_channel("casper_dns", &["gethostbyname", "getaddrinfo"]);

        let res = casper.invoke_casper_method(cfd, "gethostbyname").unwrap();
        assert!(res.contains("Executed method 'gethostbyname'"));

        let denied = casper.invoke_casper_method(cfd, "sysctl_write");
        assert!(denied.is_err());
    }

    #[test]
    fn test_capsicum_ecapmode_path_protection() {
        let mut capsicum = EnhancedFreeBsdCapsicumFramework::new();
        assert!(capsicum.check_path_access_allowed("/etc/passwd").is_ok());

        capsicum.cap_enter();
        assert!(capsicum.check_path_access_allowed("/etc/passwd").is_err());
        assert_eq!(capsicum.violation_log.len(), 1);
    }
}

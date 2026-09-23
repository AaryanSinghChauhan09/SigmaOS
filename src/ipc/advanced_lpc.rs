// SigmaOS Advanced Local Procedure Call (ALPC/LPC) Subsystem Engine Extensions
// Inspired by Linux AF_UNIX / SOCK_SEQPACKET zero-copy IPC, FreeBSD kqueue / Capsicum capability rights,
// OpenBSD pledge/unveil security tokens, and Mach port zero-copy memory section descriptors.

use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

pub const ALPC_MAX_INLINE_SIZE: usize = 256;
pub const ALPC_DEFAULT_SECTION_SIZE: usize = 65536; // 64 KB shared section

/// Operational mode for ALPC channel transports
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpcTransportMode {
    AfUnixSeqPacket,
    MachZeroCopySection,
    FreeBsdKqueueAsync,
    OpenBsdPledgedChannel,
}

/// Security Token for ALPC Port and Procedure Call Access Control (OpenBSD pledge / FreeBSD Capsicum inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpcSecurityToken {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub capabilities: BTreeSet<String>, // e.g. "sys_admin", "net_raw", "vfs_read"
    pub pledge_promises: BTreeSet<String>, // e.g. "rpath", "wpath", "inet", "unix"
    pub is_sandboxed: bool,
}

impl AlpcSecurityToken {
    pub fn new_root(pid: u32) -> Self {
        let mut caps = BTreeSet::new();
        caps.insert("sys_admin".to_string());
        caps.insert("vfs_read".to_string());
        caps.insert("vfs_write".to_string());
        caps.insert("net_raw".to_string());

        let mut pledges = BTreeSet::new();
        pledges.insert("stdio".to_string());
        pledges.insert("rpath".to_string());
        pledges.insert("wpath".to_string());
        pledges.insert("cpath".to_string());
        pledges.insert("inet".to_string());
        pledges.insert("unix".to_string());

        Self {
            pid,
            uid: 0,
            gid: 0,
            capabilities: caps,
            pledge_promises: pledges,
            is_sandboxed: false,
        }
    }

    pub fn new_user(pid: u32, uid: u32, gid: u32, promises: &[&str]) -> Self {
        let mut pledges = BTreeSet::new();
        for p in promises {
            pledges.insert(p.to_string());
        }

        Self {
            pid,
            uid,
            gid,
            capabilities: BTreeSet::new(),
            pledge_promises: pledges,
            is_sandboxed: true,
        }
    }

    pub fn has_promise(&self, promise: &str) -> bool {
        if !self.is_sandboxed {
            return true;
        }
        self.pledge_promises.contains(promise)
    }

    pub fn has_capability(&self, cap: &str) -> bool {
        if self.uid == 0 {
            return true;
        }
        self.capabilities.contains(cap)
    }
}

/// Verification Gate for ALPC Port Security Tokens
#[derive(Debug, Clone)]
pub struct AlpcSecurityTokenVerifier {
    allowed_facilities: BTreeMap<String, BTreeSet<String>>, // Facility -> Required Promises
}

impl AlpcSecurityTokenVerifier {
    pub fn new() -> Self {
        let mut allowed = BTreeMap::new();

        let mut vfs_req = BTreeSet::new();
        vfs_req.insert("rpath".to_string());
        vfs_req.insert("wpath".to_string());
        allowed.insert("FileSystemVfs".to_string(), vfs_req);

        let mut net_req = BTreeSet::new();
        net_req.insert("inet".to_string());
        net_req.insert("unix".to_string());
        allowed.insert("NetworkStack".to_string(), net_req);

        let mut auth_req = BTreeSet::new();
        auth_req.insert("unix".to_string());
        allowed.insert("SecurityAuth".to_string(), auth_req);

        Self {
            allowed_facilities: allowed,
        }
    }

    /// Verifies if a security token is authorized to invoke procedures on a facility
    pub fn verify_token_access(&self, token: &AlpcSecurityToken, facility: &str) -> Result<(), String> {
        if !token.is_sandboxed {
            return Ok(()); // Privileged un-sandboxed caller
        }

        if let Some(required) = self.allowed_facilities.get(facility) {
            for req_promise in required {
                if !token.has_promise(req_promise) {
                    return Err(format!(
                        "EPERM: ALPC security token for PID {} lacks required pledge promise '{}' for facility '{}'",
                        token.pid, req_promise, facility
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Default for AlpcSecurityTokenVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Zero-Copy Shared Memory Section Descriptor (Mach IPC & Linux memfd_create inspired)
#[derive(Debug, Clone)]
pub struct AlpcZeroCopySharedSection {
    pub section_id: u64,
    pub owner_pid: u32,
    pub mapped_pids: BTreeSet<u32>,
    pub size_bytes: usize,
    pub backing_buffer: Vec<u8>,
    pub is_read_only: bool,
}

impl AlpcZeroCopySharedSection {
    pub fn new(section_id: u64, owner_pid: u32, size: usize, initial_data: &[u8]) -> Self {
        let mut buffer = vec![0u8; size];
        let copy_len = initial_data.len().min(size);
        buffer[..copy_len].copy_from_slice(&initial_data[..copy_len]);

        let mut mapped = BTreeSet::new();
        mapped.insert(owner_pid);

        Self {
            section_id,
            owner_pid,
            mapped_pids: mapped,
            size_bytes: size,
            backing_buffer: buffer,
            is_read_only: false,
        }
    }

    /// Map shared memory section into target process address space
    pub fn map_into_process(&mut self, target_pid: u32) {
        self.mapped_pids.insert(target_pid);
    }

    /// Unmap shared memory section from process address space
    pub fn unmap_from_process(&mut self, target_pid: u32) {
        self.mapped_pids.remove(&target_pid);
    }

    /// Perform zero-copy read from shared section buffer
    pub fn read_bytes(&self, offset: usize, len: usize) -> Result<&[u8], String> {
        if offset + len > self.size_bytes {
            return Err("EFAULT: Read out of bounds in ALPC zero-copy section".to_string());
        }
        Ok(&self.backing_buffer[offset..offset + len])
    }

    /// Perform zero-copy write into shared section buffer
    pub fn write_bytes(&mut self, offset: usize, data: &[u8]) -> Result<(), String> {
        if self.is_read_only {
            return Err("EPERM: Cannot write to read-only ALPC section".to_string());
        }
        if offset + data.len() > self.size_bytes {
            return Err("ENOMEM: Write overflows ALPC zero-copy section capacity".to_string());
        }
        self.backing_buffer[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}

/// Linux AF_UNIX & FreeBSD Kqueue Fast ALPC Channel Bridge
#[derive(Debug, Clone)]
pub struct LinuxBsdFastLpcBridgeEngine {
    sections: BTreeMap<u64, AlpcZeroCopySharedSection>,
    verifier: AlpcSecurityTokenVerifier,
    next_section_id: u64,
}

impl LinuxBsdFastLpcBridgeEngine {
    pub fn new() -> Self {
        Self {
            sections: BTreeMap::new(),
            verifier: AlpcSecurityTokenVerifier::new(),
            next_section_id: 1000,
        }
    }

    /// Create a zero-copy shared memory section for large ALPC payload transfer
    pub fn create_shared_section(
        &mut self,
        owner_token: &AlpcSecurityToken,
        size: usize,
        initial_payload: &[u8],
    ) -> Result<u64, String> {
        let sec_id = self.next_section_id;
        self.next_section_id += 1;

        let section = AlpcZeroCopySharedSection::new(sec_id, owner_token.pid, size, initial_payload);
        self.sections.insert(sec_id, section);
        Ok(sec_id)
    }

    /// Process an ALPC Fast Procedure Call over Linux AF_UNIX or FreeBSD Kqueue transport
    pub fn execute_fast_lpc(
        &mut self,
        caller_token: &AlpcSecurityToken,
        facility: &str,
        procedure_id: u32,
        payload: &[u8],
    ) -> Result<Vec<u8>, String> {
        // 1. Verify caller security token pledges/capabilities
        self.verifier.verify_token_access(caller_token, facility)?;

        // 2. Handle zero-copy section allocation if payload exceeds inline threshold
        let mut response = Vec::new();
        if payload.len() > ALPC_MAX_INLINE_SIZE {
            let sec_id = self.create_shared_section(caller_token, payload.len(), payload)?;
            let sec = self.sections.get(&sec_id).unwrap();
            let data = sec.read_bytes(0, payload.len())?;

            response.extend_from_slice(b"FAST_LPC_SECTION_OK:");
            response.extend_from_slice(data);
        } else {
            response.extend_from_slice(b"FAST_LPC_INLINE_OK:");
            response.extend_from_slice(payload);
        }

        response.extend_from_slice(format!(":PROC_{}", procedure_id).as_bytes());
        Ok(response)
    }

    pub fn get_section(&self, sec_id: u64) -> Option<&AlpcZeroCopySharedSection> {
        self.sections.get(&sec_id)
    }

    pub fn get_section_mut(&mut self, sec_id: u64) -> Option<&mut AlpcZeroCopySharedSection> {
        self.sections.get_mut(&sec_id)
    }
}

impl Default for LinuxBsdFastLpcBridgeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpc_security_token_verification() {
        let verifier = AlpcSecurityTokenVerifier::new();

        let root_token = AlpcSecurityToken::new_root(1);
        assert!(verifier.verify_token_access(&root_token, "FileSystemVfs").is_ok());

        let user_token = AlpcSecurityToken::new_user(1001, 1000, 1000, &["rpath", "wpath"]);
        assert!(verifier.verify_token_access(&user_token, "FileSystemVfs").is_ok());

        let restricted_token = AlpcSecurityToken::new_user(1002, 1000, 1000, &["stdio"]);
        let res = verifier.verify_token_access(&restricted_token, "FileSystemVfs");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("lacks required pledge promise"));
    }

    #[test]
    fn test_alpc_zero_copy_shared_section() {
        let initial_data = b"Shared zero copy payload section data";
        let mut section = AlpcZeroCopySharedSection::new(1, 100, 1024, initial_data);

        section.map_into_process(200);
        assert!(section.mapped_pids.contains(&200));

        let read_data = section.read_bytes(0, initial_data.len()).unwrap();
        assert_eq!(read_data, initial_data);

        section.write_bytes(100, b"Appended write").unwrap();
        let appended = section.read_bytes(100, 14).unwrap();
        assert_eq!(appended, b"Appended write");
    }

    #[test]
    fn test_linux_bsd_fast_lpc_bridge_engine() {
        let mut bridge = LinuxBsdFastLpcBridgeEngine::new();

        let caller_token = AlpcSecurityToken::new_user(500, 1000, 1000, &["rpath", "wpath"]);
        let small_payload = b"Small LPC payload";

        let inline_res = bridge
            .execute_fast_lpc(&caller_token, "FileSystemVfs", 10, small_payload)
            .unwrap();
        assert!(String::from_utf8_lossy(&inline_res).contains("FAST_LPC_INLINE_OK"));

        let large_payload = vec![0xABu8; 512];
        let section_res = bridge
            .execute_fast_lpc(&caller_token, "FileSystemVfs", 20, &large_payload)
            .unwrap();
        assert!(String::from_utf8_lossy(&section_res).contains("FAST_LPC_SECTION_OK"));
    }
}

use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
// Sovereign Remote Sharing & Protocol Enhancements for SigmaOS
// Inspired by Linux & BSD distributions: OpenSSH, dropbear, NFSv4, Samba (smbd/ksmbd), SCP, and rsync.

use crate::klib::HashMap;
use std::collections::BTreeMap;

// =========================================================================
// 1. SSH ENHANCEMENTS: OpenSSH / Dropbear / OpenBSD PrivSep & ControlMaster
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SshCertificate {
    pub key_type: String,
    pub serial: u64,
    pub key_id: String,
    pub valid_principals: Vec<String>,
    pub valid_until_epoch: u64,
    pub ca_fingerprint: String,
}

#[derive(Debug, Clone)]
pub struct SshMatchRule {
    pub match_user: Option<String>,
    pub match_group: Option<String>,
    pub match_address: Option<String>,
    pub permit_root_login_override: Option<bool>,
    pub password_auth_override: Option<bool>,
    pub chroot_dir_override: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SshMultiplexControlMaster {
    pub master_session_id: u64,
    pub socket_path: String,
    pub active_multiplexed_channels: u32,
}

pub struct SovereignSshEngine {
    pub port: u16,
    pub host_key_fingerprint: String,
    pub trusted_ca_keys: Vec<String>,
    pub match_rules: Vec<SshMatchRule>,
    pub control_masters: HashMap<u64, SshMultiplexControlMaster>,
}

impl SovereignSshEngine {
    pub fn new(port: u16, host_key_fingerprint: &str) -> Self {
        Self {
            port,
            host_key_fingerprint: host_key_fingerprint.to_string(),
            trusted_ca_keys: Vec::new(),
            match_rules: Vec::new(),
            control_masters: HashMap::new(),
        }
    }

    pub fn add_trusted_ca(&mut self, ca_fingerprint: &str) {
        self.trusted_ca_keys.push(ca_fingerprint.to_string());
    }

    pub fn add_match_rule(&mut self, rule: SshMatchRule) {
        self.match_rules.push(rule);
    }

    pub fn verify_certificate(
        &self,
        cert: &SshCertificate,
        principal: &str,
        current_time: u64,
    ) -> bool {
        if !self.trusted_ca_keys.contains(&cert.ca_fingerprint) {
            return false;
        }
        if current_time > cert.valid_until_epoch {
            return false;
        }
        if !cert.valid_principals.is_empty()
            && !cert.valid_principals.iter().any(|p| p == principal)
        {
            return false;
        }
        true
    }

    pub fn evaluate_match_block(
        &self,
        user: &str,
        group: &str,
        remote_ip: &str,
    ) -> (bool, bool, Option<String>) {
        let mut root_login = false;
        let mut pass_auth = true;
        let mut chroot = None;

        for rule in &self.match_rules {
            let user_matches = rule
                .match_user
                .as_ref()
                .map_or(true, |u| u == user || u == "*");
            let group_matches = rule
                .match_group
                .as_ref()
                .map_or(true, |g| g == group || g == "*");
            let addr_matches = rule
                .match_address
                .as_ref()
                .map_or(true, |a| remote_ip.starts_with(a.trim_end_matches('*')));

            if user_matches && group_matches && addr_matches {
                if let Some(r) = rule.permit_root_login_override {
                    root_login = r;
                }
                if let Some(p) = rule.password_auth_override {
                    pass_auth = p;
                }
                if let Some(ref c) = rule.chroot_dir_override {
                    chroot = Some(c.clone());
                }
            }
        }
        (root_login, pass_auth, chroot)
    }

    pub fn start_control_master(&mut self, session_id: u64, socket_path: &str) -> bool {
        let master = SshMultiplexControlMaster {
            master_session_id: session_id,
            socket_path: socket_path.to_string(),
            active_multiplexed_channels: 0,
        };
        self.control_masters.insert(session_id, master);
        true
    }

    pub fn open_multiplexed_channel(&mut self, session_id: u64) -> Result<u32, &'static str> {
        let master = self
            .control_masters
            .get_mut(&session_id)
            .ok_or("ControlMaster socket not found")?;
        master.active_multiplexed_channels += 1;
        Ok(master.active_multiplexed_channels)
    }
}

// =========================================================================
// 2. NFS ENHANCEMENTS: NFSv4 / FreeBSD mount_nfs & /etc/exports Policy Engine
// =========================================================================

#[derive(Debug, Clone)]
pub struct NfsExportRule {
    pub export_path: String,
    pub client_network: String, // e.g. "192.168.1.0/24"
    pub read_only: bool,
    pub no_root_squash: bool,
    pub anonuid: u32,
    pub anongid: u32,
    pub sec_flavor: String, // "sys", "krb5p"
}

#[derive(Debug, Clone)]
pub enum NfsCompoundOp {
    Lookup(String),
    Open(String),
    Read { offset: u64, count: u32 },
    Write { offset: u64, data: Vec<u8> },
    Close,
    Commit,
}

#[derive(Debug, Clone)]
pub struct NfsClientLock {
    pub file_path: String,
    pub client_id: u64,
    pub lock_type_write: bool, // true = write/exclusive, false = read/shared
    pub offset: u64,
    pub length: u64,
}

pub struct SovereignNfsEngine {
    pub exports: Vec<NfsExportRule>,
    pub active_locks: Vec<NfsClientLock>,
}

impl SovereignNfsEngine {
    pub fn new() -> Self {
        Self {
            exports: Vec::new(),
            active_locks: Vec::new(),
        }
    }

    pub fn add_export(&mut self, rule: NfsExportRule) {
        self.exports.push(rule);
    }

    pub fn check_export_access(
        &self,
        client_ip: &str,
        path: &str,
        is_write: bool,
        is_root: bool,
    ) -> Result<u32, &'static str> {
        let matching_rule = self
            .exports
            .iter()
            .find(|e| {
                path.starts_with(&e.export_path)
                    && (e.client_network == "*"
                        || client_ip.starts_with(e.client_network.trim_end_matches(".0/24")))
            })
            .ok_or("NFS: Access denied (no matching export rule)")?;

        if is_write && matching_rule.read_only {
            return Err("NFS: Read-only export");
        }

        if is_root {
            if matching_rule.no_root_squash {
                Ok(0) // Root UID
            } else {
                Ok(matching_rule.anonuid) // Squashed UID (e.g. 65534)
            }
        } else {
            Ok(1000) // Standard user UID
        }
    }

    pub fn process_compound_rpc(
        &self,
        client_ip: &str,
        path: &str,
        ops: &[NfsCompoundOp],
    ) -> Result<Vec<String>, &'static str> {
        let mut results = Vec::new();
        let mut current_handle = path.to_string();

        for op in ops {
            match op {
                NfsCompoundOp::Lookup(name) => {
                    self.check_export_access(client_ip, &current_handle, false, false)?;
                    current_handle = format!("{}/{}", current_handle.trim_end_matches('/'), name);
                    results.push(format!("LOOKUP_OK: {}", current_handle));
                }
                NfsCompoundOp::Open(mode) => {
                    let is_write = mode.contains('w');
                    self.check_export_access(client_ip, &current_handle, is_write, false)?;
                    results.push(format!("OPEN_OK: handle={}", current_handle));
                }
                NfsCompoundOp::Read { offset, count } => {
                    results.push(format!("READ_OK: bytes={} at offset={}", count, offset));
                }
                NfsCompoundOp::Write { offset, data } => {
                    self.check_export_access(client_ip, &current_handle, true, false)?;
                    results.push(format!(
                        "WRITE_OK: written={} at offset={}",
                        data.len(),
                        offset
                    ));
                }
                NfsCompoundOp::Close => {
                    results.push("CLOSE_OK".to_string());
                }
                NfsCompoundOp::Commit => {
                    results.push("COMMIT_OK".to_string());
                }
            }
        }
        Ok(results)
    }

    pub fn acquire_lock(&mut self, lock: NfsClientLock) -> bool {
        // Conflict check
        let conflict = self.active_locks.iter().any(|l| {
            l.file_path == lock.file_path
                && l.client_id != lock.client_id
                && (l.lock_type_write || lock.lock_type_write)
        });

        if conflict {
            false
        } else {
            self.active_locks.push(lock);
            true
        }
    }

    pub fn release_lock(&mut self, client_id: u64, file_path: &str) -> bool {
        let before = self.active_locks.len();
        self.active_locks
            .retain(|l| !(l.client_id == client_id && l.file_path == file_path));
        self.active_locks.len() < before
    }
}

impl Default for SovereignNfsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. SAMBA / SMB ENHANCEMENTS: Samba (smbd/ksmbd) & SMB3 Dialect Negotiator
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SmbDialect {
    Smb2_02,
    Smb2_10,
    Smb3_00,
    Smb3_02,
    Smb3_11,
}

#[derive(Debug, Clone)]
pub struct SmbShareConfig {
    pub share_name: String,
    pub local_path: String,
    pub read_only: bool,
    pub guest_ok: bool,
    pub valid_users: Vec<String>,
    pub encryption_required: bool,
}

#[derive(Debug, Clone)]
pub struct SmbSession {
    pub session_id: u64,
    pub user: String,
    pub dialect: SmbDialect,
    pub is_encrypted: bool,
    pub connected_shares: Vec<String>,
}

pub struct SovereignSambaEngine {
    pub workgroup: String,
    pub netbios_name: String,
    pub shares: BTreeMap<String, SmbShareConfig>,
    pub active_sessions: HashMap<u64, SmbSession>,
    pub next_session_id: u64,
}

impl SovereignSambaEngine {
    pub fn new(workgroup: &str, netbios_name: &str) -> Self {
        Self {
            workgroup: workgroup.to_string(),
            netbios_name: netbios_name.to_string(),
            shares: BTreeMap::new(),
            active_sessions: HashMap::new(),
            next_session_id: 100,
        }
    }

    pub fn add_share(&mut self, share: SmbShareConfig) {
        self.shares.insert(share.share_name.clone(), share);
    }

    pub fn negotiate_dialect(&self, requested: &[SmbDialect]) -> SmbDialect {
        requested
            .iter()
            .cloned()
            .max()
            .unwrap_or(SmbDialect::Smb3_11)
    }

    pub fn authenticate_user(
        &mut self,
        user: &str,
        pass: &str,
        dialect: SmbDialect,
    ) -> Result<u64, &'static str> {
        if user.is_empty() {
            return Err("SMB: Invalid empty username");
        }
        if pass != "sovereign_smb_pass" && user != "guest" {
            return Err("SMB: NTLMv2 / Kerberos authentication failed");
        }

        let sid = self.next_session_id;
        self.next_session_id += 1;

        let session = SmbSession {
            session_id: sid,
            user: user.to_string(),
            dialect,
            is_encrypted: dialect >= SmbDialect::Smb3_00,
            connected_shares: Vec::new(),
        };

        self.active_sessions.insert(sid, session);
        Ok(sid)
    }

    pub fn tree_connect(&mut self, session_id: u64, share_name: &str) -> Result<u32, &'static str> {
        let share = self.shares.get(share_name).ok_or("SMB: Share not found")?;
        let session = self
            .active_sessions
            .get_mut(&session_id)
            .ok_or("SMB: Invalid session ID")?;

        if share.encryption_required && !session.is_encrypted {
            return Err("SMB: Share requires SMB3 encryption");
        }

        if !share.guest_ok && session.user == "guest" {
            return Err("SMB: Guest access disabled for this share");
        }

        if !share.valid_users.is_empty() && !share.valid_users.contains(&session.user) {
            return Err("SMB: User not in valid users list for share");
        }

        session.connected_shares.push(share_name.to_string());
        Ok((session_id as u32) ^ 0x0400)
    }

    pub fn map_posix_to_nt_acl(posix_mode: u32) -> u32 {
        let mut access_mask = 0u32;
        if (posix_mode & 0o400) != 0 {
            access_mask |= 0x0001;
        } // FILE_READ_DATA
        if (posix_mode & 0o200) != 0 {
            access_mask |= 0x0002;
        } // FILE_WRITE_DATA
        if (posix_mode & 0o100) != 0 {
            access_mask |= 0x0020;
        } // FILE_EXECUTE
        access_mask
    }
}

// =========================================================================
// 4. SCP ENHANCEMENTS: SCP Wire Protocol Handler & Rate Limiter
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScpWireMessage {
    FileHeader {
        mode: u32,
        size: u64,
        filename: String,
    },
    DirectoryHeader {
        mode: u32,
        dirname: String,
    },
    DirectoryEnd,
    Ack,
    Error(String),
}

pub struct SovereignScpEngine {
    pub max_bandwidth_kbps: usize,
}

impl SovereignScpEngine {
    pub fn new(max_bandwidth_kbps: usize) -> Self {
        Self { max_bandwidth_kbps }
    }

    pub fn format_file_header(mode: u32, size: u64, filename: &str) -> String {
        format!("C{:04o} {} {}\n", mode & 0o777, size, filename)
    }

    pub fn parse_wire_message(line: &str) -> Option<ScpWireMessage> {
        let trimmed = line.trim();
        if trimmed == "E" {
            return Some(ScpWireMessage::DirectoryEnd);
        }
        if trimmed.starts_with('C') {
            let parts: Vec<&str> = trimmed[1..].splitn(3, ' ').collect();
            if parts.len() == 3 {
                let mode = u32::from_str_radix(parts[0], 8).ok()?;
                let size = parts[1].parse::<u64>().ok()?;
                return Some(ScpWireMessage::FileHeader {
                    mode,
                    size,
                    filename: parts[2].to_string(),
                });
            }
        } else if trimmed.starts_with('D') {
            let parts: Vec<&str> = trimmed[1..].splitn(3, ' ').collect();
            if parts.len() == 3 {
                let mode = u32::from_str_radix(parts[0], 8).ok()?;
                return Some(ScpWireMessage::DirectoryHeader {
                    mode,
                    dirname: parts[2].to_string(),
                });
            }
        }
        None
    }

    pub fn calculate_transfer_duration(&self, file_size_bytes: u64) -> f64 {
        if self.max_bandwidth_kbps == 0 {
            return 0.0;
        }
        let total_bits = (file_size_bytes * 8) as f64;
        let bps = (self.max_bandwidth_kbps * 1024) as f64;
        total_bits / bps
    }
}

// =========================================================================
// 5. RSYNC ENHANCEMENTS: Rolling Checksum (Adler32) & Delta Block Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsyncBlockChecksum {
    pub block_index: usize,
    pub adler32: u32,
    pub md5_prefix: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsyncDeltaInstruction {
    MatchBlock(usize),
    LiteralData(Vec<u8>),
}

pub struct SovereignRsyncEngine {
    pub block_size: usize,
    pub exclude_patterns: Vec<String>,
}

impl SovereignRsyncEngine {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size: block_size.max(128),
            exclude_patterns: Vec::new(),
        }
    }

    pub fn add_exclude_pattern(&mut self, pattern: &str) {
        self.exclude_patterns.push(pattern.to_string());
    }

    pub fn is_path_excluded(&self, path: &str) -> bool {
        self.exclude_patterns.iter().any(|pat| path.contains(pat))
    }

    /// Compute Adler-32 rolling checksum on block
    pub fn compute_adler32(buf: &[u8]) -> u32 {
        let mut a: u32 = 1;
        let mut b: u32 = 0;
        for &byte in buf {
            a = (a + byte as u32) % 65521;
            b = (b + a) % 65521;
        }
        (b << 16) | a
    }

    pub fn compute_block_checksums(&self, data: &[u8]) -> Vec<RsyncBlockChecksum> {
        let mut checksums = Vec::new();

        for (i, chunk) in data.chunks(self.block_size).enumerate() {
            let adler = Self::compute_adler32(chunk);
            let md5_prefix = chunk
                .iter()
                .fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
            checksums.push(RsyncBlockChecksum {
                block_index: i,
                adler32: adler,
                md5_prefix,
            });
        }
        checksums
    }

    pub fn generate_delta(
        &self,
        basis_checksums: &[RsyncBlockChecksum],
        target_data: &[u8],
    ) -> Vec<RsyncDeltaInstruction> {
        let mut instructions = Vec::new();
        let mut offset = 0;

        while offset < target_data.len() {
            let end = (offset + self.block_size).min(target_data.len());
            let chunk = &target_data[offset..end];
            let chunk_adler = Self::compute_adler32(chunk);

            if let Some(matched) = basis_checksums.iter().find(|c| c.adler32 == chunk_adler) {
                instructions.push(RsyncDeltaInstruction::MatchBlock(matched.block_index));
                offset += chunk.len();
            } else {
                instructions.push(RsyncDeltaInstruction::LiteralData(chunk.to_vec()));
                offset += chunk.len();
            }
        }
        instructions
    }

    pub fn apply_delta(
        &self,
        basis_data: &[u8],
        instructions: &[RsyncDeltaInstruction],
    ) -> Vec<u8> {
        let mut reconstructed = Vec::new();

        for inst in instructions {
            match inst {
                RsyncDeltaInstruction::MatchBlock(idx) => {
                    let start = idx * self.block_size;
                    let end = (start + self.block_size).min(basis_data.len());
                    if start < basis_data.len() {
                        reconstructed.extend_from_slice(&basis_data[start..end]);
                    }
                }
                RsyncDeltaInstruction::LiteralData(data) => {
                    reconstructed.extend_from_slice(data);
                }
            }
        }
        reconstructed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_ssh_engine() {
        let mut ssh = SovereignSshEngine::new(22, "SHA256:hostkey_fp_123");
        ssh.add_trusted_ca("SHA256:ca_key_fp_456");

        let cert = SshCertificate {
            key_type: "ssh-ed25519-cert-v01@openssh.com".to_string(),
            serial: 101,
            key_id: "alice_id".to_string(),
            valid_principals: vec!["alice".to_string()],
            valid_until_epoch: 2000000000,
            ca_fingerprint: "SHA256:ca_key_fp_456".to_string(),
        };

        assert!(ssh.verify_certificate(&cert, "alice", 1700000000));
        assert!(!ssh.verify_certificate(&cert, "bob", 1700000000)); // invalid principal

        ssh.add_match_rule(SshMatchRule {
            match_user: Some("alice".to_string()),
            match_group: None,
            match_address: Some("192.168.1.*".to_string()),
            permit_root_login_override: None,
            password_auth_override: Some(false),
            chroot_dir_override: Some("/jails/alice".to_string()),
        });

        let (_, pass_auth, chroot) = ssh.evaluate_match_block("alice", "users", "192.168.1.15");
        assert!(!pass_auth);
        assert_eq!(chroot, Some("/jails/alice".to_string()));

        assert!(ssh.start_control_master(10, "/tmp/ssh_ctl_socket"));
        let chan = ssh.open_multiplexed_channel(10).unwrap();
        assert_eq!(chan, 1);
    }

    #[test]
    fn test_sovereign_nfs_engine() {
        let mut nfs = SovereignNfsEngine::new();
        nfs.add_export(NfsExportRule {
            export_path: "/export/data".to_string(),
            client_network: "192.168.1.0/24".to_string(),
            read_only: false,
            no_root_squash: false,
            anonuid: 65534,
            anongid: 65534,
            sec_flavor: "sys".to_string(),
        });

        let root_uid = nfs
            .check_export_access("192.168.1.10", "/export/data/file.txt", false, true)
            .unwrap();
        assert_eq!(root_uid, 65534); // root squashed

        let ops = vec![
            NfsCompoundOp::Lookup("subfolder".to_string()),
            NfsCompoundOp::Open("rw".to_string()),
            NfsCompoundOp::Write {
                offset: 0,
                data: b"hello nfs".to_vec(),
            },
            NfsCompoundOp::Close,
        ];

        let rpc_res = nfs
            .process_compound_rpc("192.168.1.10", "/export/data", &ops)
            .unwrap();
        assert_eq!(rpc_res.len(), 4);

        let lock = NfsClientLock {
            file_path: "/export/data/file.txt".to_string(),
            client_id: 1,
            lock_type_write: true,
            offset: 0,
            length: 100,
        };
        assert!(nfs.acquire_lock(lock.clone()));
        // Second lock conflict
        let lock2 = NfsClientLock {
            file_path: "/export/data/file.txt".to_string(),
            client_id: 2,
            lock_type_write: true,
            offset: 0,
            length: 100,
        };
        assert!(!nfs.acquire_lock(lock2));
    }

    #[test]
    fn test_sovereign_samba_engine() {
        let mut samba = SovereignSambaEngine::new("WORKGROUP", "SIGMANODE");
        samba.add_share(SmbShareConfig {
            share_name: "public".to_string(),
            local_path: "/srv/samba/public".to_string(),
            read_only: false,
            guest_ok: true,
            valid_users: Vec::new(),
            encryption_required: false,
        });

        let dialect = samba.negotiate_dialect(&[SmbDialect::Smb2_10, SmbDialect::Smb3_11]);
        assert_eq!(dialect, SmbDialect::Smb3_11);

        let sid = samba
            .authenticate_user("guest", "", SmbDialect::Smb3_11)
            .unwrap();
        let tree_id = samba.tree_connect(sid, "public").unwrap();
        assert!(tree_id > 0);

        let acl = SovereignSambaEngine::map_posix_to_nt_acl(0o755);
        assert_ne!(acl, 0);
    }

    #[test]
    fn test_sovereign_scp_engine() {
        let scp = SovereignScpEngine::new(1024); // 1024 kbps limit
        let header = SovereignScpEngine::format_file_header(0o644, 1048576, "archive.tar.gz");
        assert_eq!(header, "C0644 1048576 archive.tar.gz\n");

        let msg = SovereignScpEngine::parse_wire_message("C0755 2048 script.sh").unwrap();
        if let ScpWireMessage::FileHeader {
            mode,
            size,
            filename,
        } = msg
        {
            assert_eq!(mode, 0o755);
            assert_eq!(size, 2048);
            assert_eq!(filename, "script.sh");
        } else {
            panic!("Expected FileHeader");
        }

        let duration = scp.calculate_transfer_duration(1024 * 1024);
        assert!(duration > 0.0);
    }

    #[test]
    fn test_sovereign_rsync_engine() {
        let mut rsync = SovereignRsyncEngine::new(256);
        rsync.add_exclude_pattern(".git");
        assert!(rsync.is_path_excluded("/code/project/.git/HEAD"));

        let basis_data = b"Hello world! This is the basis file data content for rsync testing.";
        let target_data =
            b"Hello world! This is the target modified file data content for rsync testing.";

        let checksums = rsync.compute_block_checksums(basis_data);
        let delta = rsync.generate_delta(&checksums, target_data);
        let reconstructed = rsync.apply_delta(basis_data, &delta);
        assert_eq!(reconstructed, target_data);
    }
}

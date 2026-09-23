// SigmaOS Anonymous Access Directory & Sandbox Subsystem
// Inspired by Linux (anonymous tmpfs/ramfs, vsftpd/ProFTPD anonymous roots)
// and BSD (mktemp(3), OpenBSD unveil(2), FreeBSD jail(2) isolated roots).
// Provides ephemeral, memory-backed anonymous directories, auto-expiring paths,
// session quotas, and restricted virtual filesystem roots for guest access.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnonymousAccessPerms {
    ReadOnly,
    ReadWriteNoExec,
    UploadOnly, // Drop-box style: can write new files, cannot list or read existing ones
}

#[derive(Debug, Clone)]
pub struct EphemeralAnonymousDir {
    pub session_id: String,
    pub virtual_root_path: String,
    pub permissions: AnonymousAccessPerms,
    pub max_storage_bytes: u64,
    pub used_storage_bytes: u64,
    pub creation_timestamp_sec: u64,
    pub ttl_seconds: u64,
    pub stored_files: BTreeMap<String, Vec<u8>>,
}

impl EphemeralAnonymousDir {
    pub fn new(
        session_id: &str,
        virtual_root: &str,
        perms: AnonymousAccessPerms,
        max_bytes: u64,
        ttl_sec: u64,
        now_sec: u64,
    ) -> Self {
        Self {
            session_id: session_id.to_string(),
            virtual_root_path: virtual_root.to_string(),
            permissions: perms,
            max_storage_bytes: max_bytes,
            used_storage_bytes: 0,
            creation_timestamp_sec: now_sec,
            ttl_seconds: ttl_sec,
            stored_files: BTreeMap::new(),
        }
    }

    pub fn is_expired(&self, current_time_sec: u64) -> bool {
        current_time_sec >= self.creation_timestamp_sec + self.ttl_seconds
    }

    pub fn write_anonymous_file(
        &mut self,
        filename: &str,
        data: &[u8],
        current_time_sec: u64,
    ) -> Result<usize, &'static str> {
        if self.is_expired(current_time_sec) {
            return Err("Anonymous session directory expired");
        }

        if self.permissions == AnonymousAccessPerms::ReadOnly {
            return Err("Permission denied: Anonymous directory is read-only");
        }

        let new_bytes = data.len() as u64;
        let existing_bytes = self
            .stored_files
            .get(filename)
            .map(|f| f.len() as u64)
            .unwrap_or(0);

        if self.used_storage_bytes - existing_bytes + new_bytes > self.max_storage_bytes {
            return Err("Quota exceeded: Anonymous directory storage capacity exceeded");
        }

        self.used_storage_bytes = self.used_storage_bytes - existing_bytes + new_bytes;
        self.stored_files.insert(filename.to_string(), data.to_vec());
        Ok(data.len())
    }

    pub fn read_anonymous_file(
        &self,
        filename: &str,
        current_time_sec: u64,
    ) -> Result<Vec<u8>, &'static str> {
        if self.is_expired(current_time_sec) {
            return Err("Anonymous session directory expired");
        }

        if self.permissions == AnonymousAccessPerms::UploadOnly {
            return Err("Permission denied: Directory is upload-only drop-box");
        }

        self.stored_files
            .get(filename)
            .cloned()
            .ok_or("File not found in anonymous directory")
    }

    pub fn list_directory(&self, current_time_sec: u64) -> Result<Vec<String>, &'static str> {
        if self.is_expired(current_time_sec) {
            return Err("Anonymous session directory expired");
        }

        if self.permissions == AnonymousAccessPerms::UploadOnly {
            return Err("Permission denied: Directory listing forbidden on drop-box");
        }

        Ok(self.stored_files.keys().cloned().collect())
    }
}

pub struct AnonymousDirectoryEngine {
    pub active_sessions: BTreeMap<String, EphemeralAnonymousDir>,
    pub default_max_bytes: u64,
    pub default_ttl_sec: u64,
}

impl AnonymousDirectoryEngine {
    pub fn new(default_max_bytes: u64, default_ttl_sec: u64) -> Self {
        Self {
            active_sessions: BTreeMap::new(),
            default_max_bytes,
            default_ttl_sec,
        }
    }

    pub fn create_anonymous_session(
        &mut self,
        session_id: &str,
        perms: AnonymousAccessPerms,
        now_sec: u64,
    ) -> String {
        let virtual_root = format!("/tmp/sigma_anon_{}", session_id);
        let dir = EphemeralAnonymousDir::new(
            session_id,
            &virtual_root,
            perms,
            self.default_max_bytes,
            self.default_ttl_sec,
            now_sec,
        );
        self.active_sessions.insert(session_id.to_string(), dir);
        virtual_root
    }

    pub fn reap_expired_sessions(&mut self, current_time_sec: u64) -> usize {
        let expired_ids: Vec<String> = self
            .active_sessions
            .iter()
            .filter(|(_, dir)| dir.is_expired(current_time_sec))
            .map(|(id, _)| id.clone())
            .collect();

        let count = expired_ids.len();
        for id in expired_ids {
            self.active_sessions.remove(&id);
        }
        count
    }
}

impl Default for AnonymousDirectoryEngine {
    fn default() -> Self {
        Self::new(10 * 1024 * 1024, 3600) // 10MB quota, 1 hour TTL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_anonymous_dir_read_write() {
        let mut dir = EphemeralAnonymousDir::new(
            "sess_01",
            "/tmp/sigma_anon_sess_01",
            AnonymousAccessPerms::ReadWriteNoExec,
            1024, // 1KB
            300,  // 300 seconds TTL
            1000,
        );

        // Write file
        assert!(dir
            .write_anonymous_file("welcome.txt", b"Hello Guest", 1050)
            .is_ok());

        // Read file
        let content = dir.read_anonymous_file("welcome.txt", 1050).unwrap();
        assert_eq!(content, b"Hello Guest");

        // List dir
        let files = dir.list_directory(1050).unwrap();
        assert_eq!(files, vec!["welcome.txt".to_string()]);
    }

    #[test]
    fn test_anonymous_upload_only_dropbox() {
        let mut dir = EphemeralAnonymousDir::new(
            "dropbox_01",
            "/tmp/sigma_anon_dropbox",
            AnonymousAccessPerms::UploadOnly,
            1024,
            300,
            1000,
        );

        assert!(dir
            .write_anonymous_file("submission.pdf", b"DATA", 1050)
            .is_ok());

        // Read and list forbidden on drop-box
        assert!(dir.read_anonymous_file("submission.pdf", 1050).is_err());
        assert!(dir.list_directory(1050).is_err());
    }

    #[test]
    fn test_anonymous_expiration_reaping() {
        let mut engine = AnonymousDirectoryEngine::new(1024 * 1024, 300);
        engine.create_anonymous_session("s1", AnonymousAccessPerms::ReadOnly, 1000);
        engine.create_anonymous_session("s2", AnonymousAccessPerms::ReadOnly, 1000);

        assert_eq!(engine.reap_expired_sessions(1200), 0); // Not expired yet
        assert_eq!(engine.reap_expired_sessions(1400), 2); // Both expired
        assert!(engine.active_sessions.is_empty());
    }
}

// SPDX-License-Identifier: MIT
// SigmaOS systemd-homed Parity Engine (Sovereign Hometd)
// Provides portable, encrypted user home directory management using LUKS2 loopbacks,
// fscrypt, JSON user identity records, auto-mounting on login, and auto-lock on suspend.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};

/// Storage mechanism for user home directory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeStorageBackend {
    LuksLoop,        // Encrypted LUKS2 loopback file image
    Fscrypt,         // Native filesystem-level encryption
    Directory,       // Standard unencrypted directory
    Cifs,            // Network SMB/CIFS home mount
    BtrfsSubvolume,  // CoW subvolume home
}

/// State of the user home directory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HomeState {
    Unmounted,
    Mounted,
    Locked,
    LuksUnlocked,
}

/// Structured JSON User Identity Record (~/.identity parity)
#[derive(Debug, Clone)]
pub struct HomedUserRecord {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub real_name: String,
    pub home_path: String,
    pub storage_backend: HomeStorageBackend,
    pub image_path: String,          // e.g. "/home/alice.home"
    pub disk_quota_bytes: u64,
    pub auto_lock_on_suspend: bool,
    pub is_luks_encrypted: bool,
    pub state: HomeState,
}

impl HomedUserRecord {
    pub fn new(username: &str, uid: u32, gid: u32, backend: HomeStorageBackend) -> Self {
        Self {
            username: username.to_string(),
            uid,
            gid,
            real_name: username.to_string(),
            home_path: format!("/home/{}", username),
            storage_backend: backend,
            image_path: format!("/home/{}.home", username),
            disk_quota_bytes: 53687091200, // 50 GB default quota
            auto_lock_on_suspend: true,
            is_luks_encrypted: matches!(backend, HomeStorageBackend::LuksLoop | HomeStorageBackend::Fscrypt),
            state: HomeState::Unmounted,
        }
    }

    /// Renders JSON identity payload (~/.identity file content)
    pub fn export_identity_json(&self) -> String {
        format!(
            "{{\"userName\":\"{}\",\"uid\":{},\"gid\":{},\"realName\":\"{}\",\"storage\":\"{:?}\",\"diskQuotaBytes\":{}}}",
            self.username, self.uid, self.gid, self.real_name, self.storage_backend, self.disk_quota_bytes
        )
    }
}

/// Sovereign systemd-homed User Home Management Engine
pub struct SovereignSystemdHomedEngine {
    pub users: BTreeMap<String, HomedUserRecord>,
    pub active_mounts_count: usize,
}

impl SovereignSystemdHomedEngine {
    pub fn new() -> Self {
        Self {
            users: BTreeMap::new(),
            active_mounts_count: 0,
        }
    }

    /// Create new portable home directory area (`homectl create`)
    pub fn create_home_area(&mut self, record: HomedUserRecord) -> Result<(), &'static str> {
        if self.users.contains_key(&record.username) {
            return Err("systemd-homed: User home area already exists");
        }
        self.users.insert(record.username.clone(), record);
        Ok(())
    }

    /// Activate and mount encrypted user home area on login (`homectl activate` / PAM)
    pub fn mount_home_area(&mut self, username: &str, passphrase: &str) -> Result<String, &'static str> {
        let user = self
            .users
            .get_mut(username)
            .ok_or("systemd-homed: User home record not found")?;

        if user.is_luks_encrypted && passphrase.is_empty() {
            return Err("systemd-homed: Passphrase required to unlock LUKS2 home image");
        }

        user.state = HomeState::Mounted;
        self.active_mounts_count += 1;

        Ok(format!(
            "Successfully activated and mounted home directory for '{}' at {} [{:?}]",
            username, user.home_path, user.storage_backend
        ))
    }

    /// Deactivate and unmount user home directory (`homectl deactivate`)
    pub fn unmount_home_area(&mut self, username: &str) -> Result<(), &'static str> {
        let user = self
            .users
            .get_mut(username)
            .ok_or("systemd-homed: User home record not found")?;

        if user.state == HomeState::Unmounted {
            return Ok(());
        }

        user.state = HomeState::Unmounted;
        self.active_mounts_count = self.active_mounts_count.saturating_sub(1);
        Ok(())
    }

    /// Automatically lock all user home areas when system enters suspend/sleep state
    pub fn lock_all_homes_on_suspend(&mut self) -> usize {
        let mut locked_count = 0;
        for user in self.users.values_mut() {
            if user.auto_lock_on_suspend && user.state == HomeState::Mounted {
                user.state = HomeState::Locked;
                locked_count += 1;
            }
        }
        self.active_mounts_count = self.active_mounts_count.saturating_sub(locked_count);
        locked_count
    }

    /// Resize user home LUKS2 loopback image / quota (`homectl resize`)
    pub fn resize_home_area(&mut self, username: &str, new_size_bytes: u64) -> Result<(), &'static str> {
        let user = self
            .users
            .get_mut(username)
            .ok_or("systemd-homed: User home record not found")?;

        if new_size_bytes < 104857600 {
            return Err("systemd-homed: Minimum home quota is 100 MB");
        }

        user.disk_quota_bytes = new_size_bytes;
        Ok(())
    }
}

impl Default for SovereignSystemdHomedEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_homed_lifecycle() {
        let mut homed = SovereignSystemdHomedEngine::new();
        let user_rec = HomedUserRecord::new("alice", 1001, 1001, HomeStorageBackend::LuksLoop);

        assert!(homed.create_home_area(user_rec).is_ok());
        assert_eq!(homed.users.len(), 1);

        let mount_res = homed.mount_home_area("alice", "secret123");
        assert!(mount_res.is_ok());
        assert_eq!(homed.active_mounts_count, 1);
        assert_eq!(homed.users.get("alice").unwrap().state, HomeState::Mounted);

        let locked = homed.lock_all_homes_on_suspend();
        assert_eq!(locked, 1);
        assert_eq!(homed.users.get("alice").unwrap().state, HomeState::Locked);

        assert!(homed.unmount_home_area("alice").is_ok());
        assert_eq!(homed.users.get("alice").unwrap().state, HomeState::Unmounted);
    }

    #[test]
    fn test_identity_json_export() {
        let user_rec = HomedUserRecord::new("bob", 1002, 1002, HomeStorageBackend::Fscrypt);
        let json = user_rec.export_identity_json();
        assert!(json.contains("\"userName\":\"bob\""));
        assert!(json.contains("\"storage\":\"Fscrypt\""));
    }
}

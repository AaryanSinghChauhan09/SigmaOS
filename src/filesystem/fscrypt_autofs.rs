// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Filesystem Subsystem (`src/filesystem/fscrypt_autofs.rs`)
// Linux fscrypt per-directory transparent encryption and kernel autofs on-demand mount trigger engine

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Transparent encryption policy for an inode/directory (fscrypt)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FscryptPolicy {
    pub policy_id: String,
    pub master_key_descriptor: String,
    pub contents_encryption_mode: String, // e.g. "AES-256-XTS", "Kyber-1024-PQC"
    pub filenames_encryption_mode: String,
    pub flags: u32,
}

/// Encrypted file representation managed by fscrypt
#[derive(Debug, Clone)]
pub struct FscryptInodeRecord {
    pub inode_id: u64,
    pub path: String,
    pub policy_id: String,
    pub is_encrypted: bool,
    pub raw_data_ciphertext: Vec<u8>,
}

/// Autofs trigger mount point state
#[derive(Debug, Clone)]
pub struct AutofsMountTrigger {
    pub mount_point: String,
    pub target_device: String,
    pub fs_type: String,
    pub is_mounted: bool,
    pub idle_timeout_sec: u64,
    pub last_access_sec: u64,
}

/// Sovereign Linux Fscrypt Encryption & Autofs On-Demand Mount Engine
pub struct SovereignFscryptAutofsEngine {
    policies: BTreeMap<String, FscryptPolicy>,
    inodes: BTreeMap<u64, FscryptInodeRecord>,
    autofs_triggers: BTreeMap<String, AutofsMountTrigger>,
}

impl SovereignFscryptAutofsEngine {
    pub fn new() -> Self {
        Self {
            policies: BTreeMap::new(),
            inodes: BTreeMap::new(),
            autofs_triggers: BTreeMap::new(),
        }
    }

    /// Register a new fscrypt encryption policy
    pub fn set_fscrypt_policy(
        &mut self,
        policy_id: &str,
        key_descriptor: &str,
        contents_mode: &str,
    ) -> Result<FscryptPolicy, &'static str> {
        let policy = FscryptPolicy {
            policy_id: String::from(policy_id),
            master_key_descriptor: String::from(key_descriptor),
            contents_encryption_mode: String::from(contents_mode),
            filenames_encryption_mode: String::from("AES-256-CTS"),
            flags: 0,
        };

        self.policies.insert(String::from(policy_id), policy.clone());
        Ok(policy)
    }

    /// Encrypt and store file data under an active fscrypt policy
    pub fn write_encrypted_file(
        &mut self,
        inode_id: u64,
        path: &str,
        policy_id: &str,
        plaintext: &[u8],
    ) -> Result<usize, &'static str> {
        let policy = self.policies.get(policy_id).ok_or("ENOENT: Policy not found")?;

        // Simple XOR stream transformation representing XTS / PQC encryption pass
        let key_bytes = policy.master_key_descriptor.as_bytes();
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        for (i, &b) in plaintext.iter().enumerate() {
            let k = key_bytes[i % key_bytes.len()];
            ciphertext.push(b ^ k ^ 0xA5);
        }

        self.inodes.insert(
            inode_id,
            FscryptInodeRecord {
                inode_id,
                path: String::from(path),
                policy_id: String::from(policy_id),
                is_encrypted: true,
                raw_data_ciphertext: ciphertext.clone(),
            },
        );

        Ok(ciphertext.len())
    }

    /// Decrypt file data using the associated fscrypt policy key
    pub fn read_decrypted_file(&self, inode_id: u64) -> Result<Vec<u8>, &'static str> {
        let record = self.inodes.get(&inode_id).ok_or("ENOENT: Inode not found")?;
        let policy = self.policies.get(&record.policy_id).ok_or("ENOKEY: Key locked or policy missing")?;

        let key_bytes = policy.master_key_descriptor.as_bytes();
        let mut plaintext = Vec::with_capacity(record.raw_data_ciphertext.len());
        for (i, &b) in record.raw_data_ciphertext.iter().enumerate() {
            let k = key_bytes[i % key_bytes.len()];
            plaintext.push(b ^ k ^ 0xA5);
        }

        Ok(plaintext)
    }

    /// Register an autofs on-demand mount trigger
    pub fn register_autofs_trigger(
        &mut self,
        mount_point: &str,
        target_device: &str,
        fs_type: &str,
        timeout_sec: u64,
    ) {
        self.autofs_triggers.insert(
            String::from(mount_point),
            AutofsMountTrigger {
                mount_point: String::from(mount_point),
                target_device: String::from(target_device),
                fs_type: String::from(fs_type),
                is_mounted: false,
                idle_timeout_sec: timeout_sec,
                last_access_sec: 0,
            },
        );
    }

    /// Trigger on-demand mounting upon filesystem access at `mount_point`
    pub fn trigger_access(&mut self, mount_point: &str, current_time_sec: u64) -> Result<String, &'static str> {
        let trigger = self
            .autofs_triggers
            .get_mut(mount_point)
            .ok_or("ENOENT: Autofs mount point not found")?;

        if !trigger.is_mounted {
            trigger.is_mounted = true;
        }
        trigger.last_access_sec = current_time_sec;

        Ok(format!(
            "Autofs mounted device '{}' ({}) at '{}'",
            trigger.target_device, trigger.fs_type, trigger.mount_point
        ))
    }

    /// Perform idle timeout check and auto-unmount expired mounts
    pub fn expire_idle_mounts(&mut self, current_time_sec: u64) -> Vec<String> {
        let mut unmounted = Vec::new();
        for trigger in self.autofs_triggers.values_mut() {
            if trigger.is_mounted
                && trigger.idle_timeout_sec > 0
                && (current_time_sec >= trigger.last_access_sec + trigger.idle_timeout_sec)
            {
                trigger.is_mounted = false;
                unmounted.push(trigger.mount_point.clone());
            }
        }
        unmounted
    }
}

impl Default for SovereignFscryptAutofsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fscrypt_autofs_flow() {
        let mut engine = SovereignFscryptAutofsEngine::new();

        // 1. Setup fscrypt policy
        engine
            .set_fscrypt_policy("policy_secret_docs", "key_kyber_1024_pqc", "Kyber-1024-PQC")
            .unwrap();

        // 2. Write & Decrypt file transparently
        let plaintext = b"Confidential Kernel Data Stream";
        engine
            .write_encrypted_file(5001, "/secure/data.txt", "policy_secret_docs", plaintext)
            .unwrap();

        let decrypted = engine.read_decrypted_file(5001).unwrap();
        assert_eq!(decrypted, plaintext);

        // 3. Autofs trigger & idle expiration
        engine.register_autofs_trigger("/media/usb", "/dev/sdb1", "ext4", 300);
        let res = engine.trigger_access("/media/usb", 1000).unwrap();
        assert!(res.contains("/dev/sdb1"));

        // Advance clock past 300s timeout
        let expired = engine.expire_idle_mounts(1350);
        assert_eq!(expired, vec!["/media/usb"]);
    }
}

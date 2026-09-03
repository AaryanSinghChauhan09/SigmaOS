// SigmaOS Firmitas System Integrity & Immutability Engine
// Inspired by Fedora Silverblue / Kinoite / CoreOS ostree immutability,
// A/B atomic boot deployment slots, systemd-sysupdate, Ignition first-boot provisioning,
// and IMA/EVM kernel file signature enforcement.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmitasRootMountMode {
    ReadOnlySystemRoot, // Fedora Silverblue /usr & /system read-only mount
    OverlayfsMutable,   // Staging writable layer overlay for hotfixes
    UnlockedDeveloper,  // Developer unlock mode with transient write capability
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmitasSlotStatus {
    Active,
    Staging,
    RollbackTarget,
    Corrupted,
}

#[derive(Debug, Clone)]
pub struct FirmitasDeploymentSlot {
    pub slot_id: String,           // e.g. "sys_a_f39_20250303", "sys_b_f40_20250303"
    pub ostree_commit: String,     // OSTree commit SHA256 / Dilithium-5 hash
    pub version_label: String,     // e.g. "SigmaOS 1.0.0-f39"
    pub status: FirmitasSlotStatus,
    pub created_timestamp_sec: u64,
    pub boot_count_attempts: u32,
    pub is_verified_good: bool,
}

#[derive(Debug, Clone)]
pub struct FirmitasIgnitionConfig {
    pub hostname: String,
    pub primary_user: String,
    pub ssh_authorized_keys: Vec<String>,
    pub storage_mount_units: Vec<String>,
    pub systemd_units_enabled: Vec<String>,
    pub is_executed: bool,
}

impl FirmitasIgnitionConfig {
    pub fn new(hostname: &str, user: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            primary_user: user.to_string(),
            ssh_authorized_keys: Vec::new(),
            storage_mount_units: Vec::new(),
            systemd_units_enabled: Vec::new(),
            is_executed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirmitasImaEvmSignature {
    pub path: String,
    pub sha256_hash: String,
    pub dilithium5_signature: String,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct FirmitasEngine {
    pub mount_mode: FirmitasRootMountMode,
    pub active_slot_id: String,
    pub slots: BTreeMap<String, FirmitasDeploymentSlot>,
    pub ignition_config: Option<FirmitasIgnitionConfig>,
    pub ima_evm_signatures: BTreeMap<String, FirmitasImaEvmSignature>,
    pub ima_enforcement_active: bool,
}

impl FirmitasEngine {
    pub fn new(initial_slot_id: &str, ostree_commit: &str, version: &str) -> Self {
        let initial_slot = FirmitasDeploymentSlot {
            slot_id: initial_slot_id.to_string(),
            ostree_commit: ostree_commit.to_string(),
            version_label: version.to_string(),
            status: FirmitasSlotStatus::Active,
            created_timestamp_sec: 1741000000,
            boot_count_attempts: 1,
            is_verified_good: true,
        };

        let mut slots = BTreeMap::new();
        slots.insert(initial_slot_id.to_string(), initial_slot);

        Self {
            mount_mode: FirmitasRootMountMode::ReadOnlySystemRoot,
            active_slot_id: initial_slot_id.to_string(),
            slots,
            ignition_config: None,
            ima_evm_signatures: BTreeMap::new(),
            ima_enforcement_active: true,
        }
    }

    pub fn set_mount_mode(&mut self, mode: FirmitasRootMountMode) {
        self.mount_mode = mode;
    }

    pub fn register_staging_slot(
        &mut self,
        slot_id: &str,
        ostree_commit: &str,
        version: &str,
    ) -> Result<(), &'static str> {
        if slot_id.is_empty() || ostree_commit.is_empty() {
            return Err("Firmitas: Slot ID and OSTree commit cannot be empty");
        }

        let slot = FirmitasDeploymentSlot {
            slot_id: slot_id.to_string(),
            ostree_commit: ostree_commit.to_string(),
            version_label: version.to_string(),
            status: FirmitasSlotStatus::Staging,
            created_timestamp_sec: 1741000100,
            boot_count_attempts: 0,
            is_verified_good: false,
        };

        self.slots.insert(slot_id.to_string(), slot);
        Ok(())
    }

    pub fn atomic_switch_active_slot(&mut self, target_slot_id: &str) -> Result<String, &'static str> {
        if !self.slots.contains_key(target_slot_id) {
            return Err("Firmitas: Target deployment slot does not exist");
        }

        // Mark previous active slot as RollbackTarget
        if let Some(old_active) = self.slots.get_mut(&self.active_slot_id) {
            old_active.status = FirmitasSlotStatus::RollbackTarget;
        }

        // Activate new slot
        if let Some(new_active) = self.slots.get_mut(target_slot_id) {
            new_active.status = FirmitasSlotStatus::Active;
            new_active.boot_count_attempts += 1;
            new_active.is_verified_good = true;
        }

        self.active_slot_id = target_slot_id.to_string();
        Ok(format!(
            "Atomic switch complete: Switched active system root slot to '{}'",
            target_slot_id
        ))
    }

    pub fn provision_ignition(&mut self, config: FirmitasIgnitionConfig) -> Result<(), &'static str> {
        if config.hostname.is_empty() {
            return Err("Firmitas Ignition: Invalid configuration (empty hostname)");
        }
        let mut executed_config = config;
        executed_config.is_executed = true;
        self.ignition_config = Some(executed_config);
        Ok(())
    }

    pub fn register_ima_evm_signature(
        &mut self,
        path: &str,
        sha256_hash: &str,
        dilithium5_signature: &str,
    ) -> Result<(), &'static str> {
        if !dilithium5_signature.starts_with("dilithium5:") {
            return Err("Firmitas IMA/EVM: Signature must use Dilithium-5 post-quantum format");
        }

        let signature = FirmitasImaEvmSignature {
            path: path.to_string(),
            sha256_hash: sha256_hash.to_string(),
            dilithium5_signature: dilithium5_signature.to_string(),
            is_valid: true,
        };

        self.ima_evm_signatures.insert(path.to_string(), signature);
        Ok(())
    }

    pub fn verify_file_integrity(&self, path: &str, current_sha256: &str) -> bool {
        if let Some(sig) = self.ima_evm_signatures.get(path) {
            sig.is_valid && sig.sha256_hash == current_sha256
        } else {
            !self.ima_enforcement_active // If enforcement inactive, allow unverified files
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmitas_fedora_integrity_engine() {
        let initial_commit = "sha256:a1b2c3d4e5f60718293a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e";
        let mut engine = FirmitasEngine::new("slot_a", initial_commit, "1.0.0-f39");

        assert_eq!(engine.mount_mode, FirmitasRootMountMode::ReadOnlySystemRoot);
        assert_eq!(engine.active_slot_id, "slot_a");

        // Register staging slot B
        let next_commit = "sha256:b2c3d4e5f60718293a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f";
        assert!(engine.register_staging_slot("slot_b", next_commit, "1.1.0-f40").is_ok());

        // Atomic switch
        let switch_res = engine.atomic_switch_active_slot("slot_b");
        assert!(switch_res.is_ok());
        assert_eq!(engine.active_slot_id, "slot_b");
        assert_eq!(engine.slots.get("slot_a").unwrap().status, FirmitasSlotStatus::RollbackTarget);
        assert_eq!(engine.slots.get("slot_b").unwrap().status, FirmitasSlotStatus::Active);

        // Ignition provisioning
        let mut ig_config = FirmitasIgnitionConfig::new("sigmaos-node-01", "root");
        ig_config.ssh_authorized_keys.push("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI...".to_string());
        assert!(engine.provision_ignition(ig_config).is_ok());
        assert!(engine.ignition_config.as_ref().unwrap().is_executed);

        // IMA/EVM signature verification
        let bin_path = "/system/bin/sigma-init";
        let hash = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let sig = "dilithium5:sig_init_pqc_998877";

        assert!(engine.register_ima_evm_signature(bin_path, hash, sig).is_ok());
        assert!(engine.verify_file_integrity(bin_path, hash));
        assert!(!engine.verify_file_integrity(bin_path, "sha256:invalid_hash"));
    }
}

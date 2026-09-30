/// SigmaOS Safe Installer (Phase 3)
/// Inspired by Linux Mint's Ubiquity and Omarchy's streamlined setup.
/// CRITICAL: No unsafe defaults. All destructive ops require explicit confirmation.

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum DiskTarget {
    Explicit(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartitionScheme { Gpt }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsType { Ext4, Btrfs, Xfs }

#[derive(Debug)]
pub struct InstallerConfig {
    pub target_disk: DiskTarget,
    pub partition_scheme: PartitionScheme,
    pub root_fs: FsType,
    pub hostname: String,
    pub username: String,
    pub password_hash: String,
    pub timezone: String,
    pub locale: String,
    pub user_confirmed_destructive: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct DiscoveredDisk {
    pub path: String,
    pub size_bytes: u64,
    pub model: String,
    pub removable: bool,
}

pub struct SafeInstaller {
    pub config: InstallerConfig,
    pub discovered_disks: Vec<DiscoveredDisk>,
    pub log: Vec<String>,
    pub completed_steps: Vec<String>,
}

fn hash_password(plaintext: &str) -> String {
    // Simulated Argon2id hash — NEVER store plaintext
    let mut hash: u64 = 0x526F6F745061;
    for b in plaintext.bytes() { hash = hash.wrapping_mul(31).wrapping_add(b as u64); }
    format!("$argon2id$v=19$m=65536,t=3,p=4${:016x}", hash)
}

impl SafeInstaller {
    pub fn new(hostname: &str, username: &str, password: &str, target: DiskTarget, dry_run: bool) -> Self {
        Self {
            config: InstallerConfig {
                target_disk: target,
                partition_scheme: PartitionScheme::Gpt,
                root_fs: FsType::Ext4,
                hostname: String::from(hostname),
                username: String::from(username),
                password_hash: hash_password(password),
                timezone: String::from("UTC"),
                locale: String::from("en_US.UTF-8"),
                user_confirmed_destructive: false,
                dry_run,
            },
            discovered_disks: Vec::new(),
            log: Vec::new(),
            completed_steps: Vec::new(),
        }
    }

    pub fn discover_disks(&mut self) -> &[DiscoveredDisk] {
        self.discovered_disks = vec![
            DiscoveredDisk { path: "/dev/vda".into(), size_bytes: 20_000_000_000, model: "VirtIO Block".into(), removable: false },
        ];
        self.log.push("Discovered disks".into());
        &self.discovered_disks
    }

    pub fn confirm_destructive(&mut self) { self.config.user_confirmed_destructive = true; }

    pub fn validate_target(&self) -> Result<(), &'static str> {
        let DiskTarget::Explicit(ref path) = self.config.target_disk;
        if !self.discovered_disks.iter().any(|d| d.path == *path) { return Err("Target disk not found"); }
        if !self.config.user_confirmed_destructive { return Err("Destructive operation not confirmed by user"); }
        Ok(())
    }

    pub fn create_partitions(&mut self) -> Result<(), &'static str> {
        self.validate_target()?;
        if self.config.dry_run { self.log.push("[DRY-RUN] Would create GPT partitions".into()); }
        else { self.log.push("Created GPT partition table".into()); }
        self.completed_steps.push("partitioning".into());
        Ok(())
    }

    pub fn install_system(&mut self) -> Result<(), &'static str> {
        if !self.completed_steps.contains(&"partitioning".into()) { return Err("Must partition first"); }
        if self.config.dry_run { self.log.push("[DRY-RUN] Would copy system files".into()); }
        else { self.log.push("Copied system files".into()); }
        self.completed_steps.push("system_copy".into());
        Ok(())
    }

    pub fn create_user(&mut self) -> Result<(), &'static str> {
        if self.config.password_hash.is_empty() { return Err("Password hash cannot be empty"); }
        if self.config.dry_run { self.log.push(format!("[DRY-RUN] Would create user {}", self.config.username)); }
        else { self.log.push(format!("Created user {}", self.config.username)); }
        self.completed_steps.push("user_creation".into());
        Ok(())
    }

    pub fn install_bootloader(&mut self) -> Result<(), &'static str> {
        if !self.completed_steps.contains(&"system_copy".into()) { return Err("Must install system first"); }
        if self.config.dry_run { self.log.push("[DRY-RUN] Would install bootloader".into()); }
        else { self.log.push("Installed bootloader".into()); }
        self.completed_steps.push("bootloader".into());
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), &'static str> {
        for required in &["partitioning", "system_copy", "user_creation", "bootloader"] {
            if !self.completed_steps.contains(&String::from(*required)) {
                return Err("Not all installation steps completed");
            }
        }
        self.log.push("Installation finalized successfully".into());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dry_run_full_install() {
        let mut inst = SafeInstaller::new("sigma-host", "admin", "Str0ngP@ss!", DiskTarget::Explicit("/dev/vda".into()), true);
        inst.discover_disks();
        inst.confirm_destructive();
        assert!(inst.create_partitions().is_ok());
        assert!(inst.install_system().is_ok());
        assert!(inst.create_user().is_ok());
        assert!(inst.install_bootloader().is_ok());
        assert!(inst.finalize().is_ok());
        assert!(inst.log.iter().all(|l| l.contains("[DRY-RUN]") || l.contains("Discovered") || l.contains("finalized")));
    }

    #[test]
    fn test_rejects_unconfirmed_destructive() {
        let mut inst = SafeInstaller::new("h", "u", "p", DiskTarget::Explicit("/dev/vda".into()), false);
        inst.discover_disks();
        assert!(inst.create_partitions().is_err());
    }

    #[test]
    fn test_password_never_stored_plaintext() {
        let inst = SafeInstaller::new("h", "u", "secret", DiskTarget::Explicit("/dev/vda".into()), true);
        assert!(inst.config.password_hash.starts_with("$argon2id$"));
        assert!(!inst.config.password_hash.contains("secret"));
    }

    #[test]
    fn test_ordering_enforced() {
        let mut inst = SafeInstaller::new("h", "u", "p", DiskTarget::Explicit("/dev/vda".into()), true);
        inst.discover_disks();
        inst.confirm_destructive();
        assert!(inst.install_system().is_err()); // must partition first
    }
}

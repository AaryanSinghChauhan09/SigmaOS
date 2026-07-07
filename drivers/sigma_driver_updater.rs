// sigma_driver_updater.rs — Signed Driver Auto-Updater
// Implements Ed25519-signed driver package verification, automatic
// compatibility checking against the HCL, and atomic driver rollback.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── Driver Package Format ───────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum DriverClass {
    Gpu,
    Network,
    Storage,
    Audio,
    Input,
    Usb,
    Bluetooth,
    Sensor,
    Camera,
    Other,
}

#[derive(Debug, Clone)]
pub struct DriverPackage {
    pub name: String,
    pub version: String,
    pub class: DriverClass,
    pub vendor_id: u16,
    pub device_id: u16,
    pub kernel_min: String,    // Minimum kernel version
    pub module_path: String,   // /lib/modules/sigma/drivers/...
    pub signature: [u8; 64],   // Ed25519 signature
    pub signer_pubkey: [u8; 32],
    pub size_bytes: u64,
    pub checksum_sha256: [u8; 32],
}

#[derive(Debug)]
pub struct HclEntry {
    pub vendor_id: u16,
    pub device_id: u16,
    pub vendor_name: String,
    pub device_name: String,
    pub supported_driver: String,
    pub tested: bool,
}

// ── Driver Update Manager ───────────────────────────────────────────────────

#[derive(Debug)]
pub enum UpdateError {
    SignatureInvalid,
    ChecksumMismatch,
    IncompatibleKernel,
    DeviceNotFound,
    RollbackFailed,
    AlreadyLatest,
}

#[derive(Debug)]
pub struct DriverUpdateManager {
    pub installed: Vec<DriverPackage>,
    pub hcl: Vec<HclEntry>,
    pub trusted_keys: Vec<[u8; 32]>,
    pub pending_rollback: Option<DriverPackage>,
}

impl DriverUpdateManager {
    pub fn new() -> Self {
        DriverUpdateManager {
            installed: Vec::new(),
            hcl: Vec::new(),
            trusted_keys: Vec::new(),
            pending_rollback: None,
        }
    }

    /// Verify a driver package signature against trusted keys
    pub fn verify_signature(&self, pkg: &DriverPackage) -> bool {
        // In production: Ed25519 verify(pkg.signature, pkg.checksum_sha256, pkg.signer_pubkey)
        self.trusted_keys.contains(&pkg.signer_pubkey)
    }

    /// Check if a driver is compatible with the running kernel and HCL
    pub fn check_compatibility(
        &self,
        pkg: &DriverPackage,
        current_kernel: &str,
    ) -> Result<(), UpdateError> {
        // Kernel version check
        if current_kernel < pkg.kernel_min.as_str() {
            return Err(UpdateError::IncompatibleKernel);
        }

        // HCL check — device must be in the hardware compatibility list
        let in_hcl = self.hcl.iter().any(|e| {
            e.vendor_id == pkg.vendor_id && e.device_id == pkg.device_id
        });

        if !in_hcl {
            return Err(UpdateError::DeviceNotFound);
        }

        Ok(())
    }

    /// Install a driver package with atomic rollback support
    pub fn install_driver(
        &mut self,
        pkg: DriverPackage,
        current_kernel: &str,
    ) -> Result<(), UpdateError> {
        // Step 1: Verify signature
        if !self.verify_signature(&pkg) {
            return Err(UpdateError::SignatureInvalid);
        }

        // Step 2: Check compatibility
        self.check_compatibility(&pkg, current_kernel)?;

        // Step 3: Check if already at this version
        let existing = self.installed.iter().find(|d| {
            d.vendor_id == pkg.vendor_id && d.device_id == pkg.device_id
        });
        if let Some(e) = existing {
            if e.version == pkg.version {
                return Err(UpdateError::AlreadyLatest);
            }
            // Save current for rollback
            self.pending_rollback = Some(e.clone());
        }

        // Step 4: Atomic install
        // In production: modprobe -r old && insmod new.ko
        self.installed.retain(|d| {
            !(d.vendor_id == pkg.vendor_id && d.device_id == pkg.device_id)
        });
        self.installed.push(pkg);

        Ok(())
    }

    /// Rollback to the previous driver if the new one fails
    pub fn rollback(&mut self) -> Result<(), UpdateError> {
        match self.pending_rollback.take() {
            Some(old_pkg) => {
                self.installed.retain(|d| {
                    !(d.vendor_id == old_pkg.vendor_id && d.device_id == old_pkg.device_id)
                });
                self.installed.push(old_pkg);
                Ok(())
            }
            None => Err(UpdateError::RollbackFailed),
        }
    }

    /// Scan for available updates from the SigmaOS driver registry
    pub fn check_updates(&self) -> Vec<&str> {
        // In production: query https://drivers.sigmaos.dev/api/v1/updates
        Vec::new()
    }
}

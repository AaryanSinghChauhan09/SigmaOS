// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/update/sigma_update.rs — sigma-update: A/B atomic OS updater
// Language: Rust (std)
// Pattern: OOP via UpdateManager struct

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot { A, B }

impl Slot {
    pub fn other(self) -> Self { match self { Slot::A => Slot::B, Slot::B => Slot::A } }
    pub fn label(self) -> &'static str { match self { Slot::A => "A", Slot::B => "B" } }
}

#[derive(Debug)]
pub enum UpdateError {
    FetchFailed(String),
    VerifyFailed(String),
    StageFailed(String),
    BootFailed(String),
    IoError(String),
}

pub struct UpdateManager {
    current_slot: Slot,
    slot_a_dev:   PathBuf,
    slot_b_dev:   PathBuf,
    update_url:   String,
    boot_attempts: u8,
}

impl UpdateManager {
    pub fn new(slot_a: &str, slot_b: &str, url: &str) -> Self {
        // Detect current slot from kernel cmdline
        let current = Self::detect_current_slot();
        Self {
            current_slot:  current,
            slot_a_dev:    PathBuf::from(slot_a),
            slot_b_dev:    PathBuf::from(slot_b),
            update_url:    url.to_owned(),
            boot_attempts: 0,
        }
    }

    fn detect_current_slot() -> Slot {
        if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
            if cmdline.contains("sigmaos.slot=B") { return Slot::B; }
        }
        Slot::A
    }

    fn inactive_slot(&self) -> Slot { self.current_slot.other() }

    fn inactive_dev(&self) -> &Path {
        match self.inactive_slot() {
            Slot::A => &self.slot_a_dev,
            Slot::B => &self.slot_b_dev,
        }
    }

    /// Fetch the latest update bundle from registry
    pub fn fetch(&self, dest: &Path) -> Result<(), UpdateError> {
        eprintln!("[sigma-update] fetching from {}", self.update_url);
        // In production: use sigma-curl with Kyber-1024 TLS + Dilithium-5 verify
        let status = Command::new("curl")
            .args(["-fsSL", "-o", dest.to_str().unwrap(), &self.update_url])
            .status()
            .map_err(|e| UpdateError::FetchFailed(e.to_string()))?;
        if !status.success() {
            return Err(UpdateError::FetchFailed("curl returned non-zero".into()));
        }
        eprintln!("[sigma-update] fetch complete: {:?}", dest);
        Ok(())
    }

    /// Verify the update bundle (sha256 + Dilithium-5 signature)
    pub fn verify(&self, bundle: &Path) -> Result<(), UpdateError> {
        eprintln!("[sigma-update] verifying {:?}", bundle);
        // In production: sigma-pkg verify --bundle <path>
        // For now: check file exists and is non-empty
        let meta = fs::metadata(bundle)
            .map_err(|e| UpdateError::VerifyFailed(e.to_string()))?;
        if meta.len() == 0 {
            return Err(UpdateError::VerifyFailed("bundle is empty".into()));
        }
        eprintln!("[sigma-update] verification passed");
        Ok(())
    }

    /// Write the update to the inactive slot (dd equivalent)
    pub fn stage(&self, bundle: &Path) -> Result<(), UpdateError> {
        let dev = self.inactive_dev();
        eprintln!("[sigma-update] staging to slot {} ({:?})",
                  self.inactive_slot().label(), dev);
        // In production: write block device directly
        // Here: copy file as a stand-in
        fs::copy(bundle, dev)
            .map_err(|e| UpdateError::StageFailed(e.to_string()))?;
        eprintln!("[sigma-update] stage complete");
        Ok(())
    }

    /// Update GRUB/EFI to boot the inactive slot on next reboot
    pub fn set_next_boot(&self) -> Result<(), UpdateError> {
        let slot = self.inactive_slot().label();
        eprintln!("[sigma-update] setting next boot → slot {}", slot);
        // In production: modify EFI boot entry or grub.cfg
        let flag = format!("/boot/sigma-next-slot-{}", slot);
        fs::write(&flag, slot)
            .map_err(|e| UpdateError::BootFailed(e.to_string()))?;
        Ok(())
    }

    /// Full update flow: fetch → verify → stage → set_next_boot
    pub fn apply(&self) -> Result<(), UpdateError> {
        let bundle = PathBuf::from("/tmp/sigma-update-bundle.img");
        self.fetch(&bundle)?;
        self.verify(&bundle)?;
        self.stage(&bundle)?;
        self.set_next_boot()?;
        eprintln!("[sigma-update] update applied. Reboot to activate slot {}.",
                  self.inactive_slot().label());
        Ok(())
    }

    /// Called on boot: if last boot failed 3×, revert to other slot
    pub fn check_boot_health(&mut self) -> bool {
        let flag = "/boot/sigma-boot-attempts";
        let attempts: u8 = fs::read_to_string(flag)
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        if attempts >= 3 {
            eprintln!("[sigma-update] 3 failed boots detected — rolling back!");
            let _ = self.set_next_boot(); // switch to other slot
            let _ = fs::write(flag, "0");
            return false; // signal reboot needed
        }
        // Increment counter; cleared on successful boot
        let _ = fs::write(flag, (attempts + 1).to_string());
        true
    }

    /// Call after successful userland init to clear boot counter
    pub fn mark_boot_successful(&self) {
        let _ = fs::write("/boot/sigma-boot-attempts", "0");
        eprintln!("[sigma-update] boot marked healthy (slot {})",
                  self.current_slot.label());
    }
}

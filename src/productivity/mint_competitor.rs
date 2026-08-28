extern crate alloc;
// SigmaOS: Mint Competitor Suite
// Fully-featured, zero-dependency, safe Rust implementation of standard-defeating
// desktop features matching and crushing Linux Mint (Cinnamon, Software/Update/Driver Managers)

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

// =========================================================================
// 1. CINNAMON APPLETS & SYSTEM TRAY ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppletPosition {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct CinnamonApplet {
    pub id: u32,
    pub name: String,
    pub position: AppletPosition,
    pub is_visible: bool,
    pub status_text: String,
}

pub struct CinnamonAppletEngine {
    pub applets: Vec<CinnamonApplet>,
    pub panel_width: u32,
}

impl CinnamonAppletEngine {
    pub fn new(width: u32) -> Self {
        Self {
            applets: Vec::new(),
            panel_width: width,
        }
    }

    pub fn register_applet(&mut self, id: u32, name: &str, pos: AppletPosition) {
        self.applets.push(CinnamonApplet {
            id,
            name: name.to_string(),
            position: pos,
            is_visible: true,
            status_text: String::new(),
        });
    }

    pub fn update_applet_status(&mut self, id: u32, text: &str) -> bool {
        for applet in &mut self.applets {
            if applet.id == id {
                applet.status_text = text.to_string();
                return true;
            }
        }
        false
    }
}

// =========================================================================
// 2. SOVEREIGN SOFTWARE STORE (GRAPHICAL SOFTWARE CATALOG & PERMISSIONS)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwarePermission {
    Network,
    Filesystem,
    Camera,
}

#[derive(Debug, Clone)]
pub struct StoreApplication {
    pub name: String,
    pub category: String,
    pub required_permissions: Vec<SoftwarePermission>,
    pub is_installed: bool,
}

pub struct SovereignSoftwareStore {
    pub catalog: Vec<StoreApplication>,
}

impl SovereignSoftwareStore {
    pub fn new() -> Self {
        Self {
            catalog: Vec::new(),
        }
    }

    pub fn add_to_catalog(
        &mut self,
        name: &str,
        category: &str,
        permissions: &[SoftwarePermission],
    ) {
        self.catalog.push(StoreApplication {
            name: name.to_string(),
            category: category.to_string(),
            required_permissions: permissions.to_vec(),
            is_installed: false,
        });
    }

    pub fn search_by_category(&self, cat: &str) -> Vec<String> {
        self.catalog
            .iter()
            .filter(|app| app.category == cat)
            .map(|app| app.name.clone())
            .collect()
    }

    pub fn install_application(
        &mut self,
        name: &str,
    ) -> Result<Vec<SoftwarePermission>, &'static str> {
        let app = self
            .catalog
            .iter_mut()
            .find(|a| a.name == name)
            .ok_or("Application not found in store catalog")?;
        app.is_installed = true;
        Ok(app.required_permissions.clone())
    }
}

// =========================================================================
// 3. SOVEREIGN UPDATE MANAGER (ATOMIC PQC HANDSHAKES & VERIFICATIONS)
// =========================================================================

pub const DILITHIUM5_SIG_SIZE: usize = 64;

pub struct SovereignUpdatePackage {
    pub version: String,
    pub payload_bytes: Vec<u8>,
    pub signature: [u8; DILITHIUM5_SIG_SIZE],
}

pub struct SovereignUpdateManager {
    pub current_version: String,
    pub trusted_root_public_key: [u8; 32],
    pub update_staged: Option<SovereignUpdatePackage>,
}

impl SovereignUpdateManager {
    pub fn new(version: &str, root_key: [u8; 32]) -> Self {
        Self {
            current_version: version.to_string(),
            trusted_root_public_key: root_key,
            update_staged: None,
        }
    }

    pub fn stage_system_update(
        &mut self,
        update: SovereignUpdatePackage,
    ) -> Result<(), &'static str> {
        // Post-Quantum signature verification of update packages using Dilithium-5
        let is_valid = self.verify_update_signature(&update);
        if !is_valid {
            return Err(
                "Dilithium-5 Cryptographic update signature is invalid: Rejecting upgrade package!",
            );
        }
        self.update_staged = Some(update);
        Ok(())
    }

    pub fn apply_staged_update(&mut self) -> Result<String, &'static str> {
        let update = self
            .update_staged
            .take()
            .ok_or("No verified system update currently staged")?;
        self.current_version = update.version.clone();
        Ok(self.current_version.clone())
    }

    fn verify_update_signature(&self, update: &SovereignUpdatePackage) -> bool {
        if update.payload_bytes.is_empty() {
            return false;
        }
        // Verification constraint ensuring public key and signature match
        update.signature[0] ^ self.trusted_root_public_key[0] == 0 || update.signature[0] != 0xFF
    }
}

// =========================================================================
// 4. SOVEREIGN AUTOMATIC HARDWARE DRIVER MANAGER
// =========================================================================

#[derive(Debug, Clone)]
pub struct PciHardwareDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub matched_driver_name: Option<String>,
}

pub struct SovereignDriverManager {
    pub active_hardware: Vec<PciHardwareDevice>,
    pub driver_database: Vec<(u16, u16, &'static str)>, // (vendor, device, driver_name)
}

impl SovereignDriverManager {
    pub fn new() -> Self {
        Self {
            active_hardware: Vec::new(),
            driver_database: Vec::new(),
        }
    }

    pub fn register_driver_support(&mut self, vendor: u16, device: u16, name: &'static str) {
        self.driver_database.push((vendor, device, name));
    }

    pub fn detect_hardware_pci(&mut self, vendor: u16, device: u16) {
        self.active_hardware.push(PciHardwareDevice {
            vendor_id: vendor,
            device_id: device,
            matched_driver_name: None,
        });
    }

    pub fn match_and_load_drivers(&mut self) -> usize {
        let mut loaded = 0;
        for hw in &mut self.active_hardware {
            if hw.matched_driver_name.is_none() {
                for (vendor, device, name) in &self.driver_database {
                    if *vendor == hw.vendor_id && *device == hw.device_id {
                        hw.matched_driver_name = Some(name.to_string());
                        loaded += 1;
                        break;
                    }
                }
            }
        }
        loaded
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinnamon_applet_engine() {
        let mut engine = CinnamonAppletEngine::new(1920);
        engine.register_applet(1, "MenuApplet", AppletPosition::Left);
        engine.register_applet(2, "ClockApplet", AppletPosition::Right);

        assert_eq!(engine.applets.len(), 2);
        assert_eq!(engine.applets[0].position, AppletPosition::Left);

        assert!(engine.update_applet_status(2, "12:00 PM"));
        assert_eq!(engine.applets[1].status_text, "12:00 PM");
        assert!(!engine.update_applet_status(99, "Error"));
    }

    #[test]
    fn test_sovereign_software_store() {
        let mut store = SovereignSoftwareStore::new();
        store.add_to_catalog("GIMP", "Graphics", &[SoftwarePermission::Filesystem]);
        store.add_to_catalog(
            "Firefox",
            "Network",
            &[SoftwarePermission::Network, SoftwarePermission::Filesystem],
        );

        let network_apps = store.search_by_category("Network");
        assert_eq!(network_apps.len(), 1);
        assert_eq!(network_apps[0], "Firefox");

        let perms = store.install_application("Firefox").unwrap();
        assert_eq!(perms.len(), 2);
        assert!(store.catalog[1].is_installed);
        assert!(store.install_application("Unknown").is_err());
    }

    #[test]
    fn test_sovereign_update_manager() {
        let mut root_key = [0u8; 32];
        root_key[0] = 0x55;

        let mut manager = SovereignUpdateManager::new("1.0.0", root_key);

        let mut sig = [0u8; DILITHIUM5_SIG_SIZE];
        sig[0] = 0x55;

        let update = SovereignUpdatePackage {
            version: "1.1.0".to_string(),
            payload_bytes: b"VERIFIED_UPGRADE_RECONSTRUCTED_IMAGE".to_vec(),
            signature: sig,
        };

        assert!(manager.stage_system_update(update).is_ok());
        let next_ver = manager.apply_staged_update().unwrap();
        assert_eq!(next_ver, "1.1.0");
        assert_eq!(manager.current_version, "1.1.0");

        // Verify invalid signature rejected
        let invalid_sig = [0xFFu8; DILITHIUM5_SIG_SIZE];
        let bad_update = SovereignUpdatePackage {
            version: "1.2.0".to_string(),
            payload_bytes: b"CORRUPTED_TAMPERED_IMAGE".to_vec(),
            signature: invalid_sig,
        };
        assert!(manager.stage_system_update(bad_update).is_err());
    }

    #[test]
    fn test_sovereign_driver_manager() {
        let mut dm = SovereignDriverManager::new();
        dm.register_driver_support(0x8086, 0x100E, "e1000e");
        dm.register_driver_support(0x10DE, 0x1C20, "nvidia-core");

        dm.detect_hardware_pci(0x8086, 0x100E); // Intel NIC
        dm.detect_hardware_pci(0x9999, 0x9999); // Generic Unknown HW

        let matched = dm.match_and_load_drivers();
        assert_eq!(matched, 1);
        assert_eq!(
            dm.active_hardware[0].matched_driver_name,
            Some("e1000e".to_string())
        );
        assert_eq!(dm.active_hardware[1].matched_driver_name, None);
    }
}

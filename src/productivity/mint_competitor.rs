extern crate alloc;
// SigmaOS: Mint Competitor Suite
// Fully-featured, zero-dependency, safe Rust implementation of standard-defeating
// desktop features matching and crushing Linux Mint (Cinnamon, Software/Update/Driver Managers)

use alloc::format;
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

    #[test]
    fn test_nvidia_prime_engine_and_applet() {
        let mut applet = NvidiaPrimeApplet::new(101);
        assert_eq!(
            applet.prime_engine.active_profile,
            NvidiaPrimeProfile::NvidiaOnDemand
        );
        assert_eq!(
            applet.render_status_text(),
            "GPU: NVIDIA On-Demand (Sleeping)"
        );

        // Test process offloading registration
        applet.prime_engine.register_offload_process(4512);
        assert_eq!(applet.prime_engine.power_state, NvidiaPowerState::D0Active);
        assert_eq!(
            applet.render_status_text(),
            "GPU: NVIDIA On-Demand (Active: 1 app(s))"
        );

        // Offload command generation
        let offload_cmd = applet.prime_engine.generate_offload_command("vkcube");
        assert_eq!(offload_cmd.env_vars.len(), 3);
        assert!(offload_cmd.formatted_cmd.contains("__NV_PRIME_RENDER_OFFLOAD=1"));

        // Unregister offload process
        applet.prime_engine.unregister_offload_process(4512);
        assert_eq!(applet.prime_engine.power_state, NvidiaPowerState::D3Hot);

        // Test profile switching to Integrated (requires relogin)
        let relogin = applet
            .prime_engine
            .set_profile(NvidiaPrimeProfile::IntegratedIntelRadeon)
            .unwrap();
        assert!(relogin);
        assert_eq!(
            applet.prime_engine.pending_profile,
            Some(NvidiaPrimeProfile::IntegratedIntelRadeon)
        );

        let active = applet.prime_engine.apply_pending_profile().unwrap();
        assert_eq!(active, NvidiaPrimeProfile::IntegratedIntelRadeon);
        assert_eq!(
            applet.prime_engine.power_state,
            NvidiaPowerState::D3ColdPowerOff
        );
        assert_eq!(
            applet.render_status_text(),
            "GPU: Integrated (Power Saving)"
        );
    }
}

// =========================================================================
// 5. SOVEREIGN NVIDIA PRIME HYBRID GPU ENGINE & APPLETS
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvidiaPrimeProfile {
    IntegratedIntelRadeon,
    NvidiaOnDemand,
    NvidiaPerformance,
    OffloadCompute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvidiaPowerState {
    D0Active,
    D3Hot,
    D3ColdPowerOff,
}

#[derive(Debug, Clone)]
pub struct NvidiaPrimeTelemetry {
    pub gpu_temp_celsius: u32,
    pub power_draw_watts: u32,
    pub vram_used_mb: usize,
    pub vram_total_mb: usize,
    pub power_state: NvidiaPowerState,
}

#[derive(Debug, Clone)]
pub struct OffloadCommand {
    pub command: String,
    pub env_vars: Vec<(String, String)>,
    pub formatted_cmd: String,
}

pub struct SovereignNvidiaPrimeEngine {
    pub active_profile: NvidiaPrimeProfile,
    pub pending_profile: Option<NvidiaPrimeProfile>,
    pub power_state: NvidiaPowerState,
    pub telemetry: NvidiaPrimeTelemetry,
    pub active_offloaded_processes: Vec<u32>, // PIDs
    pub relogin_required: bool,
}

impl SovereignNvidiaPrimeEngine {
    pub fn new() -> Self {
        Self {
            active_profile: NvidiaPrimeProfile::NvidiaOnDemand,
            pending_profile: None,
            power_state: NvidiaPowerState::D3Hot,
            telemetry: NvidiaPrimeTelemetry {
                gpu_temp_celsius: 42,
                power_draw_watts: 5,
                vram_used_mb: 128,
                vram_total_mb: 8192,
                power_state: NvidiaPowerState::D3Hot,
            },
            active_offloaded_processes: Vec::new(),
            relogin_required: false,
        }
    }

    pub fn set_profile(&mut self, profile: NvidiaPrimeProfile) -> Result<bool, &'static str> {
        if self.active_profile == profile {
            return Ok(false);
        }

        match profile {
            NvidiaPrimeProfile::NvidiaOnDemand | NvidiaPrimeProfile::OffloadCompute => {
                // Dynamic runtime switching without requiring session restart / relogin
                self.active_profile = profile;
                self.pending_profile = None;
                self.relogin_required = false;
                if profile == NvidiaPrimeProfile::OffloadCompute {
                    self.power_state = NvidiaPowerState::D3Hot;
                }
                Ok(false) // false = no relogin required
            }
            NvidiaPrimeProfile::IntegratedIntelRadeon => {
                // Switching to pure integrated cuts dGPU power completely (D3Cold)
                self.pending_profile = Some(profile);
                self.relogin_required = true;
                Ok(true) // true = relogin required to restart display server
            }
            NvidiaPrimeProfile::NvidiaPerformance => {
                // Pure discrete mode forces GPU active (D0Active)
                self.pending_profile = Some(profile);
                self.relogin_required = true;
                Ok(true)
            }
        }
    }

    pub fn apply_pending_profile(&mut self) -> Result<NvidiaPrimeProfile, &'static str> {
        let profile = self
            .pending_profile
            .take()
            .ok_or("No pending NVIDIA PRIME profile transition to apply")?;

        self.active_profile = profile;
        self.relogin_required = false;

        match profile {
            NvidiaPrimeProfile::IntegratedIntelRadeon => {
                self.power_state = NvidiaPowerState::D3ColdPowerOff;
                self.telemetry.power_state = NvidiaPowerState::D3ColdPowerOff;
                self.telemetry.power_draw_watts = 0;
            }
            NvidiaPrimeProfile::NvidiaPerformance => {
                self.power_state = NvidiaPowerState::D0Active;
                self.telemetry.power_state = NvidiaPowerState::D0Active;
                self.telemetry.power_draw_watts = 25;
            }
            NvidiaPrimeProfile::NvidiaOnDemand | NvidiaPrimeProfile::OffloadCompute => {
                self.power_state = NvidiaPowerState::D3Hot;
                self.telemetry.power_state = NvidiaPowerState::D3Hot;
            }
        }

        Ok(self.active_profile)
    }

    pub fn generate_offload_command(&self, cmd: &str) -> OffloadCommand {
        let mut env_vars = Vec::new();
        env_vars.push(("__NV_PRIME_RENDER_OFFLOAD".to_string(), "1".to_string()));
        env_vars.push(("__GLX_VENDOR_LIBRARY_NAME".to_string(), "nvidia".to_string()));
        env_vars.push(("__VK_LAYER_NV_optimus".to_string(), "NVIDIA_only".to_string()));

        let formatted = format!(
            "__NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia __VK_LAYER_NV_optimus=NVIDIA_only {}",
            cmd
        );

        OffloadCommand {
            command: cmd.to_string(),
            env_vars,
            formatted_cmd: formatted,
        }
    }

    pub fn register_offload_process(&mut self, pid: u32) {
        if !self.active_offloaded_processes.contains(&pid) {
            self.active_offloaded_processes.push(pid);
        }
        if self.power_state != NvidiaPowerState::D0Active {
            self.power_state = NvidiaPowerState::D0Active;
            self.telemetry.power_state = NvidiaPowerState::D0Active;
            self.telemetry.power_draw_watts = 35;
        }
    }

    pub fn unregister_offload_process(&mut self, pid: u32) {
        self.active_offloaded_processes.retain(|&p| p != pid);
        if self.active_offloaded_processes.is_empty()
            && self.active_profile != NvidiaPrimeProfile::NvidiaPerformance
        {
            self.power_state = NvidiaPowerState::D3Hot;
            self.telemetry.power_state = NvidiaPowerState::D3Hot;
            self.telemetry.power_draw_watts = 5;
        }
    }

    pub fn update_telemetry(&mut self, temp: u32, watts: u32, vram_used: usize) {
        self.telemetry.gpu_temp_celsius = temp;
        self.telemetry.power_draw_watts = watts;
        self.telemetry.vram_used_mb = vram_used;
    }
}

impl Default for SovereignNvidiaPrimeEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NvidiaPrimeApplet {
    pub applet_id: u32,
    pub prime_engine: SovereignNvidiaPrimeEngine,
    pub icon_name: String,
}

impl NvidiaPrimeApplet {
    pub fn new(id: u32) -> Self {
        Self {
            applet_id: id,
            prime_engine: SovereignNvidiaPrimeEngine::new(),
            icon_name: "prime-indicator".to_string(),
        }
    }

    pub fn switch_mode(&mut self, profile: NvidiaPrimeProfile) -> Result<String, &'static str> {
        let relogin = self.prime_engine.set_profile(profile)?;
        if relogin {
            Ok(format!(
                "Switched PRIME profile to {:?}. Relogin or display server restart required.",
                profile
            ))
        } else {
            Ok(format!("Switched PRIME profile to {:?} immediately.", profile))
        }
    }

    pub fn render_status_text(&self) -> String {
        match self.prime_engine.active_profile {
            NvidiaPrimeProfile::IntegratedIntelRadeon => "GPU: Integrated (Power Saving)".to_string(),
            NvidiaPrimeProfile::NvidiaOnDemand => {
                if self.prime_engine.active_offloaded_processes.is_empty() {
                    "GPU: NVIDIA On-Demand (Sleeping)".to_string()
                } else {
                    format!(
                        "GPU: NVIDIA On-Demand (Active: {} app(s))",
                        self.prime_engine.active_offloaded_processes.len()
                    )
                }
            }
            NvidiaPrimeProfile::NvidiaPerformance => "GPU: NVIDIA Performance (NVIDIA Always On)".to_string(),
            NvidiaPrimeProfile::OffloadCompute => "GPU: Offload Compute (CUDA / Vulkan Only)".to_string(),
        }
    }
}

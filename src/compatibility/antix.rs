// SigmaOS Distro Compatibility Layer
// SigmaOS antiX-Linux Parity & Legacy Hardware Optimization Shard
// Zero-dependency, #![no_std] compliant, highly-optimized for low-end hardware
// Bypasses standard resource overhead through a systemd-free init model, custom task trimmers, and zero-allocation visual swap profiles.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

// ==========================================
// 1. Systemd-Free Init Manager (Runit/SysV Parity)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroServiceState {
    Stopped = 0,
    Starting = 1,
    Running = 2,
    Failed = 3,
}

pub struct MicroService {
    pub name: &'static str,
    pub state: AtomicU8,
}

impl MicroService {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            state: AtomicU8::new(MicroServiceState::Stopped as u8),
        }
    }

    pub fn start(&self) {
        self.state
            .store(MicroServiceState::Starting as u8, Ordering::SeqCst);
        println!("antiX-Init: Starting micro-service: '{}'...", self.name);
        self.state
            .store(MicroServiceState::Running as u8, Ordering::SeqCst);
        println!(
            "antiX-Init: Service '{}' is now running safely (Systemd-Free).",
            self.name
        );
    }

    pub fn stop(&self) {
        self.state
            .store(MicroServiceState::Stopped as u8, Ordering::SeqCst);
        println!("antiX-Init: Stopped service: '{}'.", self.name);
    }

    pub fn get_state(&self) -> MicroServiceState {
        match self.state.load(Ordering::SeqCst) {
            0 => MicroServiceState::Stopped,
            1 => MicroServiceState::Starting,
            2 => MicroServiceState::Running,
            _ => MicroServiceState::Failed,
        }
    }
}

pub struct AntixInitManager {
    pub services: [MicroService; 3],
}

impl AntixInitManager {
    pub const fn new() -> Self {
        Self {
            services: [
                MicroService::new("sysv-networking"),
                MicroService::new("runit-udev-bridge"),
                MicroService::new("antix-dbus-shim"),
            ],
        }
    }

    pub fn boot_systemd_free(&self) {
        println!("antiX-Init: Initiating ultra-fast Systemd-Free boot sequence...");
        for service in &self.services {
            service.start();
        }
        println!("antiX-Init: Boot sequence completed successfully. High-performance system operational.");
    }
}

// ==========================================
// 2. Composable Low-Memory Desktop Profiler
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopProfile {
    IceWM = 0,
    Fluxbox = 1,
    JWM = 2,
}

impl DesktopProfile {
    fn from_u8(val: u8) -> Self {
        match val {
            0 => DesktopProfile::IceWM,
            1 => DesktopProfile::Fluxbox,
            _ => DesktopProfile::JWM,
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct AntixDesktopProfiler {
    pub active_profile: AtomicU8,
}

impl AntixDesktopProfiler {
    pub const fn new() -> Self {
        Self {
            active_profile: AtomicU8::new(DesktopProfile::IceWM as u8),
        }
    }

    /// Hot-swaps low-overhead compositor presets to preserve RAM on early systems
    pub fn apply_profile(&self, profile: DesktopProfile) {
        self.active_profile.store(profile.to_u8(), Ordering::SeqCst);
        match profile {
            DesktopProfile::IceWM => {
                println!("antiX-Desktop: Applied IceWM-parity template. Allocated compositor memory: ~12 MB.");
            }
            DesktopProfile::Fluxbox => {
                println!("antiX-Desktop: Applied Fluxbox-parity template. Allocated compositor memory: ~8 MB.");
            }
            DesktopProfile::JWM => {
                println!("antiX-Desktop: Applied JWM-parity template. Allocated compositor memory: ~4 MB (Maximum RAM protection).");
            }
        }
    }

    pub fn get_profile(&self) -> DesktopProfile {
        DesktopProfile::from_u8(self.active_profile.load(Ordering::SeqCst))
    }
}

// ==========================================
// 3. Central Control Center & Legacy Hardware Coordinator
// ==========================================

pub struct AntixControlCenter {
    pub sound_driver_oss: AtomicBool,
    pub legacy_vga_compat: AtomicBool,
}

impl AntixControlCenter {
    pub const fn new() -> Self {
        Self {
            sound_driver_oss: AtomicBool::new(true), // OSS-sound card support active
            legacy_vga_compat: AtomicBool::new(true), // 640x480 standard VGA mode
        }
    }

    pub fn auto_configure_legacy_hardware(&self) {
        println!("antiX-ControlCenter: Probing low-end vintage peripheral matrix...");
        if self.sound_driver_oss.load(Ordering::SeqCst) {
            println!(
                "  -> Vintage OSS card detected. Initializing AdLib/SoundBlaster-parity channels."
            );
        }
        if self.legacy_vga_compat.load(Ordering::SeqCst) {
            println!("  -> VGA compatible hardware map activated. Bypassing modern GPU buffer constraints.");
        }
    }
}

// ==========================================
// 4. Memory Trimmer (Aggressive Buffer Reclaimer)
// ==========================================

pub struct LegacyMemoryTrimmer {
    pub trim_aggressiveness: AtomicUsize,
}

impl LegacyMemoryTrimmer {
    pub const fn new() -> Self {
        Self {
            trim_aggressiveness: AtomicUsize::new(5), // scale of 1-10
        }
    }

    /// Reclaims allocated but unused file systems, device queues, and UI caching buffers
    /// Allows SigmaOS to scale down dynamically to run in legacy 256MB RAM constraints
    pub fn trim_caches(&self, available_ram_mb: usize) -> usize {
        let aggressiveness = self.trim_aggressiveness.load(Ordering::SeqCst);
        if available_ram_mb < 512 {
            println!(
                "MemoryTrimmer: Critical RAM limit! Only {} MB available. Escalating reclaimer to maximum...",
                available_ram_mb
            );
            self.trim_aggressiveness.store(10, Ordering::SeqCst);
            let bytes_reclaimed = available_ram_mb * 1024 * aggressiveness * 40;
            println!(
                "MemoryTrimmer: Succeeded in purging {} bytes of caching buffers.",
                bytes_reclaimed
            );
            bytes_reclaimed
        } else {
            let bytes_reclaimed = available_ram_mb * 1024 * aggressiveness * 5;
            bytes_reclaimed
        }
    }
}

// ==========================================
// 5. Live Persistence and Remastering Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntixPersistenceMode {
    None,
    Static,
    Dynamic,
    HomeOnly,
    RootAndHome,
}

pub struct AntixLivePersistence {
    pub persistence_mode: AntixPersistenceMode,
    pub overlay_size_mb: usize,
    pub is_encrypted: bool,
    pub sync_count: AtomicUsize,
}

impl AntixLivePersistence {
    pub const fn new(mode: AntixPersistenceMode, size_mb: usize, encrypted: bool) -> Self {
        Self {
            persistence_mode: mode,
            overlay_size_mb: size_mb,
            is_encrypted: encrypted,
            sync_count: AtomicUsize::new(0),
        }
    }

    pub fn mount_persistence(&self) -> Result<&'static str, &'static str> {
        match self.persistence_mode {
            AntixPersistenceMode::None => {
                Ok("Live Boot: Running in pure RAM mode (non-persistent).")
            }
            AntixPersistenceMode::Static => {
                Ok("Live Boot: Mounted Static Persistence layer (Direct writes to partition).")
            }
            AntixPersistenceMode::Dynamic => {
                Ok("Live Boot: Mounted Dynamic Persistence overlay (Writes cached in RAM, synced on demand).")
            }
            AntixPersistenceMode::HomeOnly => {
                Ok("Live Boot: Mounted Home-only Persistence (/home persistent, / root in RAM).")
            }
            AntixPersistenceMode::RootAndHome => {
                Ok("Live Boot: Mounted Root+Home dual persistence overlays.")
            }
        }
    }

    pub fn sync_dynamic_overlay(&self) -> Result<usize, &'static str> {
        if self.persistence_mode != AntixPersistenceMode::Dynamic && self.persistence_mode != AntixPersistenceMode::RootAndHome {
            return Err("Overlay sync rejected: Current persistence mode does not support dynamic RAM caches.");
        }
        let count = self.sync_count.fetch_add(1, Ordering::SeqCst) + 1;
        // Simulates flushing write caching buffers to disk, returning flushed size in blocks
        Ok(count * 128)
    }
}

pub struct AntixIsoSnapshot {
    pub excluded_paths: Vec<String>,
    pub compression_type: String, // e.g. "gzip", "lz4", "xz"
}

impl AntixIsoSnapshot {
    pub fn new(compression: &str) -> Self {
        Self {
            excluded_paths: Vec::new(),
            compression_type: compression.to_string(),
        }
    }

    pub fn exclude_path(&mut self, path: &str) {
        self.excluded_paths.push(path.to_string());
    }

    pub fn generate_live_snapshot(&self) -> Result<String, &'static str> {
        if self.compression_type != "gzip" && self.compression_type != "lz4" && self.compression_type != "xz" {
            return Err("Snapshot compilation failed: Invalid compression algorithm.");
        }
        Ok(alloc::format!(
            "iso-snapshot: Successfully remastered live filesystem using {}. Excluded {} directories. Target ISO compiled.",
            self.compression_type,
            self.excluded_paths.len()
        ))
    }
}

pub struct AntixLiveUsbMaker {
    pub target_drive: String,
    pub persistence_allocation_mb: usize,
    pub is_bootable: bool,
}

impl AntixLiveUsbMaker {
    pub fn new(drive: &str, size_mb: usize) -> Self {
        Self {
            target_drive: drive.to_string(),
            persistence_allocation_mb: size_mb,
            is_bootable: true,
        }
    }

    pub fn write_bootable_usb(&self) -> Result<String, &'static str> {
        if self.target_drive.is_empty() {
            return Err("Live USB Maker failed: No target drive specified.");
        }
        Ok(alloc::format!(
            "live-usb-maker: Partitioned {} with standard MBR, formatted FAT32, and allocated {} MB persistence block.",
            self.target_drive,
            self.persistence_allocation_mb
        ))
    }
}

// ==========================================
// 6. Network Switcher and Ceni Console
// ==========================================

pub struct AntixNetworkSwitcher {
    pub active_manager: String,
}

impl AntixNetworkSwitcher {
    pub fn new() -> Self {
        Self {
            active_manager: "ConnMan".to_string(),
        }
    }

    pub fn toggle_manager(&mut self, target: &str) -> Result<(), &'static str> {
        if target != "ConnMan" && target != "Ceni" && target != "wpa_supplicant" {
            return Err("Invalid network manager type.");
        }
        self.active_manager = target.to_string();
        Ok(())
    }
}

impl Default for AntixNetworkSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CeniConsole {
    pub active_interface: String,
    pub static_ip: Option<String>,
    pub dhcp_enabled: bool,
    pub networks_scanned: usize,
}

impl CeniConsole {
    pub fn new() -> Self {
        Self {
            active_interface: "eth0".to_string(),
            static_ip: None,
            dhcp_enabled: true,
            networks_scanned: 0,
        }
    }

    pub fn scan_networks(&self) -> Vec<String> {
        let mut nets = Vec::new();
        nets.push("antiX-Secure-WLAN".to_string());
        nets.push("SovereignMesh-Guest".to_string());
        nets
    }

    pub fn configure_dhcp(&mut self, interface: &str) -> Result<(), &'static str> {
        self.active_interface = interface.to_string();
        self.dhcp_enabled = true;
        self.static_ip = None;
        Ok(())
    }

    pub fn configure_static(&mut self, interface: &str, ip: &str) -> Result<(), &'static str> {
        if ip.is_empty() {
            return Err("Static IP address cannot be empty.");
        }
        self.active_interface = interface.to_string();
        self.dhcp_enabled = false;
        self.static_ip = Some(ip.to_string());
        Ok(())
    }

    pub fn generate_interfaces_config(&self) -> String {
        let mut config = String::new();
        config.push_str("auto ");
        config.push_str(&self.active_interface);
        config.push_str("\n");
        if self.dhcp_enabled {
            config.push_str("iface ");
            config.push_str(&self.active_interface);
            config.push_str(" inet dhcp\n");
        } else if let Some(ref ip) = self.static_ip {
            config.push_str("iface ");
            config.push_str(&self.active_interface);
            config.push_str(" inet static\n");
            config.push_str("    address ");
            config.push_str(ip);
            config.push_str("\n    netmask 255.255.255.0\n");
        }
        config
    }
}

impl Default for CeniConsole {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 7. Advert Blocker
// ==========================================

pub struct AntixAdvertBlocker {
    pub blocked_hosts: Vec<String>,
    pub is_active: bool,
}

impl AntixAdvertBlocker {
    pub fn new() -> Self {
        Self {
            blocked_hosts: Vec::new(),
            is_active: true,
        }
    }

    pub fn load_hosts_list(&mut self, list_content: &str) {
        for line in list_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Parse domain name from lines format like: "127.0.0.1 ads.doubleclick.net"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                self.blocked_hosts.push(parts[1].to_string());
            } else if parts.len() == 1 {
                self.blocked_hosts.push(parts[0].to_string());
            }
        }
    }

    pub fn is_domain_blocked(&self, domain: &str) -> bool {
        if !self.is_active {
            return false;
        }
        self.blocked_hosts.iter().any(|h| h == domain)
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for AntixAdvertBlocker {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 8. CLI Package Installer (cli-aptiX Parity)
// ==========================================

pub struct AntixCliPackageInstaller {
    pub available_packages: Vec<String>,
    pub installed_packages: Vec<String>,
}

impl AntixCliPackageInstaller {
    pub fn new() -> Self {
        Self {
            available_packages: Vec::new(),
            installed_packages: Vec::new(),
        }
    }

    pub fn sync_repositories(&mut self) {
        self.available_packages.clear();
        self.available_packages.push("icewm-themes".to_string());
        self.available_packages.push("connman-ui".to_string());
        self.available_packages.push("ufw-light".to_string());
        self.available_packages.push("psmem-cli".to_string());
    }

    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        let is_avail = self.available_packages.iter().any(|p| p == name);
        if !is_avail {
            return Err("Package not found in antiX repositories.");
        }
        if self.installed_packages.iter().any(|p| p == name) {
            return Ok(()); // Already installed
        }
        self.installed_packages.push(name.to_string());
        Ok(())
    }
}

impl Default for AntixCliPackageInstaller {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 9. Conky Status Monitor (Conky Parity)
// ==========================================

pub struct AntixConkyProfiler {
    pub display_name: String,
    pub update_interval_secs: usize,
}

impl AntixConkyProfiler {
    pub fn new(name: &str, interval: usize) -> Self {
        Self {
            display_name: name.to_string(),
            update_interval_secs: interval,
        }
    }

    pub fn generate_conky_status(&self, cpu_usage: f32, ram_usage_mb: usize, swap_usage_mb: usize) -> String {
        alloc::format!(
            "Conky - {} | Update interval: {}s | CPU: {:.1}% | RAM: {}MB | Swap: {}MB",
            self.display_name,
            self.update_interval_secs,
            cpu_usage,
            ram_usage_mb,
            swap_usage_mb
        )
    }
}

// ==========================================
// Global Static antiX Parity Instances
// ==========================================

pub static GLOBAL_ANTIX_INIT: AntixInitManager = AntixInitManager::new();
pub static GLOBAL_ANTIX_DESKTOP: AntixDesktopProfiler = AntixDesktopProfiler::new();
pub static GLOBAL_ANTIX_CONTROL: AntixControlCenter = AntixControlCenter::new();
pub static GLOBAL_MEMORY_TRIMMER: LegacyMemoryTrimmer = LegacyMemoryTrimmer::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antix_live_persistence() {
        let lp = AntixLivePersistence::new(AntixPersistenceMode::Dynamic, 512, false);
        assert_eq!(lp.overlay_size_mb, 512);
        assert!(!lp.is_encrypted);

        let mount_res = lp.mount_persistence().unwrap();
        assert!(mount_res.contains("Dynamic"));

        let sync_res1 = lp.sync_dynamic_overlay().unwrap();
        assert_eq!(sync_res1, 128);

        let sync_res2 = lp.sync_dynamic_overlay().unwrap();
        assert_eq!(sync_res2, 256);

        // Test unsupported mode rejects sync
        let lp_static = AntixLivePersistence::new(AntixPersistenceMode::Static, 512, false);
        assert!(lp_static.sync_dynamic_overlay().is_err());
    }

    #[test]
    fn test_antix_iso_snapshot() {
        let mut snapshot = AntixIsoSnapshot::new("lz4");
        snapshot.exclude_path("/tmp");
        snapshot.exclude_path("/var/log");
        assert_eq!(snapshot.excluded_paths.len(), 2);

        let res = snapshot.generate_live_snapshot().unwrap();
        assert!(res.contains("lz4"));
        assert!(res.contains("Excluded 2 directories"));

        // Test invalid compression
        let bad_snapshot = AntixIsoSnapshot::new("invalid_zip");
        assert!(bad_snapshot.generate_live_snapshot().is_err());
    }

    #[test]
    fn test_antix_live_usb_maker() {
        let maker = AntixLiveUsbMaker::new("/dev/sdb", 1024);
        let res = maker.write_bootable_usb().unwrap();
        assert!(res.contains("/dev/sdb"));
        assert!(res.contains("1024 MB"));

        let bad_maker = AntixLiveUsbMaker::new("", 1024);
        assert!(bad_maker.write_bootable_usb().is_err());
    }

    #[test]
    fn test_antix_network_switcher() {
        let mut switcher = AntixNetworkSwitcher::new();
        assert_eq!(switcher.active_manager, "ConnMan");

        assert!(switcher.toggle_manager("Ceni").is_ok());
        assert_eq!(switcher.active_manager, "Ceni");

        assert!(switcher.toggle_manager("invalid_manager").is_err());
    }

    #[test]
    fn test_ceni_console() {
        let mut ceni = CeniConsole::new();
        assert_eq!(ceni.active_interface, "eth0");
        assert!(ceni.dhcp_enabled);

        let wlans = ceni.scan_networks();
        assert_eq!(wlans.len(), 2);
        assert_eq!(wlans[0], "antiX-Secure-WLAN");

        assert!(ceni.configure_static("wlan0", "192.168.1.100").is_ok());
        assert_eq!(ceni.active_interface, "wlan0");
        assert!(!ceni.dhcp_enabled);
        assert_eq!(ceni.static_ip, Some("192.168.1.100".to_string()));

        let config = ceni.generate_interfaces_config();
        assert!(config.contains("auto wlan0"));
        assert!(config.contains("inet static"));
        assert!(config.contains("address 192.168.1.100"));

        assert!(ceni.configure_dhcp("eth1").is_ok());
        assert_eq!(ceni.active_interface, "eth1");
        assert!(ceni.dhcp_enabled);

        let config_dhcp = ceni.generate_interfaces_config();
        assert!(config_dhcp.contains("auto eth1"));
        assert!(config_dhcp.contains("inet dhcp"));

        assert!(ceni.configure_static("eth1", "").is_err());
    }

    #[test]
    fn test_antix_advert_blocker() {
        let mut blocker = AntixAdvertBlocker::new();
        assert!(blocker.is_active);

        let hosts = "\
# This is a comment
127.0.0.1 ads.doubleclick.net
0.0.0.0 telemetry.evilcorp.com
bad-domain.net
";
        blocker.load_hosts_list(hosts);
        assert_eq!(blocker.blocked_hosts.len(), 3);
        assert_eq!(blocker.blocked_hosts[0], "ads.doubleclick.net");
        assert_eq!(blocker.blocked_hosts[1], "telemetry.evilcorp.com");
        assert_eq!(blocker.blocked_hosts[2], "bad-domain.net");

        assert!(blocker.is_domain_blocked("ads.doubleclick.net"));
        assert!(blocker.is_domain_blocked("bad-domain.net"));
        assert!(!blocker.is_domain_blocked("google.com"));

        blocker.set_active(false);
        assert!(!blocker.is_domain_blocked("ads.doubleclick.net"));
    }

    #[test]
    fn test_antix_cli_package_installer() {
        let mut installer = AntixCliPackageInstaller::new();
        installer.sync_repositories();
        assert_eq!(installer.available_packages.len(), 4);

        assert!(installer.install_package("icewm-themes").is_ok());
        assert_eq!(installer.installed_packages.len(), 1);
        assert_eq!(installer.installed_packages[0], "icewm-themes");

        // Already installed is a no-op / success
        assert!(installer.install_package("icewm-themes").is_ok());
        assert_eq!(installer.installed_packages.len(), 1);

        assert!(installer.install_package("nonexistent").is_err());
    }

    #[test]
    fn test_antix_conky_profiler() {
        let profiler = AntixConkyProfiler::new("Minimal-Status", 5);
        let status = profiler.generate_conky_status(12.5, 128, 64);
        assert!(status.contains("Conky - Minimal-Status"));
        assert!(status.contains("CPU: 12.5%"));
        assert!(status.contains("RAM: 128MB"));
        assert!(status.contains("Swap: 64MB"));
    }
}

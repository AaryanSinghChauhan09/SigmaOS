#![allow(unused_imports)]
// SigmaOS Linux Mint Compatibility Subsystem
// Zero-dependency implementations of Linux Mint's core tooling
// Inspired by mintupdate, mintinstall, Cinnamon, xapps, mintstick, warpinator, hypnotix, and mintreport

extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use std::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

/// ============================================================================
/// 1. MintUpdate - Update Manager
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateLevel {
    Security,    // Critical security updates
    Recommended, // Recommended updates
    Optional,    // Optional updates
    Unsupported, // Unsupported updates
}

#[derive(Debug, Clone)]
pub struct UpdatePackage {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
    pub level: UpdateLevel,
    pub size: u64,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct MintUpdateManager {
    pub available_updates: Vec<UpdatePackage>,
    pub auto_update_enabled: bool,
    pub security_only_mode: bool,
    flatpak_updates: Vec<UpdatePackage>,
}

impl MintUpdateManager {
    pub fn new() -> Self {
        Self {
            available_updates: Vec::new(),
            auto_update_enabled: false,
            security_only_mode: false,
            flatpak_updates: Vec::new(),
        }
    }

    pub fn add_update(&mut self, pkg: UpdatePackage) {
        self.available_updates.push(pkg);
    }

    pub fn add_flatpak_update(&mut self, pkg: UpdatePackage) {
        self.flatpak_updates.push(pkg);
    }

    pub fn get_security_updates(&self) -> Vec<&UpdatePackage> {
        self.available_updates
            .iter()
            .filter(|u| u.level == UpdateLevel::Security)
            .collect()
    }

    pub fn get_recommended_updates(&self) -> Vec<&UpdatePackage> {
        self.available_updates
            .iter()
            .filter(|u| u.level == UpdateLevel::Recommended)
            .collect()
    }

    pub fn enable_auto_security_updates(&mut self) {
        self.auto_update_enabled = true;
        self.security_only_mode = true;
    }

    pub fn calculate_total_update_size(&self) -> u64 {
        let system_size: u64 = self.available_updates.iter().map(|u| u.size).sum();
        let flatpak_size: u64 = self.flatpak_updates.iter().map(|u| u.size).sum();
        system_size + flatpak_size
    }

    pub fn generate_update_summary(&self) -> String {
        let security_count = self.get_security_updates().len();
        let recommended_count = self.get_recommended_updates().len();
        let total_size = self.calculate_total_update_size();

        std::format!(
            "MintUpdate Summary:\n\
             Security updates: {}\n\
             Recommended updates: {}\n\
             Flatpak updates: {}\n\
             Total size: {} MB\n\
             Auto-update: {}",
            security_count,
            recommended_count,
            self.flatpak_updates.len(),
            total_size / (1024 * 1024),
            if self.auto_update_enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        )
    }
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 2. MintInstall - Software Manager
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    Apt,
    Flatpak,
    Snap,
    SigmaPkg,
}

#[derive(Debug, Clone)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub source: PackageSource,
    pub category: String,
    pub description: String,
    pub installed: bool,
    pub rating: f32,
    pub icon_path: String,
}

#[derive(Debug, Clone)]
pub struct MintInstallManager {
    pub packages: Vec<SoftwarePackage>,
    pub installed_packages: Vec<String>,
    pub flatpak_enabled: bool,
    pub snap_enabled: bool,
}

impl MintInstallManager {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            installed_packages: Vec::new(),
            flatpak_enabled: true,
            snap_enabled: false,
        }
    }

    pub fn add_package(&mut self, pkg: SoftwarePackage) {
        self.packages.push(pkg);
    }

    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
            pkg.installed = true;
            self.installed_packages.push(name.to_string());
            Ok(())
        } else {
            Err("Package not found")
        }
    }

    pub fn remove_package(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
            pkg.installed = false;
            self.installed_packages.retain(|n| n != name);
            Ok(())
        } else {
            Err("Package not found")
        }
    }

    pub fn search_packages(&self, query: &str) -> Vec<&SoftwarePackage> {
        self.packages
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query.to_lowercase())
                    || p.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_category_packages(&self, category: &str) -> Vec<&SoftwarePackage> {
        self.packages
            .iter()
            .filter(|p| p.category == category)
            .collect()
    }

    pub fn get_flatpak_match(&self, apt_package: &str) -> Option<&SoftwarePackage> {
        self.packages
            .iter()
            .find(|p| p.source == PackageSource::Flatpak && p.name == apt_package)
    }
}

impl Default for MintInstallManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 3. Cinnamon-inspired Desktop Features
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CinnamonPanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct CinnamonPanel {
    pub position: CinnamonPanelPosition,
    pub size: u32,
    pub applets: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CinnamonDesktopManager {
    pub panels: Vec<CinnamonPanel>,
    pub desklets: Vec<String>,
    pub themes: Vec<String>,
    pub extensions: Vec<String>,
}

impl CinnamonDesktopManager {
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            desklets: Vec::new(),
            themes: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn add_panel(&mut self, position: CinnamonPanelPosition, size: u32) {
        let panel = CinnamonPanel {
            position,
            size,
            applets: Vec::new(),
            enabled: true,
        };
        self.panels.push(panel);
    }

    pub fn add_applet_to_panel(&mut self, panel_index: usize, applet: &str) {
        if let Some(panel) = self.panels.get_mut(panel_index) {
            panel.applets.push(applet.to_string());
        }
    }

    pub fn add_desklet(&mut self, desklet: &str) {
        self.desklets.push(desklet.to_string());
    }

    pub fn install_theme(&mut self, theme: &str) {
        self.themes.push(theme.to_string());
    }

    pub fn enable_extension(&mut self, extension: &str) {
        self.extensions.push(extension.to_string());
    }

    pub fn get_panel_count(&self) -> usize {
        self.panels.len()
    }
}

impl Default for CinnamonDesktopManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 4. XApp Cross-Desktop Integration
/// ============================================================================

#[derive(Debug, Clone)]
pub struct XAppPreferences {
    pub dark_mode: bool,
    pub accent_color: String,
    pub font_size: u32,
    pub animations_enabled: bool,
}

impl XAppPreferences {
    pub fn new() -> Self {
        Self {
            dark_mode: false,
            accent_color: String::from("#3daee9"),
            font_size: 12,
            animations_enabled: true,
        }
    }

    pub fn set_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }

    pub fn set_accent_color(&mut self, color: &str) {
        self.accent_color = color.to_string();
    }

    pub fn toggle_animations(&mut self) {
        self.animations_enabled = !self.animations_enabled;
    }

    pub fn apply_to_environment(&self) -> String {
        std::format!(
            "XApp Preferences:\n\
             Dark Mode: {}\n\
             Accent Color: {}\n\
             Font Size: {}\n\
             Animations: {}",
            self.dark_mode,
            self.accent_color,
            self.font_size,
            self.animations_enabled
        )
    }
}

impl Default for XAppPreferences {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 5. MintSystem - System Configuration
/// ============================================================================

#[derive(Debug, Clone)]
pub struct MintSystemConfig {
    pub update_level: UpdateLevel,
    pub kernel_update_auto: bool,
    pub timeshift_enabled: bool,
    pub firewall_enabled: bool,
}

impl MintSystemConfig {
    pub fn new() -> Self {
        Self {
            update_level: UpdateLevel::Recommended,
            kernel_update_auto: false,
            timeshift_enabled: true,
            firewall_enabled: true,
        }
    }

    pub fn set_update_level(&mut self, level: UpdateLevel) {
        self.update_level = level;
    }

    pub fn enable_kernel_auto_updates(&mut self) {
        self.kernel_update_auto = true;
    }

    pub fn configure_timeshift(&mut self, enabled: bool) {
        self.timeshift_enabled = enabled;
    }

    pub fn configure_firewall(&mut self, enabled: bool) {
        self.firewall_enabled = enabled;
    }

    pub fn generate_system_config(&self) -> String {
        std::format!(
            "MintSystem Configuration:\n\
             Update Level: {:?}\n\
             Kernel Auto-Updates: {}\n\
             Timeshift: {}\n\
             Firewall: {}",
            self.update_level,
            self.kernel_update_auto,
            self.timeshift_enabled,
            self.firewall_enabled
        )
    }
}

impl Default for MintSystemConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 6. MintStick - USB Image Writer & Formatter Engine
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbTargetFormat {
    Fat32,
    Ntfs,
    Ext4,
    ExFat,
}

#[derive(Debug, Clone)]
pub struct MintStickUsbDevice {
    pub dev_path: String,
    pub vendor: String,
    pub capacity_bytes: u64,
    pub is_read_only: bool,
}

#[derive(Debug, Clone)]
pub struct MintStickUsbWriterEngine {
    pub connected_drives: Vec<MintStickUsbDevice>,
    pub verify_checksum: bool,
}

impl MintStickUsbWriterEngine {
    pub fn new() -> Self {
        Self {
            connected_drives: Vec::new(),
            verify_checksum: true,
        }
    }

    pub fn register_drive(&mut self, dev_path: &str, vendor: &str, capacity_bytes: u64) {
        self.connected_drives.push(MintStickUsbDevice {
            dev_path: dev_path.to_string(),
            vendor: vendor.to_string(),
            capacity_bytes,
            is_read_only: false,
        });
    }

    pub fn format_drive(
        &mut self,
        dev_path: &str,
        format_type: UsbTargetFormat,
        label: &str,
    ) -> Result<String, &'static str> {
        let drive = self
            .connected_drives
            .iter()
            .find(|d| d.dev_path == dev_path)
            .ok_or("USB drive not found")?;

        if drive.is_read_only {
            return Err("USB drive is read-only");
        }

        Ok(std::format!(
            "Formatted {} ({}) to {:?} with label '{}'",
            drive.dev_path,
            drive.vendor,
            format_type,
            label
        ))
    }

    pub fn write_iso_image(
        &mut self,
        dev_path: &str,
        iso_path: &str,
    ) -> Result<String, &'static str> {
        let drive = self
            .connected_drives
            .iter()
            .find(|d| d.dev_path == dev_path)
            .ok_or("Target USB drive not found")?;

        if drive.is_read_only {
            return Err("Cannot write ISO to read-only drive");
        }

        Ok(std::format!(
            "Flashed ISO '{}' to drive {} ({}) successfully",
            iso_path,
            drive.dev_path,
            drive.vendor
        ))
    }
}

impl Default for MintStickUsbWriterEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 7. Warpinator - LAN File Sharing Engine
/// ============================================================================

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub name: String,
    pub ip_address: String,
    pub port: u16,
    pub is_trusted: bool,
}

#[derive(Debug, Clone)]
pub struct WarpinatorTransferRequest {
    pub request_id: u64,
    pub sender_name: String,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub is_accepted: bool,
}

#[derive(Debug, Clone)]
pub struct WarpinatorLanFileSharingEngine {
    pub discovered_peers: Vec<WarpinatorPeer>,
    pub pending_requests: Vec<WarpinatorTransferRequest>,
    pub group_code: String,
}

impl WarpinatorLanFileSharingEngine {
    pub fn new(group_code: &str) -> Self {
        Self {
            discovered_peers: Vec::new(),
            pending_requests: Vec::new(),
            group_code: group_code.to_string(),
        }
    }

    pub fn add_peer(&mut self, name: &str, ip_address: &str, port: u16) {
        self.discovered_peers.push(WarpinatorPeer {
            name: name.to_string(),
            ip_address: ip_address.to_string(),
            port,
            is_trusted: true,
        });
    }

    pub fn send_file_request(
        &mut self,
        peer_name: &str,
        file_name: &str,
        file_size_bytes: u64,
    ) -> Result<u64, &'static str> {
        let _peer = self
            .discovered_peers
            .iter()
            .find(|p| p.name == peer_name)
            .ok_or("Warpinator peer not found on LAN")?;

        let request_id = (self.pending_requests.len() + 1) as u64;
        self.pending_requests.push(WarpinatorTransferRequest {
            request_id,
            sender_name: "SigmaOS-Local".to_string(),
            file_name: file_name.to_string(),
            file_size_bytes,
            is_accepted: false,
        });

        Ok(request_id)
    }

    pub fn accept_request(&mut self, request_id: u64) -> Result<(), &'static str> {
        let req = self
            .pending_requests
            .iter_mut()
            .find(|r| r.request_id == request_id)
            .ok_or("Transfer request not found")?;
        req.is_accepted = true;
        Ok(())
    }
}

impl Default for WarpinatorLanFileSharingEngine {
    fn default() -> Self {
        Self::new("SigmaOS-Warpinator-Group")
    }
}

/// ============================================================================
/// 8. Hypnotix - IPTV Streaming Engine
/// ============================================================================

#[derive(Debug, Clone)]
pub struct HypnotixChannel {
    pub id: String,
    pub name: String,
    pub category: String,
    pub stream_url: String,
    pub logo_url: String,
}

#[derive(Debug, Clone)]
pub struct HypnotixIptvStreamingEngine {
    pub providers: Vec<String>,
    pub channels: Vec<HypnotixChannel>,
    pub current_channel: Option<String>,
}

impl HypnotixIptvStreamingEngine {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            channels: Vec::new(),
            current_channel: None,
        }
    }

    pub fn add_provider(&mut self, provider_name: &str) {
        self.providers.push(provider_name.to_string());
    }

    pub fn add_channel(&mut self, id: &str, name: &str, category: &str, stream_url: &str) {
        self.channels.push(HypnotixChannel {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            stream_url: stream_url.to_string(),
            logo_url: String::new(),
        });
    }

    pub fn play_channel(&mut self, id: &str) -> Result<String, &'static str> {
        let ch = self
            .channels
            .iter()
            .find(|c| c.id == id)
            .ok_or("Hypnotix channel not found")?;
        self.current_channel = Some(id.to_string());
        Ok(std::format!(
            "Streaming '{}' from {}",
            ch.name,
            ch.stream_url
        ))
    }
}

impl Default for HypnotixIptvStreamingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 9. MintReport - System Diagnostic & Error Reporter
/// ============================================================================

#[derive(Debug, Clone)]
pub struct MintReportIssue {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub is_resolved: bool,
}

#[derive(Debug, Clone)]
pub struct MintReportSystemDiagnosticEngine {
    pub detected_issues: Vec<MintReportIssue>,
    pub system_information_summary: String,
}

impl MintReportSystemDiagnosticEngine {
    pub fn new() -> Self {
        Self {
            detected_issues: Vec::new(),
            system_information_summary: String::from("SigmaOS Linux Mint System Info: OK"),
        }
    }

    pub fn report_issue(&mut self, id: u32, title: &str, category: &str) {
        self.detected_issues.push(MintReportIssue {
            id,
            title: title.to_string(),
            category: category.to_string(),
            is_resolved: false,
        });
    }

    pub fn resolve_issue(&mut self, id: u32) -> bool {
        if let Some(issue) = self.detected_issues.iter_mut().find(|i| i.id == id) {
            issue.is_resolved = true;
            true
        } else {
            false
        }
    }

    pub fn unresolved_issues_count(&self) -> usize {
        self.detected_issues
            .iter()
            .filter(|i| !i.is_resolved)
            .count()
    }
}

impl Default for MintReportSystemDiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// 10. Linux Mint Integration Engine
/// ============================================================================

#[derive(Debug, Clone)]
pub struct LinuxMintIntegrationEngine {
    pub update_manager: MintUpdateManager,
    pub software_manager: MintInstallManager,
    pub desktop_manager: CinnamonDesktopManager,
    pub xapp_preferences: XAppPreferences,
    pub system_config: MintSystemConfig,
    pub usb_writer: MintStickUsbWriterEngine,
    pub warpinator: WarpinatorLanFileSharingEngine,
    pub hypnotix: HypnotixIptvStreamingEngine,
    pub mint_report: MintReportSystemDiagnosticEngine,
}

impl LinuxMintIntegrationEngine {
    pub fn new() -> Self {
        Self {
            update_manager: MintUpdateManager::new(),
            software_manager: MintInstallManager::new(),
            desktop_manager: CinnamonDesktopManager::new(),
            xapp_preferences: XAppPreferences::new(),
            system_config: MintSystemConfig::new(),
            usb_writer: MintStickUsbWriterEngine::new(),
            warpinator: WarpinatorLanFileSharingEngine::new("SigmaOS-Warpinator"),
            hypnotix: HypnotixIptvStreamingEngine::new(),
            mint_report: MintReportSystemDiagnosticEngine::new(),
        }
    }

    pub fn check_updates(&mut self) -> String {
        self.update_manager.generate_update_summary()
    }

    pub fn install_package(&mut self, name: &str) -> Result<(), &'static str> {
        self.software_manager.install_package(name)
    }

    pub fn configure_desktop(&mut self) {
        self.desktop_manager
            .add_panel(CinnamonPanelPosition::Bottom, 48);
        self.desktop_manager
            .add_panel(CinnamonPanelPosition::Top, 32);
    }

    pub fn apply_mint_defaults(&mut self) {
        self.xapp_preferences.set_dark_mode(false);
        self.system_config
            .set_update_level(UpdateLevel::Recommended);
    }

    pub fn generate_integration_report(&self) -> String {
        std::format!(
            "Linux Mint Integration Report:\n\
             {}\n\
             Installed Packages: {}\n\
             Panels: {}\n\
             USB Drives: {}\n\
             Warpinator Peers: {}\n\
             Hypnotix Channels: {}\n\
             Unresolved System Reports: {}\n\
             {}\n\
             {}",
            self.update_manager.generate_update_summary(),
            self.software_manager.installed_packages.len(),
            self.desktop_manager.get_panel_count(),
            self.usb_writer.connected_drives.len(),
            self.warpinator.discovered_peers.len(),
            self.hypnotix.channels.len(),
            self.mint_report.unresolved_issues_count(),
            self.xapp_preferences.apply_to_environment(),
            self.system_config.generate_system_config()
        )
    }
}

impl Default for LinuxMintIntegrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();

        let security_update = UpdatePackage {
            name: "linux-kernel".to_string(),
            old_version: "5.15.0".to_string(),
            new_version: "5.15.1".to_string(),
            level: UpdateLevel::Security,
            size: 10 * 1024 * 1024,
            description: "Critical security fix".to_string(),
        };

        manager.add_update(security_update);
        manager.enable_auto_security_updates();

        assert_eq!(manager.get_security_updates().len(), 1);
        assert!(manager.auto_update_enabled);
    }

    #[test]
    fn test_mint_install_manager() {
        let mut manager = MintInstallManager::new();

        let vim = SoftwarePackage {
            name: "vim".to_string(),
            version: "8.2".to_string(),
            source: PackageSource::Apt,
            category: "Editor".to_string(),
            description: "Vi IMproved".to_string(),
            installed: false,
            rating: 4.5,
            icon_path: "/usr/share/icons/vim.png".to_string(),
        };

        manager.add_package(vim);
        assert!(manager.install_package("vim").is_ok());
        assert!(manager.installed_packages.contains(&"vim".to_string()));
    }

    #[test]
    fn test_cinnamon_desktop_manager() {
        let mut desktop = CinnamonDesktopManager::new();
        desktop.add_panel(CinnamonPanelPosition::Bottom, 48);
        desktop.add_applet_to_panel(0, "menu@cinnamon");

        assert_eq!(desktop.get_panel_count(), 1);
        assert!(desktop.panels[0]
            .applets
            .contains(&"menu@cinnamon".to_string()));
    }

    #[test]
    fn test_xapp_preferences() {
        let mut prefs = XAppPreferences::new();
        prefs.set_dark_mode(true);
        prefs.set_accent_color("#ff6b6b");

        assert!(prefs.dark_mode);
        assert_eq!(prefs.accent_color, "#ff6b6b");
    }

    #[test]
    fn test_mintstick_usb_writer() {
        let mut writer = MintStickUsbWriterEngine::new();
        writer.register_drive("/dev/sdb", "SanDisk", 32000000000);
        let res = writer.write_iso_image("/dev/sdb", "/home/iso/linuxmint.iso");
        assert!(res.is_ok());
        assert!(res.unwrap().contains("linuxmint.iso"));
    }

    #[test]
    fn test_warpinator_lan_sharing() {
        let mut warp = WarpinatorLanFileSharingEngine::new("Group1");
        warp.add_peer("Laptop", "192.168.1.50", 42000);
        let req_id = warp.send_file_request("Laptop", "doc.pdf", 1024).unwrap();
        assert!(warp.accept_request(req_id).is_ok());
    }

    #[test]
    fn test_hypnotix_iptv() {
        let mut iptv = HypnotixIptvStreamingEngine::new();
        iptv.add_channel("ch1", "News", "General", "http://stream.m3u8");
        let res = iptv.play_channel("ch1");
        assert!(res.is_ok());
        assert!(res.unwrap().contains("News"));
    }

    #[test]
    fn test_mint_report() {
        let mut report = MintReportSystemDiagnosticEngine::new();
        report.report_issue(1, "Missing Language Pack", "Localization");
        assert_eq!(report.unresolved_issues_count(), 1);
        assert!(report.resolve_issue(1));
        assert_eq!(report.unresolved_issues_count(), 0);
    }

    #[test]
    fn test_linux_mint_integration() {
        let mut mint = LinuxMintIntegrationEngine::new();
        mint.apply_mint_defaults();
        mint.configure_desktop();

        let report = mint.generate_integration_report();
        assert!(report.contains("Linux Mint Integration Report"));
        assert!(report.contains("Panels: 2"));
    }
}

// SigmaOS Omarchy Linux Gap Closure Components
// Clean-room, zero-dependency safe Rust implementations inspired by Omarchy 1.1.0:
// - Terminal Fastfetch ASCII Art Banner & Diagnostic Summary Generator
// - Interactive TUI Menu Applet for Terminal Tasks & Themes
// - Power Profile Manager for Dynamic CPU Governor Toggling
// - Dotfiles Snapshot Backup & Version Control Manager

use std::collections::{BTreeMap, HashMap};
use std::format;
use std::vec::Vec;

/// Omarchy Terminal Fastfetch ASCII Art Banner & Hardware Info Summary
pub struct OmarchyFastfetchEngine {
    pub os_name: String,
    pub kernel_version: String,
    pub active_theme: String,
    pub memory_used_mb: u32,
    pub memory_total_mb: u32,
}

impl OmarchyFastfetchEngine {
    pub fn new(os: &str, kernel: &str, theme: &str, used_mem: u32, total_mem: u32) -> Self {
        Self {
            os_name: os.to_string(),
            kernel_version: kernel.to_string(),
            active_theme: theme.to_string(),
            memory_used_mb: used_mem,
            memory_total_mb: total_mem,
        }
    }

    pub fn generate_summary_banner(&self) -> String {
        let ascii_art = r#"
   ____                         _   _
  / __ \                       | | | |
 | |  | |_ __ ___   __ _ _ __  | |_| |
 | |  | | '_ ` _ \ / _` | '__| |  _  |
 | |__| | | | | | | (_| | |    | | | |
  \____/|_| |_| |_|\__,_|_|    \_| |_/
"#;
        format!(
            "{}\nOS: {}\nKernel: {}\nTheme: {}\nMemory: {}MB / {}MB",
            ascii_art,
            self.os_name,
            self.kernel_version,
            self.active_theme,
            self.memory_used_mb,
            self.memory_total_mb
        )
    }
}

/// Omarchy Interactive TUI Menu Selection Applet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyTuiMenuItem {
    pub id: u32,
    pub title: String,
    pub command: String,
}

pub struct OmarchyTuiMenuApplet {
    pub items: Vec<OmarchyTuiMenuItem>,
    pub selected_index: usize,
}

impl OmarchyTuiMenuApplet {
    pub fn new() -> Self {
        let mut items = Vec::new();
        items.push(OmarchyTuiMenuItem {
            id: 1,
            title: "Switch Theme".to_string(),
            command: "sigomarchy theme-next".to_string(),
        });
        items.push(OmarchyTuiMenuItem {
            id: 2,
            title: "Launch Herdr Agent".to_string(),
            command: "sigomarchy herdr".to_string(),
        });
        items.push(OmarchyTuiMenuItem {
            id: 3,
            title: "Reload Hyprland Config".to_string(),
            command: "hyprctl reload".to_string(),
        });

        Self {
            items,
            selected_index: 0,
        }
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn get_selected_item(&self) -> Option<&OmarchyTuiMenuItem> {
        self.items.get(self.selected_index)
    }
}

impl Default for OmarchyTuiMenuApplet {
    fn default() -> Self {
        Self::new()
    }
}

/// Power Profile Mode Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyPowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

/// Omarchy Dynamic CPU Power Profile Manager
pub struct OmarchyPowerProfileManager {
    pub active_profile: OmarchyPowerProfile,
    pub cpu_governor: String,
    pub epp_setting: String,
}

impl OmarchyPowerProfileManager {
    pub fn new() -> Self {
        Self {
            active_profile: OmarchyPowerProfile::Balanced,
            cpu_governor: "powersave".to_string(),
            epp_setting: "balance_performance".to_string(),
        }
    }

    pub fn switch_profile(&mut self, profile: OmarchyPowerProfile) -> String {
        self.active_profile = profile;
        match profile {
            OmarchyPowerProfile::Performance => {
                self.cpu_governor = "performance".to_string();
                self.epp_setting = "performance".to_string();
            }
            OmarchyPowerProfile::Balanced => {
                self.cpu_governor = "powersave".to_string();
                self.epp_setting = "balance_performance".to_string();
            }
            OmarchyPowerProfile::PowerSaver => {
                self.cpu_governor = "powersave".to_string();
                self.epp_setting = "power".to_string();
            }
        }
        format!(
            "Switched power profile to {:?} ({})",
            profile, self.cpu_governor
        )
    }
}

impl Default for OmarchyPowerProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Dotfiles Snapshot Backup & Version Control Manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotfileBackupRecord {
    pub snapshot_id: u32,
    pub timestamp_epoch: u64,
    pub dotfiles_count: usize,
}

pub struct OmarchyDotfilesBackupRestoreEngine {
    pub backups: Vec<DotfileBackupRecord>,
    pub tracked_dotfiles: HashMap<String, String>,
}

impl OmarchyDotfilesBackupRestoreEngine {
    pub fn new() -> Self {
        let mut tracked = HashMap::new();
        tracked.insert(
            "hyprland.conf".to_string(),
            "~/.config/hypr/hyprland.conf".to_string(),
        );
        tracked.insert(
            "alacritty.toml".to_string(),
            "~/.config/alacritty/alacritty.toml".to_string(),
        );

        Self {
            backups: Vec::new(),
            tracked_dotfiles: tracked,
        }
    }

    pub fn create_backup_snapshot(&mut self, timestamp: u64) -> u32 {
        let snap_id = (self.backups.len() + 1) as u32;
        self.backups.push(DotfileBackupRecord {
            snapshot_id: snap_id,
            timestamp_epoch: timestamp,
            dotfiles_count: self.tracked_dotfiles.len(),
        });
        snap_id
    }
}

impl Default for OmarchyDotfilesBackupRestoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Waybar Module Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaybarModuleConfig {
    pub name: String,
    pub module_type: String,
    pub is_enabled: bool,
    pub poll_interval_sec: u32,
}

/// Omarchy Waybar Dynamic Status Bar Configuration Generator
pub struct OmarchyWaybarStatusConfigEngine {
    pub position: String,
    pub modules: Vec<WaybarModuleConfig>,
}

impl OmarchyWaybarStatusConfigEngine {
    pub fn new() -> Self {
        Self {
            position: "top".to_string(),
            modules: vec![
                WaybarModuleConfig {
                    name: "hyprland/workspaces".to_string(),
                    module_type: "workspaces".to_string(),
                    is_enabled: true,
                    poll_interval_sec: 0,
                },
                WaybarModuleConfig {
                    name: "cpu".to_string(),
                    module_type: "telemetry".to_string(),
                    is_enabled: true,
                    poll_interval_sec: 1,
                },
                WaybarModuleConfig {
                    name: "memory".to_string(),
                    module_type: "telemetry".to_string(),
                    is_enabled: true,
                    poll_interval_sec: 2,
                },
                WaybarModuleConfig {
                    name: "pulseaudio".to_string(),
                    module_type: "audio".to_string(),
                    is_enabled: true,
                    poll_interval_sec: 1,
                },
                WaybarModuleConfig {
                    name: "battery".to_string(),
                    module_type: "power".to_string(),
                    is_enabled: true,
                    poll_interval_sec: 5,
                },
            ],
        }
    }

    pub fn generate_waybar_json(&self) -> String {
        let mut json = format!("{{\"position\": \"{}\", \"modules\": [", self.position);
        for (i, m) in self.modules.iter().filter(|m| m.is_enabled).enumerate() {
            if i > 0 {
                json.push_str(", ");
            }
            json.push_str(&format!("\"{}\"", m.name));
        }
        json.push_str("]}");
        json
    }
}

impl Default for OmarchyWaybarStatusConfigEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Rofi Menu Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RofiCategory {
    Apps,
    Windows,
    Themes,
    Ssh,
    Calculator,
}

/// Omarchy Rofi Command Palette Menu Item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RofiMenuItem {
    pub label: String,
    pub action_command: String,
    pub category: RofiCategory,
}

/// Omarchy Rofi Command Palette Router
pub struct OmarchyRofiCommandPaletteApplet {
    pub items: Vec<RofiMenuItem>,
}

impl OmarchyRofiCommandPaletteApplet {
    pub fn new() -> Self {
        Self {
            items: vec![
                RofiMenuItem {
                    label: "Launch Alacritty".to_string(),
                    action_command: "alacritty".to_string(),
                    category: RofiCategory::Apps,
                },
                RofiMenuItem {
                    label: "Switch Theme: Nord".to_string(),
                    action_command: "sigomarchy theme nord".to_string(),
                    category: RofiCategory::Themes,
                },
                RofiMenuItem {
                    label: "Connect SSH prod-node".to_string(),
                    action_command: "ssh admin@10.0.0.1".to_string(),
                    category: RofiCategory::Ssh,
                },
            ],
        }
    }

    pub fn filter_by_query(&self, query: &str) -> Vec<&RofiMenuItem> {
        self.items
            .iter()
            .filter(|i| i.label.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

impl Default for OmarchyRofiCommandPaletteApplet {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy SDDM & Plymouth Display Theme Customizer Engine
pub struct OmarchySddmThemeCustomizerEngine {
    pub active_sddm_theme: String,
    pub active_plymouth_splash: String,
}

impl OmarchySddmThemeCustomizerEngine {
    pub fn new() -> Self {
        Self {
            active_sddm_theme: "omarchy-catppuccin".to_string(),
            active_plymouth_splash: "omarchy-breeze".to_string(),
        }
    }

    pub fn apply_theme(&mut self, theme_name: &str) -> String {
        self.active_sddm_theme = format!("omarchy-{}", theme_name);
        self.active_plymouth_splash = format!("omarchy-{}", theme_name);
        format!("Applied theme '{}' to SDDM & Plymouth", theme_name)
    }
}

impl Default for OmarchySddmThemeCustomizerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy WirePlumber Spatial Audio & Bluetooth Codec Engine
pub struct OmarchyWirePlumberAudioProfileEngine {
    pub active_codec: String,
    pub spatial_audio_enabled: bool,
}

impl OmarchyWirePlumberAudioProfileEngine {
    pub fn new() -> Self {
        Self {
            active_codec: "ldac".to_string(),
            spatial_audio_enabled: true,
        }
    }

    pub fn set_bluetooth_codec(&mut self, codec: &str) {
        self.active_codec = codec.to_string();
    }

    pub fn toggle_spatial_audio(&mut self) -> bool {
        self.spatial_audio_enabled = !self.spatial_audio_enabled;
        self.spatial_audio_enabled
    }
}

impl Default for OmarchyWirePlumberAudioProfileEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Pacman & YAY AUR Update Gateway Engine
pub struct OmarchyPacmanYayUpdateGatewayEngine {
    pub parallel_downloads: u32,
    pub create_snapshot_before_update: bool,
    pub pending_updates: Vec<String>,
}

impl OmarchyPacmanYayUpdateGatewayEngine {
    pub fn new() -> Self {
        Self {
            parallel_downloads: 5,
            create_snapshot_before_update: true,
            pending_updates: vec!["hyprland".to_string(), "waybar".to_string(), "alacritty".to_string()],
        }
    }

    pub fn execute_omarchy_update(&mut self) -> (usize, bool) {
        let count = self.pending_updates.len();
        self.pending_updates.clear();
        (count, self.create_snapshot_before_update)
    }
}

impl Default for OmarchyPacmanYayUpdateGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. OMARCHY HYPRLAND AUTOTILING & GESTURE BINDER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyprlandLayoutMode {
    Dwindle,
    Master,
    Hyprscroller,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyprlandGestureRule {
    pub swipe_fingers: u8,
    pub direction: String, // "left", "right", "up", "down"
    pub command: String,
}

pub struct OmarchyHyprlandAutotilingEngine {
    pub active_layout: HyprlandLayoutMode,
    pub master_factor: f32,
    pub gestures: Vec<HyprlandGestureRule>,
}

impl OmarchyHyprlandAutotilingEngine {
    pub fn new() -> Self {
        Self {
            active_layout: HyprlandLayoutMode::Dwindle,
            master_factor: 0.55,
            gestures: vec![
                HyprlandGestureRule {
                    swipe_fingers: 3,
                    direction: "right".to_string(),
                    command: "hyprctl dispatch workspace e+1".to_string(),
                },
                HyprlandGestureRule {
                    swipe_fingers: 3,
                    direction: "left".to_string(),
                    command: "hyprctl dispatch workspace e-1".to_string(),
                },
            ],
        }
    }

    pub fn set_layout_mode(&mut self, mode: HyprlandLayoutMode) {
        self.active_layout = mode;
    }

    pub fn add_gesture(&mut self, fingers: u8, dir: &str, cmd: &str) {
        self.gestures.push(HyprlandGestureRule {
            swipe_fingers: fingers,
            direction: dir.to_string(),
            command: cmd.to_string(),
        });
    }

    pub fn dispatch_gesture(&self, fingers: u8, dir: &str) -> Option<String> {
        self.gestures
            .iter()
            .find(|g| g.swipe_fingers == fingers && g.direction == dir)
            .map(|g| g.command.clone())
    }
}

impl Default for OmarchyHyprlandAutotilingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 14. OMARCHY CATPPUCCIN THEME & GTK/QT/KVANTUM SYNCS ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatppuccinFlavor {
    Mocha,
    Macchiato,
    Frappe,
    Latte,
}

pub struct OmarchyCatppuccinThemeSyncEngine {
    pub active_flavor: CatppuccinFlavor,
    pub gtk_theme: String,
    pub qt_kvantum_theme: String,
    pub accent_color: String,
}

impl OmarchyCatppuccinThemeSyncEngine {
    pub fn new() -> Self {
        Self {
            active_flavor: CatppuccinFlavor::Mocha,
            gtk_theme: "Catppuccin-Mocha-Standard-Blue-Dark".to_string(),
            qt_kvantum_theme: "Catppuccin-Mocha-Blue".to_string(),
            accent_color: "mauve".to_string(),
        }
    }

    pub fn switch_flavor(&mut self, flavor: CatppuccinFlavor) -> String {
        self.active_flavor = flavor;
        let flavor_str = match flavor {
            CatppuccinFlavor::Mocha => "Mocha",
            CatppuccinFlavor::Macchiato => "Macchiato",
            CatppuccinFlavor::Frappe => "Frappe",
            CatppuccinFlavor::Latte => "Latte",
        };

        self.gtk_theme = format!("Catppuccin-{}-Standard-Blue-Dark", flavor_str);
        self.qt_kvantum_theme = format!("Catppuccin-{}-Blue", flavor_str);
        format!("Synced GTK & Qt/Kvantum themes to Catppuccin {}", flavor_str)
    }
}

impl Default for OmarchyCatppuccinThemeSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 15. OMARCHY NETWORKMANAGER / IWD WI-FI SCANNER & CONNECTION ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WifiAccessPoint {
    pub ssid: String,
    pub signal_strength_pct: u8,
    pub security_type: String, // "WPA2", "WPA3", "Open"
    pub is_connected: bool,
}

pub struct OmarchyWifiNetworkManagerEngine {
    pub available_networks: Vec<WifiAccessPoint>,
    pub active_ssid: Option<String>,
}

impl OmarchyWifiNetworkManagerEngine {
    pub fn new() -> Self {
        Self {
            available_networks: vec![
                WifiAccessPoint {
                    ssid: "Omarchy-Mesh-5G".to_string(),
                    signal_strength_pct: 95,
                    security_type: "WPA3".to_string(),
                    is_connected: true,
                },
                WifiAccessPoint {
                    ssid: "Guest-Wi-Fi".to_string(),
                    signal_strength_pct: 70,
                    security_type: "WPA2".to_string(),
                    is_connected: false,
                },
            ],
            active_ssid: Some("Omarchy-Mesh-5G".to_string()),
        }
    }

    pub fn scan_networks(&mut self) -> usize {
        self.available_networks.len()
    }

    pub fn connect_network(&mut self, ssid: &str, _passphrase: &str) -> Result<(), &'static str> {
        let exists = self.available_networks.iter().any(|n| n.ssid == ssid);
        if !exists {
            return Err("iwd: Network SSID not found in scan results");
        }

        for net in &mut self.available_networks {
            net.is_connected = net.ssid == ssid;
        }
        self.active_ssid = Some(ssid.to_string());
        Ok(())
    }
}

impl Default for OmarchyWifiNetworkManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 16. OMARCHY BLUEZ BLUETOOTH PAIRING & BATTERY TELEMETRY ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BluetoothDevice {
    pub mac_address: String,
    pub name: String,
    pub battery_percent: Option<u8>,
    pub is_paired: bool,
    pub is_connected: bool,
}

pub struct OmarchyBluezBluetoothEngine {
    pub devices: BTreeMap<String, BluetoothDevice>,
}

impl OmarchyBluezBluetoothEngine {
    pub fn new() -> Self {
        let mut devices = BTreeMap::new();
        devices.insert(
            "00:11:22:33:44:55".to_string(),
            BluetoothDevice {
                mac_address: "00:11:22:33:44:55".to_string(),
                name: "Sony WH-1000XM5".to_string(),
                battery_percent: Some(85),
                is_paired: true,
                is_connected: true,
            },
        );

        Self { devices }
    }

    pub fn pair_device(&mut self, mac: &str, name: &str) {
        self.devices.insert(
            mac.to_string(),
            BluetoothDevice {
                mac_address: mac.to_string(),
                name: name.to_string(),
                battery_percent: Some(100),
                is_paired: true,
                is_connected: true,
            },
        );
    }

    pub fn get_battery_telemetry(&self, mac: &str) -> Option<u8> {
        self.devices.get(mac).and_then(|d| d.battery_percent)
    }
}

impl Default for OmarchyBluezBluetoothEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 17. OMARCHY BTOP RESOURCE MONITOR CONFIGURATION GENERATOR
// =========================================================================

pub struct OmarchyBtopConfigGeneratorEngine {
    pub color_theme: String,
    pub update_ms: u32,
    pub proc_sorting: String,
}

impl OmarchyBtopConfigGeneratorEngine {
    pub fn new() -> Self {
        Self {
            color_theme: "catppuccin_mocha".to_string(),
            update_ms: 1000,
            proc_sorting: "cpu lazy".to_string(),
        }
    }

    pub fn generate_btop_config_text(&self) -> String {
        format!(
            "color_theme = \"{}\"\nupdate_ms = {}\nproc_sorting = \"{}\"\nshown_boxes = \"cpu mem net proc\"",
            self.color_theme, self.update_ms, self.proc_sorting
        )
    }
}

impl Default for OmarchyBtopConfigGeneratorEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_fastfetch_engine() {
        let ff = OmarchyFastfetchEngine::new("SigmaOS", "6.8.0", "TokyoNight", 2048, 16384);
        let banner = ff.generate_summary_banner();
        assert!(banner.contains("SigmaOS"));
        assert!(banner.contains("TokyoNight"));
        assert!(banner.contains("2048MB / 16384MB"));
    }

    #[test]
    fn test_omarchy_tui_menu_applet() {
        let mut applet = OmarchyTuiMenuApplet::new();
        assert_eq!(applet.selected_index, 0);
        assert_eq!(applet.get_selected_item().unwrap().title, "Switch Theme");

        applet.select_next();
        assert_eq!(applet.selected_index, 1);
        assert_eq!(
            applet.get_selected_item().unwrap().title,
            "Launch Herdr Agent"
        );
    }

    #[test]
    fn test_omarchy_power_profile_manager() {
        let mut power = OmarchyPowerProfileManager::new();
        let res = power.switch_profile(OmarchyPowerProfile::Performance);
        assert!(res.contains("Performance"));
        assert_eq!(power.cpu_governor, "performance");
        assert_eq!(power.epp_setting, "performance");
    }

    #[test]
    fn test_omarchy_dotfiles_backup_restore_engine() {
        let mut dotfiles = OmarchyDotfilesBackupRestoreEngine::new();
        let snap_id = dotfiles.create_backup_snapshot(1700000000);
        assert_eq!(snap_id, 1);
        assert_eq!(dotfiles.backups.len(), 1);
        assert_eq!(dotfiles.backups[0].dotfiles_count, 2);
    }

    #[test]
    fn test_omarchy_waybar_status_config_engine() {
        let waybar = OmarchyWaybarStatusConfigEngine::new();
        let json = waybar.generate_waybar_json();
        assert!(json.contains("\"position\": \"top\""));
        assert!(json.contains("hyprland/workspaces"));
        assert!(json.contains("pulseaudio"));
    }

    #[test]
    fn test_omarchy_rofi_command_palette_applet() {
        let rofi = OmarchyRofiCommandPaletteApplet::new();
        let matches = rofi.filter_by_query("Nord");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].label, "Switch Theme: Nord");
    }

    #[test]
    fn test_omarchy_sddm_theme_customizer_engine() {
        let mut sddm = OmarchySddmThemeCustomizerEngine::new();
        let status = sddm.apply_theme("nord");
        assert!(status.contains("Applied theme 'nord'"));
        assert_eq!(sddm.active_sddm_theme, "omarchy-nord");
        assert_eq!(sddm.active_plymouth_splash, "omarchy-nord");
    }

    #[test]
    fn test_omarchy_wireplumber_audio_profile_engine() {
        let mut wp = OmarchyWirePlumberAudioProfileEngine::new();
        wp.set_bluetooth_codec("aptx_hd");
        assert_eq!(wp.active_codec, "aptx_hd");

        assert!(!wp.toggle_spatial_audio());
        assert!(wp.toggle_spatial_audio());
    }

    #[test]
    fn test_omarchy_pacman_yay_update_gateway_engine() {
        let mut gateway = OmarchyPacmanYayUpdateGatewayEngine::new();
        let (updated, snap_created) = gateway.execute_omarchy_update();
        assert_eq!(updated, 3);
        assert!(snap_created);
        assert_eq!(gateway.pending_updates.len(), 0);
    }

    #[test]
    fn test_omarchy_hyprland_autotiling_engine() {
        let mut hypr = OmarchyHyprlandAutotilingEngine::new();
        hypr.set_layout_mode(HyprlandLayoutMode::Master);
        assert_eq!(hypr.active_layout, HyprlandLayoutMode::Master);

        let cmd = hypr.dispatch_gesture(3, "right").unwrap();
        assert!(cmd.contains("workspace e+1"));
    }

    #[test]
    fn test_omarchy_catppuccin_theme_sync_engine() {
        let mut cat = OmarchyCatppuccinThemeSyncEngine::new();
        let msg = cat.switch_flavor(CatppuccinFlavor::Frappe);
        assert!(msg.contains("Frappe"));
        assert_eq!(cat.gtk_theme, "Catppuccin-Frappe-Standard-Blue-Dark");
        assert_eq!(cat.qt_kvantum_theme, "Catppuccin-Frappe-Blue");
    }

    #[test]
    fn test_omarchy_wifi_network_manager_engine() {
        let mut wifi = OmarchyWifiNetworkManagerEngine::new();
        assert_eq!(wifi.scan_networks(), 2);

        assert!(wifi.connect_network("Guest-Wi-Fi", "secret").is_ok());
        assert_eq!(wifi.active_ssid, Some("Guest-Wi-Fi".to_string()));
    }

    #[test]
    fn test_omarchy_bluez_bluetooth_engine() {
        let mut bt = OmarchyBluezBluetoothEngine::new();
        assert_eq!(bt.get_battery_telemetry("00:11:22:33:44:55"), Some(85));

        bt.pair_device("AA:BB:CC:DD:EE:FF", "Bose QC45");
        assert_eq!(bt.get_battery_telemetry("AA:BB:CC:DD:EE:FF"), Some(100));
    }

    #[test]
    fn test_omarchy_btop_config_generator_engine() {
        let btop = OmarchyBtopConfigGeneratorEngine::new();
        let cfg = btop.generate_btop_config_text();
        assert!(cfg.contains("color_theme = \"catppuccin_mocha\""));
        assert!(cfg.contains("shown_boxes = \"cpu mem net proc\""));
    }
}

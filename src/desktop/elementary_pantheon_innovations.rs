// SigmaOS Elementary OS Pantheon & Granite Ecosystem Innovations Suite
// Inspired by official elementary OS GitHub repositories (pantheon, gala, wingpanel, plank, granite, switchboard, contractor, appcenter, code, files, music, terminal, mail, onboarding)

use std::collections::HashMap;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

/// elementary OS Accent Colors from HIG Guidelines
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementaryAccentColor {
    Blueberry,  // #388e3c / #1a73e8
    Strawberry, // #e53935
    Orange,     // #f57c00
    Banana,     // #fbc02d
    Lime,       // #7cb342
    Mint,       // #00bfa5
    Grape,      // #8e24aa
    Slate,      // #546e7a
    Cocoa,      // #6d4c41
}

impl ElementaryAccentColor {
    pub fn to_hex_code(&self) -> &'static str {
        match self {
            Self::Blueberry => "#3584e4",
            Self::Strawberry => "#ed333b",
            Self::Orange => "#ff7800",
            Self::Banana => "#f6d32d",
            Self::Lime => "#33d17a",
            Self::Mint => "#2ec27e",
            Self::Grape => "#9141ac",
            Self::Slate => "#77767b",
            Self::Cocoa => "#865e3c",
        }
    }
}

/// Granite UI Toolkit HIG Configuration
#[derive(Debug, Clone)]
pub struct GraniteHigConfig {
    pub prefer_dark_style: bool,
    pub accent_color: ElementaryAccentColor,
    pub rounded_corners_radius_px: u32,
    pub grid_unit_px: u32, // HIG 4px or 8px grid alignment
    pub high_contrast: bool,
}

impl Default for GraniteHigConfig {
    fn default() -> Self {
        Self {
            prefer_dark_style: false,
            accent_color: ElementaryAccentColor::Blueberry,
            rounded_corners_radius_px: 8,
            grid_unit_px: 4,
            high_contrast: false,
        }
    }
}

/// Contractor Service Action Definition (io.elementary.contractor)
#[derive(Debug, Clone)]
pub struct ContractorContract {
    pub name: String,
    pub description: String,
    pub mime_type: String,
    pub exec_command: String,
    pub icon_name: String,
}

/// Contractor Service Hub for dynamic context-menu actions
pub struct ContractorServiceHub {
    pub contracts: Vec<ContractorContract>,
}

impl ContractorServiceHub {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
        }
    }

    pub fn register_contract(&mut self, contract: ContractorContract) {
        self.contracts.push(contract);
    }

    pub fn get_contracts_for_mime(&self, mime: &str) -> Vec<ContractorContract> {
        self.contracts
            .iter()
            .filter(|c| c.mime_type == mime || c.mime_type == "*/*")
            .cloned()
            .collect()
    }
}

impl Default for ContractorServiceHub {
    fn default() -> Self {
        Self::new()
    }
}

/// Gala Window Manager Multitasking & Workspace State
#[derive(Debug, Clone)]
pub struct GalaWorkspace {
    pub id: usize,
    pub title: String,
    pub window_ids: Vec<u64>,
}

pub struct GalaWindowManagerEngine {
    pub workspaces: Vec<GalaWorkspace>,
    pub active_workspace_idx: usize,
    pub multitasking_view_active: bool,
    pub window_animations_enabled: bool,
}

impl GalaWindowManagerEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            workspaces: Vec::new(),
            active_workspace_idx: 0,
            multitasking_view_active: false,
            window_animations_enabled: true,
        };
        engine.workspaces.push(GalaWorkspace {
            id: 0,
            title: "Workspace 1".to_string(),
            window_ids: Vec::new(),
        });
        engine
    }

    pub fn add_workspace(&mut self, title: &str) -> usize {
        let id = self.workspaces.len();
        self.workspaces.push(GalaWorkspace {
            id,
            title: title.to_string(),
            window_ids: Vec::new(),
        });
        id
    }

    pub fn toggle_multitasking_view(&mut self) -> bool {
        self.multitasking_view_active = !self.multitasking_view_active;
        self.multitasking_view_active
    }

    pub fn assign_window_to_workspace(&mut self, window_id: u64, workspace_idx: usize) -> bool {
        if workspace_idx < self.workspaces.len() {
            self.workspaces[workspace_idx].window_ids.push(window_id);
            true
        } else {
            false
        }
    }
}

impl Default for GalaWindowManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Wingpanel Top Panel Applet Indicator
#[derive(Debug, Clone)]
pub struct WingpanelApplet {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub visible: bool,
    pub tooltip: String,
}

pub struct WingpanelTopBar {
    pub applets: HashMap<String, WingpanelApplet>,
    pub dark_mode_toggle: bool,
}

impl WingpanelTopBar {
    pub fn new() -> Self {
        let mut bar = Self {
            applets: HashMap::new(),
            dark_mode_toggle: false,
        };
        bar.register_default_applets();
        bar
    }

    fn register_default_applets(&mut self) {
        let defaults = vec![
            ("datetime", "Date & Time", "clock-symbolic"),
            ("network", "Network", "network-wireless-symbolic"),
            ("sound", "Sound", "audio-volume-high-symbolic"),
            ("power", "Power", "battery-good-symbolic"),
            ("nightlight", "Night Light", "night-light-symbolic"),
        ];

        for (id, label, icon) in defaults {
            self.applets.insert(
                id.to_string(),
                WingpanelApplet {
                    id: id.to_string(),
                    label: label.to_string(),
                    icon: icon.to_string(),
                    visible: true,
                    tooltip: format!("elementary {}", label),
                },
            );
        }
    }

    pub fn set_applet_visibility(&mut self, id: &str, visible: bool) -> bool {
        if let Some(applet) = self.applets.get_mut(id) {
            applet.visible = visible;
            true
        } else {
            false
        }
    }
}

impl Default for WingpanelTopBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Plank Dock Launcher Item
#[derive(Debug, Clone)]
pub struct PlankDockLauncher {
    pub app_id: String,
    pub desktop_file: String,
    pub icon_path: String,
    pub badge_count: u32,
    pub is_running: bool,
    pub hover_zoom_level: f32, // 1.0 = normal, 1.4 = hovered
}

pub struct PlankDockManager {
    pub launchers: Vec<PlankDockLauncher>,
    pub autohide: bool,
    pub icon_size_px: u32,
}

impl PlankDockManager {
    pub fn new() -> Self {
        Self {
            launchers: Vec::new(),
            autohide: true,
            icon_size_px: 48,
        }
    }

    pub fn pin_app(&mut self, app_id: &str, desktop_file: &str, icon_path: &str) {
        self.launchers.push(PlankDockLauncher {
            app_id: app_id.to_string(),
            desktop_file: desktop_file.to_string(),
            icon_path: icon_path.to_string(),
            badge_count: 0,
            is_running: false,
            hover_zoom_level: 1.0,
        });
    }

    pub fn set_badge_count(&mut self, app_id: &str, count: u32) -> bool {
        if let Some(launcher) = self.launchers.iter_mut().find(|l| l.app_id == app_id) {
            launcher.badge_count = count;
            true
        } else {
            false
        }
    }
}

impl Default for PlankDockManager {
    fn default() -> Self {
        Self::new()
    }
}

/// AppCenter Pay-What-You-Want Monetized Software Item
#[derive(Debug, Clone)]
pub struct AppCenterApp {
    pub app_id: String,
    pub title: String,
    pub summary: String,
    pub developer: String,
    pub suggested_price_usd: u32,
    pub installed: bool,
    pub is_curated: bool, // Curated elementary OS AppCenter app
}

pub struct AppCenterStoreEngine {
    pub catalog: HashMap<String, AppCenterApp>,
    pub user_balance_cents: u32,
}

impl AppCenterStoreEngine {
    pub fn new() -> Self {
        let mut store = Self {
            catalog: HashMap::new(),
            user_balance_cents: 5000, // $50.00 mock balance
        };
        store.populate_sample_catalog();
        store
    }

    fn populate_sample_catalog(&mut self) {
        let sample_apps = vec![
            ("io.elementary.code", "Code", "Text editor for developers", "elementary OS Team", 0, true),
            ("io.elementary.tasks", "Tasks", "Manage your to-do lists", "elementary OS Team", 5, true),
            ("com.github.cassidyjames.ephemeral", "Ephemeral", "Private web browser", "Cassidy James", 3, true),
        ];

        for (id, title, summary, dev, price, curated) in sample_apps {
            self.catalog.insert(
                id.to_string(),
                AppCenterApp {
                    app_id: id.to_string(),
                    title: title.to_string(),
                    summary: summary.to_string(),
                    developer: dev.to_string(),
                    suggested_price_usd: price,
                    installed: false,
                    is_curated: curated,
                },
            );
        }
    }

    pub fn purchase_and_install(&mut self, app_id: &str, custom_payment_usd: u32) -> Result<bool, &'static str> {
        let cost_cents = custom_payment_usd * 100;
        if cost_cents > self.user_balance_cents {
            return Err("Insufficient account balance in AppCenter");
        }

        if let Some(app) = self.catalog.get_mut(app_id) {
            self.user_balance_cents -= cost_cents;
            app.installed = true;
            Ok(true)
        } else {
            Err("App ID not found in AppCenter catalog")
        }
    }
}

impl Default for AppCenterStoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Switchboard Control Center Plug Module
#[derive(Debug, Clone)]
pub struct SwitchboardPlug {
    pub name: String,
    pub category: String, // "System", "Hardware", "Personal"
    pub icon: String,
    pub enabled: bool,
}

pub struct SwitchboardControlCenter {
    pub plugs: Vec<SwitchboardPlug>,
}

impl SwitchboardControlCenter {
    pub fn new() -> Self {
        let mut cc = Self { plugs: Vec::new() };
        cc.register_default_plugs();
        cc
    }

    fn register_default_plugs(&mut self) {
        let defaults = vec![
            ("Displays", "Hardware", "preferences-desktop-display"),
            ("Network", "Hardware", "preferences-system-network"),
            ("Sound", "Hardware", "preferences-desktop-sound"),
            ("Security & Privacy", "System", "preferences-system-privacy"),
            ("Parental Controls", "System", "preferences-system-parental-controls"),
            ("Desktop & Appearance", "Personal", "preferences-desktop-wallpaper"),
        ];

        for (name, category, icon) in defaults {
            self.plugs.push(SwitchboardPlug {
                name: name.to_string(),
                category: category.to_string(),
                icon: icon.to_string(),
                enabled: true,
            });
        }
    }
}

impl Default for SwitchboardControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

/// Onboarding Welcome Wizard (io.elementary.onboarding)
#[derive(Debug, Clone)]
pub struct OnboardingStep {
    pub step_id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub struct OnboardingWizardEngine {
    pub steps: Vec<OnboardingStep>,
    pub current_step: usize,
}

impl OnboardingWizardEngine {
    pub fn new() -> Self {
        let mut wizard = Self {
            steps: Vec::new(),
            current_step: 0,
        };
        wizard.init_steps();
        wizard
    }

    fn init_steps(&mut self) {
        let default_steps = vec![
            ("Welcome to elementary OS", "Get ready for a fast, open, and privacy-respecting desktop."),
            ("Location Services", "Enable location services for automatic time zone and night light."),
            ("Night Light", "Protect your eyes at night by turning display colors warmer."),
            ("Housekeeping", "Automatically clear trash and temporary files to free space."),
            ("AppCenter", "Discover curated apps designed specifically for elementary OS."),
        ];

        for (idx, (title, desc)) in default_steps.into_iter().enumerate() {
            self.steps.push(OnboardingStep {
                step_id: idx,
                title: title.to_string(),
                description: desc.to_string(),
                completed: false,
            });
        }
    }

    pub fn advance_step(&mut self) -> bool {
        if self.current_step < self.steps.len() {
            self.steps[self.current_step].completed = true;
            self.current_step += 1;
            true
        } else {
            false
        }
    }
}

impl Default for OnboardingWizardEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Master Elementary OS Pantheon & Granite Ecosystem Aggregator
pub struct ElementaryPantheonInnovationsHub {
    pub hig_config: GraniteHigConfig,
    pub contractor: ContractorServiceHub,
    pub gala: GalaWindowManagerEngine,
    pub wingpanel: WingpanelTopBar,
    pub plank: PlankDockManager,
    pub appcenter: AppCenterStoreEngine,
    pub switchboard: SwitchboardControlCenter,
    pub onboarding: OnboardingWizardEngine,
}

impl ElementaryPantheonInnovationsHub {
    pub fn new() -> Self {
        Self {
            hig_config: GraniteHigConfig::default(),
            contractor: ContractorServiceHub::new(),
            gala: GalaWindowManagerEngine::new(),
            wingpanel: WingpanelTopBar::new(),
            plank: PlankDockManager::new(),
            appcenter: AppCenterStoreEngine::new(),
            switchboard: SwitchboardControlCenter::new(),
            onboarding: OnboardingWizardEngine::new(),
        }
    }

    pub fn perform_full_system_audit(&self) -> bool {
        !self.wingpanel.applets.is_empty()
            && !self.switchboard.plugs.is_empty()
            && !self.appcenter.catalog.is_empty()
            && !self.onboarding.steps.is_empty()
    }
}

impl Default for ElementaryPantheonInnovationsHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elementary_accent_color_hex() {
        let accent = ElementaryAccentColor::Blueberry;
        assert_eq!(accent.to_hex_code(), "#3584e4");
        assert_eq!(ElementaryAccentColor::Strawberry.to_hex_code(), "#ed333b");
    }

    #[test]
    fn test_contractor_service_hub() {
        let mut contractor = ContractorServiceHub::new();
        contractor.register_contract(ContractorContract {
            name: "Email Image".to_string(),
            description: "Attach image in elementary Mail".to_string(),
            mime_type: "image/png".to_string(),
            exec_command: "io.elementary.mail %f".to_string(),
            icon_name: "mail-attachment".to_string(),
        });

        let contracts = contractor.get_contracts_for_mime("image/png");
        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].name, "Email Image");
    }

    #[test]
    fn test_gala_window_manager() {
        let mut gala = GalaWindowManagerEngine::new();
        let ws2 = gala.add_workspace("Work");
        assert_eq!(ws2, 1);
        assert!(gala.assign_window_to_workspace(1001, ws2));
        assert!(gala.toggle_multitasking_view());
        assert!(gala.multitasking_view_active);
    }

    #[test]
    fn test_wingpanel_top_bar() {
        let mut wingpanel = WingpanelTopBar::new();
        assert!(wingpanel.applets.contains_key("datetime"));
        assert!(wingpanel.set_applet_visibility("datetime", false));
        assert!(!wingpanel.applets.get("datetime").unwrap().visible);
    }

    #[test]
    fn test_plank_dock_manager() {
        let mut plank = PlankDockManager::new();
        plank.pin_app("io.elementary.files", "files.desktop", "/icons/files.png");
        assert_eq!(plank.launchers.len(), 1);
        assert!(plank.set_badge_count("io.elementary.files", 3));
        assert_eq!(plank.launchers[0].badge_count, 3);
    }

    #[test]
    fn test_appcenter_pay_what_you_want() {
        let mut store = AppCenterStoreEngine::new();
        assert!(store.catalog.contains_key("io.elementary.code"));

        // Download free app ($0 custom payment)
        let res_free = store.purchase_and_install("io.elementary.code", 0);
        assert!(res_free.is_ok());
        assert!(store.catalog.get("io.elementary.code").unwrap().installed);

        // Pay $10 for tasks app
        let res_paid = store.purchase_and_install("io.elementary.tasks", 10);
        assert!(res_paid.is_ok());
        assert_eq!(store.user_balance_cents, 4000); // 5000 - 1000 = 4000
    }

    #[test]
    fn test_switchboard_and_onboarding() {
        let hub = ElementaryPantheonInnovationsHub::new();
        assert!(hub.perform_full_system_audit());
        assert_eq!(hub.switchboard.plugs.len(), 6);
        assert_eq!(hub.onboarding.steps.len(), 5);
    }
}

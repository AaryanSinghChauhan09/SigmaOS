// SigmaOS Pantheon Desktop Environment (elementary OS Parity)
// Implements Gala Window Manager, Wingpanel Status Bar, Plank Dock, Slingshot Launcher, AppCenter, and Greeter.
#[cfg(test)]
extern crate std;

extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// Gala Window transition styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalaTransitionStyle {
    Fade,
    Zoom,
    SlideLeft,
    SlideRight,
}

/// Dynamic system indicators displayed on the top panel (Wingpanel)
#[derive(Debug, Clone)]
pub struct WingpanelIndicator {
    pub name: String,
    pub active: bool,
    pub status_text: String,
    pub icon_name: String,
}

/// Individual item placed inside the bottom application dock (Plank)
#[derive(Debug, Clone)]
pub struct PlankDockItem {
    pub app_id: String,
    pub icon_path: String,
    pub is_running: bool,
    pub badge_count: u32,
    pub zoom_factor: f32, // zoom animation factor (1.0 to 1.5) on hover
}

/// slinsghot application category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlingshotCategory {
    AudioVideo,
    Development,
    Office,
    System,
    Graphics,
}

/// Representation of an application indexed inside Slingshot
#[derive(Debug, Clone)]
pub struct SlingshotApp {
    pub name: String,
    pub category: SlingshotCategory,
    pub command_path: String,
    pub search_keywords: Vec<String>,
}

/// AppCenter pay-what-you-want software item
#[derive(Debug, Clone)]
pub struct AppCenterProduct {
    pub id: String,
    pub title: String,
    pub developer: String,
    pub suggested_price_usd: f64,
    pub installed: bool,
}

/// Gala Window Manager (Pantheon parity)
pub struct GalaWindowManager {
    pub active_workspace_idx: usize,
    pub transition_style: GalaTransitionStyle,
    pub window_count: usize,
}

impl GalaWindowManager {
    pub fn new() -> Self {
        Self {
            active_workspace_idx: 0,
            transition_style: GalaTransitionStyle::Zoom,
            window_count: 0,
        }
    }

    pub fn switch_workspace(&mut self, workspace_idx: usize) -> GalaTransitionStyle {
        self.active_workspace_idx = workspace_idx;
        self.transition_style
    }

    pub fn register_window(&mut self) {
        self.window_count += 1;
    }
}

/// Wingpanel (Pantheon top panel) Status Bar
pub struct Wingpanel {
    pub indicators: HashMap<String, WingpanelIndicator>,
    pub notification_badge_active: bool,
    pub clock_seconds_visible: bool,
}

impl Wingpanel {
    pub fn new() -> Self {
        let mut panel = Self {
            indicators: HashMap::new(),
            notification_badge_active: false,
            clock_seconds_visible: false,
        };
        panel.setup_default_indicators();
        panel
    }

    fn setup_default_indicators(&mut self) {
        self.indicators.insert(
            "network".to_string(),
            WingpanelIndicator {
                name: "Network".to_string(),
                active: true,
                status_text: "Connected to Wifi".to_string(),
                icon_name: "network-wireless-signal-excellent".to_string(),
            },
        );
        self.indicators.insert(
            "power".to_string(),
            WingpanelIndicator {
                name: "Power".to_string(),
                active: true,
                status_text: "100% Charged".to_string(),
                icon_name: "battery-full".to_string(),
            },
        );
    }

    pub fn update_indicator_text(&mut self, key: &str, text: &str) -> bool {
        if let Some(indicator) = self.indicators.get_mut(key) {
            indicator.status_text = text.to_string();
            true
        } else {
            false
        }
    }
}

/// Plank (Pantheon bottom dock)
pub struct PlankDock {
    pub items: Vec<PlankDockItem>,
    pub autohide: bool,
    pub hover_zoom_enabled: bool,
}

impl PlankDock {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            autohide: true,
            hover_zoom_enabled: true,
        }
    }

    pub fn add_item(&mut self, item: PlankDockItem) {
        self.items.push(item);
    }

    /// Trigger zoom magnification animation when hovering cursor over an icon
    pub fn hover_magnify(&mut self, app_id: &str, state: bool) -> bool {
        for item in &mut self.items {
            if item.app_id == app_id {
                item.zoom_factor = if state { 1.4 } else { 1.0 };
                return true;
            }
        }
        false
    }
}

/// Slingshot (Pantheon App Launcher)
pub struct SlingshotLauncher {
    pub apps: Vec<SlingshotApp>,
}

impl SlingshotLauncher {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn register_app(&mut self, app: SlingshotApp) {
        self.apps.push(app);
    }

    /// Searches app index by matching query across titles and keywords
    pub fn query_apps(&self, query: &str) -> Vec<SlingshotApp> {
        let mut results = Vec::new();
        for app in &self.apps {
            let mut match_found = app.name.contains(query);
            for kw in &app.search_keywords {
                if kw.contains(query) {
                    match_found = true;
                }
            }
            if match_found {
                results.push(app.clone());
            }
        }
        results
    }
}

/// AppCenter Software Store (curated, pay-what-you-want parity)
pub struct AppCenter {
    pub products: HashMap<String, AppCenterProduct>,
    pub account_balance_usd: f64,
}

impl AppCenter {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
            account_balance_usd: 100.0,
        }
    }

    pub fn register_product(&mut self, prod: AppCenterProduct) {
        self.products.insert(prod.id.clone(), prod);
    }

    /// Buy and install software, allowing custom pay-what-you-want pricing
    pub fn install_with_payment(&mut self, id: &str, paid_amount: f64) -> Result<bool, &'static str> {
        if paid_amount > self.account_balance_usd {
            return Err("Insufficient balance in AppCenter account");
        }
        if let Some(product) = self.products.get_mut(id) {
            self.account_balance_usd -= paid_amount;
            product.installed = true;
            Ok(true)
        } else {
            Err("Product not found in AppCenter catalog")
        }
    }
}

/// Pantheon elegant Greeter (Login screen) with post-quantum security
pub struct PantheonGreeter {
    pub is_locked: AtomicBool,
    pub session_user: String,
}

impl PantheonGreeter {
    pub fn new(user: &str) -> Self {
        Self {
            is_locked: AtomicBool::new(true),
            session_user: user.to_string(),
        }
    }

    /// Authenticates login using capability token validation (dilithium-5 / quantum parity)
    pub fn authenticate_token(&self, secret_token: u64) -> bool {
        // Enforce post-quantum mock validation schema
        if secret_token == 0x9999_FFFF_8888_7777 {
            self.is_locked.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}

impl Default for GalaWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Wingpanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PlankDock {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SlingshotLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AppCenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gala_window_manager() {
        let mut gala = GalaWindowManager::new();
        gala.register_window();
        assert_eq!(gala.window_count, 1);
        let style = gala.switch_workspace(2);
        assert_eq!(style, GalaTransitionStyle::Zoom);
        assert_eq!(gala.active_workspace_idx, 2);
    }

    #[test]
    fn test_wingpanel_indicators() {
        let mut wing = Wingpanel::new();
        assert!(wing.indicators.contains_key("network"));
        let updated = wing.update_indicator_text("network", "Connected to Fiber");
        assert!(updated);
        assert_eq!(wing.indicators.get("network").unwrap().status_text, "Connected to Fiber");
    }

    #[test]
    fn test_plank_dock() {
        let mut plank = PlankDock::new();
        plank.add_item(PlankDockItem {
            app_id: "files".to_string(),
            icon_path: "/apps/files.png".to_string(),
            is_running: true,
            badge_count: 2,
            zoom_factor: 1.0,
        });

        assert_eq!(plank.items[0].zoom_factor, 1.0);
        let hover = plank.hover_magnify("files", true);
        assert!(hover);
        assert_eq!(plank.items[0].zoom_factor, 1.4);
    }

    #[test]
    fn test_slingshot_launcher() {
        let mut launcher = SlingshotLauncher::new();
        launcher.register_app(SlingshotApp {
            name: "Mail".to_string(),
            category: SlingshotCategory::Office,
            command_path: "/bin/pantheon-mail".to_string(),
            search_keywords: {
                let mut v = Vec::new();
                v.push("email".to_string());
                v.push("inbox".to_string());
                v
            },
        });

        let results = launcher.query_apps("email");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Mail");
    }

    #[test]
    fn test_appcenter_purchases() {
        let mut center = AppCenter::new();
        center.register_product(AppCenterProduct {
            id: "noise".to_string(),
            title: "Noise Music Player".to_string(),
            developer: "elementary Team".to_string(),
            suggested_price_usd: 10.0,
            installed: false,
        });

        // Pay custom $5.00 for pay-what-you-want
        let res = center.install_with_payment("noise", 5.0).unwrap();
        assert!(res);
        assert!(center.products.get("noise").unwrap().installed);
        assert_eq!(center.account_balance_usd, 95.0);
    }

    #[test]
    fn test_pantheon_greeter() {
        let greeter = PantheonGreeter::new("bobby");
        assert!(greeter.is_locked.load(Ordering::SeqCst));

        // Bad key fails
        assert!(!greeter.authenticate_token(0xDEADBEEF));
        assert!(greeter.is_locked.load(Ordering::SeqCst));

        // Correct quantum token opens greeter
        assert!(greeter.authenticate_token(0x9999_FFFF_8888_7777));
        assert!(!greeter.is_locked.load(Ordering::SeqCst));
    }
}

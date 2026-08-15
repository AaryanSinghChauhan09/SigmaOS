// SPDX-License-Identifier: MIT
// SigmaOS elementaryOS Parity Subsystem: Granite UI, Switchboard, Contractor & ScreenTime
// Inspired by elementaryOS Granite toolkit, Switchboard Control Center, Contractor Service, and Parental Controls

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::vec::Vec;

// ============================================================================
// 1. Granite UI Toolkit & Toast Manager
// ============================================================================

/// elementaryOS Accent Color Palette
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccentColor {
    Blue = 0,
    Mint = 1,
    Grape = 2,
    Banana = 3,
    Orange = 4,
    Strawberry = 5,
    Slate = 6,
}

/// Granite Toast Notification Popup
#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub title: &'static str,
    pub message: &'static str,
    pub action_label: &'static str,
    pub timeout_ms: u32,
}

/// Granite UI Toolkit Engine
#[derive(Debug)]
pub struct GraniteUiToolkit {
    pub active_accent: AccentColor,
    toast_queue: Vec<ToastNotification>,
}

impl GraniteUiToolkit {
    pub fn new() -> Self {
        Self {
            active_accent: AccentColor::Blue,
            toast_queue: Vec::new(),
        }
    }

    pub fn set_accent_color(&mut self, accent: AccentColor) {
        self.active_accent = accent;
    }

    pub fn push_toast(&mut self, toast: ToastNotification) {
        self.toast_queue.push(toast);
    }

    pub fn pop_toast(&mut self) -> Option<ToastNotification> {
        if !self.toast_queue.is_empty() {
            Some(self.toast_queue.remove(0))
        } else {
            None
        }
    }

    pub fn get_toast_queue_len(&self) -> usize {
        self.toast_queue.len()
    }
}

impl Default for GraniteUiToolkit {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Switchboard Modular Settings Hub
// ============================================================================

/// Switchboard plug category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchboardCategory {
    Personal,
    Hardware,
    Network,
    System,
}

/// Modular Switchboard Plug
#[derive(Debug, Clone)]
pub struct SwitchboardPlug {
    pub name: &'static str,
    pub category: SwitchboardCategory,
    pub enabled: bool,
    pub icon_name: &'static str,
}

/// Switchboard Control Center Hub
#[derive(Debug)]
pub struct SwitchboardSettingsHub {
    plugs: Vec<SwitchboardPlug>,
}

impl SwitchboardSettingsHub {
    pub fn new() -> Self {
        let mut hub = Self { plugs: Vec::new() };

        hub.register_plug(SwitchboardPlug {
            name: "Display & Brightness",
            category: SwitchboardCategory::Hardware,
            enabled: true,
            icon_name: "preferences-desktop-display",
        });

        hub.register_plug(SwitchboardPlug {
            name: "Network & VPN",
            category: SwitchboardCategory::Network,
            enabled: true,
            icon_name: "preferences-system-network",
        });

        hub.register_plug(SwitchboardPlug {
            name: "Security & Privacy",
            category: SwitchboardCategory::System,
            enabled: true,
            icon_name: "preferences-system-privacy",
        });

        hub.register_plug(SwitchboardPlug {
            name: "Parental Controls & ScreenTime",
            category: SwitchboardCategory::Personal,
            enabled: true,
            icon_name: "preferences-system-parental-controls",
        });

        hub
    }

    pub fn register_plug(&mut self, plug: SwitchboardPlug) {
        self.plugs.push(plug);
    }

    pub fn query_plugs_by_category(&self, category: SwitchboardCategory) -> Vec<SwitchboardPlug> {
        self.plugs
            .iter()
            .filter(|p| p.category == category && p.enabled)
            .cloned()
            .collect()
    }

    pub fn set_plug_state(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(plug) = self.plugs.iter_mut().find(|p| p.name == name) {
            plug.enabled = enabled;
            true
        } else {
            false
        }
    }
}

impl Default for SwitchboardSettingsHub {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Contractor Action & Service Dispatcher
// ============================================================================

/// Action export registered in Contractor
#[derive(Debug, Clone)]
pub struct ContractorAction {
    pub name: &'static str,
    pub target_mime_type: &'static str,
    pub exec_command: &'static str,
    pub description: &'static str,
}

/// Contractor Action Service Engine
#[derive(Debug)]
pub struct ContractorService {
    actions: Vec<ContractorAction>,
}

impl ContractorService {
    pub fn new() -> Self {
        let mut service = Self {
            actions: Vec::new(),
        };

        // Standard Contractor actions
        service.register_action(ContractorAction {
            name: "Send via Email",
            target_mime_type: "application/octet-stream",
            exec_command: "pantheon-mail --attach %f",
            description: "Attach and send file via Pantheon Mail",
        });

        service.register_action(ContractorAction {
            name: "Print to PDF",
            target_mime_type: "text/plain",
            exec_command: "cups-pdf-convert %f",
            description: "Convert document directly into a PDF format",
        });

        service
    }

    pub fn register_action(&mut self, action: ContractorAction) {
        self.actions.push(action);
    }

    pub fn get_actions_for_mime(&self, mime_type: &str) -> Vec<ContractorAction> {
        self.actions
            .iter()
            .filter(|a| a.target_mime_type == "application/octet-stream" || a.target_mime_type == mime_type)
            .cloned()
            .collect()
    }

    pub fn execute_contract(&self, action_name: &str, target_file: &str) -> Result<&'static str, &'static str> {
        if target_file.is_empty() {
            return Err("Target file path is empty");
        }
        if self.actions.iter().any(|a| a.name == action_name) {
            Ok("Contractor Action Executed Successfully")
        } else {
            Err("Contractor Action Name Not Registered")
        }
    }
}

impl Default for ContractorService {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. ScreenTime & Parental Control Governor
// ============================================================================

/// User Time Limit Quota Config
#[derive(Debug, Clone)]
pub struct TimeQuota {
    pub daily_max_minutes: u32,
    pub consumed_minutes: u32,
    pub curfew_start_hour: u8, // 0..24
    pub curfew_end_hour: u8,   // 0..24
}

/// ScreenTime & Parental Controls Manager
#[derive(Debug)]
pub struct ScreenTimeParentalGovernor {
    pub quota: TimeQuota,
    restricted_apps: Vec<&'static str>,
}

impl ScreenTimeParentalGovernor {
    pub fn new(daily_max_minutes: u32, curfew_start_hour: u8, curfew_end_hour: u8) -> Self {
        Self {
            quota: TimeQuota {
                daily_max_minutes,
                consumed_minutes: 0,
                curfew_start_hour,
                curfew_end_hour,
            },
            restricted_apps: Vec::new(),
        }
    }

    pub fn add_restricted_app(&mut self, app_name: &'static str) {
        if !self.restricted_apps.contains(&app_name) {
            self.restricted_apps.push(app_name);
        }
    }

    pub fn record_usage(&mut self, minutes: u32) {
        self.quota.consumed_minutes = self.quota.consumed_minutes.saturating_add(minutes);
    }

    pub fn is_curfew_active(&self, current_hour: u8) -> bool {
        if self.quota.curfew_start_hour > self.quota.curfew_end_hour {
            // Overnight curfew e.g. 21:00 to 07:00
            current_hour >= self.quota.curfew_start_hour || current_hour < self.quota.curfew_end_hour
        } else {
            current_hour >= self.quota.curfew_start_hour && current_hour < self.quota.curfew_end_hour
        }
    }

    pub fn can_launch_app(&self, app_name: &str, current_hour: u8) -> Result<(), &'static str> {
        if self.is_curfew_active(current_hour) {
            return Err("Curfew Active: Device usage restricted during curfew hours");
        }

        if self.quota.consumed_minutes >= self.quota.daily_max_minutes {
            return Err("Time Limit Reached: Daily ScreenTime quota exceeded");
        }

        if self.restricted_apps.iter().any(|&a| a == app_name) {
            return Err("Parental Control: Application launching restricted by admin");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_granite_ui_toolkit() {
        let mut granite = GraniteUiToolkit::new();
        granite.set_accent_color(AccentColor::Strawberry);
        assert_eq!(granite.active_accent, AccentColor::Strawberry);

        granite.push_toast(ToastNotification {
            title: "Download Complete",
            message: "ISO image downloaded successfully",
            action_label: "Open Folder",
            timeout_ms: 3000,
        });

        assert_eq!(granite.get_toast_queue_len(), 1);
        let popped = granite.pop_toast().unwrap();
        assert_eq!(popped.title, "Download Complete");
        assert_eq!(granite.get_toast_queue_len(), 0);
    }

    #[test]
    fn test_switchboard_hub() {
        let mut hub = SwitchboardSettingsHub::new();

        let hardware_plugs = hub.query_plugs_by_category(SwitchboardCategory::Hardware);
        assert!(!hardware_plugs.is_empty());
        assert_eq!(hardware_plugs[0].name, "Display & Brightness");

        // Disable a plug
        assert!(hub.set_plug_state("Network & VPN", false));
        let net_plugs = hub.query_plugs_by_category(SwitchboardCategory::Network);
        assert!(net_plugs.is_empty());
    }

    #[test]
    fn test_contractor_service() {
        let service = ContractorService::new();

        let text_actions = service.get_actions_for_mime("text/plain");
        assert!(text_actions.len() >= 2);

        assert!(service.execute_contract("Print to PDF", "/home/user/doc.txt").is_ok());
        assert!(service.execute_contract("NonExistentAction", "/home/user/doc.txt").is_err());
    }

    #[test]
    fn test_screentime_parental_governor() {
        let mut governor = ScreenTimeParentalGovernor::new(120, 21, 7); // 2 hours max, curfew 21:00-07:00
        governor.add_restricted_app("Steam");

        // Normal hour 14:00 launch clean app
        assert!(governor.can_launch_app("Calculator", 14).is_ok());

        // Launch restricted app
        assert!(governor.can_launch_app("Steam", 14).is_err());

        // Curfew hour 22:00 launch clean app
        assert!(governor.can_launch_app("Calculator", 22).is_err());

        // Record 130 minutes usage (exceeds 120 quota)
        governor.record_usage(130);
        assert!(governor.can_launch_app("Calculator", 14).is_err());
    }
}

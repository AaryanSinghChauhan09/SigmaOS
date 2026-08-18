// SigmaOS Unified Control Center
// OOP-based centralized settings panel with modular architecture

use std::collections::BTreeMap;

/// Control panel type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ControlPanel {
    Network,
    Display,
    Sound,
    Bluetooth,
    Wifi,
    Power,
    Storage,
    Accessibility,
    Security,
    Accounts,
    Updates,
    About,
}

/// Panel state
#[derive(Debug, Clone)]
pub struct PanelState {
    pub panel: ControlPanel,
    pub is_active: bool,
    pub settings: BTreeMap<String, String>,
}

/// Quick setting
#[derive(Debug, Clone)]
pub struct QuickSetting {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub is_enabled: bool,
    pub action_type: QuickActionType,
}

/// Quick action type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickActionType {
    Toggle,
    Slider,
    Menu,
    Navigation,
}

/// Metric data
#[derive(Debug, Clone)]
pub struct MetricData {
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub timestamp: std::time::Instant,
}

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    CpuUsage,
    MemoryUsage,
    DiskUsage,
    NetworkUpload,
    NetworkDownload,
    Temperature,
    BatteryLevel,
}

/// Widget type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    Metric,
    Chart,
    Toggle,
    Slider,
    Status,
}

/// Dashboard widget
#[derive(Debug, Clone)]
pub struct DashboardWidget {
    pub id: String,
    pub widget_type: WidgetType,
    pub title: String,
    pub data: Option<MetricData>,
    pub position: (u32, u32),
    pub size: (u32, u32),
}

/// OOP trait for panel implementations
pub trait ControlPanelImpl {
    /// Initialize panel
    fn initialize(&mut self) -> Result<(), ControlCenterError>;
    /// Get panel state
    fn get_state(&self) -> PanelState;
    /// Update setting
    fn update_setting(&mut self, key: String, value: String) -> Result<(), ControlCenterError>;
    /// Get panel name
    fn name(&self) -> &str;
}

/// Network panel implementation
pub struct NetworkPanel {
    state: PanelState,
}

impl NetworkPanel {
    pub fn new() -> Self {
        Self {
            state: PanelState {
                panel: ControlPanel::Network,
                is_active: false,
                settings: {
                    let mut map = BTreeMap::new();
                    map.insert("wifi_enabled".to_string(), "true".to_string());
                    map.insert("airplane_mode".to_string(), "false".to_string());
                    map.insert("hotspot_enabled".to_string(), "false".to_string());
                    map
                },
            },
        }
    }
}

impl ControlPanelImpl for NetworkPanel {
    fn initialize(&mut self) -> Result<(), ControlCenterError> {
        self.state.is_active = true;
        Ok(())
    }

    fn get_state(&self) -> PanelState {
        self.state.clone()
    }

    fn update_setting(&mut self, key: String, value: String) -> Result<(), ControlCenterError> {
        self.state.settings.insert(key, value);
        Ok(())
    }

    fn name(&self) -> &str {
        "Network"
    }
}

/// Display panel implementation
pub struct DisplayPanel {
    state: PanelState,
}

impl DisplayPanel {
    pub fn new() -> Self {
        Self {
            state: PanelState {
                panel: ControlPanel::Display,
                is_active: false,
                settings: {
                    let mut map = BTreeMap::new();
                    map.insert("brightness".to_string(), "80".to_string());
                    map.insert("night_mode".to_string(), "false".to_string());
                    map.insert("resolution".to_string(), "1920x1080".to_string());
                    map
                },
            },
        }
    }
}

impl ControlPanelImpl for DisplayPanel {
    fn initialize(&mut self) -> Result<(), ControlCenterError> {
        self.state.is_active = true;
        Ok(())
    }

    fn get_state(&self) -> PanelState {
        self.state.clone()
    }

    fn update_setting(&mut self, key: String, value: String) -> Result<(), ControlCenterError> {
        self.state.settings.insert(key, value);
        Ok(())
    }

    fn name(&self) -> &str {
        "Display"
    }
}

/// Sound panel implementation
pub struct SoundPanel {
    state: PanelState,
}

impl SoundPanel {
    pub fn new() -> Self {
        Self {
            state: PanelState {
                panel: ControlPanel::Sound,
                is_active: false,
                settings: {
                    let mut map = BTreeMap::new();
                    map.insert("volume".to_string(), "75".to_string());
                    map.insert("mute".to_string(), "false".to_string());
                    map.insert("output_device".to_string(), "default".to_string());
                    map
                },
            },
        }
    }
}

impl ControlPanelImpl for SoundPanel {
    fn initialize(&mut self) -> Result<(), ControlCenterError> {
        self.state.is_active = true;
        Ok(())
    }

    fn get_state(&self) -> PanelState {
        self.state.clone()
    }

    fn update_setting(&mut self, key: String, value: String) -> Result<(), ControlCenterError> {
        self.state.settings.insert(key, value);
        Ok(())
    }

    fn name(&self) -> &str {
        "Sound"
    }
}

/// OOP-based Unified Control Center
pub struct UnifiedControlCenter {
    panels: BTreeMap<ControlPanel, Box<dyn ControlPanelImpl>>,
    quick_settings: Vec<QuickSetting>,
    widgets: Vec<DashboardWidget>,
    active_panel: Option<ControlPanel>,
    search_query: String,
    pub contrast_setting: f32,
    pub sound_volume_limit: u32,
}

impl UnifiedControlCenter {
    pub fn new() -> Self {
        Self {
            panels: BTreeMap::new(),
            quick_settings: Vec::new(),
            widgets: Vec::new(),
            active_panel: None,
            search_query: String::new(),
            contrast_setting: 1.0,
            sound_volume_limit: 100,
        }
    }

    pub fn set_contrast_setting(&mut self, contrast: f32) {
        self.contrast_setting = contrast;
    }

    pub fn apply_accessibility_overlay(
        &mut self,
        overlay: &super::accessibility_gamification::AccessibilityOverlay,
    ) {
        if overlay.high_contrast {
            self.contrast_setting = 2.0;
        } else {
            self.contrast_setting = 1.0;
        }
    }

    pub fn set_volume_limit(&mut self, limit: u32) {
        self.sound_volume_limit = limit;
    }

    /// Add a control panel
    pub fn add_panel(mut self, panel: ControlPanel, impl_obj: Box<dyn ControlPanelImpl>) -> Self {
        self.panels.insert(panel, impl_obj);
        self
    }

    /// Initialize all panels
    pub fn initialize_all(&mut self) -> Result<(), ControlCenterError> {
        for panel in self.panels.values_mut() {
            panel.initialize()?;
        }
        Ok(())
    }

    /// Activate a panel
    pub fn activate_panel(&mut self, panel: ControlPanel) -> Result<(), ControlCenterError> {
        if !self.panels.contains_key(&panel) {
            return Err(ControlCenterError::PanelNotFound);
        }

        self.active_panel = Some(panel);
        Ok(())
    }

    /// Get active panel
    pub fn active_panel(&self) -> Option<&ControlPanel> {
        self.active_panel.as_ref()
    }

    /// Get panel state
    pub fn get_panel_state(&self, panel: ControlPanel) -> Option<PanelState> {
        self.panels.get(&panel).map(|p| p.get_state())
    }

    /// Update panel setting
    pub fn update_panel_setting(
        &mut self,
        panel: ControlPanel,
        key: String,
        value: String,
    ) -> Result<(), ControlCenterError> {
        if let Some(panel_impl) = self.panels.get_mut(&panel) {
            panel_impl.update_setting(key, value)
        } else {
            Err(ControlCenterError::PanelNotFound)
        }
    }

    /// Add quick setting
    pub fn add_quick_setting(&mut self, setting: QuickSetting) {
        self.quick_settings.push(setting);
    }

    /// Get quick settings
    pub fn quick_settings(&self) -> &[QuickSetting] {
        &self.quick_settings
    }

    /// Toggle quick setting
    pub fn toggle_quick_setting(&mut self, id: &str) -> Result<(), ControlCenterError> {
        if let Some(setting) = self.quick_settings.iter_mut().find(|s| s.id == id) {
            setting.is_enabled = !setting.is_enabled;
            Ok(())
        } else {
            Err(ControlCenterError::SettingNotFound)
        }
    }

    /// Add widget
    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.push(widget);
    }

    /// Get widgets
    pub fn widgets(&self) -> &[DashboardWidget] {
        &self.widgets
    }

    /// Update widget data
    pub fn update_widget_data(&mut self, widget_id: &str, data: MetricData) {
        if let Some(widget) = self.widgets.iter_mut().find(|w| w.id == widget_id) {
            widget.data = Some(data);
        }
    }

    /// Search panels
    pub fn search(&mut self, query: String) -> Vec<ControlPanel> {
        self.search_query = query.clone();
        let query_lower = query.to_lowercase();

        self.panels
            .keys()
            .filter(|panel| {
                let panel_name = format!("{:?}", panel).to_lowercase();
                panel_name.contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Create default quick settings
    pub fn create_default_quick_settings(&mut self) {
        self.quick_settings = vec![
            QuickSetting {
                id: "wifi".to_string(),
                name: "Wi-Fi".to_string(),
                icon: "wifi".to_string(),
                is_enabled: true,
                action_type: QuickActionType::Toggle,
            },
            QuickSetting {
                id: "bluetooth".to_string(),
                name: "Bluetooth".to_string(),
                icon: "bluetooth".to_string(),
                is_enabled: false,
                action_type: QuickActionType::Toggle,
            },
            QuickSetting {
                id: "airplane".to_string(),
                name: "Airplane Mode".to_string(),
                icon: "airplane".to_string(),
                is_enabled: false,
                action_type: QuickActionType::Toggle,
            },
            QuickSetting {
                id: "brightness".to_string(),
                name: "Brightness".to_string(),
                icon: "brightness".to_string(),
                is_enabled: true,
                action_type: QuickActionType::Slider,
            },
            QuickSetting {
                id: "volume".to_string(),
                name: "Volume".to_string(),
                icon: "volume".to_string(),
                is_enabled: true,
                action_type: QuickActionType::Slider,
            },
        ];
    }

    /// Create default widgets
    pub fn create_default_widgets(&mut self) {
        self.widgets = vec![
            DashboardWidget {
                id: "cpu_usage".to_string(),
                widget_type: WidgetType::Metric,
                title: "CPU Usage".to_string(),
                data: Some(MetricData {
                    metric_type: MetricType::CpuUsage,
                    value: 45.0,
                    unit: "%".to_string(),
                    timestamp: std::time::Instant::now(),
                }),
                position: (0, 0),
                size: (2, 1),
            },
            DashboardWidget {
                id: "memory_usage".to_string(),
                widget_type: WidgetType::Metric,
                title: "Memory Usage".to_string(),
                data: Some(MetricData {
                    metric_type: MetricType::MemoryUsage,
                    value: 60.0,
                    unit: "%".to_string(),
                    timestamp: std::time::Instant::now(),
                }),
                position: (2, 0),
                size: (2, 1),
            },
            DashboardWidget {
                id: "network_status".to_string(),
                widget_type: WidgetType::Status,
                title: "Network".to_string(),
                data: None,
                position: (0, 1),
                size: (2, 1),
            },
        ];
    }
}

impl Default for UnifiedControlCenter {
    fn default() -> Self {
        let mut center = Self::new()
            .add_panel(ControlPanel::Network, Box::new(NetworkPanel::new()))
            .add_panel(ControlPanel::Display, Box::new(DisplayPanel::new()))
            .add_panel(ControlPanel::Sound, Box::new(SoundPanel::new()));

        center.create_default_quick_settings();
        center.create_default_widgets();

        center
    }
}

/// Control center errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlCenterError {
    PanelNotFound,
    SettingNotFound,
    InitializationError(String),
    UpdateError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_panel() {
        let panel = NetworkPanel::new();
        assert_eq!(panel.name(), "Network");
    }

    #[test]
    fn test_display_panel() {
        let panel = DisplayPanel::new();
        assert_eq!(panel.name(), "Display");
    }

    #[test]
    fn test_sound_panel() {
        let panel = SoundPanel::new();
        assert_eq!(panel.name(), "Sound");
    }

    #[test]
    fn test_unified_control_center() {
        let center = UnifiedControlCenter::default();
        assert_eq!(center.panels.len(), 3);
        assert_eq!(center.quick_settings.len(), 5);
        assert_eq!(center.widgets.len(), 3);
    }

    #[test]
    fn test_activate_panel() {
        let mut center = UnifiedControlCenter::default();
        center.initialize_all().unwrap();
        center.activate_panel(ControlPanel::Network).unwrap();
        assert_eq!(center.active_panel(), Some(&ControlPanel::Network));
    }

    #[test]
    fn test_toggle_quick_setting() {
        let mut center = UnifiedControlCenter::default();
        center.toggle_quick_setting("wifi").unwrap();
        let setting = center
            .quick_settings
            .iter()
            .find(|s| s.id == "wifi")
            .unwrap();
        assert!(!setting.is_enabled);
    }
}

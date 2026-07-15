// SigmaOS Cross-Device Orchestration
// IoT, smart home, and cloud integration baked into the OS

use std::collections::HashMap;

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Smartphone,
    Tablet,
    Desktop,
    Laptop,
    SmartHome,
    IoT,
    Wearable,
    Cloud,
}

/// Device connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

/// Device capability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCapability {
    FileTransfer,
    NotificationSync,
    AppControl,
    Automation,
    MediaControl,
    DataSync,
}

/// Connected device
#[derive(Debug, Clone)]
pub struct ConnectedDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub connection_status: ConnectionStatus,
    pub capabilities: Vec<DeviceCapability>,
    pub last_seen: u64,
    pub metadata: HashMap<String, String>,
}

impl ConnectedDevice {
    pub fn new(id: String, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            connection_status: ConnectionStatus::Disconnected,
            capabilities: Vec::new(),
            last_seen: 0,
            metadata: HashMap::new(),
        }
    }

    pub fn with_capability(mut self, capability: DeviceCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn has_capability(&self, capability: DeviceCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn is_connected(&self) -> bool {
        self.connection_status == ConnectionStatus::Connected
    }
}

/// Automation trigger
#[derive(Debug, Clone)]
pub enum AutomationTrigger {
    DeviceConnected { device_id: String },
    DeviceDisconnected { device_id: String },
    TimeBased { time: String },
    LocationBased { location: String },
    EventBased { event: String },
    Manual,
}

/// Cross-device action
#[derive(Debug, Clone)]
pub enum CrossDeviceAction {
    SendFile { device_id: String, file_path: String },
    SyncData { device_id: String, data_type: String },
    SendNotification { device_id: String, message: String },
    ControlApp { device_id: String, app: String, action: String },
    ExecuteAutomation { automation_id: String },
}

/// Automation rule
#[derive(Debug, Clone)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<CrossDeviceAction>,
    pub enabled: bool,
}

impl AutomationRule {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            triggers: Vec::new(),
            actions: Vec::new(),
            enabled: true,
        }
    }

    pub fn with_trigger(mut self, trigger: AutomationTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    pub fn with_action(mut self, action: CrossDeviceAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Smart home device
#[derive(Debug, Clone)]
pub struct SmartHomeDevice {
    pub base_device: ConnectedDevice,
    pub device_category: String, // "lighting", "climate", "security", etc.
    pub state: HashMap<String, String>,
}

impl SmartHomeDevice {
    pub fn new(id: String, name: String, device_category: String) -> Self {
        Self {
            base_device: ConnectedDevice::new(id, name, DeviceType::SmartHome),
            device_category,
            state: HashMap::new(),
        }
    }

    pub fn set_state(&mut self, key: String, value: String) {
        self.state.insert(key, value);
    }

    pub fn get_state(&self, key: &str) -> Option<&String> {
        self.state.get(key)
    }
}

/// Cross-device orchestration system
pub struct CrossDeviceOrchestrator {
    pub devices: HashMap<String, ConnectedDevice>,
    pub smart_home_devices: HashMap<String, SmartHomeDevice>,
    pub automation_rules: Vec<AutomationRule>,
    pub cloud_sync_enabled: bool,
    pub auto_discovery_enabled: bool,
}

impl CrossDeviceOrchestrator {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            smart_home_devices: HashMap::new(),
            automation_rules: Vec::new(),
            cloud_sync_enabled: true,
            auto_discovery_enabled: true,
        }
    }

    pub fn add_device(&mut self, device: ConnectedDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    pub fn remove_device(&mut self, id: &str) {
        self.devices.remove(id);
    }

    pub fn get_device(&self, id: &str) -> Option<&ConnectedDevice> {
        self.devices.get(id)
    }

    pub fn connect_device(&mut self, id: &str) -> Result<(), OrchestrationError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.connection_status = ConnectionStatus::Connected;
            device.last_seen = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            Ok(())
        } else {
            Err(OrchestrationError::DeviceNotFound)
        }
    }

    pub fn disconnect_device(&mut self, id: &str) -> Result<(), OrchestrationError> {
        if let Some(device) = self.devices.get_mut(id) {
            device.connection_status = ConnectionStatus::Disconnected;
            Ok(())
        } else {
            Err(OrchestrationError::DeviceNotFound)
        }
    }

    pub fn add_smart_home_device(&mut self, device: SmartHomeDevice) {
        let id = device.base_device.id.clone();
        self.devices.insert(id.clone(), device.base_device.clone());
        self.smart_home_devices.insert(id, device);
    }

    pub fn control_smart_home_device(&mut self, id: &str, state: HashMap<String, String>) -> Result<(), OrchestrationError> {
        if let Some(device) = self.smart_home_devices.get_mut(id) {
            device.state = state;
            Ok(())
        } else {
            Err(OrchestrationError::DeviceNotFound)
        }
    }

    pub fn add_automation_rule(&mut self, rule: AutomationRule) {
        self.automation_rules.push(rule);
    }

    pub fn remove_automation_rule(&mut self, id: &str) {
        self.automation_rules.retain(|r| r.id != id);
    }

    pub fn trigger_automation(&mut self, trigger: AutomationTrigger) -> Vec<CrossDeviceAction> {
        let mut triggered_actions = Vec::new();

        for rule in &self.automation_rules {
            if !rule.enabled {
                continue;
            }

            if rule.triggers.iter().any(|t| self.triggers_match(t, &trigger)) {
                triggered_actions.extend(rule.actions.clone());
            }
        }

        triggered_actions
    }

    fn triggers_match(&self, rule_trigger: &AutomationTrigger, event_trigger: &AutomationTrigger) -> bool {
        match (rule_trigger, event_trigger) {
            (AutomationTrigger::DeviceConnected { device_id: rule_id }, 
             AutomationTrigger::DeviceConnected { device_id: event_id }) => rule_id == event_id,
            (AutomationTrigger::DeviceDisconnected { device_id: rule_id }, 
             AutomationTrigger::DeviceDisconnected { device_id: event_id }) => rule_id == event_id,
            (AutomationTrigger::TimeBased { time: rule_time }, 
             AutomationTrigger::TimeBased { time: event_time }) => rule_time == event_time,
            (AutomationTrigger::Manual, AutomationTrigger::Manual) => true,
            _ => false,
        }
    }

    pub fn execute_action(&mut self, action: CrossDeviceAction) -> Result<(), OrchestrationError> {
        match action {
            CrossDeviceAction::SendFile { device_id, file_path } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!("Sending file {} to device {}", file_path, device_id);
            }
            CrossDeviceAction::SyncData { device_id, data_type } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!("Syncing {} data with device {}", data_type, device_id);
            }
            CrossDeviceAction::SendNotification { device_id, message } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!("Sending notification to device {}: {}", device_id, message);
            }
            CrossDeviceAction::ControlApp { device_id, app, action } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!("Controlling app {} on device {}: {}", app, device_id, action);
            }
            CrossDeviceAction::ExecuteAutomation { automation_id } => {
                println!("Executing automation: {}", automation_id);
            }
        }
        Ok(())
    }

    pub fn discover_devices(&mut self) -> Vec<ConnectedDevice> {
        // Simulate device discovery
        let discovered = vec![
            ConnectedDevice::new("phone_1".to_string(), "iPhone".to_string(), DeviceType::Smartphone)
                .with_capability(DeviceCapability::NotificationSync)
                .with_capability(DeviceCapability::AppControl),
            ConnectedDevice::new("tablet_1".to_string(), "iPad".to_string(), DeviceType::Tablet)
                .with_capability(DeviceCapability::FileTransfer)
                .with_capability(DeviceCapability::MediaControl),
        ];

        for device in discovered {
            self.add_device(device);
        }

        self.devices.values().cloned().collect()
    }

    pub fn get_connected_devices(&self) -> Vec<&ConnectedDevice> {
        self.devices.values()
            .filter(|d| d.is_connected())
            .collect()
    }

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<&ConnectedDevice> {
        self.devices.values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    pub fn enable_cloud_sync(&mut self) {
        self.cloud_sync_enabled = true;
    }

    pub fn disable_cloud_sync(&mut self) {
        self.cloud_sync_enabled = false;
    }

    pub fn enable_auto_discovery(&mut self) {
        self.auto_discovery_enabled = true;
    }

    pub fn disable_auto_discovery(&mut self) {
        self.auto_discovery_enabled = false;
    }
}

impl Default for CrossDeviceOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Orchestration errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrchestrationError {
    DeviceNotFound,
    ConnectionFailed,
    ActionFailed,
    InvalidDevice,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = CrossDeviceOrchestrator::new();
        assert!(orchestrator.cloud_sync_enabled);
        assert!(orchestrator.auto_discovery_enabled);
    }

    #[test]
    fn test_device_addition() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = ConnectedDevice::new("test".to_string(), "Test Device".to_string(), DeviceType::Smartphone);
        orchestrator.add_device(device);
        assert_eq!(orchestrator.devices.len(), 1);
    }

    #[test]
    fn test_device_connection() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = ConnectedDevice::new("test".to_string(), "Test Device".to_string(), DeviceType::Smartphone);
        orchestrator.add_device(device);
        assert!(orchestrator.connect_device("test").is_ok());
        assert!(orchestrator.get_device("test").unwrap().is_connected());
    }

    #[test]
    fn test_smart_home_device() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = SmartHomeDevice::new("light_1".to_string(), "Living Room Light".to_string(), "lighting".to_string());
        orchestrator.add_smart_home_device(device);
        assert_eq!(orchestrator.smart_home_devices.len(), 1);
    }

    #[test]
    fn test_automation_rule() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let rule = AutomationRule::new("test_rule".to_string(), "Test Rule".to_string())
            .with_trigger(AutomationTrigger::Manual)
            .with_action(CrossDeviceAction::SendNotification {
                device_id: "test".to_string(),
                message: "Test".to_string(),
            });
        orchestrator.add_automation_rule(rule);
        assert_eq!(orchestrator.automation_rules.len(), 1);
    }

    #[test]
    fn test_device_discovery() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let discovered = orchestrator.discover_devices();
        assert!(!discovered.is_empty());
        assert_eq!(orchestrator.devices.len(), 2);
    }
}

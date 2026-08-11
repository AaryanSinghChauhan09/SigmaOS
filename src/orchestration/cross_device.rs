#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Cross-Device Orchestration
// IoT, smart home, and cloud integration baked into the OS

use crate::klib::BTreeMap;

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
    pub metadata: BTreeMap<String, String>,
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
            metadata: BTreeMap::new(),
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
    SendFile {
        device_id: String,
        file_path: String,
    },
    SyncData {
        device_id: String,
        data_type: String,
    },
    SendNotification {
        device_id: String,
        message: String,
    },
    ControlApp {
        device_id: String,
        app: String,
        action: String,
    },
    ExecuteAutomation {
        automation_id: String,
    },
    SyncClipboard {
        device_id: String,
        clipboard_data: Vec<u8>,
    },
    CastMedia {
        device_id: String,
        media_url: String,
        play: bool,
    },
    RemoteSyscall {
        device_id: String,
        syscall_number: u32,
        arguments: Vec<u64>,
        capability_token: u64,
    },
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
    pub state: BTreeMap<String, String>,
}

impl SmartHomeDevice {
    pub fn new(id: String, name: String, device_category: String) -> Self {
        Self {
            base_device: ConnectedDevice::new(id, name, DeviceType::SmartHome),
            device_category,
            state: BTreeMap::new(),
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
    pub devices: BTreeMap<String, ConnectedDevice>,
    pub smart_home_devices: BTreeMap<String, SmartHomeDevice>,
    pub automation_rules: Vec<AutomationRule>,
    pub cloud_sync_enabled: bool,
    pub auto_discovery_enabled: bool,
}

impl CrossDeviceOrchestrator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            smart_home_devices: BTreeMap::new(),
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

    pub fn control_smart_home_device(
        &mut self,
        id: &str,
        state: BTreeMap<String, String>,
    ) -> Result<(), OrchestrationError> {
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

            if rule
                .triggers
                .iter()
                .any(|t| self.triggers_match(t, &trigger))
            {
                triggered_actions.extend(rule.actions.clone());
            }
        }

        triggered_actions
    }

    fn triggers_match(
        &self,
        rule_trigger: &AutomationTrigger,
        event_trigger: &AutomationTrigger,
    ) -> bool {
        match (rule_trigger, event_trigger) {
            (
                AutomationTrigger::DeviceConnected { device_id: rule_id },
                AutomationTrigger::DeviceConnected {
                    device_id: event_id,
                },
            ) => rule_id == event_id,
            (
                AutomationTrigger::DeviceDisconnected { device_id: rule_id },
                AutomationTrigger::DeviceDisconnected {
                    device_id: event_id,
                },
            ) => rule_id == event_id,
            (
                AutomationTrigger::TimeBased { time: rule_time },
                AutomationTrigger::TimeBased { time: event_time },
            ) => rule_time == event_time,
            (AutomationTrigger::Manual, AutomationTrigger::Manual) => true,
            _ => false,
        }
    }

    pub fn execute_action(&mut self, action: CrossDeviceAction) -> Result<(), OrchestrationError> {
        match action {
            CrossDeviceAction::SendFile {
                device_id,
                file_path,
            } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!("Sending file {} to device {}", file_path, device_id);
            }
            CrossDeviceAction::SyncData {
                device_id,
                data_type,
            } => {
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
            CrossDeviceAction::ControlApp {
                device_id,
                app,
                action,
            } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!(
                    "Controlling app {} on device {}: {}",
                    app, device_id, action
                );
            }
            CrossDeviceAction::ExecuteAutomation { automation_id } => {
                println!("Executing automation: {}", automation_id);
            }
            CrossDeviceAction::SyncClipboard {
                device_id,
                clipboard_data,
            } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!(
                    "Syncing clipboard (size={}) with device {}",
                    clipboard_data.len(),
                    device_id
                );
            }
            CrossDeviceAction::CastMedia {
                device_id,
                media_url,
                play,
            } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                println!(
                    "Casting media {} to device {} (play={})",
                    media_url, device_id, play
                );
            }
            CrossDeviceAction::RemoteSyscall {
                device_id,
                syscall_number,
                arguments,
                capability_token,
            } => {
                if !self.devices.contains_key(&device_id) {
                    return Err(OrchestrationError::DeviceNotFound);
                }
                if capability_token == 0 {
                    return Err(OrchestrationError::ActionFailed);
                }
                println!(
                    "Executing remote syscall {} on device {} with cap_token={}",
                    syscall_number, device_id, capability_token
                );
            }
        }
        Ok(())
    }

    pub fn discover_localsend_peers(&mut self) -> Vec<ConnectedDevice> {
        // Simulate LocalSend-compatible peer discovery
        let mut discovered = Vec::new();
        discovered.push(
            ConnectedDevice::new(
                "localsend_peer_1".to_string(),
                "LocalSend Desktop".to_string(),
                DeviceType::Desktop,
            )
            .with_capability(DeviceCapability::FileTransfer)
            .with_metadata("protocol_version".to_string(), "1.3".to_string()),
        );
        for d in &discovered {
            self.add_device(d.clone());
        }
        discovered
    }

    pub fn sync_secure_clipboard(
        &mut self,
        device_id: &str,
        data: &[u8],
    ) -> Result<(), OrchestrationError> {
        if !self.devices.contains_key(device_id) {
            return Err(OrchestrationError::DeviceNotFound);
        }
        // Encrypt with simple XOR for transport simulation
        let encrypted: Vec<u8> = data.iter().map(|b| b ^ 0x5A).collect();
        self.execute_action(CrossDeviceAction::SyncClipboard {
            device_id: device_id.to_string(),
            clipboard_data: encrypted,
        })
    }

    pub fn cast_media_stream(
        &mut self,
        device_id: &str,
        stream_url: &str,
    ) -> Result<(), OrchestrationError> {
        if !self.devices.contains_key(device_id) {
            return Err(OrchestrationError::DeviceNotFound);
        }
        self.execute_action(CrossDeviceAction::CastMedia {
            device_id: device_id.to_string(),
            media_url: stream_url.to_string(),
            play: true,
        })
    }

    pub fn execute_secure_rpc(
        &mut self,
        device_id: &str,
        syscall_num: u32,
        cap_token: u64,
    ) -> Result<(), OrchestrationError> {
        if !self.devices.contains_key(device_id) {
            return Err(OrchestrationError::DeviceNotFound);
        }
        self.execute_action(CrossDeviceAction::RemoteSyscall {
            device_id: device_id.to_string(),
            syscall_number: syscall_num,
            arguments: vec![0, 1],
            capability_token: cap_token,
        })
    }

    pub fn discover_devices(&mut self) -> Vec<ConnectedDevice> {
        // Simulate device discovery
        let discovered = vec![
            ConnectedDevice::new(
                "phone_1".to_string(),
                "iPhone".to_string(),
                DeviceType::Smartphone,
            )
            .with_capability(DeviceCapability::NotificationSync)
            .with_capability(DeviceCapability::AppControl),
            ConnectedDevice::new(
                "tablet_1".to_string(),
                "iPad".to_string(),
                DeviceType::Tablet,
            )
            .with_capability(DeviceCapability::FileTransfer)
            .with_capability(DeviceCapability::MediaControl),
        ];

        for device in discovered {
            self.add_device(device);
        }

        self.devices.values().cloned().collect()
    }

    pub fn get_connected_devices(&self) -> Vec<&ConnectedDevice> {
        let values_iter: crate::klib::hashmap::BTreeMapValues<'_, String, ConnectedDevice> = self.devices.values();
        values_iter.filter(|d| d.is_connected()).collect()
    }

    pub fn get_devices_by_type(&self, device_type: DeviceType) -> Vec<&ConnectedDevice> {
        let values_iter: crate::klib::hashmap::BTreeMapValues<'_, String, ConnectedDevice> = self.devices.values();
        values_iter
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

/// LocalSendShard - Encrypted local P2P file and message transfer module (replaces LocalSend)
pub struct LocalSendShard {
    pub local_ip: String,
    pub active_transfers: BTreeMap<String, usize>, // maps file_id -> transfer percentage
    pub encryption_key: Vec<u8>,
}

impl LocalSendShard {
    pub fn new(ip: &str, key: Vec<u8>) -> Self {
        Self {
            local_ip: ip.to_string(),
            active_transfers: BTreeMap::new(),
            encryption_key: key,
        }
    }

    /// Prepares and encrypts a payload stream for local P2P dispatch
    pub fn prepare_p2p_payload(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::with_capacity(data.len());
        if self.encryption_key.is_empty() {
            encrypted.extend_from_slice(data);
        } else {
            for (i, &byte) in data.iter().enumerate() {
                encrypted.push(byte ^ self.encryption_key[i % self.encryption_key.len()]);
            }
        }
        encrypted
    }

    /// Updates local progress for active local file streams
    pub fn update_transfer_progress(&mut self, file_id: &str, percentage: usize) {
        self.active_transfers
            .insert(file_id.to_string(), percentage.min(100));
    }
}

/// KDEConnectShard - Multi-device synchronization hub (replaces KDE Connect)
/// Handles remote input mirroring, notification forwarding, shared clipboard sync,
/// and cross-device media control.
pub struct KdeConnectShard {
    pub paired_devices: Vec<String>,
    pub notifications_buffer: Vec<String>,
    pub synced_clipboard_content: String,
    pub remote_volume: u32,
}

impl KdeConnectShard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            paired_devices: Vec::new(),
            notifications_buffer: Vec::new(),
            synced_clipboard_content: String::new(),
            remote_volume: 75,
        }
    }

    /// Pair with a new remote smartphone, tablet, or desktop device
    pub fn pair_device(&mut self, device_id: &str) {
        if !self.paired_devices.contains(&device_id.to_string()) {
            self.paired_devices.push(device_id.to_string());
        }
    }

    /// Broadcast a notification packet to all paired companion devices
    pub fn broadcast_notification(&mut self, sender: &str, message: &str) {
        let entry = format!("{}: {}", sender, message);
        self.notifications_buffer.push(entry);
    }

    /// Syncs local clipboard changes to the remote device
    pub fn sync_clipboard(&mut self, content: &str) {
        self.synced_clipboard_content = content.to_string();
    }

    /// Adjusts media parameters dynamically on target systems
    pub fn adjust_remote_media_volume(&mut self, new_volume: u32) {
        self.remote_volume = new_volume.min(100);
    }
}

impl Default for KdeConnectShard {
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
    fn test_kdeconnect_shard() {
        let mut kde = KdeConnectShard::new();
        kde.pair_device("android_smartphone_1");
        assert_eq!(kde.paired_devices.len(), 1);

        kde.broadcast_notification("System", "Battery Low: 15%");
        assert_eq!(kde.notifications_buffer.len(), 1);
        assert_eq!(kde.notifications_buffer[0], "System: Battery Low: 15%");

        kde.sync_clipboard("copied URL or password link");
        assert_eq!(kde.synced_clipboard_content, "copied URL or password link");

        kde.adjust_remote_media_volume(90);
        assert_eq!(kde.remote_volume, 90);
    }

    #[test]
    fn test_localsend_transfer() {
        let mut local_send = LocalSendShard::new("192.168.1.50", vec![9, 8, 7]);
        let raw_data = b"sovereign cross device file transfer";
        let encrypted = local_send.prepare_p2p_payload(raw_data);
        assert_eq!(encrypted.len(), raw_data.len());

        local_send.update_transfer_progress("photo_1.png", 45);
        assert_eq!(local_send.active_transfers.get("photo_1.png"), Some(&45));
    }

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = CrossDeviceOrchestrator::new();
        assert!(orchestrator.cloud_sync_enabled);
        assert!(orchestrator.auto_discovery_enabled);
    }

    #[test]
    fn test_device_addition() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = ConnectedDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            DeviceType::Smartphone,
        );
        orchestrator.add_device(device);
        assert_eq!(orchestrator.devices.len(), 1);
    }

    #[test]
    fn test_device_connection() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = ConnectedDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            DeviceType::Smartphone,
        );
        orchestrator.add_device(device);
        assert!(orchestrator.connect_device("test").is_ok());
        assert!(orchestrator.get_device("test").unwrap().is_connected());
    }

    #[test]
    fn test_smart_home_device() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = SmartHomeDevice::new(
            "light_1".to_string(),
            "Living Room Light".to_string(),
            "lighting".to_string(),
        );
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

    #[test]
    fn test_enhanced_cross_device_features() {
        let mut orchestrator = CrossDeviceOrchestrator::new();
        let device = ConnectedDevice::new(
            "device_id_123".to_string(),
            "Parity Machine".to_string(),
            DeviceType::Laptop,
        );
        orchestrator.add_device(device);

        // 1. LocalSend-compatible peer discovery
        let localsend_peers = orchestrator.discover_localsend_peers();
        assert_eq!(localsend_peers.len(), 1);
        assert_eq!(localsend_peers[0].name, "LocalSend Desktop");

        // 2. Clipboard sharing
        let clip_res = orchestrator.sync_secure_clipboard("device_id_123", b"SovereignClipboard");
        assert!(clip_res.is_ok());

        // 3. Media casting
        let cast_res =
            orchestrator.cast_media_stream("device_id_123", "http://sigmaos.local/stream.mp4");
        assert!(cast_res.is_ok());

        // 4. Secure RPC System Calls
        let rpc_res = orchestrator.execute_secure_rpc("device_id_123", 42, 0xCAFEBABE);
        assert!(rpc_res.is_ok());

        // Failed RPC on invalid token
        let bad_rpc = orchestrator.execute_secure_rpc("device_id_123", 42, 0);
        assert!(bad_rpc.is_err());
    }
}

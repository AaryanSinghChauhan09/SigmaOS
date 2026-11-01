//! Network settings panel for SigmaOS-settings.

use crate::{SettingsError, SettingsPanel, SettingsStore};

/// A scanned Wi-Fi network.
#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal_strength: u8, // 0-100
    pub secured: bool,
}

/// Connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

impl Default for ConnectionState { fn default() -> Self { Self::Disconnected } }

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Failed => "failed",
        };
        write!(f, "{}", s)
    }
}

/// Network settings panel (Wi-Fi + Ethernet).
#[derive(Debug, Default)]
pub struct NetworkPanel {
    pub wifi_enabled: bool,
    pub ethernet_enabled: bool,
    pub wifi_state: ConnectionState,
    pub connected_ssid: Option<String>,
    pub available_networks: Vec<WifiNetwork>,
}

impl NetworkPanel {
    /// Simulate a network scan.
    pub fn scan(&mut self) {
        self.available_networks = vec![
            WifiNetwork { ssid: "SigmaOS-5G".into(), signal_strength: 90, secured: true },
            WifiNetwork { ssid: "OpenNet".into(), signal_strength: 65, secured: false },
        ];
    }

    /// Simulate connecting to a network.
    pub fn connect(&mut self, ssid: &str) -> Result<(), SettingsError> {
        let found = self.available_networks.iter().any(|n| n.ssid == ssid);
        if !found {
            return Err(SettingsError::ValidationError(format!("SSID '{}' not found", ssid)));
        }
        self.wifi_state = ConnectionState::Connected;
        self.connected_ssid = Some(ssid.to_string());
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.wifi_state = ConnectionState::Disconnected;
        self.connected_ssid = None;
    }
}

impl SettingsPanel for NetworkPanel {
    fn id(&self) -> &str { "network" }
    fn title(&self) -> &str { "Network" }

    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError> {
        store.set("network.wifi_enabled", self.wifi_enabled.to_string());
        store.set("network.ethernet_enabled", self.ethernet_enabled.to_string());
        store.set("network.wifi_state", self.wifi_state.to_string());
        if let Some(ssid) = &self.connected_ssid {
            store.set("network.connected_ssid", ssid.clone());
        }
        Ok(())
    }
}

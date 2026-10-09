//! Network Manager
//!
//! Network management system inspired by Linux Mint's NetworkManager and Omarchy's
//! network utilities, supporting WiFi, Ethernet, VPN, and network configuration.

use std::collections::HashMap;

/// Connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    WiFi,
    VPN,
    Bluetooth,
    Cellular,
}

impl ConnectionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ethernet" | "wired" => Some(ConnectionType::Ethernet),
            "wifi" | "wireless" => Some(ConnectionType::WiFi),
            "vpn" => Some(ConnectionType::VPN),
            "bluetooth" => Some(ConnectionType::Bluetooth),
            "cellular" | "mobile" => Some(ConnectionType::Cellular),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConnectionType::Ethernet => "Ethernet",
            ConnectionType::WiFi => "WiFi",
            ConnectionType::VPN => "VPN",
            ConnectionType::Bluetooth => "Bluetooth",
            ConnectionType::Cellular => "Cellular",
        }
    }
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

impl ConnectionStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "disconnected" => Some(ConnectionStatus::Disconnected),
            "connecting" => Some(ConnectionStatus::Connecting),
            "connected" => Some(ConnectionStatus::Connected),
            "failed" => Some(ConnectionStatus::Failed),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConnectionStatus::Disconnected => "Disconnected",
            ConnectionStatus::Connecting => "Connecting",
            ConnectionStatus::Connected => "Connected",
            ConnectionStatus::Failed => "Failed",
        }
    }
}

/// WiFi security type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WiFiSecurity {
    Open,
    WEP,
    WPA_PSK,
    WPA2_PSK,
    WPA3_PSK,
    WPA_EAP,
}

impl WiFiSecurity {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "open" => Some(WiFiSecurity::Open),
            "wep" => Some(WiFiSecurity::WEP),
            "wpa-psk" | "wpa" => Some(WiFiSecurity::WPA_PSK),
            "wpa2-psk" | "wpa2" => Some(WiFiSecurity::WPA2_PSK),
            "wpa3-psk" | "wpa3" => Some(WiFiSecurity::WPA3_PSK),
            "wpa-eap" | "enterprise" => Some(WiFiSecurity::WPA_EAP),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            WiFiSecurity::Open => "Open",
            WiFiSecurity::WEP => "WEP",
            WiFiSecurity::WPA_PSK => "WPA-PSK",
            WiFiSecurity::WPA2_PSK => "WPA2-PSK",
            WiFiSecurity::WPA3_PSK => "WPA3-PSK",
            WiFiSecurity::WPA_EAP => "WPA-EAP",
        }
    }
}

/// Network connection
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub id: String,
    pub name: String,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
    pub interface: String,
    pub ip_address: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub is_auto_connect: bool,
}

impl NetworkConnection {
    pub fn new(
        id: String,
        name: String,
        connection_type: ConnectionType,
        interface: String,
    ) -> Self {
        Self {
            id,
            name,
            connection_type,
            status: ConnectionStatus::Disconnected,
            interface,
            ip_address: None,
            gateway: None,
            dns_servers: Vec::new(),
            is_auto_connect: false,
        }
    }

    pub fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }

    pub fn set_ip(&mut self, ip: String) {
        self.ip_address = Some(ip);
    }

    pub fn set_gateway(&mut self, gateway: String) {
        self.gateway = Some(gateway);
    }

    pub fn add_dns(&mut self, dns: String) {
        self.dns_servers.push(dns);
    }

    pub fn set_auto_connect(&mut self, auto: bool) {
        self.is_auto_connect = auto;
    }

    pub fn is_connected(&self) -> bool {
        self.status == ConnectionStatus::Connected
    }
}

/// WiFi network
#[derive(Debug, Clone)]
pub struct WiFiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub security: WiFiSecurity,
    pub signal_strength: i32,
    pub frequency: u32,
    pub is_hidden: bool,
}

impl WiFiNetwork {
    pub fn new(ssid: String, bssid: String, security: WiFiSecurity) -> Self {
        Self {
            ssid,
            bssid,
            security,
            signal_strength: 0,
            frequency: 2400,
            is_hidden: false,
        }
    }

    pub fn set_signal(&mut self, strength: i32) {
        self.signal_strength = strength.clamp(0, 100);
    }

    pub fn set_frequency(&mut self, freq: u32) {
        self.frequency = freq;
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.is_hidden = hidden;
    }

    pub fn is_5ghz(&self) -> bool {
        self.frequency >= 5000
    }
}

/// Network manager configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub auto_connect: bool,
    pub metered_connections: bool,
    pub ipv6_enabled: bool,
    pub firewall_enabled: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            auto_connect: true,
            metered_connections: false,
            ipv6_enabled: true,
            firewall_enabled: true,
        }
    }
}

/// Network manager
#[derive(Debug)]
pub struct NetworkManager {
    connections: HashMap<String, NetworkConnection>,
    wifi_networks: Vec<WiFiNetwork>,
    config: NetworkConfig,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            wifi_networks: Vec::new(),
            config: NetworkConfig::default(),
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &NetworkConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: NetworkConfig) {
        self.config = config;
    }

    /// Add a network connection
    pub fn add_connection(&mut self, connection: NetworkConnection) {
        self.connections.insert(connection.id.clone(), connection);
    }

    /// Get a connection
    pub fn get_connection(&self, id: &str) -> Option<&NetworkConnection> {
        self.connections.get(id)
    }

    /// Get a connection mutably
    pub fn get_connection_mut(&mut self, id: &str) -> Option<&mut NetworkConnection> {
        self.connections.get_mut(id)
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&NetworkConnection> {
        self.connections.values().collect()
    }

    /// List connections by type
    pub fn list_by_type(&self, conn_type: ConnectionType) -> Vec<&NetworkConnection> {
        self.connections
            .values()
            .filter(|c| c.connection_type == conn_type)
            .collect()
    }

    /// List connected connections
    pub fn list_connected(&self) -> Vec<&NetworkConnection> {
        self.connections
            .values()
            .filter(|c| c.is_connected())
            .collect()
    }

    /// Connect to a network
    pub fn connect(&mut self, id: &str) -> Result<(), String> {
        let connection = self
            .get_connection_mut(id)
            .ok_or_else(|| format!("Connection {} not found", id))?;

        if connection.is_connected() {
            return Err(format!("Connection {} is already connected", id));
        }

        connection.set_status(ConnectionStatus::Connecting);

        // Simulate connection
        connection.set_status(ConnectionStatus::Connected);
        connection.set_ip("192.168.1.100".to_string());
        connection.set_gateway("192.168.1.1".to_string());
        connection.add_dns("8.8.8.8".to_string());
        connection.add_dns("8.8.4.4".to_string());

        Ok(())
    }

    /// Disconnect from a network
    pub fn disconnect(&mut self, id: &str) -> Result<(), String> {
        let connection = self
            .get_connection_mut(id)
            .ok_or_else(|| format!("Connection {} not found", id))?;

        if !connection.is_connected() {
            return Err(format!("Connection {} is not connected", id));
        }

        connection.set_status(ConnectionStatus::Disconnected);
        connection.ip_address = None;
        connection.gateway = None;
        connection.dns_servers.clear();

        Ok(())
    }

    /// Scan for WiFi networks
    pub fn scan_wifi(&mut self) -> Vec<&WiFiNetwork> {
        self.wifi_networks.clear();

        // Simulate WiFi scan
        let networks = vec![
            (
                "HomeNetwork",
                "00:11:22:33:44:55",
                WiFiSecurity::WPA2_PSK,
                80,
                2412,
            ),
            (
                "GuestNetwork",
                "00:11:22:33:44:56",
                WiFiSecurity::WPA2_PSK,
                60,
                2412,
            ),
            (
                "FreeWiFi",
                "00:11:22:33:44:57",
                WiFiSecurity::Open,
                40,
                2437,
            ),
            (
                "5GHz-Network",
                "00:11:22:33:44:58",
                WiFiSecurity::WPA3_PSK,
                70,
                5180,
            ),
        ];

        for (ssid, bssid, security, signal, freq) in networks {
            let mut wifi = WiFiNetwork::new(ssid.to_string(), bssid.to_string(), security);
            wifi.set_signal(signal);
            wifi.set_frequency(freq);
            self.wifi_networks.push(wifi);
        }

        self.wifi_networks.iter().collect()
    }

    /// Get scanned WiFi networks
    pub fn get_wifi_networks(&self) -> Vec<&WiFiNetwork> {
        self.wifi_networks.iter().collect()
    }

    /// Connect to WiFi network
    pub fn connect_wifi(&mut self, ssid: &str, password: Option<String>) -> Result<String, String> {
        let wifi = self
            .wifi_networks
            .iter()
            .find(|w| w.ssid == ssid)
            .ok_or_else(|| format!("WiFi network {} not found", ssid))?;

        let connection_id = format!("wifi-{}", ssid.to_lowercase().replace(' ', "-"));
        let mut connection = NetworkConnection::new(
            connection_id.clone(),
            ssid.to_string(),
            ConnectionType::WiFi,
            "wlan0".to_string(),
        );

        if wifi.security != WiFiSecurity::Open && password.is_none() {
            return Err(format!("Password required for {}", ssid));
        }

        self.add_connection(connection);
        self.connect(&connection_id)?;

        Ok(connection_id)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> NetworkStatistics {
        let total_connections = self.connections.len();
        let connected_count = self
            .connections
            .values()
            .filter(|c| c.is_connected())
            .count();
        let wifi_count = self.list_by_type(ConnectionType::WiFi).len();
        let ethernet_count = self.list_by_type(ConnectionType::Ethernet).len();

        NetworkStatistics {
            total_connections,
            connected_count,
            wifi_count,
            ethernet_count,
            vpn_count: self.list_by_type(ConnectionType::VPN).len(),
            wifi_networks_found: self.wifi_networks.len(),
        }
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub total_connections: usize,
    pub connected_count: usize,
    pub wifi_count: usize,
    pub ethernet_count: usize,
    pub vpn_count: usize,
    pub wifi_networks_found: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_type_from_str() {
        assert_eq!(ConnectionType::from_str("wifi"), Some(ConnectionType::WiFi));
        assert_eq!(
            ConnectionType::from_str("ethernet"),
            Some(ConnectionType::Ethernet)
        );
    }

    #[test]
    fn test_connection_status_from_str() {
        assert_eq!(
            ConnectionStatus::from_str("connected"),
            Some(ConnectionStatus::Connected)
        );
        assert_eq!(
            ConnectionStatus::from_str("disconnected"),
            Some(ConnectionStatus::Disconnected)
        );
    }

    #[test]
    fn test_wifi_security_from_str() {
        assert_eq!(WiFiSecurity::from_str("wpa2"), Some(WiFiSecurity::WPA2_PSK));
        assert_eq!(WiFiSecurity::from_str("open"), Some(WiFiSecurity::Open));
    }

    #[test]
    fn test_network_connection_creation() {
        let conn = NetworkConnection::new(
            "test".to_string(),
            "Test Connection".to_string(),
            ConnectionType::WiFi,
            "wlan0".to_string(),
        );
        assert_eq!(conn.name, "Test Connection");
    }

    #[test]
    fn test_network_manager_creation() {
        let manager = NetworkManager::new();
        assert_eq!(manager.list_connections().len(), 0);
    }

    #[test]
    fn test_add_connection() {
        let mut manager = NetworkManager::new();
        let conn = NetworkConnection::new(
            "test".to_string(),
            "Test".to_string(),
            ConnectionType::Ethernet,
            "eth0".to_string(),
        );
        manager.add_connection(conn);
        assert_eq!(manager.list_connections().len(), 1);
    }

    #[test]
    fn test_connect() {
        let mut manager = NetworkManager::new();
        let conn = NetworkConnection::new(
            "test".to_string(),
            "Test".to_string(),
            ConnectionType::Ethernet,
            "eth0".to_string(),
        );
        manager.add_connection(conn);
        assert!(manager.connect("test").is_ok());
        assert!(manager.get_connection("test").unwrap().is_connected());
    }

    #[test]
    fn test_disconnect() {
        let mut manager = NetworkManager::new();
        let mut conn = NetworkConnection::new(
            "test".to_string(),
            "Test".to_string(),
            ConnectionType::Ethernet,
            "eth0".to_string(),
        );
        conn.set_status(ConnectionStatus::Connected);
        manager.add_connection(conn);
        assert!(manager.disconnect("test").is_ok());
    }

    #[test]
    fn test_scan_wifi() {
        let mut manager = NetworkManager::new();
        let networks = manager.scan_wifi();
        assert!(networks.len() > 0);
    }

    #[test]
    fn test_connect_wifi() {
        let mut manager = NetworkManager::new();
        manager.scan_wifi();
        assert!(manager
            .connect_wifi("HomeNetwork", Some("password".to_string()))
            .is_ok());
    }

    #[test]
    fn test_statistics() {
        let mut manager = NetworkManager::new();
        manager.scan_wifi();
        let stats = manager.get_statistics();
        assert!(stats.wifi_networks_found > 0);
    }
}

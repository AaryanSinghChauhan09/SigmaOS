// Desktop Network Manager
// Linux Mint & Omarchy inspiration for comprehensive network management

use std::collections::HashMap;

/// Net Connection Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetConnectionType {
    Ethernet,
    WiFi,
    VPN,
    Bluetooth,
    Other,
}

impl NetConnectionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetConnectionType::Ethernet => "ethernet",
            NetConnectionType::WiFi => "wifi",
            NetConnectionType::VPN => "vpn",
            NetConnectionType::Bluetooth => "bluetooth",
            NetConnectionType::Other => "other",
        }
    }
}

/// Net Connection Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Failed,
}

impl NetConnectionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetConnectionStatus::Disconnected => "disconnected",
            NetConnectionStatus::Connecting => "connecting",
            NetConnectionStatus::Connected => "connected",
            NetConnectionStatus::Failed => "failed",
        }
    }
}

/// Security Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    None,
    WEP,
    WPA_PSK,
    WPA_EAP,
    WPA2_PSK,
    WPA2_EAP,
    WPA3_PSK,
    WPA3_EAP,
}

impl SecurityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecurityType::None => "none",
            SecurityType::WEP => "wep",
            SecurityType::WPA_PSK => "wpa-psk",
            SecurityType::WPA_EAP => "wpa-eap",
            SecurityType::WPA2_PSK => "wpa2-psk",
            SecurityType::WPA2_EAP => "wpa2-eap",
            SecurityType::WPA3_PSK => "wpa3-psk",
            SecurityType::WPA3_EAP => "wpa3-eap",
        }
    }
}

/// Network Connection
#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub id: String,
    pub name: String,
    pub connection_type: NetConnectionType,
    pub status: NetConnectionStatus,
    pub interface: String,
    pub ip_address: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub security: Option<SecurityType>,
    pub auto_connect: bool,
}

impl NetworkConnection {
    pub fn new(id: String, name: String, connection_type: NetConnectionType, interface: String) -> Self {
        Self {
            id,
            name,
            connection_type,
            status: NetConnectionStatus::Disconnected,
            interface,
            ip_address: None,
            gateway: None,
            dns_servers: Vec::new(),
            security: None,
            auto_connect: false,
        }
    }

    pub fn with_ip_address(mut self, ip: String) -> Self {
        self.ip_address = Some(ip);
        self
    }

    pub fn with_gateway(mut self, gateway: String) -> Self {
        self.gateway = Some(gateway);
        self
    }

    pub fn with_dns(mut self, dns: Vec<String>) -> Self {
        self.dns_servers = dns;
        self
    }

    pub fn with_security(mut self, security: SecurityType) -> Self {
        self.security = Some(security);
        self
    }

    pub fn set_status(&mut self, status: NetConnectionStatus) {
        self.status = status;
    }

    pub fn set_auto_connect(&mut self, auto_connect: bool) {
        self.auto_connect = auto_connect;
    }
}

/// Desktop Network Manager
pub struct DesktopNetworkManager {
    connections: HashMap<String, NetworkConnection>,
    active_connection: Option<String>,
    counter: u32,
}

impl DesktopNetworkManager {
    pub fn new() -> Self {
        let mut manager = Self {
            connections: HashMap::new(),
            active_connection: None,
            counter: 0,
        };

        // Add default connections
        manager.add_default_connections();

        manager
    }

    fn add_default_connections(&mut self) {
        let eth0 = NetworkConnection::new(
            "conn_0".to_string(),
            "Wired Connection 1".to_string(),
            NetConnectionType::Ethernet,
            "eth0".to_string(),
        );

        let wlan0 = NetworkConnection::new(
            "conn_1".to_string(),
            "WiFi Connection".to_string(),
            NetConnectionType::WiFi,
            "wlan0".to_string(),
        )
        .with_security(SecurityType::WPA2_PSK);

        self.connections.insert(eth0.id.clone(), eth0);
        self.connections.insert(wlan0.id.clone(), wlan0);
    }

    pub fn add_connection(&mut self, connection: NetworkConnection) -> String {
        let id = format!("conn_{}", self.counter);
        self.counter += 1;

        let connection = NetworkConnection {
            id: id.clone(),
            ..connection
        };

        self.connections.insert(id.clone(), connection);
        id
    }

    pub fn remove_connection(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.active_connection {
            self.active_connection = None;
        }
        self.connections.remove(id).is_some()
    }

    pub fn get_connection(&self, id: &str) -> Option<&NetworkConnection> {
        self.connections.get(id)
    }

    pub fn get_connections(&self) -> Vec<&NetworkConnection> {
        self.connections.values().collect()
    }

    pub fn get_connections_by_type(&self, connection_type: NetConnectionType) -> Vec<&NetworkConnection> {
        self.connections
            .values()
            .filter(|c| c.connection_type == connection_type)
            .collect()
    }

    pub fn get_connections_by_status(&self, status: NetConnectionStatus) -> Vec<&NetworkConnection> {
        self.connections
            .values()
            .filter(|c| c.status == status)
            .collect()
    }

    pub fn set_connection_status(&mut self, id: &str, status: NetConnectionStatus) -> bool {
        if let Some(connection) = self.connections.get_mut(id) {
            connection.set_status(status);
            true
        } else {
            false
        }
    }

    pub fn connect(&mut self, id: &str) -> bool {
        if self.connections.contains_key(id) {
            // Disconnect previous active connection
            if let Some(prev_id) = self.active_connection.clone() {
                self.set_connection_status(&prev_id, NetConnectionStatus::Disconnected);
            }

            // Connect new connection
            self.set_connection_status(id, NetConnectionStatus::Connected);
            self.active_connection = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn disconnect(&mut self, id: &str) -> bool {
        if self.connections.contains_key(id) {
            self.set_connection_status(id, NetConnectionStatus::Disconnected);
            if Some(id.to_string()) == self.active_connection {
                self.active_connection = None;
            }
            true
        } else {
            false
        }
    }

    pub fn get_active_connection(&self) -> Option<&NetworkConnection> {
        self.active_connection
            .as_ref()
            .and_then(|id| self.connections.get(id))
    }

    pub fn set_auto_connect(&mut self, id: &str, auto_connect: bool) -> bool {
        if let Some(connection) = self.connections.get_mut(id) {
            connection.set_auto_connect(auto_connect);
            true
        } else {
            false
        }
    }

    pub fn update_ip_address(&mut self, id: &str, ip: String) -> bool {
        if let Some(connection) = self.connections.get_mut(id) {
            connection.ip_address = Some(ip);
            true
        } else {
            false
        }
    }

    pub fn update_gateway(&mut self, id: &str, gateway: String) -> bool {
        if let Some(connection) = self.connections.get_mut(id) {
            connection.gateway = Some(gateway);
            true
        } else {
            false
        }
    }

    pub fn update_dns_servers(&mut self, id: &str, dns: Vec<String>) -> bool {
        if let Some(connection) = self.connections.get_mut(id) {
            connection.dns_servers = dns;
            true
        } else {
            false
        }
    }

    pub fn get_statistics(&self) -> NetworkManagerStatistics {
        NetworkManagerStatistics {
            total_connections: self.connections.len(),
            connected: self.connections.values().filter(|c| c.status == NetConnectionStatus::Connected).count(),
            ethernet_connections: self.get_connections_by_type(NetConnectionType::Ethernet).len(),
            wifi_connections: self.get_connections_by_type(NetConnectionType::WiFi).len(),
            vpn_connections: self.get_connections_by_type(NetConnectionType::VPN).len(),
            active_connection_set: self.active_connection.is_some(),
        }
    }
}

impl Default for DesktopNetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// NetworkManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct NetworkManagerStatistics {
    pub total_connections: usize,
    pub connected: usize,
    pub ethernet_connections: usize,
    pub wifi_connections: usize,
    pub vpn_connections: usize,
    pub active_connection_set: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopNetworkManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_connections >= 2);
        assert!(stats.ethernet_connections >= 1);
        assert!(stats.wifi_connections >= 1);
    }

    #[test]
    fn test_add_connection() {
        let mut manager = DesktopNetworkManager::new();
        let initial_count = manager.get_connections().len();

        let conn = NetworkConnection::new(
            "custom".to_string(),
            "Custom Connection".to_string(),
            NetConnectionType::WiFi,
            "wlan1".to_string(),
        );

        let id = manager.add_connection(conn);
        assert!(manager.get_connection(&id).is_some());
        assert_eq!(manager.get_connections().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_connection() {
        let mut manager = DesktopNetworkManager::new();

        let conn = NetworkConnection::new(
            "custom".to_string(),
            "Custom Connection".to_string(),
            NetConnectionType::WiFi,
            "wlan1".to_string(),
        );

        let id = manager.add_connection(conn);
        assert!(manager.remove_connection(&id));
        assert!(manager.get_connection(&id).is_none());
    }

    #[test]
    fn test_set_connection_status() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        assert!(manager.set_connection_status(conn_id, NetConnectionStatus::Connected));
        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.status, NetConnectionStatus::Connected);
    }

    #[test]
    fn test_connect() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        assert!(manager.connect(conn_id));
        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.status, NetConnectionStatus::Connected);

        let active = manager.get_active_connection().unwrap();
        assert_eq!(active.id, conn_id);
    }

    #[test]
    fn test_disconnect() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        manager.connect(conn_id);
        assert!(manager.disconnect(conn_id));

        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.status, NetConnectionStatus::Disconnected);
    }

    #[test]
    fn test_set_auto_connect() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        assert!(manager.set_auto_connect(conn_id, true));
        let conn = manager.get_connection(conn_id).unwrap();
        assert!(conn.auto_connect);
    }

    #[test]
    fn test_update_ip_address() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        assert!(manager.update_ip_address(conn_id, "192.168.1.100".to_string()));
        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.ip_address, Some("192.168.1.100".to_string()));
    }

    #[test]
    fn test_update_gateway() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        assert!(manager.update_gateway(conn_id, "192.168.1.1".to_string()));
        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.gateway, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_update_dns_servers() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        let dns = vec![
            "8.8.8.8".to_string(),
            "8.8.4.4".to_string(),
        ];

        assert!(manager.update_dns_servers(conn_id, dns.clone()));
        let conn = manager.get_connection(conn_id).unwrap();
        assert_eq!(conn.dns_servers, dns);
    }

    #[test]
    fn test_get_connections_by_type() {
        let manager = DesktopNetworkManager::new();

        let eth = manager.get_connections_by_type(NetConnectionType::Ethernet);
        let wifi = manager.get_connections_by_type(NetConnectionType::WiFi);

        assert!(!eth.is_empty());
        assert!(!wifi.is_empty());

        for conn in eth {
            assert_eq!(conn.connection_type, NetConnectionType::Ethernet);
        }

        for conn in wifi {
            assert_eq!(conn.connection_type, NetConnectionType::WiFi);
        }
    }

    #[test]
    fn test_get_connections_by_status() {
        let mut manager = DesktopNetworkManager::new();
        let conn_id = "conn_0";

        manager.connect(conn_id);

        let connected = manager.get_connections_by_status(NetConnectionStatus::Connected);
        assert!(!connected.is_empty());

        let disconnected = manager.get_connections_by_status(NetConnectionStatus::Disconnected);
        assert!(!disconnected.is_empty());
    }

    #[test]
    fn test_connection_with_security() {
        let mut manager = DesktopNetworkManager::new();

        let conn = NetworkConnection::new(
            "custom".to_string(),
            "Secure WiFi".to_string(),
            NetConnectionType::WiFi,
            "wlan1".to_string(),
        )
        .with_security(SecurityType::WPA3_PSK);

        let id = manager.add_connection(conn);
        let retrieved = manager.get_connection(&id).unwrap();
        assert_eq!(retrieved.security, Some(SecurityType::WPA3_PSK));
    }
}

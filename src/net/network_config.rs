// Network Configuration Manager for SigmaOS
// Implements network interface configuration with DHCP and static IP support
// Inspired by Linux NetworkManager, systemd-networkd, and BSD network configuration

#[cfg(not(test))]
use crate::klib::HashMap;
#[cfg(test)]
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::string::{String, ToString};
use std::vec::Vec;

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Wired,
    Wireless,
    Loopback,
    Virtual,
}

/// Network configuration method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigMethod {
    Dhcp,
    Static,
}

/// Network interface configuration
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub name: String,
    pub if_type: InterfaceType,
    pub method: ConfigMethod,
    pub address: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
    pub dns_servers: Vec<Ipv4Addr>,
    pub enabled: bool,
}

impl InterfaceConfig {
    pub fn new(name: String, if_type: InterfaceType) -> Self {
        InterfaceConfig {
            name,
            if_type,
            method: ConfigMethod::Dhcp,
            address: None,
            netmask: None,
            gateway: None,
            dns_servers: Vec::new(),
            enabled: false,
        }
    }

    pub fn set_static_ip(&mut self, address: Ipv4Addr, netmask: Ipv4Addr, gateway: Ipv4Addr) {
        self.method = ConfigMethod::Static;
        self.address = Some(address);
        self.netmask = Some(netmask);
        self.gateway = Some(gateway);
    }

    pub fn add_dns_server(&mut self, dns: Ipv4Addr) {
        self.dns_servers.push(dns);
    }

    pub fn set_dhcp(&mut self) {
        self.method = ConfigMethod::Dhcp;
        self.address = None;
        self.netmask = None;
        self.gateway = None;
    }
}

/// Network configuration manager
pub struct NetworkConfigManager {
    pub interfaces: HashMap<String, InterfaceConfig>,
}

impl NetworkConfigManager {
    pub fn new() -> Self {
        let mut manager = NetworkConfigManager {
            interfaces: HashMap::new(),
        };
        manager.initialize_default_interfaces();
        manager
    }

    fn initialize_default_interfaces(&mut self) {
        // Initialize loopback interface
        let lo = InterfaceConfig::new(String::from("lo"), InterfaceType::Loopback);
        self.interfaces.insert(String::from("lo"), lo);

        // Initialize potential wired interface
        let eth0 = InterfaceConfig::new(String::from("eth0"), InterfaceType::Wired);
        self.interfaces.insert(String::from("eth0"), eth0);
    }

    pub fn add_interface(&mut self, config: InterfaceConfig) {
        self.interfaces.insert(config.name.clone(), config);
    }

    pub fn get_interface(&self, name: &str) -> Option<&InterfaceConfig> {
        self.interfaces.get(name)
    }

    pub fn get_interface_mut(&mut self, name: &str) -> Option<&mut InterfaceConfig> {
        self.interfaces.get_mut(name)
    }

    pub fn list_interfaces(&self) -> Vec<String> {
        self.interfaces.keys().cloned().collect()
    }

    pub fn enable_interface(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_interface(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn configure_dhcp(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.set_dhcp();
            true
        } else {
            false
        }
    }

    pub fn configure_static(
        &mut self,
        name: &str,
        address: Ipv4Addr,
        netmask: Ipv4Addr,
        gateway: Ipv4Addr,
    ) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.set_static_ip(address, netmask, gateway);
            true
        } else {
            false
        }
    }

    pub fn add_dns(&mut self, name: &str, dns: Ipv4Addr) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.add_dns_server(dns);
            true
        } else {
            false
        }
    }
}

impl Default for NetworkConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_manager_creation() {
        let manager = NetworkConfigManager::new();
        assert!(manager.get_interface("lo").is_some());
        assert!(manager.get_interface("eth0").is_some());
    }

    #[test]
    fn test_add_interface() {
        let mut manager = NetworkConfigManager::new();
        let wlan0 = InterfaceConfig::new(String::from("wlan0"), InterfaceType::Wireless);
        manager.add_interface(wlan0);
        assert!(manager.get_interface("wlan0").is_some());
    }

    #[test]
    fn test_configure_static_ip() {
        let mut manager = NetworkConfigManager::new();
        let success = manager.configure_static(
            "eth0",
            Ipv4Addr::new(192, 168, 1, 100),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 1),
        );
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert_eq!(iface.method, ConfigMethod::Static);
        assert_eq!(iface.address, Some(Ipv4Addr::new(192, 168, 1, 100)));
        assert_eq!(iface.gateway, Some(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn test_configure_dhcp() {
        let mut manager = NetworkConfigManager::new();
        manager.configure_static(
            "eth0",
            Ipv4Addr::new(192, 168, 1, 100),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 168, 1, 1),
        );

        let success = manager.configure_dhcp("eth0");
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert_eq!(iface.method, ConfigMethod::Dhcp);
        assert!(iface.address.is_none());
    }

    #[test]
    fn test_add_dns() {
        let mut manager = NetworkConfigManager::new();
        let success = manager.add_dns("eth0", Ipv4Addr::new(8, 8, 8, 8));
        assert!(success);

        let iface = manager.get_interface("eth0").unwrap();
        assert!(iface.dns_servers.contains(&Ipv4Addr::new(8, 8, 8, 8)));
    }

    #[test]
    fn test_enable_disable_interface() {
        let mut manager = NetworkConfigManager::new();
        assert!(!manager.get_interface("eth0").unwrap().enabled);

        manager.enable_interface("eth0");
        assert!(manager.get_interface("eth0").unwrap().enabled);

        manager.disable_interface("eth0");
        assert!(!manager.get_interface("eth0").unwrap().enabled);
    }

    #[test]
    fn test_list_interfaces() {
        let manager = NetworkConfigManager::new();
        let interfaces = manager.list_interfaces();
        assert!(interfaces.contains(&String::from("lo")));
        assert!(interfaces.contains(&String::from("eth0")));
    }
}

// ============================================================================
// Arch Linux-Inspired Network Manager Extensions
// Inspired by: iwd, netctl, systemd-networkd, wpa_supplicant
// ============================================================================

/// Wireless security protocol (iwd/wpa_supplicant inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WirelessSecurity {
    Open,
    Wep(String),
    WpaPsk(String),
    Wpa2Psk(String),
    Wpa3Sae(String),
    Enterprise { identity: String, password: String },
}

/// Wireless network profile (iwd-style)
#[derive(Debug, Clone)]
pub struct WirelessProfile {
    pub ssid: String,
    pub security: WirelessSecurity,
    pub auto_connect: bool,
    pub hidden: bool,
    pub priority: i32,
}

impl WirelessProfile {
    pub fn new_open(ssid: &str) -> Self {
        Self {
            ssid: ssid.to_string(),
            security: WirelessSecurity::Open,
            auto_connect: true,
            hidden: false,
            priority: 0,
        }
    }

    pub fn new_wpa2(ssid: &str, passphrase: &str) -> Self {
        Self {
            ssid: ssid.to_string(),
            security: WirelessSecurity::Wpa2Psk(passphrase.to_string()),
            auto_connect: true,
            hidden: false,
            priority: 0,
        }
    }

    pub fn new_wpa3(ssid: &str, passphrase: &str) -> Self {
        Self {
            ssid: ssid.to_string(),
            security: WirelessSecurity::Wpa3Sae(passphrase.to_string()),
            auto_connect: true,
            hidden: false,
            priority: 10,
        }
    }
}

/// iwd (iNet wireless daemon) inspired wireless manager
#[derive(Debug, Clone)]
pub struct IwdManager {
    pub profiles: Vec<WirelessProfile>,
    pub scan_results: Vec<WirelessNetwork>,
    pub connected: Option<String>,
}

/// Scanned wireless network
#[derive(Debug, Clone)]
pub struct WirelessNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal_dbm: i32,
    pub frequency_mhz: u32,
    pub security: Vec<String>,
    pub connected: bool,
}

impl IwdManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            scan_results: Vec::new(),
            connected: None,
        }
    }

    pub fn add_profile(&mut self, profile: WirelessProfile) {
        self.profiles.push(profile);
    }

    pub fn remove_profile(&mut self, ssid: &str) {
        self.profiles.retain(|p| p.ssid != ssid);
    }

    pub fn get_profile(&self, ssid: &str) -> Option<&WirelessProfile> {
        self.profiles.iter().find(|p| p.ssid == ssid)
    }

    pub fn connect(&mut self, ssid: &str) -> Result<(), NetworkConfigError> {
        if self.get_profile(ssid).is_some() {
            self.connected = Some(ssid.to_string());
            Ok(())
        } else {
            Err(NetworkConfigError::ProfileNotFound(ssid.to_string()))
        }
    }

    pub fn disconnect(&mut self) {
        self.connected = None;
    }

    pub fn scan(&mut self) -> &[WirelessNetwork] {
        &self.scan_results
    }

    pub fn list_profiles(&self) -> &[WirelessProfile] {
        &self.profiles
    }
}

/// Network configuration error
#[derive(Debug, Clone)]
pub enum NetworkConfigError {
    ProfileNotFound(String),
    AlreadyConnected(String),
    AuthFailed(String),
    InterfaceNotFound(String),
    InvalidConfig(String),
}

/// netctl profile type (Arch Linux netctl inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetctlConnectionType {
    Ethernet,
    Wireless,
    Bond,
    Bridge,
    Vlan,
    Tunnel,
}

/// netctl network profile (Arch Linux)
#[derive(Debug, Clone)]
pub struct NetctlProfile {
    pub name: String,
    pub interface: String,
    pub connection: NetctlConnectionType,
    pub ip: ConfigMethod,
    pub address: Option<String>,
    pub gateway: Option<String>,
    pub dns: Vec<String>,
    pub essid: Option<String>,
    pub security: Option<String>,
    pub key: Option<String>,
}

impl NetctlProfile {
    pub fn new_ethernet_dhcp(name: &str, interface: &str) -> Self {
        Self {
            name: name.to_string(),
            interface: interface.to_string(),
            connection: NetctlConnectionType::Ethernet,
            ip: ConfigMethod::Dhcp,
            address: None,
            gateway: None,
            dns: Vec::new(),
            essid: None,
            security: None,
            key: None,
        }
    }

    pub fn new_wireless_wpa2(name: &str, interface: &str, essid: &str, key: &str) -> Self {
        Self {
            name: name.to_string(),
            interface: interface.to_string(),
            connection: NetctlConnectionType::Wireless,
            ip: ConfigMethod::Dhcp,
            address: None,
            gateway: None,
            dns: Vec::new(),
            essid: Some(essid.to_string()),
            security: Some("wpa".to_string()),
            key: Some(key.to_string()),
        }
    }

    /// Generate netctl profile file content
    pub fn to_profile_content(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Description='SigmaOS netctl profile: {}'", self.name));
        lines.push(format!("Interface={}", self.interface));
        lines.push(format!("Connection={}", format!("{:?}", self.connection).to_lowercase()));
        lines.push(format!("IP={}", format!("{:?}", self.ip).to_lowercase()));
        if let Some(ref addr) = self.address {
            lines.push(format!("Address=('{}')", addr));
        }
        if let Some(ref gw) = self.gateway {
            lines.push(format!("Gateway='{}'", gw));
        }
        if !self.dns.is_empty() {
            lines.push(format!("DNS=({})", self.dns.iter().map(|d| format!("'{}'", d)).collect::<Vec<_>>().join(" ")));
        }
        if let Some(ref essid) = self.essid {
            lines.push(format!("ESSID='{}'", essid));
        }
        if let Some(ref sec) = self.security {
            lines.push(format!("Security={}", sec));
        }
        if let Some(ref key) = self.key {
            lines.push(format!("Key='{}'", key));
        }
        lines.join("\n")
    }
}

/// netctl manager — manages network profiles like Arch Linux's netctl
#[derive(Debug, Clone)]
pub struct NetctlManager {
    pub profiles: Vec<NetctlProfile>,
    pub active: Option<String>,
}

impl NetctlManager {
    pub fn new() -> Self {
        Self { profiles: Vec::new(), active: None }
    }

    pub fn add(&mut self, profile: NetctlProfile) { self.profiles.push(profile); }

    pub fn start(&mut self, name: &str) -> Result<(), NetworkConfigError> {
        if self.profiles.iter().any(|p| p.name == name) {
            self.active = Some(name.to_string());
            Ok(())
        } else {
            Err(NetworkConfigError::ProfileNotFound(name.to_string()))
        }
    }

    pub fn stop(&mut self) { self.active = None; }

    pub fn restart(&mut self) -> Result<(), NetworkConfigError> {
        if let Some(name) = self.active.clone() {
            self.stop();
            self.start(&name)
        } else {
            Err(NetworkConfigError::InvalidConfig("No active profile".to_string()))
        }
    }

    pub fn list(&self) -> Vec<&str> {
        self.profiles.iter().map(|p| p.name.as_str()).collect()
    }

    pub fn enable(&self, name: &str) -> Result<(), NetworkConfigError> {
        if self.profiles.iter().any(|p| p.name == name) {
            Ok(())  // Would create systemd service symlink in real impl
        } else {
            Err(NetworkConfigError::ProfileNotFound(name.to_string()))
        }
    }
}

#[cfg(test)]
mod arch_net_tests {
    use super::*;

    #[test]
    fn test_wireless_profile_wpa2() {
        // Arrange
        let ssid = "HomeNetwork";
        let pass = "securepassword";
        // Act
        let profile = WirelessProfile::new_wpa2(ssid, pass);
        // Assert
        assert_eq!(profile.ssid, ssid);
        assert_eq!(profile.security, WirelessSecurity::Wpa2Psk(pass.to_string()));
        assert!(profile.auto_connect);
    }

    #[test]
    fn test_iwd_connect_known_profile() {
        // Arrange
        let mut iwd = IwdManager::new();
        iwd.add_profile(WirelessProfile::new_wpa3("SecureNet", "hunter2"));
        // Act
        let result = iwd.connect("SecureNet");
        // Assert
        assert!(result.is_ok());
        assert_eq!(iwd.connected, Some("SecureNet".to_string()));
    }

    #[test]
    fn test_iwd_connect_unknown_profile_fails() {
        // Arrange
        let mut iwd = IwdManager::new();
        // Act
        let result = iwd.connect("UnknownSSID");
        // Assert
        assert!(result.is_err());
        assert!(iwd.connected.is_none());
    }

    #[test]
    fn test_netctl_profile_content_generation() {
        // Arrange
        let profile = NetctlProfile::new_ethernet_dhcp("eth-home", "eth0");
        // Act
        let content = profile.to_profile_content();
        // Assert
        assert!(content.contains("Interface=eth0"));
        assert!(content.contains("IP=dhcp") || content.contains("IP=Dhcp"));
    }

    #[test]
    fn test_netctl_start_stop() {
        // Arrange
        let mut mgr = NetctlManager::new();
        mgr.add(NetctlProfile::new_ethernet_dhcp("wired", "eth0"));
        // Act
        let start = mgr.start("wired");
        let active = mgr.active.clone();
        mgr.stop();
        // Assert
        assert!(start.is_ok());
        assert_eq!(active, Some("wired".to_string()));
        assert!(mgr.active.is_none());
    }

    #[test]
    fn test_netctl_start_nonexistent_fails() {
        // Arrange
        let mut mgr = NetctlManager::new();
        // Act
        let result = mgr.start("ghost");
        // Assert
        assert!(result.is_err());
    }
}

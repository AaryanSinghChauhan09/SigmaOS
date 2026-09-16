// SigmaOS Desktop Network Panel GUI Engine
// Zero-dependency #![no_std] network panel UI backend, interface status, Wi-Fi config, and VPN management

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkConnectionType {
    Wifi,
    Ethernet,
    Vpn,
    Cellular,
    Loopback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInterfaceStatus {
    pub name: String,
    pub mac_address: String,
    pub ip_address: String,
    pub conn_type: NetworkConnectionType,
    pub status: NetworkConnectionStatus,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpnProfile {
    pub id: String,
    pub name: String,
    pub server_address: String,
    pub protocol: String,
    pub is_active: bool,
}

pub struct NetworkPanelGuiEngine {
    interfaces: Vec<NetworkInterfaceStatus>,
    vpn_profiles: Vec<VpnProfile>,
}

impl NetworkPanelGuiEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            interfaces: Vec::new(),
            vpn_profiles: Vec::new(),
        };
        engine.init_default_hardware();
        engine
    }

    fn init_default_hardware(&mut self) {
        self.interfaces.push(NetworkInterfaceStatus {
            name: String::from("wlan0"),
            mac_address: String::from("AA:BB:CC:11:22:33"),
            ip_address: String::from("192.168.1.50"),
            conn_type: NetworkConnectionType::Wifi,
            status: NetworkConnectionStatus::Connected,
            rx_bytes: 1024 * 1024 * 45,
            tx_bytes: 1024 * 1024 * 12,
        });

        self.vpn_profiles.push(VpnProfile {
            id: String::from("vpn_wireguard_main"),
            name: String::from("Sovereign WireGuard Munich"),
            server_address: String::from("vpn.sigmaos.org:51820"),
            protocol: String::from("WireGuard"),
            is_active: false,
        });
    }

    pub fn list_interfaces(&self) -> &[NetworkInterfaceStatus] {
        &self.interfaces
    }

    pub fn connect_wifi(&mut self, interface_name: &str, _ssid: &str, _passphrase: &str) -> bool {
        if let Some(iface) = self.interfaces.iter_mut().find(|i| i.name == interface_name && i.conn_type == NetworkConnectionType::Wifi) {
            iface.status = NetworkConnectionStatus::Connected;
            true
        } else {
            false
        }
    }

    pub fn disconnect_interface(&mut self, interface_name: &str) -> bool {
        if let Some(iface) = self.interfaces.iter_mut().find(|i| i.name == interface_name) {
            iface.status = NetworkConnectionStatus::Disconnected;
            true
        } else {
            false
        }
    }

    pub fn toggle_vpn(&mut self, vpn_id: &str) -> Result<bool, &'static str> {
        let vpn = self
            .vpn_profiles
            .iter_mut()
            .find(|v| v.id == vpn_id)
            .ok_or("VPN profile not found")?;

        vpn.is_active = !vpn.is_active;
        Ok(vpn.is_active)
    }

    pub fn get_vpn_profiles(&self) -> &[VpnProfile] {
        &self.vpn_profiles
    }

    pub fn record_traffic(&mut self, interface_name: &str, rx_delta: u64, tx_delta: u64) {
        if let Some(iface) = self.interfaces.iter_mut().find(|i| i.name == interface_name) {
            iface.rx_bytes += rx_delta;
            iface.tx_bytes += tx_delta;
        }
    }
}

impl Default for NetworkPanelGuiEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_panel_gui_engine() {
        let mut engine = NetworkPanelGuiEngine::new();
        assert_eq!(engine.list_interfaces().len(), 1);

        assert!(engine.connect_wifi("wlan0", "Home_5G", "secure_pass_123"));
        assert_eq!(engine.list_interfaces()[0].status, NetworkConnectionStatus::Connected);

        let vpn_res = engine.toggle_vpn("vpn_wireguard_main");
        assert!(vpn_res.is_ok());
        assert!(vpn_res.unwrap());
    }
}

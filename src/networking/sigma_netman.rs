extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileType {
    Ethernet,
    WiFi,
    VPN,
}

#[derive(Debug, Clone)]
pub struct NetworkProfile {
    pub name: String,
    pub profile_type: ProfileType,
    pub auto_connect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DhcpState {
    Init,
    Discover,
    Request,
    Bound,
}

pub struct NetworkInterface {
    pub name: String,
    pub is_up: bool,
    pub ip_address: Option<String>,
}

pub struct NetworkManager {
    pub profiles: Vec<NetworkProfile>,
    pub interfaces: Vec<NetworkInterface>,
    pub dhcp_state: DhcpState,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            interfaces: Vec::new(),
            dhcp_state: DhcpState::Init,
        }
    }

    pub fn add_profile(&mut self, profile: NetworkProfile) {
        self.profiles.push(profile);
    }

    pub fn add_interface(&mut self, interface: NetworkInterface) {
        self.interfaces.push(interface);
    }

    pub fn bring_up(&mut self, iface_name: &str) -> Result<(), &'static str> {
        if let Some(iface) = self.interfaces.iter_mut().find(|i| i.name == iface_name) {
            iface.is_up = true;
            Ok(())
        } else {
            Err("Interface not found")
        }
    }

    pub fn bring_down(&mut self, iface_name: &str) -> Result<(), &'static str> {
        if let Some(iface) = self.interfaces.iter_mut().find(|i| i.name == iface_name) {
            iface.is_up = false;
            Ok(())
        } else {
            Err("Interface not found")
        }
    }

    pub fn run_dhcp(&mut self) {
        self.dhcp_state = DhcpState::Discover;
        self.dhcp_state = DhcpState::Request;
        self.dhcp_state = DhcpState::Bound;
    }

    pub fn wifi_scan(&self) -> Vec<String> {
        let mut ve = Vec::new();
        ve.push("SigmaOS_Guest".into());
        ve.push("CoffeeShop".into());
        ve
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager() {
        let nm = NetworkManager::new();
        assert_eq!(nm.profiles.len(), 0);
        assert_eq!(nm.dhcp_state, DhcpState::Init);
    }

    #[test]
    fn test_add_profile() {
        let mut nm = NetworkManager::new();
        nm.add_profile(NetworkProfile {
            name: "Home_WiFi".into(),
            profile_type: ProfileType::WiFi,
            auto_connect: true,
        });
        assert_eq!(nm.profiles.len(), 1);
        assert_eq!(nm.profiles[0].profile_type, ProfileType::WiFi);
    }

    #[test]
    fn test_interface_up_down() {
        let mut nm = NetworkManager::new();
        nm.add_interface(NetworkInterface {
            name: "eth0".into(),
            is_up: false,
            ip_address: None,
        });
        assert!(nm.bring_up("eth0").is_ok());
        assert_eq!(nm.interfaces[0].is_up, true);
        assert!(nm.bring_down("eth0").is_ok());
        assert_eq!(nm.interfaces[0].is_up, false);
    }

    #[test]
    fn test_interface_not_found() {
        let mut nm = NetworkManager::new();
        let res = nm.bring_up("wlan0");
        assert!(res.is_err());
    }

    #[test]
    fn test_dhcp_flow() {
        let mut nm = NetworkManager::new();
        nm.run_dhcp();
        assert_eq!(nm.dhcp_state, DhcpState::Bound);
    }

    #[test]
    fn test_wifi_scan() {
        let nm = NetworkManager::new();
        let networks = nm.wifi_scan();
        assert!(networks.contains(&"SigmaOS_Guest".into()));
    }
}

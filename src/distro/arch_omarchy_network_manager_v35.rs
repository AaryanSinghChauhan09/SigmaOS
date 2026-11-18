// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Network Manager Abstraction Engine
//!
//! Abstracts IWD (iNet wireless daemon) and NetworkManager (`nmcli`) for
//! Wi-Fi and wired network profile management, DNS resolver dispatch,
//! and connection state machine — inspired by Arch Linux's iwd default
//! and Omarchy's minimal-but-functional network setup.

use std::collections::BTreeMap;
use std::fmt;

// ── Connection State Machine ─────────────────────────────────────────────────

/// Network connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Scanning,
    Connecting,
    Connected,
    NeedAuth,
    Failed(String),
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disconnected => write!(f, "disconnected"),
            Self::Scanning => write!(f, "scanning"),
            Self::Connecting => write!(f, "connecting"),
            Self::Connected => write!(f, "connected"),
            Self::NeedAuth => write!(f, "need-auth"),
            Self::Failed(e) => write!(f, "failed({})", e),
        }
    }
}

// ── DNS Resolver Strategy ────────────────────────────────────────────────────

/// DNS resolver backend used by SigmaOS
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsResolver {
    Systemd,       // systemd-resolved
    Unbound,       // local unbound cache
    Dnsmasq,       // dnsmasq (NetworkManager plugin)
    Static,        // /etc/resolv.conf direct
}

impl fmt::Display for DnsResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Systemd => write!(f, "systemd-resolved"),
            Self::Unbound => write!(f, "unbound"),
            Self::Dnsmasq => write!(f, "dnsmasq"),
            Self::Static => write!(f, "static"),
        }
    }
}

impl DnsResolver {
    /// Generate the resolv.conf symlink command for each resolver
    pub fn resolv_conf_link_cmd(&self) -> String {
        match self {
            Self::Systemd => {
                "ln -sf /run/systemd/resolve/stub-resolv.conf /etc/resolv.conf".to_string()
            }
            Self::Unbound => "ln -sf /etc/unbound/resolv.conf /etc/resolv.conf".to_string(),
            Self::Dnsmasq => "ln -sf /run/dnsmasq/resolv.conf /etc/resolv.conf".to_string(),
            Self::Static => "# Edit /etc/resolv.conf directly".to_string(),
        }
    }
}

// ── Wi-Fi Network Profile ────────────────────────────────────────────────────

/// Security protocol for a Wi-Fi profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WifiSecurity {
    Open,
    Wpa2Personal,
    Wpa3Sae,
    WpaEnterprise { identity: String },
}

impl fmt::Display for WifiSecurity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Wpa2Personal => write!(f, "wpa2-personal"),
            Self::Wpa3Sae => write!(f, "wpa3-sae"),
            Self::WpaEnterprise { identity } => write!(f, "wpa-enterprise({})", identity),
        }
    }
}

/// A Wi-Fi network profile (IWD or NM format)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WifiProfile {
    pub ssid: String,
    pub security: WifiSecurity,
    pub auto_connect: bool,
    pub hidden: bool,
    pub priority: u8,
}

impl WifiProfile {
    pub fn new(ssid: &str, security: WifiSecurity) -> Self {
        Self {
            ssid: ssid.to_string(),
            security,
            auto_connect: true,
            hidden: false,
            priority: 0,
        }
    }

    /// Render as an IWD network profile (`/var/lib/iwd/<ssid>.psk`)
    pub fn to_iwd_profile(&self) -> String {
        let security_section = match &self.security {
            WifiSecurity::Open => "[Security]\n# open network — no passphrase\n".to_string(),
            WifiSecurity::Wpa2Personal => "[Security]\nPassphrase=<SET_PASSPHRASE>\n".to_string(),
            WifiSecurity::Wpa3Sae => {
                "[Security]\nPassphrase=<SET_PASSPHRASE>\nSAE=true\n".to_string()
            }
            WifiSecurity::WpaEnterprise { identity } => {
                format!("[Security]\nEAP-Method=PEAP\nEAP-Identity={}\n", identity)
            }
        };
        format!(
            "[Settings]\nAutoConnect={}\nHidden={}\n\n{}\n[IPv4]\nMethod=auto\n",
            if self.auto_connect { "true" } else { "false" },
            if self.hidden { "true" } else { "false" },
            security_section,
        )
    }

    /// Render as `nmcli connection add` command
    pub fn to_nmcli_add_cmd(&self) -> String {
        let sec_arg = match &self.security {
            WifiSecurity::Open => "".to_string(),
            WifiSecurity::Wpa2Personal | WifiSecurity::Wpa3Sae => {
                " wifi-sec.key-mgmt wpa-psk wifi-sec.psk '<PASSPHRASE>'".to_string()
            }
            WifiSecurity::WpaEnterprise { identity } => {
                format!(" wifi-sec.key-mgmt wpa-eap 802-1x.identity '{}'", identity)
            }
        };
        format!(
            "nmcli connection add type wifi ifname '*' con-name '{}' ssid '{}'{}",
            self.ssid, self.ssid, sec_arg
        )
    }
}

// ── Wired Profile ────────────────────────────────────────────────────────────

/// IP assignment mode for wired connections
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpMode {
    Dhcp,
    Static { address: String, gateway: String, dns: Vec<String> },
}

/// Wired (Ethernet) profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WiredProfile {
    pub interface: String,
    pub ip_mode: IpMode,
}

impl WiredProfile {
    pub fn new_dhcp(interface: &str) -> Self {
        Self {
            interface: interface.to_string(),
            ip_mode: IpMode::Dhcp,
        }
    }

    pub fn to_nmcli_add_cmd(&self) -> String {
        match &self.ip_mode {
            IpMode::Dhcp => format!(
                "nmcli connection add type ethernet ifname '{}' con-name '{}' ipv4.method auto",
                self.interface, self.interface
            ),
            IpMode::Static { address, gateway, dns } => format!(
                "nmcli connection add type ethernet ifname '{}' con-name '{}' ipv4.method manual ipv4.addresses '{}' ipv4.gateway '{}' ipv4.dns '{}'",
                self.interface, self.interface, address, gateway, dns.join(",")
            ),
        }
    }
}

// ── Network Manager ──────────────────────────────────────────────────────────

/// Backend network stack choice
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkBackend {
    Iwd,
    NetworkManager,
    Both, // IWD for Wi-Fi, NM for wired
}

/// Central network manager abstraction
#[derive(Debug)]
pub struct NetworkManager {
    pub backend: NetworkBackend,
    pub dns_resolver: DnsResolver,
    pub wifi_profiles: Vec<WifiProfile>,
    pub wired_profiles: Vec<WiredProfile>,
    pub state: BTreeMap<String, ConnectionState>,
}

impl NetworkManager {
    pub fn new(backend: NetworkBackend, dns: DnsResolver) -> Self {
        Self {
            backend,
            dns_resolver: dns,
            wifi_profiles: Vec::new(),
            wired_profiles: Vec::new(),
            state: BTreeMap::new(),
        }
    }

    /// Default SigmaOS network setup (IWD + systemd-resolved)
    pub fn sigma_default() -> Self {
        Self::new(NetworkBackend::Iwd, DnsResolver::Systemd)
    }

    pub fn add_wifi(&mut self, profile: WifiProfile) {
        self.state
            .insert(profile.ssid.clone(), ConnectionState::Disconnected);
        self.wifi_profiles.push(profile);
    }

    pub fn add_wired(&mut self, profile: WiredProfile) {
        self.state
            .insert(profile.interface.clone(), ConnectionState::Disconnected);
        self.wired_profiles.push(profile);
    }

    /// Transition a network interface to a new state
    pub fn set_state(&mut self, iface: &str, state: ConnectionState) {
        self.state.insert(iface.to_string(), state);
    }

    pub fn get_state(&self, iface: &str) -> Option<&ConnectionState> {
        self.state.get(iface)
    }

    /// Generate IWD systemd enable commands
    pub fn iwd_enable_cmds(&self) -> Vec<String> {
        vec![
            "systemctl enable --now iwd.service".to_string(),
            self.dns_resolver.resolv_conf_link_cmd(),
            "systemctl enable --now systemd-networkd.service".to_string(),
        ]
    }

    /// Generate NM enable commands
    pub fn nm_enable_cmds(&self) -> Vec<String> {
        vec![
            "systemctl enable --now NetworkManager.service".to_string(),
            self.dns_resolver.resolv_conf_link_cmd(),
        ]
    }

    /// Render all Wi-Fi profiles as IWD station connect commands
    pub fn iwd_station_cmds(&self) -> Vec<String> {
        self.wifi_profiles
            .iter()
            .map(|p| format!("iwctl station wlan0 connect '{}'", p.ssid))
            .collect()
    }

    /// Render all setup commands based on backend
    pub fn setup_commands(&self) -> Vec<String> {
        let mut cmds = match self.backend {
            NetworkBackend::Iwd => self.iwd_enable_cmds(),
            NetworkBackend::NetworkManager => self.nm_enable_cmds(),
            NetworkBackend::Both => {
                let mut v = self.iwd_enable_cmds();
                v.extend(self.nm_enable_cmds());
                v
            }
        };
        for wp in &self.wifi_profiles {
            cmds.push(wp.to_iwd_profile());
        }
        for ep in &self.wired_profiles {
            cmds.push(ep.to_nmcli_add_cmd());
        }
        cmds
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_profile_iwd_wpa2_aaa() {
        // Arrange
        let profile = WifiProfile::new("SigmaNet", WifiSecurity::Wpa2Personal);
        // Act
        let iwd = profile.to_iwd_profile();
        // Assert
        assert!(iwd.contains("[Settings]"), "must have Settings section");
        assert!(iwd.contains("AutoConnect=true"), "auto connect by default");
        assert!(iwd.contains("[Security]"), "must have Security section");
        assert!(iwd.contains("Passphrase="), "WPA2 must include Passphrase key");
    }

    #[test]
    fn test_wifi_profile_nmcli_cmd_aaa() {
        // Arrange
        let profile = WifiProfile::new("HomeNet", WifiSecurity::Wpa3Sae);
        // Act
        let cmd = profile.to_nmcli_add_cmd();
        // Assert
        assert!(cmd.starts_with("nmcli connection add"), "must be nmcli add");
        assert!(cmd.contains("HomeNet"), "must include SSID");
        assert!(cmd.contains("wpa-psk"), "SAE falls back to wpa-psk key-mgmt");
    }

    #[test]
    fn test_dns_resolver_symlink_cmd_aaa() {
        // Arrange
        let resolver = DnsResolver::Systemd;
        // Act
        let cmd = resolver.resolv_conf_link_cmd();
        // Assert
        assert!(
            cmd.contains("stub-resolv.conf"),
            "systemd-resolved uses stub-resolv.conf"
        );
        assert!(cmd.contains("ln -sf"), "command must be a symlink");
    }

    #[test]
    fn test_network_manager_state_machine_aaa() {
        // Arrange
        let mut nm = NetworkManager::sigma_default();
        let profile = WifiProfile::new("SigmaAP", WifiSecurity::Open);
        nm.add_wifi(profile);
        // Act
        nm.set_state("SigmaAP", ConnectionState::Connected);
        // Assert
        assert_eq!(
            nm.get_state("SigmaAP"),
            Some(&ConnectionState::Connected),
            "state must transition to Connected"
        );
    }

    #[test]
    fn test_setup_commands_iwd_aaa() {
        // Arrange
        let nm = NetworkManager::sigma_default();
        // Act
        let cmds = nm.setup_commands();
        // Assert
        assert!(
            cmds.iter().any(|c| c.contains("iwd.service")),
            "setup must include enabling iwd"
        );
        assert!(
            cmds.iter().any(|c| c.contains("stub-resolv.conf")),
            "setup must configure systemd-resolved"
        );
    }
}

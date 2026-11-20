// SPDX-License-Identifier: MIT
// SigmaOS Network Manager - Arch Linux Inspired
// Implements netctl-style network configuration and management

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Network connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    Wifi,
    Bridge,
    Bond,
    Vlan,
    Tunnel,
    Pppoe,
    Vpn,
    Wireless,
}

impl ConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionType::Ethernet => "ethernet",
            ConnectionType::Wifi => "wifi",
            ConnectionType::Bridge => "bridge",
            ConnectionType::Bond => "bond",
            ConnectionType::Vlan => "vlan",
            ConnectionType::Tunnel => "tunnel",
            ConnectionType::Pppoe => "pppoe",
            ConnectionType::Vpn => "vpn",
            ConnectionType::Wireless => "wireless",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ethernet" => Some(ConnectionType::Ethernet),
            "wifi" => Some(ConnectionType::Wifi),
            "bridge" => Some(ConnectionType::Bridge),
            "bond" => Some(ConnectionType::Bond),
            "vlan" => Some(ConnectionType::Vlan),
            "tunnel" => Some(ConnectionType::Tunnel),
            "pppoe" => Some(ConnectionType::Pppoe),
            "vpn" => Some(ConnectionType::Vpn),
            "wireless" => Some(ConnectionType::Wireless),
            _ => None,
        }
    }
}

/// IP configuration method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpMethod {
    Dhcp,
    Static,
    Ipv4Ll,
    Ipv6Ll,
    Disabled,
    Auto6,
    Dhcp6,
}

impl IpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            IpMethod::Dhcp => "dhcp",
            IpMethod::Static => "static",
            IpMethod::Ipv4Ll => "ipv4ll",
            IpMethod::Ipv6Ll => "ipv6ll",
            IpMethod::Disabled => "disabled",
            IpMethod::Auto6 => "auto6",
            IpMethod::Dhcp6 => "dhcp6",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "dhcp" => Some(IpMethod::Dhcp),
            "static" => Some(IpMethod::Static),
            "ipv4ll" => Some(IpMethod::Ipv4Ll),
            "ipv6ll" => Some(IpMethod::Ipv6Ll),
            "disabled" => Some(IpMethod::Disabled),
            "auto6" => Some(IpMethod::Auto6),
            "dhcp6" => Some(IpMethod::Dhcp6),
            _ => None,
        }
    }
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Down,
    Connecting,
    Up,
    Failed,
    Unknown,
}

impl ConnectionState {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionState::Down => "down",
            ConnectionState::Connecting => "connecting",
            ConnectionState::Up => "up",
            ConnectionState::Failed => "failed",
            ConnectionState::Unknown => "unknown",
        }
    }

    pub fn is_up(&self) -> bool {
        matches!(self, ConnectionState::Up)
    }

    pub fn is_down(&self) -> bool {
        matches!(self, ConnectionState::Down)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, ConnectionState::Failed)
    }
}

/// IPv4 address configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv4Address {
    pub address: Ipv4Addr,
    pub prefix_length: u8,
    pub gateway: Option<Ipv4Addr>,
}

impl Ipv4Address {
    pub fn new(address: Ipv4Addr, prefix_length: u8) -> Self {
        Self {
            address,
            prefix_length,
            gateway: None,
        }
    }

    pub fn with_gateway(mut self, gateway: Ipv4Addr) -> Self {
        self.gateway = Some(gateway);
        self
    }

    pub fn to_cidr(&self) -> String {
        format!("{}/{}", self.address, self.prefix_length)
    }
}

/// IPv6 address configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Ipv6Address {
    pub address: Ipv6Addr,
    pub prefix_length: u8,
    pub gateway: Option<Ipv6Addr>,
}

impl Ipv6Address {
    pub fn new(address: Ipv6Addr, prefix_length: u8) -> Self {
        Self {
            address,
            prefix_length,
            gateway: None,
        }
    }

    pub fn with_gateway(mut self, gateway: Ipv6Addr) -> Self {
        self.gateway = Some(gateway);
        self
    }

    pub fn to_cidr(&self) -> String {
        format!("{}/{}", self.address, self.prefix_length)
    }
}

/// DNS configuration
#[derive(Debug, Clone)]
pub struct DnsConfig {
    pub nameservers: Vec<IpAddr>,
    pub search_domains: Vec<String>,
    pub domain: Option<String>,
}

impl DnsConfig {
    pub fn new() -> Self {
        Self {
            nameservers: Vec::new(),
            search_domains: Vec::new(),
            domain: None,
        }
    }

    pub fn nameserver(mut self, ns: IpAddr) -> Self {
        self.nameservers.push(ns);
        self
    }

    pub fn search_domain(mut self, domain: impl Into<String>) -> Self {
        self.search_domains.push(domain.into());
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.nameservers.is_empty() && self.search_domains.is_empty() && self.domain.is_none()
    }
}

/// Network connection profile
#[derive(Debug, Clone)]
pub struct ConnectionProfile {
    pub name: String,
    pub description: String,
    pub connection_type: ConnectionType,
    pub interface: String,
    pub ipv4_method: IpMethod,
    pub ipv6_method: IpMethod,
    pub ipv4_addresses: Vec<Ipv4Address>,
    pub ipv6_addresses: Vec<Ipv6Address>,
    pub dns: DnsConfig,
    pub mtu: Option<u32>,
    pub mac_address: Option<String>,
    pub ssid: Option<String>,
    pub password: Option<String>,
    pub hidden: bool,
    pub priority: i32,
    pub autoconnect: bool,
    pub state: ConnectionState,
    pub last_connected: Option<u64>,
    pub uptime: Option<u64>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub custom_options: BTreeMap<String, String>,
}

impl ConnectionProfile {
    /// Create a new connection profile
    pub fn new(name: impl Into<String>, connection_type: ConnectionType, interface: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            connection_type,
            interface: interface.into(),
            ipv4_method: IpMethod::Dhcp,
            ipv6_method: IpMethod::Auto6,
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            dns: DnsConfig::new(),
            mtu: None,
            mac_address: None,
            ssid: None,
            password: None,
            hidden: false,
            priority: 0,
            autoconnect: true,
            state: ConnectionState::Down,
            last_connected: None,
            uptime: None,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
            rx_dropped: 0,
            tx_dropped: 0,
            custom_options: BTreeMap::new(),
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set IPv4 method
    pub fn ipv4_method(mut self, method: IpMethod) -> Self {
        self.ipv4_method = method;
        self
    }

    /// Set IPv6 method
    pub fn ipv6_method(mut self, method: IpMethod) -> Self {
        self.ipv6_method = method;
        self
    }

    /// Add IPv4 address
    pub fn ipv4_address(mut self, addr: Ipv4Address) -> Self {
        self.ipv4_addresses.push(addr);
        self
    }

    /// Add IPv6 address
    pub fn ipv6_address(mut self, addr: Ipv6Address) -> Self {
        self.ipv6_addresses.push(addr);
        self
    }

    /// Set DNS configuration
    pub fn dns(mut self, dns: DnsConfig) -> Self {
        self.dns = dns;
        self
    }

    /// Set MTU
    pub fn mtu(mut self, mtu: u32) -> Self {
        self.mtu = Some(mtu);
        self
    }

    /// Set MAC address
    pub fn mac_address(mut self, mac: impl Into<String>) -> Self {
        self.mac_address = Some(mac.into());
        self
    }

    /// Set SSID (for WiFi)
    pub fn ssid(mut self, ssid: impl Into<String>) -> Self {
        self.ssid = Some(ssid.into());
        self
    }

    /// Set password (for WiFi)
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set hidden network
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Set autoconnect
    pub fn autoconnect(mut self, autoconnect: bool) -> Self {
        self.autoconnect = autoconnect;
        self
    }

    /// Set connection state
    pub fn state(mut self, state: ConnectionState) -> Self {
        self.state = state;
        self
    }

    /// Set last connected time
    pub fn last_connected(mut self, time: u64) -> Self {
        self.last_connected = Some(time);
        self
    }

    /// Set uptime
    pub fn uptime(mut self, uptime: u64) -> Self {
        self.uptime = Some(uptime);
        self
    }

    /// Set statistics
    pub fn stats(mut self, rx_bytes: u64, tx_bytes: u64, rx_packets: u64, tx_packets: u64) -> Self {
        self.rx_bytes = rx_bytes;
        self.tx_bytes = tx_bytes;
        self.rx_packets = rx_packets;
        self.tx_packets = tx_packets;
        self
    }

    /// Set custom option
    pub fn custom_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_options.insert(key.into(), value.into());
        self
    }

    /// Check if connection is up
    pub fn is_up(&self) -> bool {
        self.state.is_up()
    }

    /// Check if connection is down
    pub fn is_down(&self) -> bool {
        self.state.is_down()
    }

    /// Check if connection is failed
    pub fn is_failed(&self) -> bool {
        self.state.is_failed()
    }

    /// Get total bytes transferred
    pub fn get_total_bytes(&self) -> u64 {
        self.rx_bytes + self.tx_bytes
    }

    /// Get total packets transferred
    pub fn get_total_packets(&self) -> u64 {
        self.rx_packets + self.tx_packets
    }

    /// Convert to netctl profile format
    pub fn to_netctl_profile(&self) -> String {
        let mut content = String::new();

        content.push_str(&format!("[Connection]\n"));
        content.push_str(&format!("Description={}\n", self.description));
        content.push_str(&format!("Interface={}\n", self.interface));
        content.push_str(&format!("Connection={}\n", self.connection_type.as_str()));
        content.push_str(&format!("IP={}.method\n", self.ipv4_method.as_str()));
        content.push_str(&format!("IP6={}.method\n", self.ipv6_method.as_str()));

        if self.ipv4_method == IpMethod::Static && !self.ipv4_addresses.is_empty() {
            for addr in &self.ipv4_addresses {
                content.push_str(&format!("Address={}\n", addr.to_cidr()));
                if let Some(gateway) = addr.gateway {
                    content.push_str(&format!("Gateway={}\n", gateway));
                }
            }
        }

        if self.ipv6_method == IpMethod::Static && !self.ipv6_addresses.is_empty() {
            for addr in &self.ipv6_addresses {
                content.push_str(&format!("Address6={}\n", addr.to_cidr()));
                if let Some(gateway) = addr.gateway {
                    content.push_str(&format!("Gateway6={}\n", gateway));
                }
            }
        }

        if !self.dns.nameservers.is_empty() {
            for ns in &self.dns.nameservers {
                content.push_str(&format!("DNS={}\n", ns));
            }
        }

        if let Some(domain) = &self.dns.domain {
            content.push_str(&format!("Domain={}\n", domain));
        }

        if !self.dns.search_domains.is_empty() {
            content.push_str(&format!("DNSSearch={}\n", self.dns.search_domains.join(" ")));
        }

        if let Some(mtu) = self.mtu {
            content.push_str(&format!("MTU={}\n", mtu));
        }

        if let Some(mac) = &self.mac_address {
            content.push_str(&format!("MAC={}\n", mac));
        }

        if let Some(ssid) = &self.ssid {
            content.push_str(&format!("SSID={}\n", ssid));
        }

        if let Some(password) = &self.password {
            content.push_str(&format!("Key={}\n", password));
        }

        if self.hidden {
            content.push_str("Hidden=yes\n");
        }

        if self.priority != 0 {
            content.push_str(&format!("Priority={}\n", self.priority));
        }

        if !self.autoconnect {
            content.push_str("AutoConnect=no\n");
        }

        for (key, value) in &self.custom_options {
            content.push_str(&format!("{}={}\n", key, value));
        }

        content
    }

    /// Convert to NetworkManager connection format
    pub fn to_nm_connection(&self) -> String {
        let mut content = String::new();

        content.push_str("[connection]\n");
        content.push_str(&format!("id={}\n", self.name));
        content.push_str(&format!("uuid={}\n", uuid::Uuid::new_v4()));
        content.push_str(&format!("type={}\n", self.connection_type.as_str()));
        content.push_str(&format!("interface-name={}\n", self.interface));

        if let Some(mac) = &self.mac_address {
            content.push_str(&format!("mac-address={}\n", mac));
        }

        content.push_str("autoconnect=");
        content.push_str(if self.autoconnect { "true" } else { "false" });
        content.push('\n');

        if self.priority != 0 {
            content.push_str(&format!("autoconnect-priority={}\n", self.priority));
        }

        content.push_str("\n[ipv4]\n");
        content.push_str(&format!("method={}\n", self.ipv4_method.as_str()));

        for addr in &self.ipv4_addresses {
            content.push_str(&format!("address1={},{}\n", addr.to_cidr(), addr.address));
        }

        for addr in &self.ipv4_addresses {
            if let Some(gateway) = addr.gateway {
                content.push_str(&format!("gateway={}\n", gateway));
            }
        }

        content.push_str("\n[ipv6]\n");
        content.push_str(&format!("method={}\n", self.ipv6_method.as_str()));

        for addr in &self.ipv6_addresses {
            content.push_str(&format!("address1={},{}\n", addr.to_cidr(), addr.address));
        }

        for addr in &self.ipv6_addresses {
            if let Some(gateway) = addr.gateway {
                content.push_str(&format!("gateway={}\n", gateway));
            }
        }

        if !self.dns.nameservers.is_empty() {
            content.push_str("\n[dns]\n");
            for ns in &self.dns.nameservers {
                content.push_str(&format!("{}\n", ns));
            }
        }

        content
    }
}

/// Network interface information
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: String,
    pub ip_addresses: Vec<IpAddr>,
    pub ipv6_addresses: Vec<IpAddr>,
    pub gateway: Option<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub mtu: u32,
    pub speed: u32,
    pub duplex: String,
    pub operstate: String,
    pub wireless: bool,
    pub ssid: Option<String>,
    pub signal_strength: Option<i32>,
    pub frequency: Option<u32>,
    pub channel: Option<u32>,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub is_up: bool,
    pub is_running: bool,
    pub carrier: bool,
    pub broadcast: bool,
    pub multicast: bool,
    pub promisc: bool,
    pub allmulti: bool,
}

impl NetworkInterface {
    /// Create a new network interface
    pub fn new(name: String, mac_address: String) -> Self {
        Self {
            name,
            mac_address,
            ip_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            gateway: None,
            dns_servers: Vec::new(),
            mtu: 1500,
            speed: 0,
            duplex: String::new(),
            operstate: String::new(),
            wireless: false,
            ssid: None,
            signal_strength: None,
            frequency: None,
            channel: None,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
            rx_dropped: 0,
            tx_dropped: 0,
            is_up: false,
            is_running: false,
            carrier: false,
            broadcast: false,
            multicast: false,
            promisc: false,
            allmulti: false,
        }
    }

    /// Add IP address
    pub fn add_ip(mut self, ip: IpAddr) -> Self {
        if ip.is_ipv4() {
            self.ip_addresses.push(ip);
        } else {
            self.ipv6_addresses.push(ip);
        }
        self
    }

    /// Set gateway
    pub fn gateway(mut self, gateway: IpAddr) -> Self {
        self.gateway = Some(gateway);
        self
    }

    /// Add DNS server
    pub fn dns_server(mut self, dns: IpAddr) -> Self {
        self.dns_servers.push(dns);
        self
    }

    /// Set MTU
    pub fn mtu(mut self, mtu: u32) -> Self {
        self.mtu = mtu;
        self
    }

    /// Set speed
    pub fn speed(mut self, speed: u32) -> Self {
        self.speed = speed;
        self
    }

    /// Set duplex
    pub fn duplex(mut self, duplex: impl Into<String>) -> Self {
        self.duplex = duplex.into();
        self
    }

    /// Set operstate
    pub fn operstate(mut self, state: impl Into<String>) -> Self {
        self.operstate = state.into();
        self
    }

    /// Set wireless
    pub fn wireless(mut self, wireless: bool) -> Self {
        self.wireless = wireless;
        self
    }

    /// Set SSID
    pub fn ssid(mut self, ssid: impl Into<String>) -> Self {
        self.ssid = Some(ssid.into());
        self
    }

    /// Set signal strength
    pub fn signal_strength(mut self, strength: i32) -> Self {
        self.signal_strength = Some(strength);
        self
    }

    /// Set frequency
    pub fn frequency(mut self, freq: u32) -> Self {
        self.frequency = Some(freq);
        self
    }

    /// Set channel
    pub fn channel(mut self, channel: u32) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Set statistics
    pub fn stats(mut self, rx_bytes: u64, tx_bytes: u64, rx_packets: u64, tx_packets: u64) -> Self {
        self.rx_bytes = rx_bytes;
        self.tx_bytes = tx_bytes;
        self.rx_packets = rx_packets;
        self.tx_packets = tx_packets;
        self
    }

    /// Set error statistics
    pub fn error_stats(mut self, rx_errors: u64, tx_errors: u64, rx_dropped: u64, tx_dropped: u64) -> Self {
        self.rx_errors = rx_errors;
        self.tx_errors = tx_errors;
        self.rx_dropped = rx_dropped;
        self.tx_dropped = tx_dropped;
        self
    }

    /// Set interface state
    pub fn state(mut self, up: bool, running: bool, carrier: bool) -> Self {
        self.is_up = up;
        self.is_running = running;
        self.carrier = carrier;
        self
    }

    /// Set flags
    pub fn flags(mut self, broadcast: bool, multicast: bool, promisc: bool, allmulti: bool) -> Self {
        self.broadcast = broadcast;
        self.multicast = multicast;
        self.promisc = promisc;
        self.allmulti = allmulti;
        self
    }

    /// Get total bytes transferred
    pub fn get_total_bytes(&self) -> u64 {
        self.rx_bytes + self.tx_bytes
    }

    /// Get total packets transferred
    pub fn get_total_packets(&self) -> u64 {
        self.rx_packets + self.tx_packets
    }

    /// Check if interface has IP addresses
    pub fn has_ip(&self) -> bool {
        !self.ip_addresses.is_empty() || !self.ipv6_addresses.is_empty()
    }

    /// Check if interface is wireless
    pub fn is_wireless(&self) -> bool {
        self.wireless
    }

    /// Check if interface is up
    pub fn is_up(&self) -> bool {
        self.is_up
    }

    /// Check if interface is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

/// Network manager
#[derive(Debug, Clone)]
pub struct NetworkManager {
    pub profiles: BTreeMap<String, ConnectionProfile>,
    pub interfaces: BTreeMap<String, NetworkInterface>,
    pub active_connections: BTreeMap<String, String>, // interface -> profile name
    pub config_dir: PathBuf,
    pub netctl_dir: PathBuf,
    pub nm_dir: PathBuf,
    pub default_profile: Option<String>,
    pub last_update: Option<u64>,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            interfaces: BTreeMap::new(),
            active_connections: BTreeMap::new(),
            config_dir: PathBuf::from("/etc/sigmaos/network"),
            netctl_dir: PathBuf::from("/etc/netctl"),
            nm_dir: PathBuf::from("/etc/NetworkManager/system-connections"),
            default_profile: None,
            last_update: None,
        }
    }

    /// Create network manager with custom directories
    pub fn with_dirs(config_dir: PathBuf, netctl_dir: PathBuf, nm_dir: PathBuf) -> Self {
        Self {
            profiles: BTreeMap::new(),
            interfaces: BTreeMap::new(),
            active_connections: BTreeMap::new(),
            config_dir,
            netctl_dir,
            nm_dir,
            default_profile: None,
            last_update: None,
        }
    }

    /// Initialize network manager
    pub fn initialize(&mut self) -> Result<(), String> {
        // Create directories
        fs::create_dir_all(&self.config_dir).map_err(|e| format!("Failed to create config dir: {}", e))?;
        fs::create_dir_all(&self.netctl_dir).map_err(|e| format!("Failed to create netctl dir: {}", e))?;
        fs::create_dir_all(&self.nm_dir).map_err(|e| format!("Failed to create NM dir: {}", e))?;

        // Load existing profiles
        self.load_profiles()?;

        // Detect network interfaces
        self.detect_interfaces()?;

        self.last_update = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        Ok(())
    }

    /// Load network profiles
    pub fn load_profiles(&mut self) -> Result<(), String> {
        // Load from netctl directory
        if self.netctl_dir.exists() {
            for entry in fs::read_dir(&self.netctl_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().map_or(false, |ext| ext == "profile") {
                    if let Ok(profile) = self.load_netctl_profile(&path) {
                        self.profiles.insert(profile.name.clone(), profile);
                    }
                }
            }
        }

        // Load from NetworkManager directory
        if self.nm_dir.exists() {
            for entry in fs::read_dir(&self.nm_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().map_or(false, |ext| ext == "nmconnection") {
                    if let Ok(profile) = self.load_nm_connection(&path) {
                        self.profiles.insert(profile.name.clone(), profile);
                    }
                }
            }
        }

        Ok(())
    }

    /// Load netctl profile
    pub fn load_netctl_profile(&self, path: &Path) -> Result<ConnectionProfile, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read profile: {}", e))?;

        let mut profile = ConnectionProfile::new("unknown", ConnectionType::Ethernet, "eth0");

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Description" => profile.description = value.to_string(),
                    "Interface" => profile.interface = value.to_string(),
                    "Connection" => {
                        if let Some(conn_type) = ConnectionType::from_str(value) {
                            profile.connection_type = conn_type;
                        }
                    }
                    "IP" => {
                        if value.ends_with(".method") {
                            let method_str = &value[..value.len()-7];
                            if let Some(method) = IpMethod::from_str(method_str) {
                                profile.ipv4_method = method;
                            }
                        }
                    }
                    "IP6" => {
                        if value.ends_with(".method") {
                            let method_str = &value[..value.len()-7];
                            if let Some(method) = IpMethod::from_str(method_str) {
                                profile.ipv6_method = method;
                            }
                        }
                    }
                    "Address" => {
                        if let Ok(addr) = self.parse_ipv4_address(value) {
                            profile.ipv4_addresses.push(addr);
                        }
                    }
                    "Address6" => {
                        if let Ok(addr) = self.parse_ipv6_address(value) {
                            profile.ipv6_addresses.push(addr);
                        }
                    }
                    "Gateway" => {
                        if let Ok(ip) = value.parse::<Ipv4Addr>() {
                            if let Some(last_addr) = profile.ipv4_addresses.last_mut() {
                                last_addr.gateway = Some(ip);
                            }
                        }
                    }
                    "Gateway6" => {
                        if let Ok(ip) = value.parse::<Ipv6Addr>() {
                            if let Some(last_addr) = profile.ipv6_addresses.last_mut() {
                                last_addr.gateway = Some(ip);
                            }
                        }
                    }
                    "DNS" => {
                        if let Ok(ip) = value.parse::<IpAddr>() {
                            profile.dns.nameservers.push(ip);
                        }
                    }
                    "Domain" => {
                        profile.dns.domain = Some(value.to_string());
                    }
                    "DNSSearch" => {
                        profile.dns.search_domains = value.split_whitespace().map(String::from).collect();
                    }
                    "MTU" => {
                        if let Ok(mtu) = value.parse::<u32>() {
                            profile.mtu = Some(mtu);
                        }
                    }
                    "MAC" => {
                        profile.mac_address = Some(value.to_string());
                    }
                    "SSID" => {
                        profile.ssid = Some(value.to_string());
                    }
                    "Key" => {
                        profile.password = Some(value.to_string());
                    }
                    "Hidden" => {
                        profile.hidden = value == "yes" || value == "true";
                    }
                    "Priority" => {
                        if let Ok(priority) = value.parse::<i32>() {
                            profile.priority = priority;
                        }
                    }
                    "AutoConnect" => {
                        profile.autoconnect = value != "no" && value != "false";
                    }
                    _ => {
                        profile.custom_options.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        // Use filename as profile name if not set
        if profile.name == "unknown" {
            profile.name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
        }

        Ok(profile)
    }

    /// Load NetworkManager connection
    pub fn load_nm_connection(&self, path: &Path) -> Result<ConnectionProfile, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read NM connection: {}", e))?;

        let mut profile = ConnectionProfile::new("unknown", ConnectionType::Ethernet, "eth0");
        let mut in_ipv4 = false;
        let mut in_ipv6 = false;
        let mut in_dns = false;
        let mut in_wifi = false;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section = &line[1..line.len()-1];
                in_ipv4 = section == "ipv4";
                in_ipv6 = section == "ipv6";
                in_dns = section == "dns";
                in_wifi = section == "wifi" || section == "wifi-security";
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                if in_ipv4 {
                    match key {
                        "method" => {
                            if let Some(method) = IpMethod::from_str(value) {
                                profile.ipv4_method = method;
                            }
                        }
                        "address1" => {
                            if let Ok(addr) = self.parse_nm_ipv4_address(value) {
                                profile.ipv4_addresses.push(addr);
                            }
                        }
                        "gateway" => {
                            if let Ok(ip) = value.parse::<Ipv4Addr>() {
                                if let Some(last_addr) = profile.ipv4_addresses.last_mut() {
                                    last_addr.gateway = Some(ip);
                                }
                            }
                        }
                        _ => {}
                    }
                } else if in_ipv6 {
                    match key {
                        "method" => {
                            if let Some(method) = IpMethod::from_str(value) {
                                profile.ipv6_method = method;
                            }
                        }
                        "address1" => {
                            if let Ok(addr) = self.parse_nm_ipv6_address(value) {
                                profile.ipv6_addresses.push(addr);
                            }
                        }
                        "gateway" => {
                            if let Ok(ip) = value.parse::<Ipv6Addr>() {
                                if let Some(last_addr) = profile.ipv6_addresses.last_mut() {
                                    last_addr.gateway = Some(ip);
                                }
                            }
                        }
                        _ => {}
                    }
                } else if in_dns {
                    if let Ok(ip) = value.parse::<IpAddr>() {
                        profile.dns.nameservers.push(ip);
                    }
                } else if in_wifi {
                    match key {
                        "ssid" => profile.ssid = Some(value.to_string()),
                        "psk" => profile.password = Some(value.to_string()),
                        "hidden" => profile.hidden = value == "true",
                        _ => {}
                    }
                } else {
                    match key {
                        "id" => profile.name = value.to_string(),
                        "type" => {
                            if let Some(conn_type) = ConnectionType::from_str(value) {
                                profile.connection_type = conn_type;
                            }
                        }
                        "interface-name" => profile.interface = value.to_string(),
                        "mac-address" => profile.mac_address = Some(value.to_string()),
                        "autoconnect" => profile.autoconnect = value == "true",
                        "autoconnect-priority" => {
                            if let Ok(priority) = value.parse::<i32>() {
                                profile.priority = priority;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(profile)
    }

    /// Parse IPv4 address from netctl format
    fn parse_ipv4_address(&self, value: &str) -> Result<Ipv4Address, String> {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid IPv4 address format".to_string());
        }

        let address: Ipv4Addr = parts[0].parse()
            .map_err(|e| format!("Invalid IPv4 address: {}", e))?;
        let prefix_length: u8 = parts[1].parse()
            .map_err(|e| format!("Invalid prefix length: {}", e))?;

        Ok(Ipv4Address::new(address, prefix_length))
    }

    /// Parse IPv6 address from netctl format
    fn parse_ipv6_address(&self, value: &str) -> Result<Ipv6Address, String> {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid IPv6 address format".to_string());
        }

        let address: Ipv6Addr = parts[0].parse()
            .map_err(|e| format!("Invalid IPv6 address: {}", e))?;
        let prefix_length: u8 = parts[1].parse()
            .map_err(|e| format!("Invalid prefix length: {}", e))?;

        Ok(Ipv6Address::new(address, prefix_length))
    }

    /// Parse IPv4 address from NM format
    fn parse_nm_ipv4_address(&self, value: &str) -> Result<Ipv4Address, String> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() < 2 {
            return Err("Invalid NM IPv4 address format".to_string());
        }

        let cidr = parts[0];
        let addr_parts: Vec<&str> = cidr.split('/').collect();
        if addr_parts.len() != 2 {
            return Err("Invalid CIDR format".to_string());
        }

        let address: Ipv4Addr = addr_parts[0].parse()
            .map_err(|e| format!("Invalid IPv4 address: {}", e))?;
        let prefix_length: u8 = addr_parts[1].parse()
            .map_err(|e| format!("Invalid prefix length: {}", e))?;

        let mut ipv4_addr = Ipv4Address::new(address, prefix_length);

        // Parse gateway if present
        if parts.len() > 1 {
            let gateway_str = parts[1].trim();
            if let Ok(gateway) = gateway_str.parse::<Ipv4Addr>() {
                ipv4_addr.gateway = Some(gateway);
            }
        }

        Ok(ipv4_addr)
    }

    /// Parse IPv6 address from NM format
    fn parse_nm_ipv6_address(&self, value: &str) -> Result<Ipv6Address, String> {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() < 2 {
            return Err("Invalid NM IPv6 address format".to_string());
        }

        let cidr = parts[0];
        let addr_parts: Vec<&str> = cidr.split('/').collect();
        if addr_parts.len() != 2 {
            return Err("Invalid CIDR format".to_string());
        }

        let address: Ipv6Addr = addr_parts[0].parse()
            .map_err(|e| format!("Invalid IPv6 address: {}", e))?;
        let prefix_length: u8 = addr_parts[1].parse()
            .map_err(|e| format!("Invalid prefix length: {}", e))?;

        let mut ipv6_addr = Ipv6Address::new(address, prefix_length);

        // Parse gateway if present
        if parts.len() > 1 {
            let gateway_str = parts[1].trim();
            if let Ok(gateway) = gateway_str.parse::<Ipv6Addr>() {
                ipv6_addr.gateway = Some(gateway);
            }
        }

        Ok(ipv6_addr)
    }

    /// Detect network interfaces
    pub fn detect_interfaces(&mut self) -> Result<(), String> {
        // Read network interfaces from /sys/class/net
        let net_path = Path::new("/sys/class/net");
        if !net_path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(net_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip loopback
            if name == "lo" {
                continue;
            }

            let mut interface = NetworkInterface::new(name.clone(), String::new());

            // Read MAC address
            if let Ok(mac) = fs::read_to_string(path.join("address")) {
                interface.mac_address = mac.trim().to_string();
            }

            // Read operstate
            if let Ok(state) = fs::read_to_string(path.join("operstate")) {
                interface.operstate = state.trim().to_string();
            }

            // Read MTU
            if let Ok(mtu) = fs::read_to_string(path.join("mtu")) {
                if let Ok(mtu_val) = mtu.trim().parse::<u32>() {
                    interface.mtu = mtu_val;
                }
            }

            // Read speed
            if let Ok(speed) = fs::read_to_string(path.join("speed")) {
                if let Ok(speed_val) = speed.trim().parse::<u32>() {
                    interface.speed = speed_val;
                }
            }

            // Read duplex
            if let Ok(duplex) = fs::read_to_string(path.join("duplex")) {
                interface.duplex = duplex.trim().to_string();
            }

            // Check if wireless
            if path.join("wireless").exists() {
                interface.wireless = true;
            }

            // Read statistics
            if let Ok(stats) = fs::read_to_string(path.join("statistics/rx_bytes")) {
                if let Ok(rx_bytes) = stats.trim().parse::<u64>() {
                    interface.rx_bytes = rx_bytes;
                }
            }

            if let Ok(stats) = fs::read_to_string(path.join("statistics/tx_bytes")) {
                if let Ok(tx_bytes) = stats.trim().parse::<u64>() {
                    interface.tx_bytes = tx_bytes;
                }
            }

            if let Ok(stats) = fs::read_to_string(path.join("statistics/rx_packets")) {
                if let Ok(rx_packets) = stats.trim().parse::<u64>() {
                    interface.rx_packets = rx_packets;
                }
            }

            if let Ok(stats) = fs::read_to_string(path.join("statistics/tx_packets")) {
                if let Ok(tx_packets) = stats.trim().parse::<u64>() {
                    interface.tx_packets = tx_packets;
                }
            }

            // Set interface state
            interface.is_up = interface.operstate == "up" || interface.operstate == "unknown";
            interface.is_running = interface.operstate == "up";
            interface.carrier = path.join("carrier").exists();

            self.interfaces.insert(name, interface);
        }

        Ok(())
    }

    /// Add connection profile
    pub fn add_profile(&mut self, profile: ConnectionProfile) -> Result<(), String> {
        if self.profiles.contains_key(&profile.name) {
            return Err(format!("Profile '{}' already exists", profile.name));
        }
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Remove connection profile
    pub fn remove_profile(&mut self, name: &str) -> Result<(), String> {
        if self.profiles.remove(name).is_some() {
            // Also remove from active connections
            self.active_connections.retain(|_, v| v != name);
            Ok(())
        } else {
            Err(format!("Profile '{}' not found", name))
        }
    }

    /// Get connection profile
    pub fn get_profile(&self, name: &str) -> Option<&ConnectionProfile> {
        self.profiles.get(name)
    }

    /// Get connection profile mutably
    pub fn get_profile_mut(&mut self, name: &str) -> Option<&mut ConnectionProfile> {
        self.profiles.get_mut(name)
    }

    /// Get all profiles
    pub fn get_all_profiles(&self) -> Vec<&ConnectionProfile> {
        self.profiles.values().collect()
    }

    /// Get profiles by type
    pub fn get_profiles_by_type(&self, conn_type: ConnectionType) -> Vec<&ConnectionProfile> {
        self.profiles.values()
            .filter(|p| p.connection_type == conn_type)
            .collect()
    }

    /// Get profiles for interface
    pub fn get_profiles_for_interface(&self, interface: &str) -> Vec<&ConnectionProfile> {
        self.profiles.values()
            .filter(|p| p.interface == interface)
            .collect()
    }

    /// Get active profiles
    pub fn get_active_profiles(&self) -> Vec<&ConnectionProfile> {
        self.profiles.values()
            .filter(|p| p.is_up())
            .collect()
    }

    /// Get interface information
    pub fn get_interface(&self, name: &str) -> Option<&NetworkInterface> {
        self.interfaces.get(name)
    }

    /// Get all interfaces
    pub fn get_all_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values().collect()
    }

    /// Get active interfaces
    pub fn get_active_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|i| i.is_up())
            .collect()
    }

    /// Get wireless interfaces
    pub fn get_wireless_interfaces(&self) -> Vec<&NetworkInterface> {
        self.interfaces.values()
            .filter(|i| i.wireless)
            .collect()
    }

    /// Connect to profile
    pub fn connect(&mut self, profile_name: &str) -> Result<(), String> {
        let profile = self.profiles.get(profile_name)
            .ok_or_else(|| format!("Profile '{}' not found", profile_name))?
            .clone();

        // Check if interface exists
        if !self.interfaces.contains_key(&profile.interface) {
            return Err(format!("Interface '{}' not found", profile.interface));
        }

        // Disconnect existing connection on this interface
        if let Some(existing_profile) = self.active_connections.get(&profile.interface) {
            self.disconnect(existing_profile)?;
        }

        // Set profile state to connecting
        if let Some(profile_mut) = self.profiles.get_mut(profile_name) {
            profile_mut.state = ConnectionState::Connecting;
        }

        // Execute connection command based on type
        match profile.connection_type {
            ConnectionType::Ethernet => self.connect_ethernet(&profile)?,
            ConnectionType::Wifi => self.connect_wifi(&profile)?,
            ConnectionType::Bridge => self.connect_bridge(&profile)?,
            ConnectionType::Bond => self.connect_bond(&profile)?,
            ConnectionType::Vlan => self.connect_vlan(&profile)?,
            ConnectionType::Tunnel => self.connect_tunnel(&profile)?,
            ConnectionType::Pppoe => self.connect_pppoe(&profile)?,
            ConnectionType::Vpn => self.connect_vpn(&profile)?,
            ConnectionType::Wireless => self.connect_wireless(&profile)?,
        }

        // Update state
        if let Some(profile_mut) = self.profiles.get_mut(profile_name) {
            profile_mut.state = ConnectionState::Up;
            profile_mut.last_connected = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
        }

        // Update interface state
        if let Some(interface) = self.interfaces.get_mut(&profile.interface) {
            interface.is_up = true;
            interface.is_running = true;
        }

        // Record active connection
        self.active_connections.insert(profile.interface.clone(), profile_name.to_string());

        Ok(())
    }

    /// Disconnect profile
    pub fn disconnect(&mut self, profile_name: &str) -> Result<(), String> {
        let profile = self.profiles.get(profile_name)
            .ok_or_else(|| format!("Profile '{}' not found", profile_name))?
            .clone();

        // Execute disconnection command
        self.disconnect_interface(&profile.interface)?;

        // Update profile state
        if let Some(profile_mut) = self.profiles.get_mut(profile_name) {
            profile_mut.state = ConnectionState::Down;
            profile_mut.uptime = None;
        }

        // Update interface state
        if let Some(interface) = self.interfaces.get_mut(&profile.interface) {
            interface.is_up = false;
            interface.is_running = false;
        }

        // Remove from active connections
        self.active_connections.remove(&profile.interface);

        Ok(())
    }

    /// Connect Ethernet profile
    fn connect_ethernet(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // Use dhclient or dhcpcd for DHCP
        if profile.ipv4_method == IpMethod::Dhcp {
            let output = Command::new("dhclient")
                .arg(&profile.interface)
                .output()
                .map_err(|e| format!("Failed to run dhclient: {}", e))?;

            if !output.status.success() {
                return Err(format!("dhclient failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        // Set static IP if configured
        if profile.ipv4_method == IpMethod::Static {
            for addr in &profile.ipv4_addresses {
                let output = Command::new("ip")
                    .arg("addr")
                    .arg("add")
                    .arg(addr.to_cidr())
                    .arg("dev")
                    .arg(&profile.interface)
                    .output()
                    .map_err(|e| format!("Failed to add IP address: {}", e))?;

                if !output.status.success() {
                    return Err(format!("Failed to add IP: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
        }

        // Set gateway
        for addr in &profile.ipv4_addresses {
            if let Some(gateway) = addr.gateway {
                let output = Command::new("ip")
                    .arg("route")
                    .arg("add")
                    .arg("default")
                    .arg("via")
                    .arg(gateway.to_string())
                    .arg("dev")
                    .arg(&profile.interface)
                    .output()
                    .map_err(|e| format!("Failed to add route: {}", e))?;

                if !output.status.success() {
                    return Err(format!("Failed to add route: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
        }

        // Set DNS
        if !profile.dns.nameservers.is_empty() {
            let mut resolv_content = String::new();
            for ns in &profile.dns.nameservers {
                resolv_content.push_str(&format!("nameserver {}\n", ns));
            }
            fs::write("/etc/resolv.conf", resolv_content)
                .map_err(|e| format!("Failed to write resolv.conf: {}", e))?;
        }

        Ok(())
    }

    /// Connect WiFi profile
    fn connect_wifi(&self, profile: &ConnectionProfile) -> Result<(), String> {
        if let (Some(ssid), Some(password)) = (&profile.ssid, &profile.password) {
            // Use wpa_supplicant or similar
            let output = Command::new("wpa_cli")
                .args(["-i", &profile.interface, "add_network"])
                .output()
                .map_err(|e| format!("Failed to run wpa_cli: {}", e))?;

            if !output.status.success() {
                return Err(format!("wpa_cli failed: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Configure SSID and password
            let output = Command::new("wpa_cli")
                .args(["-i", &profile.interface, "set_network", "0", "ssid", &format!("\"{}\"", ssid)])
                .output()
                .map_err(|e| format!("Failed to set SSID: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to set SSID: {}", String::from_utf8_lossy(&output.stderr)));
            }

            let output = Command::new("wpa_cli")
                .args(["-i", &profile.interface, "set_network", "0", "psk", &format!("\"{}\"", password)])
                .output()
                .map_err(|e| format!("Failed to set password: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to set password: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Enable network
            let output = Command::new("wpa_cli")
                .args(["-i", &profile.interface, "enable_network", "0"])
                .output()
                .map_err(|e| format!("Failed to enable network: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to enable network: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Select network
            let output = Command::new("wpa_cli")
                .args(["-i", &profile.interface, "select_network", "0"])
                .output()
                .map_err(|e| format!("Failed to select network: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to select network: {}", String::from_utf8_lossy(&output.stderr)));
            }
        } else {
            return Err("SSID and password required for WiFi connection".to_string());
        }

        // Get IP via DHCP
        self.connect_ethernet(profile)?;

        Ok(())
    }

    /// Connect bridge profile
    fn connect_bridge(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // Create bridge interface
        let output = Command::new("ip")
            .args(["link", "add", "name", &profile.name, "type", "bridge"])
            .output()
            .map_err(|e| format!("Failed to create bridge: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to create bridge: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Add interfaces to bridge
        for member in profile.custom_options.get("members").map(|s| s.split(',').collect::<Vec<_>>()).unwrap_or_default() {
            let output = Command::new("ip")
                .args(["link", "set", member, "master", &profile.name])
                .output()
                .map_err(|e| format!("Failed to add member to bridge: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to add member: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        // Bring up bridge
        let output = Command::new("ip")
            .args(["link", "set", &profile.name, "up"])
            .output()
            .map_err(|e| format!("Failed to bring up bridge: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to bring up bridge: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Connect bond profile
    fn connect_bond(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // Create bond interface
        let output = Command::new("ip")
            .args(["link", "add", "name", &profile.name, "type", "bond"])
            .output()
            .map_err(|e| format!("Failed to create bond: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to create bond: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Set bond mode
        if let Some(mode) = profile.custom_options.get("mode") {
            let output = Command::new("ip")
                .args(["link", "set", &profile.name, "type", "bond", "mode", mode])
                .output()
                .map_err(|e| format!("Failed to set bond mode: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to set bond mode: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        // Add interfaces to bond
        for member in profile.custom_options.get("members").map(|s| s.split(',').collect::<Vec<_>>()).unwrap_or_default() {
            let output = Command::new("ip")
                .args(["link", "set", member, "master", &profile.name])
                .output()
                .map_err(|e| format!("Failed to add member to bond: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to add member: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        // Bring up bond
        let output = Command::new("ip")
            .args(["link", "set", &profile.name, "up"])
            .output()
            .map_err(|e| format!("Failed to bring up bond: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to bring up bond: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Connect VLAN profile
    fn connect_vlan(&self, profile: &ConnectionProfile) -> Result<(), String> {
        if let Some(vlan_id) = profile.custom_options.get("vlan_id") {
            let output = Command::new("ip")
                .args(["link", "add", "link", &profile.interface, "name", &profile.name, "type", "vlan", "id", vlan_id])
                .output()
                .map_err(|e| format!("Failed to create VLAN: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to create VLAN: {}", String::from_utf8_lossy(&output.stderr)));
            }

            // Bring up VLAN
            let output = Command::new("ip")
                .args(["link", "set", &profile.name, "up"])
                .output()
                .map_err(|e| format!("Failed to bring up VLAN: {}", e))?;

            if !output.status.success() {
                return Err(format!("Failed to bring up VLAN: {}", String::from_utf8_lossy(&output.stderr)));
            }
        } else {
            return Err("VLAN ID required for VLAN connection".to_string());
        }

        Ok(())
    }

    /// Connect tunnel profile
    fn connect_tunnel(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // This would implement tunnel creation (GRE, IPIP, etc.)
        // For now, just log the connection attempt
        println!("Connecting tunnel: {}", profile.name);
        Ok(())
    }

    /// Connect PPPoE profile
    fn connect_pppoe(&self, profile: &ConnectionProfile) -> Result<(), String> {
        if let Some(user) = profile.custom_options.get("user") {
            let output = Command::new("pppoeconf")
                .args(["-I", &profile.interface, user])
                .output()
                .map_err(|e| format!("Failed to run pppoeconf: {}", e))?;

            if !output.status.success() {
                return Err(format!("pppoeconf failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        } else {
            return Err("PPPoE user required".to_string());
        }

        Ok(())
    }

    /// Connect VPN profile
    fn connect_vpn(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // This would implement VPN connection
        // For now, just log the connection attempt
        println!("Connecting VPN: {}", profile.name);
        Ok(())
    }

    /// Connect wireless profile
    fn connect_wireless(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // Wireless is similar to WiFi
        self.connect_wifi(profile)
    }

    /// Disconnect interface
    fn disconnect_interface(&self, interface: &str) -> Result<(), String> {
        // Bring interface down
        let output = Command::new("ip")
            .args(["link", "set", interface, "down"])
            .output()
            .map_err(|e| format!("Failed to bring down interface: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to bring down interface: {}", String::from_utf8_lossy(&output.stderr)));
        }

        // Remove IP addresses
        let output = Command::new("ip")
            .args(["addr", "flush", "dev", interface])
            .output()
            .map_err(|e| format!("Failed to flush addresses: {}", e))?;

        if !output.status.success() {
            return Err(format!("Failed to flush addresses: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Save profile to file
    pub fn save_profile(&self, profile: &ConnectionProfile) -> Result<(), String> {
        // Save as netctl profile
        let filename = format!("{}.profile", profile.name);
        let path = self.netctl_dir.join(&filename);
        let content = profile.to_netctl_profile();

        fs::write(&path, content).map_err(|e| format!("Failed to save profile: {}", e))?;

        // Also save as NetworkManager connection
        let nm_filename = format!("{}.nmconnection", profile.name);
        let nm_path = self.nm_dir.join(&nm_filename);
        let nm_content = profile.to_nm_connection();

        fs::write(&nm_path, nm_content).map_err(|e| format!("Failed to save NM connection: {}", e))?;

        Ok(())
    }

    /// Delete profile file
    pub fn delete_profile_file(&self, profile_name: &str) -> Result<(), String> {
        // Delete netctl profile
        let netctl_path = self.netctl_dir.join(format!("{}.profile", profile_name));
        if netctl_path.exists() {
            fs::remove_file(&netctl_path).map_err(|e| format!("Failed to delete netctl profile: {}", e))?;
        }

        // Delete NetworkManager connection
        let nm_path = self.nm_dir.join(format!("{}.nmconnection", profile_name));
        if nm_path.exists() {
            fs::remove_file(&nm_path).map_err(|e| format!("Failed to delete NM connection: {}", e))?;
        }

        Ok(())
    }

    /// Set default profile
    pub fn set_default_profile(&mut self, name: &str) -> Result<(), String> {
        if !self.profiles.contains_key(name) {
            return Err(format!("Profile '{}' not found", name));
        }
        self.default_profile = Some(name.to_string());
        Ok(())
    }

    /// Get default profile
    pub fn get_default_profile(&self) -> Option<&ConnectionProfile> {
        self.default_profile.as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    /// Connect to default profile
    pub fn connect_default(&mut self) -> Result<(), String> {
        if let Some(default) = self.default_profile.clone() {
            self.connect(&default)
        } else {
            Err("No default profile set".to_string())
        }
    }

    /// Scan for available networks (WiFi)
    pub fn scan_networks(&self) -> Result<Vec<WifiNetwork>, String> {
        let mut networks = Vec::new();

        // Use iwlist or similar to scan WiFi networks
        let output = Command::new("iwlist")
            .arg("scan")
            .output()
            .map_err(|e| format!("Failed to scan networks: {}", e))?;

        if !output.status.success() {
            return Err(format!("iwlist failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse iwlist output
        // This is a simplified parser
        for line in stdout.lines() {
            if line.contains("ESSID:") {
                let ssid = line.split("ESSID:").nth(1)
                    .unwrap_or("")
                    .trim_matches('"')
                    .to_string();

                if !ssid.is_empty() {
                    networks.push(WifiNetwork {
                        ssid,
                        bssid: String::new(),
                        signal_strength: 0,
                        frequency: 0,
                        channel: 0,
                        encryption: String::new(),
                        hidden: false,
                    });
                }
            }
        }

        Ok(networks)
    }

    /// Get network statistics
    pub fn get_statistics(&self) -> NetworkStatistics {
        let total_profiles = self.profiles.len();
        let active_profiles = self.profiles.values().filter(|p| p.is_up()).count();
        let total_interfaces = self.interfaces.len();
        let active_interfaces = self.interfaces.values().filter(|i| i.is_up()).count();
        let wireless_interfaces = self.interfaces.values().filter(|i| i.wireless).count();

        NetworkStatistics {
            total_profiles,
            active_profiles,
            total_interfaces,
            active_interfaces,
            wireless_interfaces,
            last_update: self.last_update,
        }
    }

    /// Export configuration
    pub fn export_config(&self) -> String {
        let mut config = String::new();
        config.push_str("# SigmaOS Network Configuration\n");
        config.push_str("# Generated by SigmaOS Network Manager\n\n");

        config.push_str("## Profiles\n");
        for profile in self.profiles.values() {
            config.push_str(&format!("profile={} type={} interface={} state={}\n",
                profile.name,
                profile.connection_type.as_str(),
                profile.interface,
                profile.state.as_str()
            ));
        }

        config.push_str("\n## Interfaces\n");
        for interface in self.interfaces.values() {
            config.push_str(&format!("interface={} mac={} state={} wireless={}\n",
                interface.name,
                interface.mac_address,
                if interface.is_up { "up" } else { "down" },
                interface.wireless
            ));
        }

        config.push_str("\n## Active Connections\n");
        for (interface, profile) in &self.active_connections {
            config.push_str(&format!("{} -> {}\n", interface, profile));
        }

        config
    }

    /// Import configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        // In real implementation, would parse the export format
        // For now, just return success
        let _ = content;
        Ok(())
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

/// WiFi network information
#[derive(Debug, Clone, PartialEq)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: i32,
    pub frequency: u32,
    pub channel: u32,
    pub encryption: String,
    pub hidden: bool,
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub total_profiles: usize,
    pub active_profiles: usize,
    pub total_interfaces: usize,
    pub active_interfaces: usize,
    pub wireless_interfaces: usize,
    pub last_update: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_type() {
        assert_eq!(ConnectionType::Ethernet.as_str(), "ethernet");
        assert_eq!(ConnectionType::from_str("wifi"), Some(ConnectionType::Wifi));
    }

    #[test]
    fn test_ip_method() {
        assert_eq!(IpMethod::Dhcp.as_str(), "dhcp");
        assert_eq!(IpMethod::from_str("static"), Some(IpMethod::Static));
    }

    #[test]
    fn test_connection_state() {
        assert!(ConnectionState::Up.is_up());
        assert!(!ConnectionState::Down.is_up());
        assert!(ConnectionState::Failed.is_failed());
    }

    #[test]
    fn test_ipv4_address() {
        let addr = Ipv4Address::new(Ipv4Addr::new(192, 168, 1, 100), 24)
            .with_gateway(Ipv4Addr::new(192, 168, 1, 1));

        assert_eq!(addr.to_cidr(), "192.168.1.100/24");
        assert_eq!(addr.gateway, Some(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn test_ipv6_address() {
        let addr = Ipv6Address::new("fe80::1".parse().unwrap(), 64);
        assert_eq!(addr.to_cidr(), "fe80::1/64");
    }

    #[test]
    fn test_dns_config() {
        let dns = DnsConfig::new()
            .nameserver("8.8.8.8".parse().unwrap())
            .nameserver("8.8.4.4".parse().unwrap())
            .domain("example.com")
            .search_domain("example.com")
            .search_domain("example.org");

        assert_eq!(dns.nameservers.len(), 2);
        assert_eq!(dns.domain, Some("example.com".to_string()));
        assert_eq!(dns.search_domains.len(), 2);
    }

    #[test]
    fn test_connection_profile_builder() {
        let profile = ConnectionProfile::new("home-wifi", ConnectionType::Wifi, "wlan0")
            .description("Home WiFi network")
            .ipv4_method(IpMethod::Dhcp)
            .ipv6_method(IpMethod::Auto6)
            .ssid("HomeNetwork")
            .password("password123")
            .priority(10)
            .autoconnect(true);

        assert_eq!(profile.name, "home-wifi");
        assert_eq!(profile.connection_type, ConnectionType::Wifi);
        assert_eq!(profile.ssid, Some("HomeNetwork".to_string()));
        assert_eq!(profile.password, Some("password123".to_string()));
        assert_eq!(profile.priority, 10);
        assert!(profile.autoconnect);
    }

    #[test]
    fn test_connection_profile_to_netctl() {
        let profile = ConnectionProfile::new("eth0-dhcp", ConnectionType::Ethernet, "eth0")
            .description("Ethernet DHCP")
            .ipv4_method(IpMethod::Dhcp)
            .ipv6_method(IpMethod::Auto6)
            .dns(DnsConfig::new()
                .nameserver("8.8.8.8".parse().unwrap())
                .nameserver("8.8.4.4".parse().unwrap()));

        let netctl = profile.to_netctl_profile();
        assert!(netctl.contains("[Connection]"));
        assert!(netctl.contains("Interface=eth0"));
        assert!(netctl.contains("Connection=ethernet"));
        assert!(netctl.contains("IP=dhcp.method"));
        assert!(netctl.contains("DNS=8.8.8.8"));
    }

    #[test]
    fn test_connection_profile_to_nm() {
        let profile = ConnectionProfile::new("wifi-home", ConnectionType::Wifi, "wlan0")
            .ssid("HomeNetwork")
            .password("password123")
            .ipv4_method(IpMethod::Dhcp);

        let nm = profile.to_nm_connection();
        assert!(nm.contains("[connection]"));
        assert!(nm.contains("id=wifi-home"));
        assert!(nm.contains("type=wifi"));
        assert!(nm.contains("interface-name=wlan0"));
        assert!(nm.contains("[ipv4]"));
        assert!(nm.contains("method=dhcp"));
    }

    #[test]
    fn test_network_interface() {
        let interface = NetworkInterface::new("eth0".to_string(), "00:11:22:33:44:55".to_string())
            .add_ip("192.168.1.100".parse().unwrap())
            .gateway("192.168.1.1".parse().unwrap())
            .dns_server("8.8.8.8".parse().unwrap())
            .mtu(1500)
            .speed(1000)
            .duplex("full")
            .operstate("up")
            .state(true, true, true)
            .stats(1000000, 500000, 1000, 500);

        assert_eq!(interface.name, "eth0");
        assert_eq!(interface.mac_address, "00:11:22:33:44:55");
        assert_eq!(interface.ip_addresses.len(), 1);
        assert!(interface.has_ip());
        assert!(interface.is_up());
        assert_eq!(interface.get_total_bytes(), 1500000);
    }

    #[test]
    fn test_network_manager() {
        let mut manager = NetworkManager::new();

        let profile = ConnectionProfile::new("test-eth", ConnectionType::Ethernet, "eth0")
            .ipv4_method(IpMethod::Dhcp);

        assert!(manager.add_profile(profile).is_ok());
        assert_eq!(manager.get_all_profiles().len(), 1);
        assert!(manager.get_profile("test-eth").is_some());
    }

    #[test]
    fn test_network_manager_profiles() {
        let mut manager = NetworkManager::new();

        let profile1 = ConnectionProfile::new("eth0-dhcp", ConnectionType::Ethernet, "eth0");
        let profile2 = ConnectionProfile::new("wlan0-wifi", ConnectionType::Wifi, "wlan0");

        manager.add_profile(profile1).unwrap();
        manager.add_profile(profile2).unwrap();

        let ethernet_profiles = manager.get_profiles_by_type(ConnectionType::Ethernet);
        assert_eq!(ethernet_profiles.len(), 1);

        let wifi_profiles = manager.get_profiles_by_type(ConnectionType::Wifi);
        assert_eq!(wifi_profiles.len(), 1);
    }

    #[test]
    fn test_network_statistics() {
        let mut manager = NetworkManager::new();

        let profile = ConnectionProfile::new("test", ConnectionType::Ethernet, "eth0")
            .state(ConnectionState::Up);

        manager.add_profile(profile).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_profiles, 1);
        assert_eq!(stats.active_profiles, 1);
    }

    #[test]
    fn test_network_export_import() {
        let mut manager = NetworkManager::new();

        let profile = ConnectionProfile::new("test", ConnectionType::Ethernet, "eth0")
            .ipv4_method(IpMethod::Dhcp);

        manager.add_profile(profile).unwrap();

        let export = manager.export_config();
        assert!(export.contains("test"));
        assert!(export.contains("ethernet"));
        assert!(export.contains("eth0"));
    }
}

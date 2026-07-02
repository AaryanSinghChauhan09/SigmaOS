// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/daemon/sigmad_netd.rs — sigmad-netd: Network Manager Daemon
// Language: Rust (std) — OOP via NetDaemon + InterfaceManager

use std::collections::BTreeMap;
use std::net::{Ipv4Addr, UdpSocket};
use std::process::Command;
use std::time::{Duration, Instant};

// ── Interface State ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfState { Down, Up, Dhcp, Configured, Failed }

#[derive(Clone, Debug)]
pub struct Interface {
    pub name:    String,
    pub state:   IfState,
    pub mac:     [u8; 6],
    pub ip:      Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub gateway: Option<Ipv4Addr>,
    pub dns:     Vec<Ipv4Addr>,
    pub is_wifi: bool,
    pub ssid:    Option<String>,
    pub metric:  u32,   // route metric (lower = preferred)
}

impl Interface {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(), state: IfState::Down,
            mac: [0;6], ip: None, netmask: None, gateway: None,
            dns: Vec::new(), is_wifi: name.starts_with('w'),
            ssid: None, metric: 100,
        }
    }
    pub fn is_up(&self)         -> bool { self.state != IfState::Down }
    pub fn has_ip(&self)        -> bool { self.ip.is_some() }
    pub fn display_ip(&self)    -> String {
        self.ip.map(|ip| ip.to_string()).unwrap_or_else(|| "unconfigured".to_owned())
    }
}

// ── DNS Resolver Config ───────────────────────────────────────────────────────

pub struct DnsConfig {
    pub servers:  Vec<Ipv4Addr>,
    pub search:   Vec<String>,
    pub use_doh:  bool,
    pub doh_url:  String,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            servers:  vec![Ipv4Addr::new(1,1,1,1), Ipv4Addr::new(9,9,9,9)],
            search:   Vec::new(),
            use_doh:  true,
            doh_url:  "https://cloudflare-dns.com/dns-query".to_owned(),
        }
    }
}

// ── Network Events ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum NetEvent {
    IfUp(String),
    IfDown(String),
    IpAssigned { iface: String, ip: Ipv4Addr },
    DhcpRenewed { iface: String, lease_secs: u32 },
    WifiConnected { iface: String, ssid: String, rssi: i8 },
    WifiDisconnected { iface: String },
    DnsChanged(Vec<Ipv4Addr>),
}

// ── Net Daemon ────────────────────────────────────────────────────────────────

pub struct NetDaemon {
    interfaces:  BTreeMap<String, Interface>,
    dns:         DnsConfig,
    events:      Vec<NetEvent>,
    dhcp_leases: BTreeMap<String, Instant>, // iface → lease expiry
    hostname:    String,
}

impl NetDaemon {
    pub fn new() -> Self {
        Self {
            interfaces:  BTreeMap::new(),
            dns:         DnsConfig::default(),
            events:      Vec::new(),
            dhcp_leases: BTreeMap::new(),
            hostname:    "sigmaos".to_owned(),
        }
    }

    /// Discover interfaces from /sys/class/net
    pub fn discover_interfaces(&mut self) {
        #[cfg(target_os = "linux")]
        if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "lo" { continue; }
                self.interfaces.entry(name.clone())
                    .or_insert_with(|| Interface::new(&name));
                // Read MAC
                let mac_path = format!("/sys/class/net/{}/address", name);
                if let Ok(mac_str) = std::fs::read_to_string(&mac_path) {
                    let parts: Vec<u8> = mac_str.trim().split(':')
                        .filter_map(|h| u8::from_str_radix(h, 16).ok()).collect();
                    if parts.len() == 6 {
                        if let Some(iface) = self.interfaces.get_mut(&name) {
                            iface.mac.copy_from_slice(&parts);
                        }
                    }
                }
                // Check operational state
                let oper = std::fs::read_to_string(
                    format!("/sys/class/net/{}/operstate", name))
                    .unwrap_or_default().trim().to_owned();
                if let Some(iface) = self.interfaces.get_mut(&name) {
                    iface.state = if oper == "up" { IfState::Up } else { IfState::Down };
                }
            }
        }
    }

    /// Bring up an interface using ip(8) or sigma syscalls
    pub fn bring_up(&mut self, name: &str) -> bool {
        #[cfg(target_os = "linux")]
        let _ = Command::new("ip").args(["link", "set", name, "up"]).status();
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.state = IfState::Up;
            self.events.push(NetEvent::IfUp(name.to_owned()));
            return true;
        }
        false
    }

    /// Request DHCP lease on an interface
    pub fn dhcp_request(&mut self, name: &str) -> bool {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.state = IfState::Dhcp;
        }
        // On Linux host: invoke udhcpc or dhclient as fallback
        #[cfg(target_os = "linux")]
        {
            let status = Command::new("udhcpc")
                .args(["-i", name, "-n", "-q"])
                .status();
            if let Ok(s) = status {
                if s.success() {
                    // Read assigned IP from /proc/net/fib_trie or ip addr
                    if let Some(iface) = self.interfaces.get_mut(name) {
                        iface.state = IfState::Configured;
                        // Placeholder IP (real impl parses netlink)
                        iface.ip = Some(Ipv4Addr::new(192,168,1,100));
                        let ip = iface.ip.unwrap();
                        self.events.push(NetEvent::IpAssigned { iface: name.to_owned(), ip });
                        self.dhcp_leases.insert(name.to_owned(), Instant::now() + Duration::from_secs(3600));
                    }
                    return true;
                }
            }
        }
        false
    }

    /// Configure static IP
    pub fn set_static(&mut self, name: &str, ip: Ipv4Addr,
                      mask: Ipv4Addr, gw: Ipv4Addr) {
        if let Some(iface) = self.interfaces.get_mut(name) {
            iface.ip      = Some(ip);
            iface.netmask = Some(mask);
            iface.gateway = Some(gw);
            iface.state   = IfState::Configured;
            self.events.push(NetEvent::IpAssigned { iface: name.to_owned(), ip });
        }
    }

    /// Write /etc/resolv.conf equivalent
    pub fn write_resolv_conf(&self, path: &str) {
        let mut content = format!("# Generated by sigmad-netd\nhostname {}\n", self.hostname);
        for s in &self.dns.search { content.push_str(&format!("search {}\n", s)); }
        for ns in &self.dns.servers { content.push_str(&format!("nameserver {}\n", ns)); }
        let _ = std::fs::write(path, content);
    }

    /// Periodic supervision — renew expiring DHCP leases
    pub fn tick(&mut self) {
        let now = Instant::now();
        let renew: Vec<String> = self.dhcp_leases.iter()
            .filter(|(_, exp)| now + Duration::from_secs(300) >= **exp)
            .map(|(k, _)| k.clone()).collect();
        for iface in renew { self.dhcp_request(&iface); }
    }

    pub fn drain_events(&mut self) -> Vec<NetEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn interface(&self, name: &str) -> Option<&Interface> { self.interfaces.get(name) }

    pub fn list_interfaces(&self) -> impl Iterator<Item = &Interface> {
        self.interfaces.values()
    }

    pub fn status_report(&self) -> String {
        let mut lines = vec!["=== sigmad-netd status ===".to_owned()];
        for iface in self.interfaces.values() {
            lines.push(format!("  {} [{:?}] {} mac={:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                iface.name, iface.state, iface.display_ip(),
                iface.mac[0],iface.mac[1],iface.mac[2],iface.mac[3],iface.mac[4],iface.mac[5]));
        }
        lines.push(format!("  DNS: {}",
            self.dns.servers.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
        lines.join("\n")
    }
}

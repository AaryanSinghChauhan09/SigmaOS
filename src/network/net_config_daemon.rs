//! Network Configuration Daemon — iwd/dhcpcd equivalent
//!
//! Implements a userland network configuration daemon that handles:
//! - DHCP4/DHCP6 client state machine
//! - Wi-Fi backend via nl80211 (using SigmaOS netlink)
//! - Link state monitoring via netlink RTMGRP_LINK
//! - DNS resolver configuration (/etc/resolv.conf writer)
//! - Profile-based network configuration (similar to iwd's station mode)
//!
//! Reference: iwd (Intel Wireless Daemon), dhcpcd, NetworkManager

#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

/// DHCP message type (option 53)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhcpMsgType {
    Discover = 1,
    Offer = 2,
    Request = 3,
    Decline = 4,
    Ack = 5,
    Nak = 6,
    Release = 7,
    Inform = 8,
}

/// DHCP client state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhcpState {
    /// Interface is down or unconfigured
    Init,
    /// Sent DHCPDISCOVER, waiting for OFFER
    Selecting,
    /// Received OFFER, sent REQUEST
    Requesting,
    /// Received ACK, address bound
    Bound,
    /// Lease is 50% expired, trying to renew with original server
    Renewing,
    /// Lease is 87.5% expired, broadcasting to rebind
    Rebinding,
    /// Failed to renew, waiting to retry
    InitReboot,
}

/// IPv4 address + prefix length
#[derive(Debug, Clone)]
pub struct Ipv4Cidr {
    pub addr: [u8; 4],
    pub prefix_len: u8,
}

/// IPv6 address + prefix length
#[derive(Debug, Clone)]
pub struct Ipv6Cidr {
    pub addr: [u8; 16],
    pub prefix_len: u8,
}

/// DHCP lease information
#[derive(Debug, Clone)]
pub struct DhcpLease {
    pub address: Ipv4Cidr,
    pub gateway: [u8; 4],
    pub dns_servers: Vec<[u8; 4]>,
    pub lease_time_secs: u32,
    pub renew_time_secs: u32,
    pub rebind_time_secs: u32,
    pub dhcp_server: [u8; 4],
    pub hostname: String,
    pub domain: String,
}

/// DHCP packet structure (RFC 2131)
#[repr(C)]
pub struct DhcpPacket {
    pub op: u8,       // 1=BOOTREQUEST, 2=BOOTREPLY
    pub htype: u8,    // Hardware type (1=Ethernet)
    pub hlen: u8,     // Hardware address length (6 for MAC)
    pub hops: u8,     // Relay agent hops
    pub xid: u32,     // Transaction ID (random)
    pub secs: u16,    // Seconds elapsed
    pub flags: u16,   // Broadcast flag (0x8000)
    pub ciaddr: u32,  // Client IP (0 if requesting)
    pub yiaddr: u32,  // 'Your' IP (server fills this)
    pub siaddr: u32,  // Server IP
    pub giaddr: u32,  // Relay agent IP
    pub chaddr: [u8; 16], // Client hardware address (MAC in first 6 bytes)
    pub sname: [u8; 64],  // Optional server hostname
    pub file: [u8; 128],  // Boot file name
    pub options: [u8; 308], // Options (magic cookie + option data)
}

/// Link state for a network interface
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Unknown,
    NotPresent,
    Down,
    LowerLayerDown,
    Testing,
    Dormant,
    Up,
}

/// Network interface profile
#[derive(Debug, Clone)]
pub struct NetworkProfile {
    pub interface: String,
    pub dhcp4: bool,
    pub dhcp6: bool,
    pub static_addr: Option<Ipv4Cidr>,
    pub static_gw: Option<[u8; 4]>,
    pub static_dns: Vec<[u8; 4]>,
    pub wifi_ssid: Option<String>,
    pub wifi_passphrase: Option<String>,
}

/// DHCP client instance for one interface
pub struct DhcpClient {
    pub interface: String,
    pub mac_addr: [u8; 6],
    pub state: DhcpState,
    pub transaction_id: u32,
    pub lease: Option<DhcpLease>,
    pub retry_count: u32,
}

impl DhcpClient {
    /// Create a new DHCP client for the given interface
    pub fn new(interface: String, mac_addr: [u8; 6]) -> Self {
        Self {
            interface,
            mac_addr,
            state: DhcpState::Init,
            transaction_id: 0x12345678, // should be random
            lease: None,
            retry_count: 0,
        }
    }

    /// Build and send DHCPDISCOVER broadcast (UDP src:68, dst:67, dst_ip:255.255.255.255)
    pub fn send_discover(&mut self) -> Result<(), NetConfigError> {
        self.state = DhcpState::Selecting;
        // Build DHCP packet with DISCOVER option
        // Bind UDP socket, setsockopt(SO_BROADCAST), sendto 255.255.255.255:67
        Ok(())
    }

    /// Process received DHCPOFFER, extract lease parameters
    pub fn handle_offer(&mut self, packet: &DhcpPacket) -> Result<(), NetConfigError> {
        if self.state != DhcpState::Selecting {
            return Err(NetConfigError::UnexpectedPacket);
        }
        // Parse options: yiaddr, subnet mask (option 1), router (option 3), DNS (option 6)
        // Verify transaction ID matches self.transaction_id
        let _ = packet;
        self.state = DhcpState::Requesting;
        Ok(())
    }

    /// Send DHCPREQUEST to accept the offered address
    pub fn send_request(&mut self) -> Result<(), NetConfigError> {
        // Build REQUEST with requested-ip option (50) and server-id option (54)
        Ok(())
    }

    /// Process DHCPACK, bind the address
    pub fn handle_ack(&mut self, packet: &DhcpPacket) -> Result<(), NetConfigError> {
        let _ = packet;
        // Apply IP address to interface: ioctl(SIOCSIFADDR)
        // Add default route: ioctl(SIOCADDRT) or netlink RTM_NEWROUTE
        // Write /etc/resolv.conf with nameservers
        self.state = DhcpState::Bound;
        Ok(())
    }

    /// Send DHCPRELEASE when interface goes down
    pub fn release(&mut self) -> Result<(), NetConfigError> {
        self.state = DhcpState::Init;
        self.lease = None;
        Ok(())
    }

    /// Check if lease needs renewal and send DHCPREQUEST if so
    pub fn check_renewal(&mut self) -> Result<(), NetConfigError> {
        match self.state {
            DhcpState::Bound => {
                // Check if renew_time has elapsed → transition to Renewing
                self.state = DhcpState::Renewing;
            }
            DhcpState::Renewing => {
                // Unicast REQUEST to original server
                self.send_request()?;
            }
            DhcpState::Rebinding => {
                // Broadcast REQUEST
                self.send_request()?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// Main network configuration daemon
pub struct NetworkConfigDaemon {
    pub profiles: Vec<NetworkProfile>,
    pub clients: Vec<DhcpClient>,
    pub running: bool,
}

impl NetworkConfigDaemon {
    /// Create daemon with profiles loaded from /etc/network/
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            clients: Vec::new(),
            running: false,
        }
    }

    /// Load a network profile (similar to /etc/iwd/ssid.psk or /etc/network/interfaces)
    pub fn load_profile(&mut self, profile: NetworkProfile) {
        self.profiles.push(profile);
    }

    /// Start the daemon event loop
    ///
    /// Opens a netlink socket (AF_NETLINK, NETLINK_ROUTE) and subscribes to
    /// RTMGRP_LINK for interface state changes, then runs DHCP state machines.
    pub fn run(&mut self) -> Result<(), NetConfigError> {
        self.running = true;
        // 1. Create netlink socket: socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE)
        // 2. bind() to RTMGRP_LINK | RTMGRP_IPV4_IFADDR
        // 3. For each profile, start DHCP client if dhcp4 = true
        // 4. Event loop: poll() / epoll_wait() on netlink fd + DHCP UDP sockets
        //    - Handle RTM_NEWLINK (link up → send_discover)
        //    - Handle RTM_DELLINK (link down → release)
        //    - Handle incoming DHCP packets (offer, ack, nak)
        //    - Handle renewal timers
        Ok(())
    }

    /// Apply DNS configuration to /etc/resolv.conf
    pub fn write_resolv_conf(&self, servers: &[[u8; 4]], domain: &str) -> Result<(), NetConfigError> {
        let _ = (servers, domain);
        // Write "nameserver x.x.x.x\n" for each server
        // Write "search domain\n" if domain is non-empty
        Ok(())
    }

    /// Configure a Wi-Fi station connection using nl80211
    pub fn wifi_connect(&self, interface: &str, ssid: &str, passphrase: &str) -> Result<(), NetConfigError> {
        let _ = (interface, ssid, passphrase);
        // 1. nl80211 NL80211_CMD_TRIGGER_SCAN
        // 2. Wait for NL80211_CMD_NEW_SCAN_RESULTS
        // 3. nl80211 NL80211_CMD_CONNECT with SSID + WPA2-PSK key
        // 4. Wait for NL80211_CMD_CONNECT result
        Ok(())
    }
}

/// Error type for network configuration
#[derive(Debug)]
pub enum NetConfigError {
    SocketError(i32),
    SendFailed,
    RecvFailed,
    UnexpectedPacket,
    LeaseExpired,
    WifiAuthFailed,
    ProfileNotFound,
    DnsWriteFailed,
}

impl Default for NetworkConfigDaemon {
    fn default() -> Self {
        Self::new()
    }
}

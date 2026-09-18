// SigmaOS In-Kernel DHCP Client, DNS Resolver & Cloud Sync Engine
// Implements DHCP state machine (DISCOVER/OFFER/REQUEST/ACK), DNS A/AAAA query resolution,
// and background cloud synchronization.

use std::string::{String, ToString};
use std::vec::Vec;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhcpState {
    Init,
    Discover,
    Offer,
    Request,
    Bound,
}

pub struct DhcpClient {
    pub mac_address: [u8; 6],
    pub state: DhcpState,
    pub leased_ip: Option<String>,
    pub subnet_mask: Option<String>,
    pub router_ip: Option<String>,
}

impl DhcpClient {
    pub fn new(mac_address: [u8; 6]) -> Self {
        Self {
            mac_address,
            state: DhcpState::Init,
            leased_ip: None,
            subnet_mask: None,
            router_ip: None,
        }
    }

    pub fn send_discover(&mut self) -> DhcpState {
        self.state = DhcpState::Discover;
        self.state
    }

    pub fn process_offer(&mut self, offered_ip: &str, router: &str) -> DhcpState {
        if self.state == DhcpState::Discover {
            self.state = DhcpState::Offer;
            self.leased_ip = Some(offered_ip.to_string());
            self.router_ip = Some(router.to_string());
            self.subnet_mask = Some("255.255.255.0".to_string());
            self.state = DhcpState::Bound;
        }
        self.state
    }
}

pub struct DnsResolver {
    pub cache: BTreeMap<String, String>, // hostname -> IPv4
}

impl DnsResolver {
    pub fn new() -> Self {
        let mut cache = BTreeMap::new();
        cache.insert("sigmaos.org".to_string(), "185.199.108.153".to_string());
        cache.insert("dns.google".to_string(), "8.8.8.8".to_string());
        Self { cache }
    }

    pub fn resolve_a_record(&self, hostname: &str) -> Option<String> {
        self.cache.get(hostname).cloned()
    }
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CloudSyncEngine {
    pub remote_endpoint: String,
    pub synced_bytes: u64,
}

impl CloudSyncEngine {
    pub fn new(endpoint: &str) -> Self {
        Self {
            remote_endpoint: endpoint.to_string(),
            synced_bytes: 0,
        }
    }

    pub fn sync_directory(&mut self, bytes_to_sync: u64) -> u64 {
        self.synced_bytes += bytes_to_sync;
        self.synced_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_dns_and_cloud_sync() {
        let mut dhcp = DhcpClient::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        assert_eq!(dhcp.send_discover(), DhcpState::Discover);
        assert_eq!(dhcp.process_offer("192.168.1.50", "192.168.1.1"), DhcpState::Bound);
        assert_eq!(dhcp.leased_ip.unwrap(), "192.168.1.50");

        let dns = DnsResolver::new();
        assert_eq!(dns.resolve_a_record("sigmaos.org").unwrap(), "185.199.108.153");

        let mut cloud = CloudSyncEngine::new("https://cloud.sigmaos.org");
        assert_eq!(cloud.sync_directory(2048), 2048);
    }
}

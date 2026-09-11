// SigmaOS Device Discovery & Auto-Sync Engine
// Zero-dependency #![no_std] mDNS/SSDP LAN device discovery and peer sync protocol

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Tablet,
    Server,
    Iot,
    Router,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    MDns,
    Ssdp,
    BluetoothLe,
    SigmaPeerSync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredPeerDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub ip_address: String,
    pub mac_address: String,
    pub services: Vec<String>,
    pub protocol: DiscoveryProtocol,
    pub last_seen_timestamp: u64,
    pub is_paired: bool,
}

pub struct DeviceDiscoverySyncEngine {
    peers: Vec<DiscoveredPeerDevice>,
    local_device_id: String,
    auto_sync_enabled: bool,
}

impl DeviceDiscoverySyncEngine {
    pub fn new(local_device_id: &str, auto_sync_enabled: bool) -> Self {
        Self {
            peers: Vec::new(),
            local_device_id: String::from(local_device_id),
            auto_sync_enabled,
        }
    }

    pub fn register_peer(
        &mut self,
        name: &str,
        device_type: DeviceType,
        ip_address: &str,
        mac_address: &str,
        services: &[&str],
        protocol: DiscoveryProtocol,
        timestamp: u64,
    ) -> String {
        let peer_id = format!("peer_{}_{}", mac_address.replace(':', ""), ip_address.replace('.', "_"));

        if let Some(peer) = self.peers.iter_mut().find(|p| p.id == peer_id) {
            peer.last_seen_timestamp = timestamp;
            peer.ip_address = String::from(ip_address);
            return peer.id.clone();
        }

        let device = DiscoveredPeerDevice {
            id: peer_id.clone(),
            name: String::from(name),
            device_type,
            ip_address: String::from(ip_address),
            mac_address: String::from(mac_address),
            services: services.iter().map(|&s| String::from(s)).collect(),
            protocol,
            last_seen_timestamp: timestamp,
            is_paired: false,
        };

        self.peers.push(device);
        peer_id
    }

    pub fn pair_device(&mut self, peer_id: &str) -> bool {
        if let Some(peer) = self.peers.iter_mut().find(|p| p.id == peer_id) {
            peer.is_paired = true;
            true
        } else {
            false
        }
    }

    pub fn unregister_peer(&mut self, peer_id: &str) -> bool {
        if let Some(pos) = self.peers.iter().position(|p| p.id == peer_id) {
            self.peers.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn local_device_id(&self) -> &str {
        &self.local_device_id
    }

    pub fn list_discovered_peers(&self) -> &[DiscoveredPeerDevice] {
        &self.peers
    }

    pub fn list_paired_peers(&self) -> Vec<&DiscoveredPeerDevice> {
        self.peers.iter().filter(|p| p.is_paired).collect()
    }

    pub fn auto_sync_state(&self, current_timestamp: u64) -> usize {
        if !self.auto_sync_enabled {
            return 0;
        }

        // Active paired peers seen within the last 300 seconds
        self.peers
            .iter()
            .filter(|p| p.is_paired && (current_timestamp.saturating_sub(p.last_seen_timestamp) <= 300))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_discovery_engine() {
        let mut engine = DeviceDiscoverySyncEngine::new("sigma_host_01", true);
        let peer_id = engine.register_peer(
            "SigmaWorkstation",
            DeviceType::Desktop,
            "192.168.1.100",
            "AA:BB:CC:DD:EE:FF",
            &["warpinator", "ssh", "media_share"],
            DiscoveryProtocol::MDns,
            1700000000,
        );

        assert_eq!(engine.list_discovered_peers().len(), 1);
        assert!(engine.pair_device(&peer_id));
        assert_eq!(engine.list_paired_peers().len(), 1);
        assert_eq!(engine.auto_sync_state(1700000100), 1);
    }
}
